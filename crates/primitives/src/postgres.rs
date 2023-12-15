//! support for the postgres crate.

use std::{
    error::Error,
    iter,
    str::{from_utf8, FromStr},
};

use thiserror::Error;

use bytes::{BufMut, BytesMut};
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type, WrongType};

use crate::{FixedBytes, Signed};

impl<const BITS: usize> ToSql for FixedBytes<BITS> {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, BoxedError> {
        out.put_slice(&self[..]);

        Ok(IsNull::No)
    }

    accepts!(BYTEA);

    to_sql_checked!();
}

impl<'a, const BITS: usize> FromSql<'a> for FixedBytes<BITS> {
    accepts!(BYTEA);

    fn from_sql(_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(FixedBytes::try_from(raw)?)
    }
}

//https://github.com/recmo/uint/blob/6c755ad7cd54a0706d20f11f3f63b0d977af0226/src/support/postgres.rs#L22

type BoxedError = Box<dyn Error + Sync + Send + 'static>;

const fn rem_up(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem > 0 {
        rem
    } else {
        b
    }
}

fn last_idx<T: PartialEq>(x: &[T], value: &T) -> usize {
    x.iter().rposition(|b| b != value).map_or(0, |idx| idx + 1)
}

fn trim_end_vec<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
    vec.truncate(last_idx(vec, value));
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum ToSqlError {
    #[error("Uint<{0}> value too large to fit target type {1}")]
    Overflow(usize, Type),
}

impl<const BITS: usize, const LIMBS: usize> ToSql for Signed<BITS, LIMBS> {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, BoxedError> {
        match *ty {
            // Big-endian simple types
            // Note `BufMut::put_*` methods write big-endian by default.
            Type::BOOL => out.put_u8(u8::from(bool::try_from(self.0)?)),
            Type::INT2 => out.put_i16(self.0.try_into()?),
            Type::INT4 => out.put_i32(self.0.try_into()?),
            Type::OID => out.put_u32(self.0.try_into()?),
            Type::INT8 => out.put_i64(self.0.try_into()?),
            Type::FLOAT4 => out.put_f32(self.0.into()),
            Type::FLOAT8 => out.put_f64(self.0.into()),
            Type::MONEY => {
                // Like i64, but with two decimals.
                out.put_i64(
                    i64::try_from(self.0)?
                        .checked_mul(100)
                        .ok_or(ToSqlError::Overflow(BITS, ty.clone()))?,
                );
            }

            // Binary strings
            Type::BYTEA => out.put_slice(&self.0.to_be_bytes_vec()),
            Type::BIT | Type::VARBIT => {
                // Bit in little-endian so the the first bit is the least significant.
                // Length must be at least one bit.
                if BITS == 0 {
                    if *ty == Type::BIT {
                        // `bit(0)` is not a valid type, but varbit can be empty.
                        return Err(Box::new(WrongType::new::<Self>(ty.clone())));
                    }
                    out.put_i32(0);
                } else {
                    // Bits are output in big-endian order, but padded at the
                    // least significant end.
                    let padding = 8 - rem_up(BITS, 8);
                    out.put_i32(Self::BITS.try_into()?);
                    let bytes = self.0.as_le_bytes();
                    let mut bytes = bytes.iter().rev();
                    let mut shifted = bytes.next().unwrap() << padding;
                    for byte in bytes {
                        shifted |= if padding > 0 { byte >> (8 - padding) } else { 0 };
                        out.put_u8(shifted);
                        shifted = byte << padding;
                    }
                    out.put_u8(shifted);
                }
            }

            // Hex strings
            Type::CHAR | Type::TEXT | Type::VARCHAR => {
                out.put_slice(format!("{self:#x}").as_bytes());
            }
            Type::JSON | Type::JSONB => {
                if *ty == Type::JSONB {
                    // Version 1 of JSONB is just plain text JSON.
                    out.put_u8(1);
                }
                out.put_slice(format!("\"{self:#x}\"").as_bytes());
            }

            // Binary coded decimal types
            // See <https://github.com/postgres/postgres/blob/05a5a1775c89f6beb326725282e7eea1373cbec8/src/backend/utils/adt/numeric.c#L253>
            Type::NUMERIC => {
                // Everything is done in big-endian base 1000 digits.
                const BASE: u64 = 10000;
                let mut digits: Vec<_> = self.0.to_base_be(BASE).collect();
                let exponent = digits.len().saturating_sub(1).try_into()?;

                // Trailing zeros are removed.
                trim_end_vec(&mut digits, &0);

                out.put_i16(digits.len().try_into()?); // Number of digits.
                out.put_i16(exponent); // Exponent of first digit.
                out.put_i16(0); // sign: 0x0000 = positive, 0x4000 = negative.
                out.put_i16(0); // dscale: Number of digits to the right of the decimal point.
                for digit in digits {
                    debug_assert!(digit < BASE);
                    #[allow(clippy::cast_possible_truncation)] // 10000 < i16::MAX
                    out.put_i16(digit as i16);
                }
            }

            // Unsupported types
            _ => {
                return Err(Box::new(WrongType::new::<Self>(ty.clone())));
            }
        };
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, |Type::BOOL| Type::CHAR
            | Type::INT2
            | Type::INT4
            | Type::INT8
            | Type::OID
            | Type::FLOAT4
            | Type::FLOAT8
            | Type::MONEY
            | Type::NUMERIC
            | Type::BYTEA
            | Type::TEXT
            | Type::VARCHAR
            | Type::JSON
            | Type::JSONB
            | Type::BIT
            | Type::VARBIT)
    }

    to_sql_checked!();
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum FromSqlError {
    #[error("The value is too large for the Uint type")]
    Overflow,

    #[error("Unexpected data for type {0}")]
    ParseError(Type),
}

