//! Support for the [`sqlx`](https://crates.io/crates/sqlx) crate.
//!
//! Currently only encodes to/from a big-endian byte array.
//!
//! **Note:** The database column type must be `BINARY(20)` (MySQL/SQLite), `BYTEA` (Postgres), or
//! equivalent binary type for correct Address roundtrip.

#![cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]

use alloc::{boxed::Box, vec::Vec};

use sqlx_core::{
    database::Database,
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};

use crate::Address;

impl<DB: Database> Type<DB> for Address
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

impl<'a, DB: Database> Encode<'a, DB> for Address
where
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        Vec::from(self.as_slice()).encode_by_ref(buf)
    }
}

impl<'a, DB: Database> Decode<'a, DB> for Address
where
    Vec<u8>: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let bytes = Vec::<u8>::decode(value)?;
        Self::try_from(bytes.as_slice()).map_err(|e| Box::new(e) as BoxDynError)
    }
}
