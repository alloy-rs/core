use crate::{EventParam, Param, StateMutability};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use alloy_primitives::Selector;
use core::{fmt::Write, num::NonZeroUsize};
use parser::{ParameterSpecifier, TypeSpecifier, TypeStem};

/// Capacity to allocate per [Param].
const PARAM: usize = 32;

macro_rules! validate_identifier {
    ($name:expr) => {
        if !$name.is_empty() && !parser::is_valid_identifier($name) {
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

macro_rules! event_full_signature {
    ($inputs:expr, $preimage:expr) => {
        $preimage.push('(');
        for (i, input) in $inputs.iter().enumerate() {
            if i > 0 {
                $preimage.push(',');
                $preimage.push(' ');
            }
            input.full_selector_type_raw($preimage);
            if input.indexed {
                $preimage.push_str(" indexed");
            }
            if !input.name.is_empty() {
                $preimage.push(' ');
                $preimage.push_str(&input.name);
            }
        }
        $preimage.push(')');
    };
}

macro_rules! full_signature {
    ($inputs:expr, $preimage:expr) => {
        $preimage.push('(');
        for (i, input) in $inputs.iter().enumerate() {
            if i > 0 {
                $preimage.push(',');
                $preimage.push(' ');
            }
            input.full_selector_type_raw($preimage);
            if !input.name.is_empty() {
                $preimage.push(' ');
                $preimage.push_str(&input.name);
            }
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

pub(crate) fn full_signature(
    name: &str,
    inputs: &[Param],
    outputs: Option<&[Param]>,
    state_mutability: StateMutability,
) -> String {
    let parens = 2 + outputs.is_some() as usize * 2;
    let n_outputs = outputs.map(<[_]>::len).unwrap_or(0);
    let mut state_mutability_str = format!(" {}", state_mutability.as_str().unwrap_or_default());
    if state_mutability_str.trim().is_empty() {
        state_mutability_str = "".to_string();
    }
    let cap = "function ".len()
        + name.len()
        + parens
        + (inputs.len() + n_outputs) * PARAM
        + state_mutability_str.len();
    let mut preimage = String::with_capacity(cap);

    preimage.push_str("function ");
    preimage.push_str(name);
    full_signature_raw(inputs, &mut preimage);
    preimage.push_str(&state_mutability_str);
    if let Some(outputs) = outputs {
        if !outputs.is_empty() {
            preimage.push_str(" returns ");
            full_signature_raw(outputs, &mut preimage);
        }
    }
    preimage
}

/// `($($params),*)`
pub(crate) fn signature_raw(params: &[Param], preimage: &mut String) {
    signature!(params, preimage);
}

pub(crate) fn full_signature_raw(params: &[Param], preimage: &mut String) {
    full_signature!(params, preimage);
}

/// `$name($($inputs),*)`
pub(crate) fn event_signature(name: &str, inputs: &[EventParam]) -> String {
    let mut preimage = String::with_capacity(name.len() + 2 + inputs.len() * PARAM);
    preimage.push_str(name);
    signature!(inputs, &mut preimage);
    preimage
}

/// `$name($($inputs indexed names),*)`
pub(crate) fn event_full_signature(name: &str, inputs: &[EventParam]) -> String {
    let mut preimage =
        String::with_capacity("event ".len() + name.len() + 2 + inputs.len() * PARAM);
    preimage.push_str("event ");
    preimage.push_str(name);
    event_full_signature!(inputs, &mut preimage);
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

pub(crate) type ParseSigTuple<T> = (String, Vec<T>, Vec<T>, bool);
pub(crate) type ParseSigResult<T> = parser::Result<ParseSigTuple<T>>;

/// Strips `prefix` from `s` before parsing with `parser`. `prefix` must be followed by whitespace.
pub(crate) fn parse_maybe_prefixed<F: FnOnce(&str) -> R, R>(
    mut s: &str,
    prefix: &str,
    parser: F,
) -> R {
    if let Some(stripped) = s.strip_prefix(prefix) {
        if stripped.starts_with(char::is_whitespace) {
            s = stripped.trim_start();
        }
    }
    parser(s)
}

#[inline]
pub(crate) fn parse_sig<const O: bool>(s: &str) -> ParseSigResult<Param> {
    parser::utils::parse_signature::<O, _, _>(s, |p| mk_param(p.name, p.ty))
}

#[inline]
pub(crate) fn parse_event_sig(s: &str) -> ParseSigResult<EventParam> {
    parser::utils::parse_signature::<false, _, _>(s, mk_eparam)
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
        eparam_with_indexed(kind, "param", false)
    }

    fn eparam2(kind: &str, name: &str, indexed: bool) -> EventParam {
        eparam_with_indexed(kind, name, indexed)
    }

    fn eparam_with_indexed(kind: &str, name: &str, indexed: bool) -> EventParam {
        EventParam {
            name: name.into(),
            ty: kind.into(),
            internal_type: None,
            components: vec![],
            indexed,
        }
    }

    fn params(components: impl IntoIterator<Item = &'static str>) -> Param {
        let components = components.into_iter().map(param).collect();
        crate::Param { name: "param".into(), ty: "tuple".into(), internal_type: None, components }
    }

    fn full_signature_raw(
        name: &str,
        inputs: &[Param],
        outputs: Option<&[Param]>,
        state_mutability: StateMutability,
    ) -> String {
        full_signature(name, inputs, outputs, state_mutability)
    }

    fn full_signature_np(name: &str, inputs: &[Param], outputs: Option<&[Param]>) -> String {
        full_signature_raw(name, inputs, outputs, StateMutability::NonPayable)
    }

    fn full_signature_with_sm(
        name: &str,
        inputs: &[Param],
        outputs: Option<&[Param]>,
        state_mutability: StateMutability,
    ) -> String {
        full_signature_raw(name, inputs, outputs, state_mutability)
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
    fn test_full_signature() {
        assert_eq!(full_signature_np("foo", &[], None), "function foo()");
        assert_eq!(full_signature_np("foo", &[], Some(&[])), "function foo()");
        assert_eq!(full_signature_np("bar", &[param2("bool", "")], None), "function bar(bool)");
        assert_eq!(
            full_signature_np("bar", &[param2("bool", "")], Some(&[param2("bool", "")])),
            "function bar(bool) returns (bool)"
        );
        assert_eq!(
            full_signature_np(
                "foo",
                &[param2("address", "asset"), param2("uint256", "amount")],
                None
            ),
            "function foo(address asset, uint256 amount)"
        );
        assert_eq!(
            full_signature_np(
                "foo",
                &[param2("address", "asset")],
                Some(&[param2("uint256", "amount")])
            ),
            "function foo(address asset) returns (uint256 amount)"
        );

        let components = vec![
            param2("address", "pool"),
            param2("uint256", "tokenInParam"),
            param2("uint256", "tokenOutParam"),
            param2("uint256", "maxPrice"),
        ];
        let swaps =
            Param { name: "swaps".into(), ty: "tuple[]".into(), internal_type: None, components };

        assert_eq!(
            full_signature_with_sm(
                "batchSwapExactIn",
                &[
                    swaps,
                    param2("address", "tokenIn"),
                    param2("address", "tokenOut"),
                    param2("uint256", "totalAmountIn"),
                    param2("uint256", "minTotalAmountOut"),
                ],
                Some(&[param2("uint256", "totalAmountOut")]),
                StateMutability::Payable,
            ),
            "function batchSwapExactIn(tuple(address pool, uint256 tokenInParam, uint256 tokenOutParam, uint256 maxPrice)[] swaps, address tokenIn, address tokenOut, uint256 totalAmountIn, uint256 minTotalAmountOut) payable returns (uint256 totalAmountOut)"
        );

        assert_eq!(
            full_signature_with_sm(
                "name",
                &[],
                Some(&[param2("string", "")]),
                StateMutability::View
            ),
            "function name() view returns (string)"
        );

        assert_eq!(
            full_signature_with_sm(
                "calculateHash",
                &[param2("address[]", "_addresses")],
                Some(&[param2("bytes32", "")]),
                StateMutability::Pure,
            ),
            "function calculateHash(address[] _addresses) pure returns (bytes32)"
        );
    }

    #[test]
    fn test_event_signature() {
        assert_eq!(event_signature("foo", &[]), "foo()");
        assert_eq!(event_signature("foo", &[eparam("bool")]), "foo(bool)");
        assert_eq!(event_signature("foo", &[eparam("bool"), eparam("string")]), "foo(bool,string)");
    }

    #[test]
    fn test_event_full_signature() {
        assert_eq!(event_full_signature("foo", &[]), "event foo()");
        assert_eq!(
            event_full_signature("foo", &[eparam2("bool", "confirmed", true)]),
            "event foo(bool indexed confirmed)"
        );
        assert_eq!(
            event_full_signature(
                "foo",
                &[eparam2("bool", "confirmed", true), eparam2("string", "message", false)]
            ),
            "event foo(bool indexed confirmed, string message)"
        );

        let components = vec![
            param2("uint256", "amount"),
            param2("uint256", "startTime"),
            param2("uint256", "interval"),
        ];
        let info = EventParam {
            name: "info".into(),
            ty: "tuple".into(),
            internal_type: None,
            components,
            indexed: false,
        };
        assert_eq!(
            event_full_signature(
                "SetupDirectDebit",
                &[
                    eparam2("address", "debtor", true),
                    eparam2("address", "receiver", true),
                    info,
                ]            ),
            "event SetupDirectDebit(address indexed debtor, address indexed receiver, tuple(uint256 amount, uint256 startTime, uint256 interval) info)"
        );
    }

    #[test]
    fn test_item_parse() {
        assert_eq!(parse_sig::<true>("foo()"), Ok(("foo".into(), vec![], vec![], false)));
        assert_eq!(parse_sig::<true>("foo()()"), Ok(("foo".into(), vec![], vec![], false)));
        assert_eq!(parse_sig::<true>("foo() \t ()"), Ok(("foo".into(), vec![], vec![], false)));
        assert_eq!(parse_sig::<true>("foo()  ()"), Ok(("foo".into(), vec![], vec![], false)));

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
