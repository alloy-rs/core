//! Support for the [`sqlx`](https://crates.io/crates/sqlx) crate.
//!
//! Supports big-endian binary serialization via sqlx binary types (e.g., BINARY(N), BYTEA, BLOB).
//! With the `sqlx-postgres` feature, `Vec<T>` of these types maps to `BYTEA[]`.
//! Similar to [`ruint`'s implementation](https://github.com/recmo/uint/blob/main/src/support/sqlx.rs)

#![cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]

use alloc::{boxed::Box, vec::Vec};

use ruint::support::sqlx::DecodeError;
use sqlx_core::{
    database::Database,
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};

use crate::{Bytes, FixedBytes, Signed};

#[cfg(feature = "sqlx-postgres")]
use sqlx_postgres::{PgHasArrayType, PgTypeInfo};

impl<const BYTES: usize, DB> Type<DB> for FixedBytes<BYTES>
where
    DB: Database,
    Vec<u8>: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Vec<u8> as Type<DB>>::compatible(ty)
    }
}

impl<'a, const BYTES: usize, DB> Encode<'a, DB> for FixedBytes<BYTES>
where
    DB: Database,
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.as_slice().to_vec().encode_by_ref(buf)
    }
}

impl<'a, const BYTES: usize, DB> Decode<'a, DB> for FixedBytes<BYTES>
where
    DB: Database,
    Vec<u8>: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let bytes = Vec::<u8>::decode(value)?;
        Self::try_from(bytes.as_slice()).map_err(|e| Box::new(e) as BoxDynError)
    }
}

impl<const BITS: usize, const LIMBS: usize, DB: Database> Type<DB> for Signed<BITS, LIMBS>
where
    Vec<u8>: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Vec<u8> as Type<DB>>::compatible(ty)
    }
}

impl<'a, const BITS: usize, const LIMBS: usize, DB: Database> Encode<'a, DB> for Signed<BITS, LIMBS>
where
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.0.to_be_bytes_vec().encode_by_ref(buf)
    }
}

impl<'a, const BITS: usize, const LIMBS: usize, DB: Database> Decode<'a, DB> for Signed<BITS, LIMBS>
where
    Vec<u8>: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let bytes = Vec::<u8>::decode(value)?;
        Self::try_from_be_slice(bytes.as_slice()).ok_or_else(|| DecodeError::Overflow.into())
    }
}

impl<DB: Database> Type<DB> for Bytes
where
    Vec<u8>: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Vec<u8> as Type<DB>>::compatible(ty)
    }
}

impl<'a, DB: Database> Encode<'a, DB> for Bytes
where
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.to_vec().encode_by_ref(buf)
    }
}

impl<'a, DB: Database> Decode<'a, DB> for Bytes
where
    Vec<u8>: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        Vec::<u8>::decode(value).map(Self::from)
    }
}

#[cfg(feature = "sqlx-postgres")]
impl<const BYTES: usize> PgHasArrayType for FixedBytes<BYTES> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("bytea")
    }
}

#[cfg(feature = "sqlx-postgres")]
impl<const BITS: usize, const LIMBS: usize> PgHasArrayType for Signed<BITS, LIMBS> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("bytea")
    }
}

#[cfg(feature = "sqlx-postgres")]
impl PgHasArrayType for Bytes {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("bytea")
    }
}

#[cfg(all(test, feature = "sqlx-postgres"))]
mod test {
    use super::*;
    use crate::{Address, B256, Bloom, Function, I256};
    use sqlx_core::{decode::Decode, encode::Encode, type_info::TypeInfo, types::Type};
    use sqlx_postgres::{PgArgumentBuffer, Postgres};

    fn encode_pg<T>(value: &T) -> Vec<u8>
    where
        T: for<'q> Encode<'q, Postgres>,
    {
        let mut buf = PgArgumentBuffer::default();
        let _ = value.encode_by_ref(&mut buf).unwrap();
        buf.to_vec()
    }

    fn assert_bytea_array<T>()
    where
        T: Type<Postgres> + for<'q> Encode<'q, Postgres> + for<'r> Decode<'r, Postgres>,
    {
        assert_eq!(T::type_info().name(), "bytea[]");
    }

    #[test]
    fn vec_primitives_are_bytea_array() {
        assert_bytea_array::<Vec<Address>>();
        assert_bytea_array::<Vec<Bloom>>();
        assert_bytea_array::<Vec<Function>>();
        assert_bytea_array::<Vec<FixedBytes<20>>>();
        assert_bytea_array::<Vec<B256>>();
        assert_bytea_array::<Vec<Bytes>>();
        assert_bytea_array::<Vec<I256>>();
    }

    #[test]
    fn vec_primitives_encode_as_bytea_array() {
        let addresses = vec![Address::repeat_byte(0x11), Address::repeat_byte(0x22)];
        assert_eq!(
            encode_pg(&addresses),
            encode_pg(&addresses.iter().map(|a| a.as_slice().to_vec()).collect::<Vec<_>>())
        );
        assert_eq!(encode_pg(&Vec::<Address>::new()), encode_pg(&Vec::<Vec<u8>>::new()));

        let fixed = vec![FixedBytes::<20>::repeat_byte(0x33), FixedBytes::<20>::repeat_byte(0x44)];
        assert_eq!(
            encode_pg(&fixed),
            encode_pg(&fixed.iter().map(|b| b.as_slice().to_vec()).collect::<Vec<_>>())
        );

        let hashes = vec![B256::repeat_byte(0x55), B256::ZERO];
        assert_eq!(
            encode_pg(&hashes),
            encode_pg(&hashes.iter().map(|h| h.as_slice().to_vec()).collect::<Vec<_>>())
        );

        let bytes = vec![Bytes::from_static(&[0xde, 0xad]), Bytes::from_static(&[0xbe, 0xef])];
        assert_eq!(
            encode_pg(&bytes),
            encode_pg(&bytes.iter().map(|b| b.to_vec()).collect::<Vec<_>>())
        );

        let signed = vec![I256::ONE, I256::MINUS_ONE, I256::ZERO];
        assert_eq!(
            encode_pg(&signed),
            encode_pg(&signed.iter().map(|s| s.to_be_bytes::<32>().to_vec()).collect::<Vec<_>>())
        );
    }
}
