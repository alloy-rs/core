use crate::{ParseSignedError, I256, U256};
use alloc::string::{String, ToString};
use core::fmt;

/// Converts the input to a U256 and converts from Ether to Wei.
///
/// # Examples
///
/// ```
/// use alloy_primitives::{
///     utils::{parse_ether, Units},
///     U256,
/// };
///
/// let eth = Units::ETHER.wei();
/// assert_eq!(parse_ether("1").unwrap(), eth);
/// ```
pub fn parse_ether(eth: &str) -> Result<U256, UnitsError> {
    ParseUnits::parse_units(eth, Units::ETHER).map(Into::into)
}

/// Parses a decimal number and multiplies it with 10^units.
///
/// # Examples
///
/// ```
/// use alloy_primitives::{utils::parse_units, U256};
///
/// let amount_in_eth = U256::from_str_radix("15230001000000000000", 10).unwrap();
/// let amount_in_gwei = U256::from_str_radix("15230001000", 10).unwrap();
/// let amount_in_wei = U256::from_str_radix("15230001000", 10).unwrap();
/// assert_eq!(amount_in_eth, parse_units("15.230001000000000000", "ether").unwrap().into());
/// assert_eq!(amount_in_gwei, parse_units("15.230001000000000000", "gwei").unwrap().into());
/// assert_eq!(amount_in_wei, parse_units("15230001000", "wei").unwrap().into());
/// ```
///
/// Example of trying to parse decimal WEI, which should fail, as WEI is the smallest
/// ETH denominator. 1 ETH = 10^18 WEI.
///
/// ```should_panic
/// use alloy_primitives::{utils::parse_units, U256};
/// let amount_in_wei = U256::from_str_radix("15230001000", 10).unwrap();
/// assert_eq!(amount_in_wei, parse_units("15.230001000000000000", "wei").unwrap().into());
/// ```
pub fn parse_units<K>(amount: &str, units: K) -> Result<ParseUnits, UnitsError>
where
    K: TryInto<Units, Error = UnitsError> + Copy,
{
    ParseUnits::parse_units(amount, units.try_into()?)
}

/// Formats the given number of Wei as an Ether amount.
///
/// # Examples
///
/// ```
/// use alloy_primitives::{utils::format_ether, U256};
///
/// let eth = format_ether(1395633240123456000_u128);
/// assert_eq!(format_ether(1395633240123456000_u128), "1.395633240123456000");
/// ```
pub fn format_ether<T: Into<ParseUnits>>(amount: T) -> String {
    amount.into().format_units(Units::ETHER)
}

/// Formats the given number of Wei as the given unit.
///
/// # Examples
///
/// ```
/// use alloy_primitives::{utils::format_units, U256};
///
/// let eth = U256::from_str_radix("1395633240123456000", 10).unwrap();
/// assert_eq!(format_units(eth, "eth").unwrap(), "1.395633240123456000");
///
/// assert_eq!(format_units(i64::MIN, "gwei").unwrap(), "-9223372036.854775808");
///
/// assert_eq!(format_units(i128::MIN, 36).unwrap(), "-170.141183460469231731687303715884105728");
/// ```
pub fn format_units<T, K, E>(amount: T, units: K) -> Result<String, UnitsError>
where
    T: Into<ParseUnits>,
    K: TryInto<Units, Error = E>,
    UnitsError: From<E>,
{
    units.try_into().map(|units| amount.into().format_units(units)).map_err(UnitsError::from)
}

/// Error type for [`Units`]-related operations.
#[derive(Debug)]
pub enum UnitsError {
    /// The provided units are not recognized.
    InvalidUnit(String),
    /// Overflow when parsing a signed number.
    ParseSigned(ParseSignedError),
}

#[cfg(feature = "std")]
impl std::error::Error for UnitsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUnit(_) => None,
            Self::ParseSigned(e) => Some(e),
        }
    }
}

impl fmt::Display for UnitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUnit(s) => write!(f, "{s:?} is not a valid unit"),
            Self::ParseSigned(e) => e.fmt(f),
        }
    }
}

impl From<ruint::ParseError> for UnitsError {
    fn from(value: ruint::ParseError) -> Self {
        Self::ParseSigned(value.into())
    }
}

impl From<ParseSignedError> for UnitsError {
    fn from(value: ParseSignedError) -> Self {
        Self::ParseSigned(value)
    }
}

/// This enum holds the numeric types that a possible to be returned by `parse_units` and
/// that are taken by `format_units`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseUnits {
    /// Unsigned 256-bit integer.
    U256(U256),
    /// Signed 256-bit integer.
    I256(I256),
}

impl From<ParseUnits> for U256 {
    fn from(value: ParseUnits) -> Self {
        value.get_absolute()
    }
}

impl fmt::Display for ParseUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseUnits::U256(val) => val.fmt(f),
            ParseUnits::I256(val) => val.fmt(f),
        }
    }
}

