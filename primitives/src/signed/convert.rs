use core::str::FromStr;

#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::{
    signed::{errors, Sign, TryFromBigIntError, I256},
    U256,
};

use super::twos_complement;

impl TryFrom<U256> for I256 {
    type Error = TryFromBigIntError;

    #[inline(always)]
    fn try_from(from: U256) -> Result<Self, Self::Error> {
        let value = I256(from);
        match value.sign() {
            Sign::Positive => Ok(value),
            Sign::Negative => Err(TryFromBigIntError),
        }
    }
}

impl TryFrom<I256> for U256 {
    type Error = TryFromBigIntError;

    #[inline(always)]
    fn try_from(value: I256) -> Result<Self, Self::Error> {
        match value.sign() {
            Sign::Positive => Ok(value.0),
            Sign::Negative => Err(TryFromBigIntError),
        }
    }
}

impl TryFrom<&str> for I256 {
    type Error = <Self as FromStr>::Err;

    #[inline(always)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&String> for I256 {
    type Error = <Self as FromStr>::Err;

    #[inline(always)]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl TryFrom<String> for I256 {
    type Error = <Self as FromStr>::Err;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl FromStr for I256 {
    type Err = errors::ParseI256Error;

    #[inline(always)]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        I256::from_hex_str(value).or_else(|_| I256::from_dec_str(value))
    }
}

impl From<i128> for I256 {
    fn from(value: i128) -> Self {
        let u = value as u128;

        if value.is_negative() {
            let abs = (!u).wrapping_add(1);
            let u = twos_complement(U256::from(abs));
            I256(u)
        } else {
            I256(U256::from(u))
        }
    }
}

impl From<u128> for I256 {
    fn from(value: u128) -> Self {
        I256(U256::from(value))
    }
}

impl TryFrom<I256> for i128 {
    type Error = TryFromBigIntError;

    fn try_from(value: I256) -> Result<Self, Self::Error> {
        if value > I256::from(i128::MAX) || value < I256::from(i128::MIN) {
            return Err(TryFromBigIntError);
        }
        // this inverts the from impl above
        if value.is_positive() {
            Ok(u128::try_from(value.0).unwrap() as i128)
        } else {
            let u = twos_complement(value.0);
            let u = u128::try_from(u).unwrap() as i128;
            Ok((!u).wrapping_add(1))
        }
    }
}

impl TryFrom<I256> for u128 {
    type Error = TryFromBigIntError;

    fn try_from(value: I256) -> Result<Self, Self::Error> {
        if value.is_negative() || value > I256::from(u128::MAX) {
            return Err(TryFromBigIntError);
        }

        value.into_raw().try_into().map_err(|_| TryFromBigIntError)
    }
}

// conversions
macro_rules! impl_conversions {
    ($(
        $u:ty [$actual_low_u:ident -> $low_u:ident, $as_u:ident],
        $i:ty [$actual_low_i:ident -> $low_i:ident, $as_i:ident];
    )+) => {
        // low_*, as_*
        impl I256 {
            $(
                impl_conversions!(@impl_fns $u, $actual_low_u $low_u $as_u);
                impl_conversions!(@impl_fns $i, $actual_low_i $low_i $as_i);
            )+
        }

        // From<$>, TryFrom
        $(
            impl From<$u> for I256 {
                #[inline(always)]
                fn from(value: $u) -> Self {
                    Self(U256::from(value))
                }
            }

            impl From<$i> for I256 {
                #[inline(always)]
                fn from(value: $i) -> Self {
                    let uint: $u = value as $u;
                    Self(if value.is_negative() {
                        let abs = (!uint).wrapping_add(1);
                        super::twos_complement(U256::from(abs))
                    } else {
                        U256::from(uint)
                    })
                }
            }

            impl TryFrom<I256> for $u {
                type Error = TryFromBigIntError;

                #[inline(always)]
                fn try_from(value: I256) -> Result<$u, Self::Error> {
                    u128::try_from(value)?.try_into().map_err(|_| TryFromBigIntError)
                }
            }

            impl TryFrom<I256> for $i {
                type Error = TryFromBigIntError;

                #[inline(always)]
                fn try_from(value: I256) -> Result<$i, Self::Error> {
                    i128::try_from(value)?.try_into().map_err(|_| TryFromBigIntError)
                }
            }
        )+
    };

    (@impl_fns $t:ty, $actual_low:ident $low:ident $as:ident) => {
        /// Low word.
        #[inline(always)]
        pub fn $low(&self) -> $t {
            self.0.to::<$t>()
        }

        #[doc = concat!("Conversion to ", stringify!($t) ," with overflow checking.")]
        ///
        /// # Panics
        ///
        #[doc = concat!("If the number is outside the ", stringify!($t), " valid range.")]
        #[inline(always)]
        #[track_caller]
        pub fn $as(&self) -> $t {
            <$t as TryFrom<Self>>::try_from(*self).unwrap()
        }
    };
}

// Use `U256::low_u64` for types which fit in one word.
impl_conversions! {
    u8   [low_u64  -> low_u8,    as_u8],    i8   [low_u64  -> low_i8,    as_i8];
    u16  [low_u64  -> low_u16,   as_u16],   i16  [low_u64  -> low_i16,   as_i16];
    u32  [low_u64  -> low_u32,   as_u32],   i32  [low_u64  -> low_i32,   as_i32];
    u64  [low_u64  -> low_u64,   as_u64],   i64  [low_u64  -> low_i64,   as_i64];
    usize[low_u64  -> low_usize, as_usize], isize[low_u64  -> low_isize, as_isize];
}
