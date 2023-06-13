use crate::{EventParam, Param};
use alloc::string::String;

macro_rules! signature {
    ($name:expr, $inputs:expr) => {{
        let mut preimage = String::with_capacity($name.len() + 2 + $inputs.len() * 32);
        preimage.push_str($name);

        preimage.push('(');
        let mut first = true;
        for input in $inputs {
            if !first {
                preimage.push(',');
            }
            preimage.push_str(&input.selector_type());
            first = false;
        }
        preimage.push(')');

        preimage
    }};
}

pub(crate) fn signature(name: &str, inputs: &[Param]) -> String {
    signature!(name, inputs)
}

pub(crate) fn event_signature(name: &str, inputs: &[EventParam]) -> String {
    signature!(name, inputs)
}

/// `keccak256(preimage)[..4]`
pub(crate) fn selector(preimage: &str) -> [u8; 4] {
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
    use alloc::string::ToString;

    fn param(kind: &str) -> Param {
        Param::Simple(crate::SimpleParam {
            name: "param".to_string(),
            ty: kind.to_string(),
            internal_type: "internalType".to_string(),
        })
    }

    fn params(components: impl IntoIterator<Item = &'static str>) -> Param {
        let components = components.into_iter().map(param).collect();
        Param::Complex(crate::ComplexParam {
            name: "param".to_string(),
            ty: "ty".to_string(),
            internal_type: "internalType".to_string(),
            components,
        })
    }

    #[test]
    fn test_signature() {
        assert_eq!(signature("foo", &[]), "foo()");
        assert_eq!(signature("foo", &[param("bool")]), "foo(bool)");
        assert_eq!(signature("foo", &[param("bool")]), "foo(bool)");
        assert_eq!(
            signature("foo", &[param("bool"), param("bool")]),
            "foo(bool,bool)"
        );
        assert_eq!(
            signature("foo", &[param("bool"), params(["bool[]"]), param("bool")]),
            "foo(bool,(bool[]),bool)"
        );
    }
}
