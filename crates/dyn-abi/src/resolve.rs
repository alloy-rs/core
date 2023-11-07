//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use crate::{DynSolEvent, DynSolType, Result};
use alloc::vec::Vec;
use alloy_json_abi::{EventParam, Param};
use alloy_sol_type_parser::{
    Error as ParserError, ParameterSpecifier, Parameters, RootType, TupleSpecifier, TypeSpecifier,
    TypeStem,
};

#[cfg(feature = "eip712")]
use alloy_json_abi::InternalType;

/// Resolve a type into a [`DynSolEvent`].
///
/// The `ResolveSolEvent` trait is implemented by types that can be resolved
/// into Solidity-style event descriptors (i.e. a list of indexed parameters and
/// a tuple of non-indexed parameters).
///
/// This trait is implemented for [`alloy_json_abi::Event`].
pub trait ResolveSolEvent {
    /// Resolve the type into a [`DynSolEvent`].
    fn resolve(&self) -> Result<DynSolEvent>;
}

/// Resolve a type into a [`DynSolType`].
///
/// The `ResolveSolType` trait is implemented by types that can be resolved into
/// a [`DynSolType`]. ABI and related systems have many different ways of
/// encoding solidity types. This trait provides a single pattern for resolving
/// those encodings into solidity types.
///
/// ABI and related systems have many different ways of encoding Solidity types.
/// This trait provides a single pattern for resolving those encodings into
/// Solidity types.
///
/// This trait is implemented for all the [`alloy_sol_type_parser`] types, the
/// [`Param`] and [`EventParam`] structs, and [`str`].
///
/// The [`str`] implementation calls [`DynSolType::parse`].
///
/// # Examples
///
/// ```
/// # use alloy_dyn_abi::{DynSolType, ResolveSolType};
/// # use alloy_sol_type_parser::{RootType, TypeSpecifier};
/// let my_ty = TypeSpecifier::parse("bool")?.resolve()?;
/// assert_eq!(my_ty, DynSolType::Bool);
///
/// let my_ty = RootType::parse("uint256")?.resolve()?;
/// assert_eq!(my_ty, DynSolType::Uint(256));
///
/// assert_eq!("bytes32".resolve()?, DynSolType::FixedBytes(32));
/// # Ok::<_, alloy_dyn_abi::Error>(())
/// ```
pub trait ResolveSolType {
    /// Resolve this object into a [`DynSolType`].
    ///
    /// See the [trait documentation](ResolveSolType) for more details.
    fn resolve(&self) -> Result<DynSolType>;
}

impl ResolveSolType for str {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        DynSolType::parse(self)
    }
}

impl ResolveSolType for RootType<'_> {
    fn resolve(&self) -> Result<DynSolType> {
        match self.span() {
            "address" => Ok(DynSolType::Address),
            "function" => Ok(DynSolType::Function),
            "bool" => Ok(DynSolType::Bool),
            "string" => Ok(DynSolType::String),
            "bytes" => Ok(DynSolType::Bytes),
            "uint" => Ok(DynSolType::Uint(256)),
            "int" => Ok(DynSolType::Int(256)),
            name => {
                if let Some(sz) = name.strip_prefix("bytes") {
                    if let Ok(sz) = sz.parse() {
                        if sz != 0 && sz <= 32 {
                            return Ok(DynSolType::FixedBytes(sz));
                        }
                    }
                    return Err(ParserError::invalid_size(name).into());
                }

                // fast path both integer types
                let (s, is_uint) =
                    if let Some(s) = name.strip_prefix('u') { (s, true) } else { (name, false) };

                if let Some(sz) = s.strip_prefix("int") {
                    if let Ok(sz) = sz.parse() {
                        if sz != 0 && sz <= 256 && sz % 8 == 0 {
                            return if is_uint {
                                Ok(DynSolType::Uint(sz))
                            } else {
                                Ok(DynSolType::Int(sz))
                            };
                        }
                    }
                    Err(ParserError::invalid_size(name).into())
                } else {
                    Err(ParserError::invalid_type_string(name).into())
                }
            }
        }
    }
}

impl ResolveSolType for TupleSpecifier<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        tuple(&self.types).map(DynSolType::Tuple)
    }
}

impl ResolveSolType for TypeStem<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        match self {
            Self::Root(root) => root.resolve(),
            Self::Tuple(tuple) => tuple.resolve(),
        }
    }
}

impl ResolveSolType for TypeSpecifier<'_> {
    fn resolve(&self) -> Result<DynSolType> {
        self.stem.resolve().map(|ty| ty.array_wrap_from_iter(self.sizes.iter().copied()))
    }
}

impl ResolveSolType for ParameterSpecifier<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        self.ty.resolve()
    }
}

impl ResolveSolType for Parameters<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        tuple(&self.params).map(DynSolType::Tuple)
    }
}

impl ResolveSolType for Param {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        resolve_param(
            &self.ty,
            &self.components,
            #[cfg(feature = "eip712")]
            self.internal_type(),
        )
    }
}

impl ResolveSolType for EventParam {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        resolve_param(
            &self.ty,
            &self.components,
            #[cfg(feature = "eip712")]
            self.internal_type(),
        )
    }
}

