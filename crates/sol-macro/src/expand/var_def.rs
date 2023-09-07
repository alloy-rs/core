//! State variable ([`VariableDefinition`]) expansion.

use super::ExpCtxt;
use ast::{ItemFunction, ParameterList, Spanned, Type, VariableDeclaration, VariableDefinition};
use proc_macro2::TokenStream;
use syn::{Error, Result};

/// Expands a [`VariableDefinition`].
///
/// See [`ItemFunction::from_variable_definition`].
pub(super) fn expand(cx: &ExpCtxt<'_>, var_def: &VariableDefinition) -> Result<TokenStream> {
    // only expand public or external state variables
    if !var_def
        .attributes
        .visibility()
        .map_or(false, |v| v.is_public() || v.is_external())
    {
        return Ok(TokenStream::new())
    }

    let mut function = ItemFunction::from_variable_definition(var_def.clone());
    expand_returns(cx, &mut function)?;
    super::function::expand(cx, &function)
}

/// Expands return-position custom types.
fn expand_returns(cx: &ExpCtxt<'_>, f: &mut ItemFunction) -> Result<()> {
    let returns = f
        .returns
        .as_mut()
        .expect("generated getter function with no returns");
    let ret = returns.returns.first_mut().unwrap();
    if !ret.ty.has_custom_simple() {
        return Ok(())
    }

    let mut ty = &ret.ty;

    // resolve if custom
    if let Type::Custom(name) = ty {
        ty = cx.custom_type(name);
    }

    // skip if not tuple with complex types
    let Type::Tuple(tup) = ty else { return Ok(()) };
    if !tup.types.iter().any(type_is_complex) {
        return Ok(())
    }

    // retain only non-complex types
    // TODO: assign return types' names from the original struct
    let mut new_returns = ParameterList::new();
    for p in tup.types.pairs() {
        let (ty, comma) = p.into_tuple();
        if !type_is_complex(ty) {
            new_returns.push_value(VariableDeclaration::new(ty.clone()));
            if let Some(comma) = comma {
                new_returns.push_punct(*comma);
            }
        }
    }

    // all types were complex, Solidity doesn't accept this
    if new_returns.is_empty() {
        return Err(Error::new(f.name().span(), "invalid state variable type"))
    }

    returns.returns = new_returns;
    Ok(())
}

/// Returns `true` if a type is complex for the purposes of state variable
/// getters.
///
/// Technically tuples are also complex if they contain complex types, but only
/// at the first "depth" level.
///
/// Here, `mapA` is fine but `mapB` throws an error; you can test that pushing
/// and reading to `mapA` works fine (last checked at Solc version `0.8.21`):
///
/// ```solidity
/// contract Complex {
///     struct A {
///         B b;
///     }
///     struct B {
///         uint[] arr;
///     }
///
///     mapping(uint => A) public mapA;
///
///     function pushValueA(uint idx, uint val) public {
///         mapA[idx].b.arr.push(val);
///     }
///
///     mapping(uint => B) public mapB;
///
///     function pushValueB(uint idx, uint val) public {
///         mapB[idx].arr.push(val);
///     }
/// }
/// ```
fn type_is_complex(ty: &Type) -> bool {
    matches!(ty, Type::Mapping(_) | Type::Array(_))
}
