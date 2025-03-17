//! Support for the [`diesel`](https://crates.io/crates/diesel) crate.
//!
//! Supports big-endian binary serialization via via the [`BYTEA`] Postgres type.
//! Similar to [`ruint`'s implementation](https://github.com/recmo/uint/blob/fd57517b36cda8341f7740dacab4b1ec186af948/src/support/diesel.rs)

use super::FixedBytes;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, Result as DeserResult},
    query_builder::bind_collector::RawBytesBindCollector,
    serialize::{IsNull, Output, Result as SerResult, ToSql},
    sql_types::Binary,
};
use std::io::Write;

impl<const BITS: usize, Db> ToSql<Binary, Db> for FixedBytes<BITS>
where
    for<'c> Db: Backend<BindCollector<'c> = RawBytesBindCollector<Db>>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Db>) -> SerResult {
        out.write_all(&self[..])?;
        Ok(IsNull::No)
    }
}

impl<'a, const BITS: usize, Db: Backend> FromSql<Binary, Db> for FixedBytes<BITS>
where
    *const [u8]: FromSql<Binary, Db>,
{
    fn from_sql(bytes: Db::RawValue<'_>) -> DeserResult<Self> {
        let bytes: *const [u8] = FromSql::<Binary, Db>::from_sql(bytes)?;
        let bytes = unsafe { &*bytes };
        Ok(Self::from_slice(&bytes))
    }
}
