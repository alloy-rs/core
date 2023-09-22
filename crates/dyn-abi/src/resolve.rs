//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use crate::{DynSolType, Result};
use alloc::vec::Vec;
use alloy_json_abi::{EventParam, InternalType, Param};
use alloy_sol_type_parser::{
    Error as TypeStrError, RootType, TupleSpecifier, TypeSpecifier, TypeStem,
};

/// The ResolveSolType trait is implemented by types that can be resolved into
/// a [`DynSolType`]. ABI and related systems have many different ways of
/// encoding solidity types. This trait provides a single pattern for resolving
/// those encodings into solidity types.
///
/// This trait is implemented for [`RootType`], [`TupleSpecifier`],
/// [`TypeStem`], and [`TypeSpecifier`], as well as the [`EventParam`] and
/// [`Param`] structs. The impl on `&str` parses a [`TypeSpecifier`] from the
/// string and resolves it.
///
/// ## Example
///
/// ```
/// # use alloy_dyn_abi::{DynSolType, ResolveSolType};
/// # use alloy_sol_type_parser::{RootType, TypeSpecifier};
/// let my_ty = TypeSpecifier::try_from("bool")?.resolve()?;
/// assert_eq!(my_ty, DynSolType::Bool);
///
/// let my_ty = RootType::try_from("uint256")?.resolve()?;
/// assert_eq!(my_ty, DynSolType::Uint(256));
///
/// assert_eq!("bytes32".resolve()?, DynSolType::FixedBytes(32));
/// # Ok::<_, alloy_dyn_abi::Error>(())
/// ```
pub trait ResolveSolType {
    /// Resolve this object into a [`DynSolType`].
    fn resolve(&self) -> Result<DynSolType>;
}

impl ResolveSolType for str {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        TypeSpecifier::parse(self)?.resolve()
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
                            return Ok(DynSolType::FixedBytes(sz))
                        }
                    }
                    return Err(TypeStrError::invalid_size(name).into())
                }

                // fast path both integer types
                let (s, is_uint) = if let Some(s) = name.strip_prefix('u') {
                    (s, true)
                } else {
                    (name, false)
                };

                if let Some(sz) = s.strip_prefix("int") {
                    if let Ok(sz) = sz.parse() {
                        if sz != 0 && sz <= 256 && sz % 8 == 0 {
                            return if is_uint {
                                Ok(DynSolType::Uint(sz))
                            } else {
                                Ok(DynSolType::Int(sz))
                            }
                        }
                    }
                    Err(TypeStrError::invalid_size(name).into())
                } else {
                    Err(TypeStrError::invalid_type_string(name).into())
                }
            }
        }
    }
}

impl ResolveSolType for TupleSpecifier<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        self.types
            .iter()
            .map(TypeSpecifier::resolve)
            .collect::<Result<Vec<_>, _>>()
            .map(DynSolType::Tuple)
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
        self.stem
            .resolve()
            .map(|ty| ty.array_wrap_from_iter(self.sizes.iter().copied()))
    }
}

impl ResolveSolType for Param {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        resolve_param(&self.ty, &self.components, self.internal_type())
    }
}

impl ResolveSolType for EventParam {
    #[inline]
    fn resolve(&self) -> Result<DynSolType> {
        resolve_param(&self.ty, &self.components, self.internal_type())
    }
}

fn resolve_param(ty: &str, components: &[Param], _it: Option<&InternalType>) -> Result<DynSolType> {
    let ty = TypeSpecifier::parse(ty)?;

    // type is simple, and we can resolve it via the specifier
    if components.is_empty() {
        return ty.resolve()
    }

    // type is complex
    let tuple = components
        .iter()
        .map(Param::resolve)
        .collect::<Result<Vec<_>, _>>()?;

    #[cfg(feature = "eip712")]
    let resolved = if let Some((_, name)) = _it.and_then(|i| i.as_struct()) {
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

macro_rules! deref_impl {
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

deref_impl! {
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
        parse("bool,uint256))").unwrap_err();
    }

    #[test]
    fn extra_open_parents() {
        parse("(bool,uint256").unwrap_err();
    }

    #[test]
    fn it_parses_tuples() {
        assert_eq!(
            parse("(bool,)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Bool])
        );
        assert_eq!(
            parse("(uint256,uint256)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
        );
        assert_eq!(
            parse("(uint256,uint256)[2]").unwrap(),
            DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Uint(256),
                    DynSolType::Uint(256)
                ])),
                2
            )
        );
    }

    #[test]
    fn nested_tuples() {
        assert_eq!(
            parse("(bool,(uint256,uint256))").unwrap(),
            DynSolType::Tuple(vec![
                DynSolType::Bool,
                DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
            ])
        );
        assert_eq!(
            parse("(((bool),),)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Bool
            ])])])
        );
    }

    #[test]
    fn empty_tuples() {
        assert_eq!(parse("()").unwrap(), DynSolType::Tuple(vec![]));
        assert_eq!(
            parse("((),())").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Tuple(vec![]), DynSolType::Tuple(vec![])])
        );
        assert_eq!(
            parse("((()))"),
            Ok(DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Tuple(vec![])
            ])]))
        );
    }

    #[test]
    fn it_parses_simple_types() {
        assert_eq!(parse("uint256").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("uint8").unwrap(), DynSolType::Uint(8));
        assert_eq!(parse("uint").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("address").unwrap(), DynSolType::Address);
        assert_eq!(parse("bool").unwrap(), DynSolType::Bool);
        assert_eq!(parse("string").unwrap(), DynSolType::String);
        assert_eq!(parse("bytes").unwrap(), DynSolType::Bytes);
        assert_eq!(parse("bytes32").unwrap(), DynSolType::FixedBytes(32));
    }

    #[test]
    fn it_parses_complex_solidity_types() {
        assert_eq!(
            parse("uint256[]").unwrap(),
            DynSolType::Array(Box::new(DynSolType::Uint(256)))
        );
        assert_eq!(
            parse("uint256[2]").unwrap(),
            DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)
        );
        assert_eq!(
            parse("uint256[2][3]").unwrap(),
            DynSolType::FixedArray(
                Box::new(DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)),
                3
            )
        );
        assert_eq!(
            parse("uint256[][][]").unwrap(),
            DynSolType::Array(Box::new(DynSolType::Array(Box::new(DynSolType::Array(
                Box::new(DynSolType::Uint(256))
            )))))
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
    fn try_basic_solidity() {
        assert_eq!(
            TypeSpecifier::try_from("uint256")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("uint256[]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("(uint256,uint256)")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("(uint256,uint256)[2]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("tuple(uint256,uint256)")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("tuple(address,bytes,(bool,(string,uint256)[][3]))[2]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
    }

    #[test]
    fn not_basic_solidity() {
        assert_eq!(
            TypeSpecifier::try_from("MyStruct")
                .unwrap()
                .try_basic_solidity(),
            Err(TypeStrError::invalid_type_string("MyStruct"))
        );
    }
}
