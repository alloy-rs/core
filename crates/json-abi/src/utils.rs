use crate::{EventParam, Param};
use alloc::{string::String, vec::Vec};
use alloy_primitives::Selector;
use alloy_sol_type_parser::{ParameterSpecifier, TypeSpecifier, TypeStem};
use core::{fmt::Write, num::NonZeroUsize};

/// Capacity to allocate per [Param].
const PARAM: usize = 32;

macro_rules! validate_identifier {
    ($name:expr) => {
        if !$name.is_empty() && !alloy_sol_type_parser::is_valid_identifier($name) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str($name),
                &"a valid Solidity identifier",
            ));
        }
    };
}
pub(crate) use validate_identifier;

/// `($($params),*)`
macro_rules! signature {
    ($inputs:expr, $preimage:expr) => {
        $preimage.push('(');
        for (i, input) in $inputs.iter().enumerate() {
            if i > 0 {
                $preimage.push(',');
            }
            input.selector_type_raw($preimage);
        }
        $preimage.push(')');
    };
}

/// `$name($($inputs),*)($($outputs),*)`
pub(crate) fn signature(name: &str, inputs: &[Param], outputs: Option<&[Param]>) -> String {
    let parens = 2 + outputs.is_some() as usize * 2;
    let n_outputs = outputs.map(<[_]>::len).unwrap_or(0);
    let cap = name.len() + parens + (inputs.len() + n_outputs) * PARAM;
    let mut preimage = String::with_capacity(cap);
    preimage.push_str(name);
    signature_raw(inputs, &mut preimage);
    if let Some(outputs) = outputs {
        signature_raw(outputs, &mut preimage);
    }
    preimage
}

/// `($($params),*)`
pub(crate) fn signature_raw(params: &[Param], preimage: &mut String) {
    signature!(params, preimage);
}

/// `$name($($inputs),*)`
pub(crate) fn event_signature(name: &str, inputs: &[EventParam]) -> String {
    let mut preimage = String::with_capacity(name.len() + 2 + inputs.len() * PARAM);
    preimage.push_str(name);
    signature!(inputs, &mut preimage);
    preimage
}

/// `keccak256(preimage)[..4]`
pub(crate) fn selector(preimage: &str) -> Selector {
    // SAFETY: splitting an array
    unsafe {
        alloy_primitives::keccak256(preimage.as_bytes())
            .0
            .get_unchecked(..4)
            .try_into()
            .unwrap_unchecked()
    }
}

type Ret<T> = alloy_sol_type_parser::Result<(String, Vec<T>, Vec<T>, bool)>;

#[inline]
pub(crate) fn parse_sig<const O: bool>(s: &str) -> Ret<Param> {
    alloy_sol_type_parser::utils::parse_signature::<O, _, _>(s, |p| mk_param(p.name, p.ty))
}

#[inline]
pub(crate) fn parse_event_sig(s: &str) -> Ret<EventParam> {
    alloy_sol_type_parser::utils::parse_signature::<false, _, _>(s, mk_eparam)
}

pub(crate) fn mk_param(name: Option<&str>, ty: TypeSpecifier<'_>) -> Param {
    let name = name.unwrap_or_default().into();
    let internal_type = None;
    match ty.stem {
        TypeStem::Root(s) => {
            Param { name, ty: ty_string(s.span(), &ty.sizes), components: vec![], internal_type }
        }
        TypeStem::Tuple(t) => Param {
            name,
            ty: ty_string("tuple", &ty.sizes),
            components: t.types.into_iter().map(|ty| mk_param(None, ty)).collect(),
            internal_type,
        },
    }
}

pub(crate) fn mk_eparam(spec: ParameterSpecifier<'_>) -> EventParam {
    let p = mk_param(spec.name, spec.ty);
    EventParam {
        name: p.name,
        ty: p.ty,
        indexed: spec.indexed,
        components: p.components,
        internal_type: p.internal_type,
    }
}

fn ty_string(s: &str, sizes: &[Option<NonZeroUsize>]) -> String {
    let mut ty = String::with_capacity(s.len() + sizes.len() * 4);
    ty.push_str(s);
    for size in sizes {
        ty.push('[');
        if let Some(size) = size {
            write!(ty, "{size}").unwrap();
        }
        ty.push(']');
    }
    ty
}

#[cfg(test)]
mod tests {
    use super::*;

    fn param(kind: &str) -> Param {
        param2(kind, "param")
    }