macro_rules! impl_from_integers {
    ($convert:ident($($t:ty),* $(,)?)) => {$(
        impl From<$t> for ParseUnits {
            fn from(value: $t) -> Self {
                Self::$convert($convert::try_from(value).unwrap())
            }
        }
    )*}
}

impl_from_integers!(U256(u8, u16, u32, u64, u128, usize, U256));
impl_from_integers!(I256(i8, i16, i32, i64, i128, isize, I256));

impl ParseUnits {
    /// Parses a decimal number and multiplies it with 10^units.
    ///
    /// See [`parse_units`] for more information.
    pub fn parse_units(amount: &str, units: Units) -> Result<Self, UnitsError> {
        let exponent = units.get() as usize;

        let mut amount = amount.to_string();
        let negative = amount.chars().next() == Some('-');
        let dec_len = if let Some(di) = amount.find('.') {
            amount.remove(di);
            amount[di..].len()
        } else {
            0
        };
        let amount = amount.as_str();

        if dec_len > exponent {
            // Truncate the decimal part if it is longer than the exponent
            let amount = &amount[..(amount.len() - (dec_len - exponent) as usize)];
            if negative {
                // Edge case: We have removed the entire number and only the negative sign is left.
                //            Return 0 as a I256 given the input was signed.
                if amount == "-" {
                    Ok(Self::I256(I256::ZERO))
                } else {
                    Ok(Self::I256(I256::from_dec_str(amount)?))
                }
            } else {
                Ok(Self::U256(U256::from_str_radix(amount, 10)?))
            }
        } else if negative {
            // Edge case: Only a negative sign was given, return 0 as a I256 given the input was
            // signed.
            if amount == "-" {
                Ok(Self::I256(I256::ZERO))
            } else {
                let mut n = I256::from_dec_str(&amount)?;
                n *= I256::try_from(10u8)
                    .unwrap()
                    .checked_pow(U256::from(exponent - dec_len))
                    .ok_or(UnitsError::ParseSigned(ParseSignedError::IntegerOverflow))?;
                Ok(Self::I256(n))
            }
        } else {
            let mut a_uint = U256::from_str_radix(&amount, 10)?;
            a_uint *= U256::from(10)
                .checked_pow(U256::from(exponent - dec_len))
                .ok_or(UnitsError::ParseSigned(ParseSignedError::IntegerOverflow))?;
            Ok(Self::U256(a_uint))
        }
    }

    /// Formats the given number of Wei as the given unit.
    ///
    /// See [`format_units`] for more information.
    pub fn format_units(&self, units: Units) -> String {
        let units = units.get() as usize;
        let exp10 = U256::from(10).pow(U256::from(units));

        // TODO: `decimals` are formatted twice because U256 does not support alignment
        // (`:0>width`).
        match *self {
            Self::U256(amount) => {
                let integer = amount / exp10;
                let decimals = (amount % exp10).to_string();
                format!("{integer}.{decimals:0>units$}")
            }
            Self::I256(amount) => {
                let exp10 = I256::from_raw(exp10);
                let sign = if amount.is_negative() { "-" } else { "" };
                let integer = (amount / exp10).twos_complement();
                let decimals = ((amount % exp10).twos_complement()).to_string();
                format!("{sign}{integer}.{decimals:0>units$}")
            }
        }
    }

    /// Returns `true` if the number is negative.
    #[inline]
    pub fn is_negative(&self) -> bool {
        match self {
            Self::U256(_) => false,
            Self::I256(n) => n.is_negative(),
        }
    }

    /// Returns `true` if the number is positive.
    #[inline]
    pub fn is_positive(&self) -> bool {
        match self {
            Self::U256(_) => true,
            Self::I256(n) => n.is_positive(),
        }
    }

    /// Returns `true` if the number is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        match self {
            Self::U256(n) => n.is_zero(),
            Self::I256(n) => n.is_zero(),
        }
    }

    /// Returns the absolute value of the number.
    #[inline]
    pub fn get_absolute(self) -> U256 {
        match self {
            Self::U256(n) => n,
            Self::I256(n) => n.into_raw(),
        }
    }
}

/// Ethereum unit. Always less than [`77`](Units::MAX).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Units(u8);

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl TryFrom<u8> for Units {
    type Error = UnitsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or_else(|| UnitsError::InvalidUnit(value.to_string()))
    }
}

impl TryFrom<String> for Units {
    type Error = UnitsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'a> TryFrom<&'a String> for Units {
    type Error = UnitsError;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Units {
    type Error = UnitsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl core::str::FromStr for Units {
    type Err = UnitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "eth" | "ether" => Self::ETHER,
            "pwei" | "milli" | "milliether" | "finney" => Self::PWEI,
            "twei" | "micro" | "microether" | "szabo" => Self::TWEI,
            "gwei" | "nano" | "nanoether" | "shannon" => Self::GWEI,
            "mwei" | "pico" | "picoether" | "lovelace" => Self::MWEI,
            "kwei" | "femto" | "femtoether" | "babbage" => Self::KWEI,
            "wei" => Self::WEI,
            _ => return Err(UnitsError::InvalidUnit(s.to_string())),
        })
    }
}

