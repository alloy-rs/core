//! Support for the [`sqlx`](https://crates.io/crates/sqlx) crate.
//!
//! Implements SQLx traits for [`Address`], allowing seamless integration with MySQL, PostgreSQL,
//! and SQLite.

use crate::Address;
use core::str::FromStr;

#[cfg(feature = "sqlx")]
mod sqlx_impl {
    use super::*;

    // MySQL
    impl sqlx::Type<sqlx::MySql> for Address {
        fn type_info() -> sqlx::mysql::MySqlTypeInfo {
            <String as sqlx::Type<sqlx::MySql>>::type_info()
        }
        fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
            <String as sqlx::Type<sqlx::MySql>>::compatible(ty)
        }
    }

    impl<'r> sqlx::Decode<'r, sqlx::MySql> for Address {
        fn decode(
            value: sqlx::mysql::MySqlValueRef<'r>,
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            let s = <String as sqlx::Decode<'r, sqlx::MySql>>::decode(value)?;
            Self::from_str(&s).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Sync + Send>)
        }
    }

    impl<'q> sqlx::Encode<'q, sqlx::MySql> for Address {
        fn encode_by_ref(
            &self,
            buf: &mut <sqlx::MySql as sqlx::Database>::ArgumentBuffer<'q>,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            <String as sqlx::Encode<'q, sqlx::MySql>>::encode_by_ref(&self.to_string(), buf)
        }
    }

    // PostgreSQL
    impl sqlx::Type<sqlx::Postgres> for Address {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            <String as sqlx::Type<sqlx::Postgres>>::type_info()
        }
        fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
            <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
    }

    impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Address {
        fn decode(
            value: sqlx::postgres::PgValueRef<'r>,
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            let s = <String as sqlx::Decode<'r, sqlx::Postgres>>::decode(value)?;
            Self::from_str(&s).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Sync + Send>)
        }
    }

    impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Address {
        fn encode_by_ref(
            &self,
            buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.to_string(), buf)
        }
    }

    // SQLite
    impl sqlx::Type<sqlx::Sqlite> for Address {
        fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
            <String as sqlx::Type<sqlx::Sqlite>>::type_info()
        }
        fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
            <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
        }
    }

    impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for Address {
        fn decode(
            value: sqlx::sqlite::SqliteValueRef<'r>,
        ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
            let s = <String as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
            Self::from_str(&s).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Sync + Send>)
        }
    }
    impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Address {
        fn encode_by_ref(
            &self,
            buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&self.to_string(), buf)
        }
    }
}

#[cfg(all(test, feature = "sqlx"))]
mod tests {
    use crate::Address;
    use std::str::FromStr;

    #[test]
    fn address_roundtrip_string() {
        let addr_str = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
        let addr = Address::from_str(addr_str).unwrap();
        assert_eq!(addr.to_string(), addr_str);
    }

    #[test]
    fn address_sqlx_type_impls() {
        // Compile-time trait checks (will fail to compile if not implemented)
        fn assert_type<
            T: for<'q> sqlx::Type<sqlx::MySql>
                + for<'q> sqlx::Encode<'q, sqlx::MySql>
                + for<'r> sqlx::Decode<'r, sqlx::MySql>,
        >() {
        }
        fn assert_pg<
            T: for<'q> sqlx::Type<sqlx::Postgres>
                + for<'q> sqlx::Encode<'q, sqlx::Postgres>
                + for<'r> sqlx::Decode<'r, sqlx::Postgres>,
        >() {
        }
        fn assert_sqlite<
            T: for<'q> sqlx::Type<sqlx::Sqlite>
                + for<'q> sqlx::Encode<'q, sqlx::Sqlite>
                + for<'r> sqlx::Decode<'r, sqlx::Sqlite>,
        >() {
        }
        assert_type::<Address>();
        assert_pg::<Address>();
        assert_sqlite::<Address>();
    }
}
