use core::str::FromStr;
use ruint::Uint;

#[cfg(not(feature = "std"))]
use alloc::string::String;

use super::{utils::twos_complement, BigIntConversionError, ParseSignedError, Sign, Signed};

impl<const BITS: usize, const LIMBS: usize> TryFrom<Uint<BITS, LIMBS>> for Signed<BITS, LIMBS> {
    type Error = BigIntConversionError;

    #[inline(always)]
    fn try_from(from: Uint<BITS, LIMBS>) -> Result<Self, Self::Error> {
        let value = Signed(from);
        match value.sign() {
            Sign::Positive => Ok(value),
            Sign::Negative => Err(BigIntConversionError),
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for Uint<BITS, LIMBS> {
    type Error = BigIntConversionError;

    #[inline(always)]
    fn try_from(value: Signed<BITS, LIMBS>) -> Result<Self, Self::Error> {
        match value.sign() {
            Sign::Positive => Ok(value.0),
            Sign::Negative => Err(BigIntConversionError),
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<&str> for Signed<BITS, LIMBS> {
    type Error = <Self as FromStr>::Err;

    #[inline(always)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<&String> for Signed<BITS, LIMBS> {
    type Error = <Self as FromStr>::Err;

    #[inline(always)]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<String> for Signed<BITS, LIMBS> {
    type Error = <Self as FromStr>::Err;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const BITS: usize, const LIMBS: usize> FromStr for Signed<BITS, LIMBS> {
    type Err = ParseSignedError;

    #[inline(always)]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Signed::from_dec_str(value).or_else(|_| Signed::from_hex_str(value))
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for i128 {
    type Error = BigIntConversionError;

    fn try_from(value: Signed<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if value.bits() > 128 {
            return Err(BigIntConversionError)
        }

        if value.is_positive() {
            Ok(u128::try_from(value.0).unwrap() as i128)
        } else {
            let u = twos_complement(value.0);
            let u = u128::try_from(u).unwrap() as i128;
            Ok((!u).wrapping_add(1))
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<i128> for Signed<BITS, LIMBS> {
    type Error = BigIntConversionError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let u = value as u128;
        if value >= 0 {
            return Self::try_from(u)
        }

        // This is a bit messy :(
        let tc = (!u).wrapping_add(1);
        let stc = Uint::<128, 2>::saturating_from(tc);
        let (num, overflow) = Uint::<BITS, LIMBS>::overflowing_from_limbs_slice(stc.as_limbs());
        if overflow {
            return Err(BigIntConversionError)
        }
        Ok(Signed(twos_complement(num)))
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for u128 {
    type Error = BigIntConversionError;

    fn try_from(value: Signed<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if value.is_negative() {
            return Err(BigIntConversionError)
        }

        let saturated = Uint::<BITS, LIMBS>::saturating_from(u128::MAX);

        // if the value is greater than the saturated value, return an error
        if value > Signed(saturated) {
            return Err(BigIntConversionError)
        }

        value
            .into_raw()
            .try_into()
            .map_err(|_| BigIntConversionError)
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<u128> for Signed<BITS, LIMBS> {
    type Error = BigIntConversionError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let saturated = Uint::<BITS, LIMBS>::saturating_from(value);

        if value != saturated.to::<u128>() {
            return Err(BigIntConversionError)
        }

        Signed::try_from(saturated)
    }
}

// conversions
macro_rules! impl_conversions {
    ($(
        $u:ty [$actual_low_u:ident -> $low_u:ident, $as_u:ident],
        $i:ty [$actual_low_i:ident -> $low_i:ident, $as_i:ident];
    )+) => {
        // low_*, as_*
        impl<const BITS: usize, const LIMBS: usize> Signed<BITS, LIMBS> {
            $(
                impl_conversions!(@impl_fns $u, $actual_low_u $low_u $as_u);
                impl_conversions!(@impl_fns $i, $actual_low_i $low_i $as_i);
            )+
        }

        // From<$>, TryFrom
        $(
            impl<const BITS: usize, const LIMBS: usize> TryFrom<$u> for Signed<BITS, LIMBS> {
                type Error = BigIntConversionError;

                #[inline(always)]
                fn try_from(value: $u) -> Result<Self, Self::Error> {
                    let u = Uint::<BITS, LIMBS>::try_from(value).map_err(|_| BigIntConversionError)?;
                    Signed::checked_from_sign_and_abs(Sign::Positive, u).ok_or(BigIntConversionError)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> TryFrom<$i> for Signed<BITS, LIMBS> {
                type Error = BigIntConversionError;

                #[inline(always)]
                fn try_from(value: $i) -> Result<Self, Self::Error> {
                    let uint: $u = value as $u;

                    if value.is_positive() {
                        return Self::try_from(uint);
                    }

                    let abs = (!uint).wrapping_add(1);
                    let tc = twos_complement(Uint::<BITS, LIMBS>::from(abs));
                    Ok(Self(tc))
                }
            }

            impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for $u {
                type Error = BigIntConversionError;

                #[inline(always)]
                fn try_from(value: Signed<BITS, LIMBS>) -> Result<$u, Self::Error> {
                    u128::try_from(value)?.try_into().map_err(|_| BigIntConversionError)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for $i {
                type Error = BigIntConversionError;

                #[inline(always)]
                fn try_from(value: Signed<BITS, LIMBS>) -> Result<$i, Self::Error> {
                    i128::try_from(value)?.try_into().map_err(|_| BigIntConversionError)
                }
            }
        )+
    };

    (@impl_fns $t:ty, $actual_low:ident $low:ident $as:ident) => {
        /// Low word.
        #[inline(always)]
        pub const fn $low(&self) -> $t {
            if BITS == 0 {
                return 0
            }

            self.0.as_limbs()[0] as $t
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

impl_conversions! {
    u8   [low_u64  -> low_u8,    as_u8],    i8   [low_u64  -> low_i8,    as_i8];
    u16  [low_u64  -> low_u16,   as_u16],   i16  [low_u64  -> low_i16,   as_i16];
    u32  [low_u64  -> low_u32,   as_u32],   i32  [low_u64  -> low_i32,   as_i32];
    u64  [low_u64  -> low_u64,   as_u64],   i64  [low_u64  -> low_i64,   as_i64];
    usize[low_u64  -> low_usize, as_usize], isize[low_u64  -> low_isize, as_isize];
}