impl Units {
    /// Wei is equivalent to 1 wei.
    pub const WEI: Self = unsafe { Self::new_unchecked(0) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::WEI` instead")]
    pub const Wei: Self = Self::WEI;

    /// Kwei is equivalent to 1e3 wei.
    pub const KWEI: Self = unsafe { Self::new_unchecked(3) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::KWEI` instead")]
    pub const Kwei: Self = Self::KWEI;

    /// Mwei is equivalent to 1e6 wei.
    pub const MWEI: Self = unsafe { Self::new_unchecked(6) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::MWEI` instead")]
    pub const Mwei: Self = Self::MWEI;

    /// Gwei is equivalent to 1e9 wei.
    pub const GWEI: Self = unsafe { Self::new_unchecked(9) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::GWEI` instead")]
    pub const Gwei: Self = Self::GWEI;

    /// Twei is equivalent to 1e12 wei.
    pub const TWEI: Self = unsafe { Self::new_unchecked(12) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::TWEI` instead")]
    pub const Twei: Self = Self::TWEI;

    /// Pwei is equivalent to 1e15 wei.
    pub const PWEI: Self = unsafe { Self::new_unchecked(15) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::PWEI` instead")]
    pub const Pwei: Self = Self::PWEI;

    /// Ether is equivalent to 1e18 wei.
    pub const ETHER: Self = unsafe { Self::new_unchecked(18) };
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `Units::ETHER` instead")]
    pub const Ether: Self = Self::ETHER;

    /// The smallest unit.
    pub const MIN: Self = Self::WEI;
    /// The largest unit.
    pub const MAX: Self = unsafe { Self::new_unchecked(77) };

    /// Creates a new `Units` instance, checking for overflow.
    #[inline]
    pub const fn new(units: u8) -> Option<Self> {
        if units <= Self::MAX.get() {
            // SAFETY: `units` is contained in the valid range.
            Some(unsafe { Self::new_unchecked(units) })
        } else {
            None
        }
    }

    /// Creates a new `Units` instance.
    ///
    /// # Safety
    ///
    /// `x` must be less than [`Units::MAX`].
    #[inline]
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        Self(x)
    }

    /// Returns `10^self`, which is the number of Wei in this unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use alloy_primitives::{utils::Units, U256};
    ///
    /// assert_eq!(U256::from(1u128), Units::WEI.wei());
    /// assert_eq!(U256::from(1_000u128), Units::KWEI.wei());
    /// assert_eq!(U256::from(1_000_000u128), Units::MWEI.wei());
    /// assert_eq!(U256::from(1_000_000_000u128), Units::GWEI.wei());
    /// assert_eq!(U256::from(1_000_000_000_000u128), Units::TWEI.wei());
    /// assert_eq!(U256::from(1_000_000_000_000_000u128), Units::PWEI.wei());
    /// assert_eq!(U256::from(1_000_000_000_000_000_000u128), Units::ETHER.wei());
    /// ```
    #[inline]
    pub fn wei(self) -> U256 {
        // TODO(MSRV-1.67): Replace with `usize::MAX.ilog10()`
        const MAX_USIZE_EXP: u8 = if cfg!(target_pointer_width = "16") {
            4
        } else if cfg!(target_pointer_width = "32") {
            9
        } else if cfg!(target_pointer_width = "64") {
            19
        } else {
            38
        };
        if self.get() <= MAX_USIZE_EXP {
            U256::from(10usize.pow(self.get() as u32))
        } else {
            U256::from(10u8).pow(U256::from(self.get()))
        }
    }

    /// Returns the numeric value of the unit.
    #[inline]
    pub const fn get(self) -> u8 {
        self.0
    }

    #[doc(hidden)]
    #[deprecated(since = "0.5.0", note = "use `get` instead")]
    pub const fn as_num(&self) -> u8 {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_values() {
        assert_eq!(Units::WEI.get(), 0);
        assert_eq!(Units::KWEI.get(), 3);
        assert_eq!(Units::MWEI.get(), 6);
        assert_eq!(Units::GWEI.get(), 9);
        assert_eq!(Units::TWEI.get(), 12);
        assert_eq!(Units::PWEI.get(), 15);
        assert_eq!(Units::ETHER.get(), 18);
        assert_eq!(Units::new(10).unwrap().get(), 10);
        assert_eq!(Units::new(20).unwrap().get(), 20);
    }

    #[test]
    fn parse() {
        assert_eq!(Units::try_from("wei").unwrap(), Units::WEI);
        assert_eq!(Units::try_from("kwei").unwrap(), Units::KWEI);
        assert_eq!(Units::try_from("mwei").unwrap(), Units::MWEI);
        assert_eq!(Units::try_from("gwei").unwrap(), Units::GWEI);
        assert_eq!(Units::try_from("twei").unwrap(), Units::TWEI);
        assert_eq!(Units::try_from("pwei").unwrap(), Units::PWEI);
        assert_eq!(Units::try_from("ether").unwrap(), Units::ETHER);
    }
}
