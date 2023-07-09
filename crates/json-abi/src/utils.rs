use crate::{EventParam, Param};
use alloc::string::String;

macro_rules! signature {
    ($name:expr, $inputs:expr, $preimage:expr) => {{
        $preimage.push_str($name);

        $preimage.push('(');
        for (i, input) in $inputs.iter().enumerate() {
            if i > 0 {
                $preimage.push(',');
            }
            input.selector_type_raw($preimage);
        }
        $preimage.push(')');
    }};
}

macro_rules! validate_identifier {
    ($name:expr) => {
        if !$name.is_empty() && !alloy_sol_type_parser::is_valid_identifier($name) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str($name),
                &"a valid solidity identifier in the name field",
            ))
        }
    };
}
pub(crate) use validate_identifier;

macro_rules! validate_ty {
    ($ty:expr) => {
        // dirty hacks to allow `address payable` in the ABI JSON internalType
        // field
        if $ty != "address payable" {
            alloy_sol_type_parser::TypeSpecifier::parse($ty).map_err(|_| {
                serde::de::Error::invalid_value(
                    Unexpected::Str($ty),
                    &"a valid solidity type specifier",
                )
            })?;
        }
    };
}

pub(crate) use validate_ty;

pub(crate) fn signature(name: &str, inputs: &[Param]) -> String {
    let mut preimage = String::with_capacity(name.len() + 2 + inputs.len() * 32);
    signature_raw(name, inputs, &mut preimage);
    preimage
}

pub(crate) fn signature_raw(name: &str, inputs: &[Param], preimage: &mut String) {
    signature!(name, inputs, preimage)
}

pub(crate) fn event_signature(name: &str, inputs: &[EventParam]) -> String {
    let mut preimage = String::with_capacity(name.len() + 2 + inputs.len() * 32);
    event_signature_raw(name, inputs, &mut preimage);
    preimage
}

pub(crate) fn event_signature_raw(name: &str, inputs: &[EventParam], preimage: &mut String) {
    signature!(name, inputs, preimage)
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
        crate::Param {
            name: "param".to_string(),
            ty: kind.to_string(),
            internal_type: None,
            components: vec![],
        }
    }

    fn params(components: impl IntoIterator<Item = &'static str>) -> Param {
        let components = components.into_iter().map(param).collect();
        crate::Param {
            name: "param".to_string(),
            ty: "ty".to_string(),
            internal_type: None,
            components,
        }
    }

    #[test]
    fn test_signature() {
        assert_eq!(signature("foo", &[]), "foo()");
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
