use crate::{EventParam, Param};
use alloc::string::String;
use alloy_primitives::Selector;

/// Capacity to allocate per [Param].
const PARAM: usize = 32;

macro_rules! validate_identifier {
    ($name:expr) => {
        if !$name.is_empty() && !alloy_sol_type_parser::is_valid_identifier($name) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str($name),
                &"a valid Solidity identifier",
            ))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn param(kind: &str) -> Param {
        crate::Param {
            name: "param".into(),
            ty: kind.into(),
            internal_type: None,
            components: vec![],
        }
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
        crate::Param {
            name: "param".into(),
            ty: "tuple".into(),
            internal_type: None,
            components,
        }
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
            signature(
                "foo",
                &[param("int"), params(["uint[]"]), param("string")],
                None
            ),
            "foo(int,(uint[]),string)"
        );

        assert_eq!(signature("foo", &[], Some(&[])), "foo()()");
        assert_eq!(
            signature("foo", &[param("a")], Some(&[param("b")])),
            "foo(a)(b)"
        );
        assert_eq!(
            signature(
                "foo",
                &[param("a"), param("c")],
                Some(&[param("b"), param("d")])
            ),
            "foo(a,c)(b,d)"
        );
    }

    #[test]
    fn test_event_signature() {
        assert_eq!(event_signature("foo", &[]), "foo()");
        assert_eq!(event_signature("foo", &[eparam("bool")]), "foo(bool)");
        assert_eq!(
            event_signature("foo", &[eparam("bool"), eparam("string")]),
            "foo(bool,string)"
        );
    }
}