    fn param2(kind: &str, name: &str) -> Param {
        Param { ty: kind.into(), name: name.into(), internal_type: None, components: vec![] }
    }

    fn eparam(kind: &str) -> EventParam {
        EventParam {
            name: "param".into(),
            ty: kind.into(),
            internal_type: None,
            components: vec![],
            indexed: false,
        }
    }

    fn params(components: impl IntoIterator<Item = &'static str>) -> Param {
        let components = components.into_iter().map(param).collect();
        crate::Param { name: "param".into(), ty: "tuple".into(), internal_type: None, components }
    }

    #[test]
    fn test_signature() {
        assert_eq!(signature("foo", &[], None), "foo()");
        assert_eq!(signature("bar", &[param("bool")], None), "bar(bool)");
        assert_eq!(
            signature("foo", &[param("bytes"), param("bytes32")], None),
            "foo(bytes,bytes32)"
        );
        assert_eq!(
            signature("foo", &[param("int"), params(["uint[]"]), param("string")], None),
            "foo(int,(uint[]),string)"
        );

        assert_eq!(signature("foo", &[], Some(&[])), "foo()()");
        assert_eq!(signature("foo", &[param("a")], Some(&[param("b")])), "foo(a)(b)");
        assert_eq!(
            signature("foo", &[param("a"), param("c")], Some(&[param("b"), param("d")])),
            "foo(a,c)(b,d)"
        );
    }

    #[test]
    fn test_event_signature() {
        assert_eq!(event_signature("foo", &[]), "foo()");
        assert_eq!(event_signature("foo", &[eparam("bool")]), "foo(bool)");
        assert_eq!(event_signature("foo", &[eparam("bool"), eparam("string")]), "foo(bool,string)");
    }

    #[test]
    fn test_item_parse() {
        assert_eq!(parse_sig::<true>("foo()"), Ok(("foo".into(), vec![], vec![], false)));
        assert_eq!(parse_sig::<true>("foo()()"), Ok(("foo".into(), vec![], vec![], false)));
        assert_eq!(parse_sig::<true>("foo(,) \t ()"), Ok(("foo".into(), vec![], vec![], false)));
        assert_eq!(parse_sig::<true>("foo(,)  (,)"), Ok(("foo".into(), vec![], vec![], false)));

        assert_eq!(parse_sig::<false>("foo()"), Ok(("foo".into(), vec![], vec![], false)));
        parse_sig::<false>("foo()()").unwrap_err();
        parse_sig::<false>("foo(,)()").unwrap_err();
        parse_sig::<false>("foo(,)(,)").unwrap_err();

        assert_eq!(parse_sig::<false>("foo()anonymous"), Ok(("foo".into(), vec![], vec![], true)));
        assert_eq!(
            parse_sig::<false>("foo()\t anonymous"),
            Ok(("foo".into(), vec![], vec![], true))
        );

        assert_eq!(parse_sig::<true>("foo()anonymous"), Ok(("foo".into(), vec![], vec![], true)));
        assert_eq!(
            parse_sig::<true>("foo()\t anonymous"),
            Ok(("foo".into(), vec![], vec![], true))
        );

        assert_eq!(
            parse_sig::<true>("foo() \t ()anonymous"),
            Ok(("foo".into(), vec![], vec![], true))
        );
        assert_eq!(parse_sig::<true>("foo()()anonymous"), Ok(("foo".into(), vec![], vec![], true)));
        assert_eq!(
            parse_sig::<true>("foo()()\t anonymous"),
            Ok(("foo".into(), vec![], vec![], true))
        );

        assert_eq!(
            parse_sig::<false>("foo(uint256 param)"),
            Ok(("foo".into(), vec![param("uint256")], vec![], false))
        );
        assert_eq!(
            parse_sig::<false>("bar(uint256 param)"),
            Ok(("bar".into(), vec![param("uint256")], vec![], false))
        );
        assert_eq!(
            parse_sig::<false>("baz(uint256 param, bool param)"),
            Ok(("baz".into(), vec![param("uint256"), param("bool")], vec![], false))
        );

        assert_eq!(
            parse_sig::<true>("f(a b)(c d)"),
            Ok(("f".into(), vec![param2("a", "b")], vec![param2("c", "d")], false))
        );

        assert_eq!(
            parse_sig::<true>("toString(uint number)(string s)"),
            Ok((
                "toString".into(),
                vec![param2("uint256", "number")],
                vec![param2("string", "s")],
                false
            ))
        )
    }
}