impl<'a, const BITS: usize, const LIMBS: usize> FromSql<'a> for Signed<BITS, LIMBS> {
    fn accepts(ty: &Type) -> bool {
        <Self as ToSql>::accepts(ty)
    }

    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(match *ty {
            Type::BOOL => match raw {
                [0] => Self::ZERO,
                [1] => Self::try_from(1)?,
                _ => return Err(Box::new(FromSqlError::ParseError(ty.clone()))),
            },
            Type::INT2 => i16::from_be_bytes(raw.try_into()?).try_into()?,
            Type::INT4 => i32::from_be_bytes(raw.try_into()?).try_into()?,
            Type::OID => u32::from_be_bytes(raw.try_into()?).try_into()?,
            Type::INT8 => i64::from_be_bytes(raw.try_into()?).try_into()?,

            Type::MONEY => (i64::from_be_bytes(raw.try_into()?) / 100).try_into()?,

            // Binary strings
            Type::BYTEA => Self::try_from_be_slice(raw).ok_or(FromSqlError::Overflow)?,
            Type::BIT | Type::VARBIT => {
                // Parse header
                if raw.len() < 4 {
                    return Err(Box::new(FromSqlError::ParseError(ty.clone())));
                }
                let len: usize = i32::from_be_bytes(raw[..4].try_into()?).try_into()?;
                let raw = &raw[4..];

                // Shift padding to the other end
                let padding = 8 - rem_up(len, 8);
                let mut raw = raw.to_owned();
                if padding > 0 {
                    for i in (1..raw.len()).rev() {
                        raw[i] = raw[i] >> padding | raw[i - 1] << (8 - padding);
                    }
                    raw[0] >>= padding;
                }
                // Construct from bits
                Self::try_from_be_slice(&raw).ok_or(FromSqlError::Overflow)?
            }

            // Hex strings
            Type::CHAR | Type::TEXT | Type::VARCHAR => Self::from_str(from_utf8(raw)?)?,

            // Hex strings
            Type::JSON | Type::JSONB => {
                let raw = if *ty == Type::JSONB {
                    if raw[0] == 1 {
                        &raw[1..]
                    } else {
                        // Unsupported version
                        return Err(Box::new(FromSqlError::ParseError(ty.clone())));
                    }
                } else {
                    raw
                };
                let str = from_utf8(raw)?;
                let str = if str.starts_with('"') && str.ends_with('"') {
                    // Stringified number
                    &str[1..str.len() - 1]
                } else {
                    str
                };
                Self::from_str(str)?
            }

            // Numeric types
            Type::NUMERIC => {
                // Parse header
                if raw.len() < 8 {
                    return Err(Box::new(FromSqlError::ParseError(ty.clone())));
                }
                let digits = i16::from_be_bytes(raw[0..2].try_into()?);
                let exponent = i16::from_be_bytes(raw[2..4].try_into()?);
                let sign = i16::from_be_bytes(raw[4..6].try_into()?);
                let dscale = i16::from_be_bytes(raw[6..8].try_into()?);
                let raw = &raw[8..];
                #[allow(clippy::cast_sign_loss)] // Signs are checked
                if digits < 0
                    || exponent < 0
                    || sign != 0x0000
                    || dscale != 0
                    || digits > exponent + 1
                    || raw.len() != digits as usize * 2
                {
                    return Err(Box::new(FromSqlError::ParseError(ty.clone())));
                }
                let mut error = false;
                let iter = raw.chunks_exact(2).filter_map(|raw| {
                    if error {
                        return None;
                    }
                    let digit = i16::from_be_bytes(raw.try_into().unwrap());
                    if !(0..10000).contains(&digit) {
                        error = true;
                        return None;
                    }
                    #[allow(clippy::cast_sign_loss)] // Signs are checked
                    Some(digit as u64)
                });
                #[allow(clippy::cast_sign_loss)]
                // Expression can not be negative due to checks above
                let iter = iter.chain(iter::repeat(0).take((exponent + 1 - digits) as usize));

                let value = Self::from_base_be(10000, iter)?;
                if error {
                    return Err(Box::new(FromSqlError::ParseError(ty.clone())));
                }

                value
            }

            // Unsupported types
            _ => return Err(Box::new(WrongType::new::<Self>(ty.clone()))),
        })
    }
}