fn resolve_param(
    ty: &str,
    components: &[Param],
    #[cfg(feature = "eip712")] it: Option<&InternalType>,
) -> Result<DynSolType> {
    let ty = TypeSpecifier::parse(ty)?;

    // type is simple, and we can resolve it via the specifier
    if components.is_empty() {
        return ty.resolve();
    }

    // type is complex
    let tuple = tuple(components)?;

    #[cfg(feature = "eip712")]
    let resolved = if let Some((_, name)) = it.and_then(|i| i.as_struct()) {
        DynSolType::CustomStruct {
            // skip array sizes, since we have them already from parsing `ty`
            name: name.split('[').next().unwrap().into(),
            prop_names: components.iter().map(|c| c.name.clone()).collect(),
            tuple,
        }
    } else {
        DynSolType::Tuple(tuple)
    };

    #[cfg(not(feature = "eip712"))]
    let resolved = DynSolType::Tuple(tuple);

    Ok(resolved.array_wrap_from_iter(ty.sizes))
}

fn tuple<T: ResolveSolType>(slice: &[T]) -> Result<Vec<DynSolType>> {
    let mut types = Vec::with_capacity(slice.len());
    for ty in slice {
        types.push(ty.resolve()?);
    }
    Ok(types)
}

macro_rules! deref_impls {
    ($($(#[$attr:meta])* [$($gen:tt)*] $t:ty),+ $(,)?) => {$(
        $(#[$attr])*
        impl<$($gen)*> ResolveSolType for $t {
            #[inline]
            fn resolve(&self) -> Result<DynSolType> {
                (**self).resolve()
            }
        }
    )+};
}

deref_impls! {
    [] alloc::string::String,
    [T: ?Sized + ResolveSolType] &T,
    [T: ?Sized + ResolveSolType] &mut T,
    [T: ?Sized + ResolveSolType] alloc::boxed::Box<T>,
    [T: ?Sized + alloc::borrow::ToOwned + ResolveSolType] alloc::borrow::Cow<'_, T>,
    [T: ?Sized + ResolveSolType] alloc::rc::Rc<T>,
    [T: ?Sized + ResolveSolType] alloc::sync::Arc<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    fn parse(s: &str) -> Result<DynSolType> {
        s.parse()
    }

    #[test]
    fn extra_close_parens() {
        parse("(bool,uint256))").unwrap_err();
        parse("bool,uint256))").unwrap_err();
        parse("bool,uint256)").unwrap_err();
    }

    #[test]
    fn extra_open_parents() {
        parse("((bool,uint256)").unwrap_err();
        parse("((bool,uint256").unwrap_err();
        parse("(bool,uint256").unwrap_err();
    }

    #[test]
    fn it_parses_tuples() {
        assert_eq!(parse("(bool,)"), Ok(DynSolType::Tuple(vec![DynSolType::Bool])));
        assert_eq!(
            parse("(uint256,uint256)"),
            Ok(DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)]))
        );
        assert_eq!(
            parse("(uint256,uint256)[2]"),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])),
                2
            ))
        );
    }

    #[test]
    fn nested_tuples() {
        assert_eq!(
            parse("(bool,(uint256,uint256))"),
            Ok(DynSolType::Tuple(vec![
                DynSolType::Bool,
                DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
            ]))
        );
        assert_eq!(
            parse("(((bool),),)"),
            Ok(DynSolType::Tuple(vec![DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Bool
            ])])]))
        );
    }

    #[test]
    fn empty_tuples() {
        assert_eq!(parse("()"), Ok(DynSolType::Tuple(vec![])));
        assert_eq!(
            parse("((),())"),
            Ok(DynSolType::Tuple(vec![DynSolType::Tuple(vec![]), DynSolType::Tuple(vec![])]))
        );
        assert_eq!(
            parse("((()))"),
            Ok(DynSolType::Tuple(vec![DynSolType::Tuple(vec![DynSolType::Tuple(vec![])])]))
        );
    }

    #[test]
    fn it_parses_simple_types() {
        assert_eq!(parse("uint256"), Ok(DynSolType::Uint(256)));
        assert_eq!(parse("uint8"), Ok(DynSolType::Uint(8)));
        assert_eq!(parse("uint"), Ok(DynSolType::Uint(256)));
        assert_eq!(parse("address"), Ok(DynSolType::Address));
        assert_eq!(parse("bool"), Ok(DynSolType::Bool));
        assert_eq!(parse("string"), Ok(DynSolType::String));
        assert_eq!(parse("bytes"), Ok(DynSolType::Bytes));
        assert_eq!(parse("bytes32"), Ok(DynSolType::FixedBytes(32)));
    }

    #[test]
    fn it_parses_complex_solidity_types() {
        assert_eq!(parse("uint256[]"), Ok(DynSolType::Array(Box::new(DynSolType::Uint(256)))));
        assert_eq!(
            parse("uint256[2]"),
            Ok(DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2))
        );
        assert_eq!(
            parse("uint256[2][3]"),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)),
                3
            ))
        );
        assert_eq!(
            parse("uint256[][][]"),
            Ok(DynSolType::Array(Box::new(DynSolType::Array(Box::new(DynSolType::Array(
                Box::new(DynSolType::Uint(256))
            ))))))
        );

        assert_eq!(
            parse("tuple(address,bytes,(bool,(string,uint256)[][3]))[2]"),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Address,
                    DynSolType::Bytes,
                    DynSolType::Tuple(vec![
                        DynSolType::Bool,
                        DynSolType::FixedArray(
                            Box::new(DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                                DynSolType::String,
                                DynSolType::Uint(256)
                            ])))),
                            3
                        ),
                    ]),
                ])),
                2
            ))
        );
    }

    #[test]
    fn library_enum_workaround() {
        assert_eq!(parse("MyLibrary.MyEnum"), Ok(DynSolType::Uint(8)));
        assert_eq!(
            parse("MyLibrary.MyEnum[]"),
            Ok(DynSolType::Array(Box::new(DynSolType::Uint(8))))
        );
    }
}
