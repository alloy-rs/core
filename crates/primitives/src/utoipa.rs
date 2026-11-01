//! Support for the [`utoipa`](https://crates.io/crates/utoipa) crate.
//!
//! Implements [`ToSchema`] and [`PartialSchema`] so that primitive types can be used in
//! OpenAPI documents. The schemas describe the human-readable [`serde`] representation of each
//! type.
//!
//! [`serde`]: https://crates.io/crates/serde

use crate::{Bytes, FixedBytes, Signed};
use alloc::{borrow::Cow, format};
use utoipa::{
    PartialSchema, ToSchema,
    openapi::{
        RefOr,
        schema::{ObjectBuilder, Schema, Type},
    },
};

impl<const N: usize> PartialSchema for FixedBytes<N> {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .description(Some(format!("{N} bytes, represented as a hex string")))
            .min_length(Some(N * 2))
            .max_length(Some(2 + N * 2))
            .pattern(Some(format!("^(0x)?[0-9a-fA-F]{{{}}}$", N * 2)))
            .into()
    }
}

impl<const N: usize> ToSchema for FixedBytes<N> {
    fn name() -> Cow<'static, str> {
        Cow::Owned(format!("FixedBytes{N}"))
    }
}

impl PartialSchema for Bytes {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .description(Some("Arbitrary bytes, represented as a hex string"))
            .pattern(Some("^(0x)?([0-9a-fA-F]{2})*$"))
            .into()
    }
}

impl ToSchema for Bytes {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Bytes")
    }
}

impl<const BITS: usize, const LIMBS: usize> PartialSchema for Signed<BITS, LIMBS> {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .description(Some(format!(
                "{BITS}-bit signed integer, represented as a decimal string"
            )))
            .pattern(Some("^-?[0-9]+$"))
            .into()
    }
}

impl<const BITS: usize, const LIMBS: usize> ToSchema for Signed<BITS, LIMBS> {
    fn name() -> Cow<'static, str> {
        Cow::Owned(format!("I{BITS}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Address, B256, Bloom, I256, Selector};
    use alloc::string::String;

    fn pattern<T: PartialSchema>() -> String {
        let RefOr::T(Schema::Object(obj)) = T::schema() else { panic!("expected object schema") };
        obj.pattern.unwrap()
    }

    #[test]
    fn names() {
        assert_eq!(B256::name(), "FixedBytes32");
        assert_eq!(Address::name(), "Address");
        assert_eq!(Bloom::name(), "Bloom");
        assert_eq!(Selector::name(), "FixedBytes4");
        assert_eq!(Bytes::name(), "Bytes");
        assert_eq!(I256::name(), "I256");
    }

    #[test]
    fn wrapper_schema_matches_inner() {
        let json = |s: RefOr<Schema>| serde_json::to_value(s).unwrap();
        assert_eq!(json(Address::schema()), json(FixedBytes::<20>::schema()));
        assert_eq!(json(Bloom::schema()), json(FixedBytes::<256>::schema()));
    }

    #[test]
    fn address_schema() {
        assert_eq!(
            serde_json::to_value(Address::schema()).unwrap(),
            serde_json::json!({
                "type": "string",
                "description": "20 bytes, represented as a hex string",
                "minLength": 40,
                "maxLength": 42,
                "pattern": "^(0x)?[0-9a-fA-F]{40}$",
            })
        );
    }

    #[test]
    fn patterns() {
        assert_eq!(pattern::<Address>(), "^(0x)?[0-9a-fA-F]{40}$");
        assert_eq!(pattern::<B256>(), "^(0x)?[0-9a-fA-F]{64}$");
        assert_eq!(pattern::<Bytes>(), "^(0x)?([0-9a-fA-F]{2})*$");
        assert_eq!(pattern::<I256>(), "^-?[0-9]+$");
    }

    #[test]
    fn derive() {
        #[derive(utoipa::ToSchema)]
        #[allow(dead_code)]
        struct Transfer {
            from: Address,
            hash: B256,
            data: Bytes,
            delta: I256,
        }

        let mut schemas = alloc::vec::Vec::new();
        Transfer::schemas(&mut schemas);
        let mut names: alloc::vec::Vec<_> = schemas.into_iter().map(|(name, _)| name).collect();
        names.sort();
        assert_eq!(names, ["Address", "Bytes", "FixedBytes32", "I256"]);
    }
}
