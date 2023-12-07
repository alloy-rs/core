//! support for the postgres crate.

#![cfg(feature = "postgres")]
#![cfg_attr(docsrs, doc(cfg(feature = "postgres")))]

use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type, WrongType};

use crate::{FixedBytes, Signed};

// TODO: add types for the postgres and sqlx support to alloy-primitives
// for I*** types and FixedBytes<N> types

type BoxedError = Box<dyn Error + Sync + Send + 'static>;

impl<const BITS: usize> ToSql for FixedBytes<BITS> {
    fn accepts(ty: &Type) -> bool {
        todo!()
    }

    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, BoxedError> {
        todo!()
    }
}

impl<'a, const BITS: usize> FromSql<'a> for FixedByts<BITS> {
    fn accepts(ty: &Type) -> bool {
        <Self as ToSql>::accepts(ty)
    }

    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        todo!()
    }
}

impl<const BITS: usize, const LIMBS: usize> ToSql for Signed<BITS, LIMBS> {
    fn accepts(ty: &Type) -> bool {
        todo!()
    }

    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, BoxedError> {
        todo!()
    }
}

impl<'a, const BITS: usize, const LIMBS: usize> FromSql<'a> for Signed<BITS, LIMBS> {
    fn accepts(ty: &Type) -> bool {
        <Self as ToSql>::accepts(ty)
    }

    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        todo!()
    }
}
