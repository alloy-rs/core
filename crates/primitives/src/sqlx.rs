//! Support for the [`sqlx`](https://crates.io/crates/sqlx) crate.
//!
//! Supports big-endian binary serialization via sqlx binary types (e.g., BINARY(N), BYTEA, BLOB).
//! Similar to [`ruint`'s implementation](https://github.com/recmo/uint/blob/main/src/support/sqlx.rs)

#![cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]

use alloc::{boxed::Box, vec::Vec};

use sqlx_core::{
    database::Database,
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};

use crate::FixedBytes;

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
