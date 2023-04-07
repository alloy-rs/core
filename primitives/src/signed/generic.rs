use core::{cmp, fmt, iter, ops, str::FromStr};
use ruint::Uint;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use crate::BigIntConversionError;

use super::{errors, Sign};

/// Panic if overflow on debug mode.
#[inline(always)]
#[track_caller]
fn handle_overflow<const BITS: usize, const LIMBS: usize>(
    (result, overflow): (Signed<BITS, LIMBS>, bool),
) -> Signed<BITS, LIMBS> {
    debug_assert!(!overflow, "overflow");
    result
}

/// Compute the two's complement of a U256.
#[inline(always)]
fn twos_complement<const BITS: usize, const LIMBS: usize>(
    u: Uint<BITS, LIMBS>,
) -> Uint<BITS, LIMBS> {
    (!u).overflowing_add(Uint::<BITS, LIMBS>::from(1)).0
}

/// Compile-time equality of signed integers.
#[inline(always)]
pub const fn const_eq<const BITS: usize, const LIMBS: usize>(
    left: Signed<BITS, LIMBS>,
    right: Signed<BITS, LIMBS>,
) -> bool {
    let mut i = 0;
    let llimbs = left.0.as_limbs();
    let rlimbs = right.0.as_limbs();
    loop {
        if llimbs[i] != rlimbs[i] {
            return false;
        }
        i += 1;
        if i == LIMBS {
            break;
        }
    }
    true
}

/// Compute the max value at compile time
const fn max<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    let mut limbs = [u64::MAX; LIMBS];
    limbs[LIMBS - 1] &= Signed::<BITS, LIMBS>::MASK; // unset all high bits
    limbs[LIMBS - 1] &= !Signed::<BITS, LIMBS>::SIGN_BIT; // unset the sign bit
    Signed(Uint::from_limbs(limbs))
}

const fn min<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    let mut limbs = [0; LIMBS];
    limbs[LIMBS - 1] = Signed::<BITS, LIMBS>::SIGN_BIT;
    Signed(Uint::from_limbs(limbs))
}

const fn zero<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    let limbs = [0; LIMBS];
    Signed(Uint::from_limbs(limbs))
}

const fn one<const BITS: usize, const LIMBS: usize>() -> Signed<BITS, LIMBS> {
    let mut limbs = [0; LIMBS];
    limbs[0] = 1;
    Signed(Uint::from_limbs(limbs))
}

/// Location of the sign bit within the highest limb.
const fn sign_bit(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        1 << 63
    } else {
        1 << (bits - 1)
    }
}

/// Mask to apply to the highest limb to get the correct number of bits.
#[must_use]
const fn mask(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        u64::MAX
    } else {
        (1 << bits) - 1
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
/// Signed integer wrapping a `ruint::Uint`.
pub struct Signed<const BITS: usize, const LIMBS: usize>(pub(crate) Uint<BITS, LIMBS>);

// formatting
impl<const BITS: usize, const LIMBS: usize> fmt::Debug for Signed<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Display for Signed<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        fmt::Display::fmt(&sign, f)?;
        write!(f, "{abs}")
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::LowerHex for Signed<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        fmt::Display::fmt(&sign, f)?;
        write!(f, "{abs:x}")
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::UpperHex for Signed<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        fmt::Display::fmt(&sign, f)?;

        // NOTE: Work around `U256: !UpperHex`.
        let mut buffer = format!("{abs:x}");
        buffer.make_ascii_uppercase();
        f.write_str(&buffer)
    }
}

// cmp
impl<const BITS: usize, const LIMBS: usize> cmp::PartialOrd for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize, const LIMBS: usize> cmp::Ord for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // TODO(nlordell): Once subtraction is implemented:
        // self.saturating_sub(*other).signum64().partial_cmp(&0)

        use cmp::Ordering::*;
        use Sign::*;

        match (self.into_sign_and_abs(), other.into_sign_and_abs()) {
            ((Positive, _), (Negative, _)) => Greater,
            ((Negative, _), (Positive, _)) => Less,
            ((Positive, this), (Positive, other)) => this.cmp(&other),
            ((Negative, this), (Negative, other)) => other.cmp(&this),
        }
    }
}

// arithmetic ops - implemented above
impl<T, const BITS: usize, const LIMBS: usize> ops::Add<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_add(rhs.into()))
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::AddAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Sub<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_sub(rhs.into()))
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::SubAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Mul<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_mul(rhs.into()))
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::MulAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Div<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_div(rhs.into()))
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::DivAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::Rem<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    type Output = Self;

    #[track_caller]
    fn rem(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_rem(rhs.into()))
    }
}

impl<T, const BITS: usize, const LIMBS: usize> ops::RemAssign<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs;
    }
}

impl<T, const BITS: usize, const LIMBS: usize> iter::Sum<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Signed::zero(), |acc, x| acc + x)
    }
}

impl<T, const BITS: usize, const LIMBS: usize> iter::Product<T> for Signed<BITS, LIMBS>
where
    T: Into<Signed<BITS, LIMBS>>,
{
    #[track_caller]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Signed::one(), |acc, x| acc * x)
    }
}

// bitwise ops - delegated to U256
impl<const BITS: usize, const LIMBS: usize> ops::BitAnd for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Signed(self.0 & rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitAndAssign for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitOr for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Signed(self.0 | rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitOrAssign for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitXor for Signed<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Signed(self.0 ^ rhs.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::BitXorAssign for Signed<BITS, LIMBS> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

// unary ops
impl<const BITS: usize, const LIMBS: usize> ops::Neg for Signed<BITS, LIMBS> {
    type Output = Signed<BITS, LIMBS>;

    #[inline(always)]
    #[track_caller]
    fn neg(self) -> Self::Output {
        handle_overflow(self.overflowing_neg())
    }
}

impl<const BITS: usize, const LIMBS: usize> ops::Not for Signed<BITS, LIMBS> {
    type Output = Signed<BITS, LIMBS>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Signed(!self.0)
    }
}

impl<const BITS: usize, const LIMBS: usize> Signed<BITS, LIMBS> {
    /// Mask for the highest limb
    const MASK: u64 = mask(BITS);

    /// Location of the sign bit within the highest limb
    const SIGN_BIT: u64 = sign_bit(BITS);

    /// Number of bits
    pub const BITS: usize = BITS;

    /// The minimum value
    pub const MIN: Self = min();

    /// The maximum value
    pub const MAX: Self = max();

    /// Zero (additive identity) of this type.
    pub const ZERO: Self = zero();

    /// One (multiplicative identity) of this type.
    pub const ONE: Self = one();

    /// Minus one (multiplicative inverse) of this type.
    pub const MINUS_ONE: Self = Self(Uint::<BITS, LIMBS>::MAX);

    /// Zero (additive iden
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// One (multiplicative identity) of this type.
    #[inline(always)]
    pub const fn one() -> Self {
        Self::ONE
    }

    /// Minus one (multiplicative inverse) of this type.
    #[inline(always)]
    pub const fn minus_one() -> Self {
        Self::MINUS_ONE
    }

    /// The maximum value which can be inhabited by this type.
    #[inline(always)]
    pub const fn max_value() -> Self {
        Self::MAX
    }

    /// The minimum value which can be inhabited by this type.
    #[inline(always)]
    pub const fn min_value() -> Self {
        Self::MIN
    }

    /// Coerces an unsigned integer into a signed one. If the unsigned integer
    /// is greater than the greater than or equal to `1 << 255`, then the result
    /// will overflow into a negative value.
    #[inline(always)]
    pub const fn from_raw(val: Uint<BITS, LIMBS>) -> Self {
        Self(val)
    }

    /// Attempt to perform the conversion via a `TryInto` implementation, and
    /// panic on failure
    ///
    /// This is a shortcut for `val.try_into().unwrap()`
    #[inline(always)]
    pub fn unchecked_from<T>(val: T) -> Self
    where
        T: TryInto<Self>,
        <T as TryInto<Self>>::Error: fmt::Debug,
    {
        val.try_into().unwrap()
    }

    /// Attempt to perform the conversion via a `TryInto` implementation, and
    /// panic on failure
    ///
    /// This is a shortcut for `self.try_into().unwrap()`
    #[inline(always)]
    pub fn unchecked_into<T>(self) -> T
    where
        Self: TryInto<T>,
        <Self as TryInto<T>>::Error: fmt::Debug,
    {
        self.try_into().unwrap()
    }

    /// Returns the signed integer as a unsigned integer. If the value of `self` negative, then the
    /// two's complement of its absolute value will be returned.
    #[inline(always)]
    pub const fn into_raw(self) -> Uint<BITS, LIMBS> {
        self.0
    }

    /// Returns the sign of self.
    #[inline(always)]
    pub const fn sign(self) -> Sign {
        // if the last limb contains the sign bit, then we're negative
        // because we can't set any higher bits to 1, we use >= as a proxy
        // check to avoid bit comparison
        if let Some(limb) = self.0.as_limbs().last() {
            if *limb >= Self::SIGN_BIT {
                return Sign::Negative;
            }
        }
        Sign::Positive
    }

    /// Returns `true` if `self` is zero and `false` if the number is negative
    /// or positive.
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        const_eq(self, Self::ZERO)
    }

    /// Returns `true` if `self` is positive and `false` if the number is zero
    /// or negative
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        !self.is_zero() && matches!(self.sign(), Sign::Positive)
    }

    /// Returns `true` if `self` is negative and `false` if the number is zero
    /// or positive
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        matches!(self.sign(), Sign::Negative)
    }

    /// Returns the number of ones in the binary representation of `self`.
    #[inline(always)]
    pub fn count_ones(&self) -> usize {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representation of `self`.
    #[inline(always)]
    pub fn count_zeros(&self) -> usize {
        self.0.count_zeros()
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn leading_zeros(&self) -> usize {
        self.0.leading_zeros()
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn trailing_zeros(&self) -> usize {
        self.0.trailing_zeros()
    }

    /// Returns the number of leading ones in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn trailing_ones(&self) -> usize {
        self.0.trailing_ones()
    }

    /// Return if specific bit is set.
    ///
    /// # Panics
    ///
    /// If index exceeds the bit width of the number.
    #[inline(always)]
    #[track_caller]
    pub const fn bit(&self, index: usize) -> bool {
        self.0.bit(index)
    }

    /// Return specific byte.
    ///
    /// # Panics
    ///
    /// If index exceeds the byte width of the number.
    #[inline(always)]
    #[track_caller]
    pub const fn byte(&self, index: usize) -> u8 {
        let limbs = self.0.as_limbs();
        match index {
            0..=7 => limbs[3].to_be_bytes()[index],
            8..=15 => limbs[2].to_be_bytes()[index - 8],
            16..=23 => limbs[1].to_be_bytes()[index - 16],
            24..=31 => limbs[0].to_be_bytes()[index - 24],
            _ => panic!(),
        }
    }

    /// Return the least number of bits needed to represent the number
    #[inline(always)]
    pub fn bits(self) -> u32 {
        let unsigned = self.unsigned_abs();
        let unsigned_bits = unsigned.bit_len();

        // NOTE: We need to deal with two special cases:
        //   - the number is 0
        //   - the number is a negative power of `2`. These numbers are written as `0b11..1100..00`.
        //   In the case of a negative power of two, the number of bits required
        //   to represent the negative signed value is equal to the number of
        //   bits required to represent its absolute value as an unsigned
        //   integer. This is best illustrated by an example: the number of bits
        //   required to represent `-128` is `8` since it is equal to `i8::MIN`
        //   and, therefore, obviously fits in `8` bits. This is equal to the
        //   number of bits required to represent `128` as an unsigned integer
        //   (which fits in a `u8`).  However, the number of bits required to
        //   represent `128` as a signed integer is `9`, as it is greater than
        //   `i8::MAX`.  In the general case, an extra bit is needed to
        //   represent the sign.
        let bits = if self.count_zeros() == self.trailing_zeros() {
            // `self` is zero or a negative power of two
            unsigned_bits
        } else {
            unsigned_bits + 1
        };

        bits as _
    }

    /// Creates a `Signed` from a sign and an absolute value. Returns the value
    /// and a bool that is true if the conversion caused an overflow.
    #[inline(always)]
    pub fn overflowing_from_sign_and_abs(sign: Sign, abs: Uint<BITS, LIMBS>) -> (Self, bool) {
        let value = Self(match sign {
            Sign::Positive => abs,
            Sign::Negative => twos_complement(abs),
        });

        (value, value.sign() != sign)
    }

    /// Creates a `Signed` from an absolute value and a negative flag. Returns
    /// `None` if it would overflow as `Signed`.
    #[inline(always)]
    pub fn checked_from_sign_and_abs(sign: Sign, abs: Uint<BITS, LIMBS>) -> Option<Self> {
        let (result, overflow) = Self::overflowing_from_sign_and_abs(sign, abs);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    /// Convert from a decimal string.
    pub fn from_dec_str(value: &str) -> Result<Self, errors::ParseSignedError> {
        let (sign, value) = match value.as_bytes().first() {
            Some(b'+') => (Sign::Positive, &value[1..]),
            Some(b'-') => (Sign::Negative, &value[1..]),
            _ => (Sign::Positive, value),
        };
        let abs = Uint::<BITS, LIMBS>::from_str_radix(value, 10)?;
        Self::checked_from_sign_and_abs(sign, abs).ok_or(errors::ParseSignedError::IntegerOverflow)
    }

    /// Convert from a hex string.
    pub fn from_hex_str(value: &str) -> Result<Self, errors::ParseSignedError> {
        let (sign, value) = match value.as_bytes().first() {
            Some(b'+') => (Sign::Positive, &value[1..]),
            Some(b'-') => (Sign::Negative, &value[1..]),
            _ => (Sign::Positive, value),
        };

        let value = value.strip_prefix("0x").unwrap_or(value);

        if value.len() > 64 {
            return Err(errors::ParseSignedError::IntegerOverflow);
        }

        let abs = Uint::<BITS, LIMBS>::from_str_radix(value, 16)?;
        Self::checked_from_sign_and_abs(sign, abs).ok_or(errors::ParseSignedError::IntegerOverflow)
    }

    /// Splits a Signed into its absolute value and negative flag.
    #[inline(always)]
    pub fn into_sign_and_abs(self) -> (Sign, Uint<BITS, LIMBS>) {
        let sign = self.sign();
        let abs = match sign {
            Sign::Positive => self.0,
            Sign::Negative => twos_complement(self.0),
        };
        (sign, abs)
    }

    /// Convert to a slice in BE format
    ///
    /// # Panics
    ///
    /// If the given slice is not exactly 32 bytes long.
    #[inline(always)]
    #[track_caller]
    pub fn to_big_endian(self) -> [u8; 32] {
        self.0.to_be_bytes()
    }

    /// Convert to a slice in LE format
    ///
    /// # Panics
    ///
    /// If the given slice is not exactly 32 bytes long.
    #[inline(always)]
    #[track_caller]
    pub fn to_little_endian(self) -> [u8; 32] {
        self.0.to_le_bytes()
    }
}

// ops impl
impl<const BITS: usize, const LIMBS: usize> Signed<BITS, LIMBS> {
    /// Computes the absolute value of `self`.
    ///
    /// # Overflow behavior
    ///
    /// The absolute value of `Self::MIN` cannot be represented as `Self` and
    /// attempting to calculate it will cause an overflow. This means that code
    /// in debug mode will trigger a panic on this case and optimized code will
    /// return `Self::MIN` without a panic.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn abs(self) -> Self {
        handle_overflow(self.overflowing_abs())
    }

    /// Computes the absolute value of `self`.
    ///
    /// Returns a tuple of the absolute version of self along with a boolean indicating whether an
    /// overflow happened. If self is the minimum value then the minimum value will be returned
    /// again and true will be returned for an overflow happening.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_abs(self) -> (Self, bool) {
        if self == Self::MIN {
            (self, true)
        } else {
            (Self(self.unsigned_abs()), false)
        }
    }

    /// Checked absolute value. Computes `self.abs()`, returning `None` if `self == MIN`.
    #[inline(always)]
    #[must_use]
    pub fn checked_abs(self) -> Option<Self> {
        match self.overflowing_abs() {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating absolute value. Computes `self.abs()`, returning `MAX` if `self == MIN` instead
    /// of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_abs(self) -> Self {
        match self.overflowing_abs() {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    /// Wrapping absolute value. Computes `self.abs()`, wrapping around at the boundary of the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }

    /// Computes the absolute value of `self` without any wrapping or panicking.
    #[inline(always)]
    #[must_use]
    pub fn unsigned_abs(self) -> Uint<BITS, LIMBS> {
        self.into_sign_and_abs().1
    }

    /// Negates self, overflowing if this is equal to the minimum value.
    ///
    /// Returns a tuple of the negated version of self along with a boolean indicating whether an
    /// overflow happened. If `self` is the minimum value, then the minimum value will be returned
    /// again and `true` will be returned for an overflow happening.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_neg(self) -> (Self, bool) {
        if self == Self::MIN {
            (self, true)
        } else {
            (Self(twos_complement(self.0)), false)
        }
    }

    /// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
    #[inline(always)]
    #[must_use]
    pub fn checked_neg(self) -> Option<Self> {
        match self.overflowing_neg() {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating negation. Computes `-self`, returning `MAX` if `self == MIN` instead of
    /// overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_neg(self) -> Self {
        match self.overflowing_neg() {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    /// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary of the type.
    ///
    /// The only case where such wrapping can occur is when one negates `MIN` on a signed type
    /// (where `MIN` is the negative minimal value for the type); this is a positive value that is
    /// too large to represent in the type. In such a case, this function returns `MIN` itself.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    /// Calculates `self` + `rhs`
    ///
    /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (unsigned, _) = self.0.overflowing_add(rhs.0);
        let result = Self(unsigned);

        // NOTE: Overflow is determined by checking the sign of the operands and
        //   the result.
        let overflow = matches!(
            (self.sign(), rhs.sign(), result.sign()),
            (Sign::Positive, Sign::Positive, Sign::Negative)
                | (Sign::Negative, Sign::Negative, Sign::Positive)
        );

        (result, overflow)
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.overflowing_add(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at the numeric bounds instead
    /// of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_add(rhs);
        if overflow {
            match result.sign() {
                Sign::Positive => Self::MIN,
                Sign::Negative => Self::MAX,
            }
        } else {
            result
        }
    }

    /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the boundary of the
    /// type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    /// Calculates `self` - `rhs`
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        // NOTE: We can't just compute the `self + (-rhs)` because `-rhs` does
        //   not always exist, specifically this would be a problem in case
        //   `rhs == Self::MIN`

        let (unsigned, _) = self.0.overflowing_sub(rhs.0);
        let result = Self(unsigned);

        // NOTE: Overflow is determined by checking the sign of the operands and
        //   the result.
        let overflow = matches!(
            (self.sign(), rhs.sign(), result.sign()),
            (Sign::Positive, Sign::Negative, Sign::Negative)
                | (Sign::Negative, Sign::Positive, Sign::Positive)
        );

        (result, overflow)
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.overflowing_sub(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_sub(rhs);
        if overflow {
            match result.sign() {
                Sign::Positive => Self::MIN,
                Sign::Negative => Self::MAX,
            }
        } else {
            result
        }
    }

    /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of
    /// the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    /// Calculates `self` * `rhs`
    ///
    /// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        if self.is_zero() || rhs.is_zero() {
            return (Self::ZERO, false);
        }
        let sign = self.sign() * rhs.sign();
        let (unsigned, overflow_mul) = self.unsigned_abs().overflowing_mul(rhs.unsigned_abs());
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_mul || overflow_conv)
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning None if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating integer multiplication. Computes `self * rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_mul(rhs);
        if overflow {
            match self.sign() * rhs.sign() {
                Sign::Positive => Self::MAX,
                Sign::Negative => Self::MIN,
            }
        } else {
            result
        }
    }

    /// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at the boundary of
    /// the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// Calculates `self` / `rhs`
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would occur then self is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            panic!("attempt to divide by zero");
        }
        let sign = self.sign() * rhs.sign();
        // Note, signed division can't overflow!
        let unsigned = self.unsigned_abs() / rhs.unsigned_abs();
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_conv && !result.is_zero())
    }

    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0` or the
    /// division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::min_value() && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.overflowing_div(rhs).0)
        }
    }

    /// Saturating integer division. Computes `self / rhs`, saturating at the numeric bounds instead
    /// of overflowing.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn saturating_div(self, rhs: Self) -> Self {
        match self.overflowing_div(rhs) {
            (value, false) => value,
            // MIN / -1 is the only possible saturating overflow
            _ => Self::MAX,
        }
    }

    /// Wrapping (modular) division. Computes `self / rhs`, wrapping around at the boundary of the
    /// type.
    ///
    /// The only case where such wrapping can occur is when one divides `MIN / -1` on a signed type
    /// (where `MIN` is the negative minimal value for the type); this is equivalent to `-MIN`, a
    /// positive value that is too large to represent in the type. In such a case, this function
    /// returns `MIN` itself.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    /// Calculates `self` % `rhs`
    ///
    /// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
    /// arithmetic overflow would occur. If an overflow would occur then 0 is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if self == Self::MIN && rhs == Self::minus_one() {
            (Self::zero(), true)
        } else {
            let div_res = self / rhs;
            (self - div_res * rhs, false)
        }
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs == 0` or the
    /// division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::MIN && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.overflowing_rem(rhs).0)
        }
    }

    /// Wrapping (modular) remainder. Computes `self % rhs`, wrapping around at the boundary of the
    /// type.
    ///
    /// Such wrap-around never actually occurs mathematically; implementation artifacts make `x % y`
    /// invalid for `MIN / -1` on a signed type (where `MIN` is the negative minimal value). In such
    /// a case, this function returns `0`.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }

    /// Calculates the quotient of Euclidean division of `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with
    /// `r = self.rem_euclid(rhs)` and `0 <= r < abs(rhs)`.
    ///
    /// In other words, the result is `self / rhs` rounded to the integer `q` such that `self >= q *
    /// rhs`.
    /// If `self > 0`, this is equal to round towards zero (the default in Rust);
    /// if `self < 0`, this is equal to round towards +/- infinity.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0 or the division results in overflow.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        let q = self / rhs;
        if (self % rhs).is_negative() {
            if rhs.is_positive() {
                q - Self::one()
            } else {
                q + Self::one()
            }
        } else {
            q
        }
    }

    /// Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would occur then `self` is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        if self == Self::min_value() && rhs == Self::minus_one() {
            (self, true)
        } else {
            (self.div_euclid(rhs), false)
        }
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None` if `rhs == 0`
    /// or the division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::min_value() && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.div_euclid(rhs))
        }
    }

    /// Wrapping Euclidean division. Computes `self.div_euclid(rhs)`,
    /// wrapping around at the boundary of the type.
    ///
    /// Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is the negative minimal
    /// value for the type). This is equivalent to `-MIN`, a positive value that is too large to
    /// represent in the type. In this case, this method returns `MIN` itself.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidean division algorithm -- given `r = self.rem_euclid(rhs)`,
    /// `self = rhs * self.div_euclid(rhs) + r`, and `0 <= r < abs(rhs)`.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0 or the division results in overflow.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;
        if r < Self::zero() {
            if rhs < Self::zero() {
                r - rhs
            } else {
                r + rhs
            }
        } else {
            r
        }
    }

    /// Overflowing Euclidean remainder. Calculates `self.rem_euclid(rhs)`.
    ///
    /// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
    /// arithmetic overflow would occur. If an overflow would occur then 0 is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        if self == Self::min_value() && rhs == Self::minus_one() {
            (Self::zero(), true)
        } else {
            (self.rem_euclid(rhs), false)
        }
    }

    /// Wrapping Euclidean remainder. Computes `self.rem_euclid(rhs)`, wrapping around at the
    /// boundary of the type.
    ///
    /// Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the negative minimal
    /// value for the type). In this case, this method returns 0.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
    }

    /// Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`, returning `None` if `rhs == 0`
    /// or the division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::min_value() && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.rem_euclid(rhs))
        }
    }

    /// Returns the sign of `self` to the exponent `exp`.
    ///
    /// Note that this method does not actually try to compute the `self` to the
    /// exponent `exp`, but instead uses the property that a negative number to
    /// an odd exponent will be negative. This means that the sign of the result
    /// of exponentiation can be computed even if the actual result is too large
    /// to fit in 256-bit signed integer.
    #[inline(always)]
    const fn pow_sign(self, exp: u32) -> Sign {
        let is_exp_odd = exp % 2 != 0;
        if is_exp_odd && self.is_negative() {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    /// Create `10**n` as this type.
    ///
    /// # Panics
    ///
    /// If the result overflows the type.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn exp10(n: usize) -> Self {
        Uint::<BITS, LIMBS>::from(10)
            .pow(Uint::from(n))
            .try_into()
            .expect("overflow")
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Panics
    ///
    /// If the result overflows the type in debug mode.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        handle_overflow(self.overflowing_pow(exp))
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// Returns a tuple of the exponentiation along with a bool indicating whether an overflow
    /// happened.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        let sign = self.pow_sign(exp);
        let (unsigned, overflow_pow) = self
            .unsigned_abs()
            .overflowing_pow(Uint::<BITS, LIMBS>::from(exp));
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_pow || overflow_conv)
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_pow(self, exp: u32) -> Option<Self> {
        let (result, overflow) = self.overflowing_pow(exp);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    /// Saturating integer exponentiation. Computes `self.pow(exp)`, saturating at the numeric
    /// bounds instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_pow(self, exp: u32) -> Self {
        let (result, overflow) = self.overflowing_pow(exp);
        if overflow {
            match self.pow_sign(exp) {
                Sign::Positive => Self::MAX,
                Sign::Negative => Self::MIN,
            }
        } else {
            result
        }
    }

    /// Raises self to the power of `exp`, wrapping around at the
    /// boundary of the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_pow(self, exp: u32) -> Self {
        self.overflowing_pow(exp).0
    }

    /// Shifts self left by `rhs` bits.
    ///
    /// Returns a tuple of the shifted version of self along with a boolean indicating whether the
    /// shift value was larger than or equal to the number of bits.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_shl(self, rhs: usize) -> (Self, bool) {
        if rhs >= 256 {
            (Self::zero(), true)
        } else {
            (Self(self.0 << rhs), false)
        }
    }

    /// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger than or
    /// equal to the number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn checked_shl(self, rhs: usize) -> Option<Self> {
        match self.overflowing_shl(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Wrapping shift left. Computes `self << rhs`, returning 0 if larger than or equal to the
    /// number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_shl(self, rhs: usize) -> Self {
        self.overflowing_shl(rhs).0
    }

    /// Shifts self right by `rhs` bits.
    ///
    /// Returns a tuple of the shifted version of self along with a boolean indicating whether the
    /// shift value was larger than or equal to the number of bits.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_shr(self, rhs: usize) -> (Self, bool) {
        if rhs >= 256 {
            (Self::zero(), true)
        } else {
            (Self(self.0 >> rhs), false)
        }
    }

    /// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is larger than or
    /// equal to the number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn checked_shr(self, rhs: usize) -> Option<Self> {
        match self.overflowing_shr(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Wrapping shift right. Computes `self >> rhs`, returning 0 if larger than or equal to the
    /// number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_shr(self, rhs: usize) -> Self {
        self.overflowing_shr(rhs).0
    }

    /// Arithmetic shift right operation. Computes `self >> rhs` maintaining the original sign. If
    /// the number is positive this is the same as logic shift right.
    #[inline(always)]
    #[must_use]
    pub fn asr(self, rhs: usize) -> Self {
        // Avoid shifting if we are going to know the result regardless of the value.
        if rhs == 0 {
            return self;
        }

        if rhs >= BITS - 1 {
            match self.sign() {
                Sign::Positive => return Self::zero(),
                Sign::Negative => return Self::minus_one(),
            }
        }

        match self.sign() {
            // Perform the shift.
            Sign::Positive => self.wrapping_shr(rhs),
            Sign::Negative => {
                // We need to do: `for 0..shift { self >> 1 | 2^255 }`
                // We can avoid the loop by doing: `self >> shift | ~(2^(255 - shift) - 1)`
                // where '~' represents ones complement
                let two: Uint<BITS, LIMBS> = Uint::from(2);
                let bitwise_or = Self::from_raw(
                    !(two.pow(Uint::<BITS, LIMBS>::from(BITS - rhs))
                        - Uint::<BITS, LIMBS>::from(1)),
                );
                (self.wrapping_shr(rhs)) | bitwise_or
            }
        }
    }

    /// Arithmetic shift left operation. Computes `self << rhs`, checking for overflow on the final
    /// result.
    ///
    /// Returns `None` if the operation overflowed (most significant bit changes).
    #[inline(always)]
    #[must_use]
    pub fn asl(self, rhs: usize) -> Option<Self> {
        if rhs == 0 {
            Some(self)
        } else {
            let result = self.wrapping_shl(rhs);
            if result.sign() != self.sign() {
                // Overflow occurred
                None
            } else {
                Some(result)
            }
        }
    }

    /// Compute the [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) of this number.
    #[inline(always)]
    #[must_use]
    pub fn twos_complement(self) -> Uint<BITS, LIMBS> {
        let abs = self.into_raw();
        match self.sign() {
            Sign::Positive => abs,
            Sign::Negative => twos_complement(abs),
        }
    }
}

// Implement Shl and Shr only for types <= usize, since U256 uses .as_usize() which panics
macro_rules! impl_shift {
    ($($t:ty),+) => {
        // We are OK with wrapping behaviour here because it's how Rust behaves with the primitive
        // integer types.

        // $t <= usize: cast to usize
        $(
            impl<const BITS: usize, const LIMBS: usize> ops::Shl<$t> for Signed<BITS, LIMBS> {
                type Output = Self;

                #[inline(always)]
                fn shl(self, rhs: $t) -> Self::Output {
                    self.wrapping_shl(rhs as usize)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> ops::ShlAssign<$t> for Signed<BITS, LIMBS> {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: $t) {
                    *self = *self << rhs;
                }
            }

            impl<const BITS: usize, const LIMBS: usize> ops::Shr<$t> for Signed<BITS, LIMBS> {
                type Output = Self;

                #[inline(always)]
                fn shr(self, rhs: $t) -> Self::Output {
                    self.wrapping_shr(rhs as usize)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> ops::ShrAssign<$t> for Signed<BITS, LIMBS> {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: $t) {
                    *self = *self >> rhs;
                }
            }
        )+
    };
}

#[cfg(target_pointer_width = "16")]
impl_shift!(i8, u8, i16, u16, isize, usize);

#[cfg(target_pointer_width = "32")]
impl_shift!(i8, u8, i16, u16, i32, u32, isize, usize);

#[cfg(target_pointer_width = "64")]
impl_shift!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

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
    type Err = errors::ParseSignedError;

    #[inline(always)]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Signed::from_hex_str(value).or_else(|_| Signed::from_dec_str(value))
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for i128 {
    type Error = BigIntConversionError;

    fn try_from(value: Signed<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if value.bits() > 128 {
            return Err(BigIntConversionError);
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
            return Self::try_from(u);
        }

        // This is a bit messy :(
        let tc = (!u).wrapping_add(1);
        let stc = Uint::<128, 2>::saturating_from(tc);
        let (num, overflow) = Uint::<BITS, LIMBS>::overflowing_from_limbs_slice(stc.as_limbs());
        if overflow {
            return Err(BigIntConversionError);
        }
        Ok(Signed(twos_complement(num)))
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<Signed<BITS, LIMBS>> for u128 {
    type Error = BigIntConversionError;

    fn try_from(value: Signed<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if value.is_negative() {
            return Err(BigIntConversionError);
        }

        let saturated = Uint::<BITS, LIMBS>::saturating_from(u128::MAX);

        // if the value is greater than the saturated value, return an error
        if value > Signed(saturated) {
            return Err(BigIntConversionError);
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
            return Err(BigIntConversionError);
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
                    Ok(Signed(Uint::<BITS, LIMBS>::from(value)))
                }
            }

            impl<const BITS: usize, const LIMBS: usize> TryFrom<$i> for Signed<BITS, LIMBS> {
                type Error = BigIntConversionError;

                #[inline(always)]
                fn try_from(value: $i) -> Result<Self, Self::Error> {
                    let uint: $u = value as $u;
                    Ok(Self(if value.is_negative() {
                        let abs = (!uint).wrapping_add(1);
                        twos_complement(Uint::<BITS, LIMBS>::from(abs))
                    } else {
                        Uint::<BITS, LIMBS>::from(uint)
                    }))
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

#[cfg(test)]
mod tests {
    use ruint::{
        aliases::{U128, U160, U192, U256},
        BaseConvertError, ParseError,
    };
    // use serde_json::json;
    use std::ops::Neg;

    use super::*;
    use crate::{
        aliases::{I128, I160, I192, I256},
        ParseSignedError,
    };

    type I96 = Signed<96, 2>;
    type U96 = Uint<96, 2>;

    #[test]
    fn identities() {
        macro_rules! test_identities {
            ($signed:ty, $max:literal, $min:literal) => {
                assert_eq!(<$signed>::zero().to_string(), "0");
                assert_eq!(<$signed>::one().to_string(), "1");
                assert_eq!(<$signed>::minus_one().to_string(), "-1");
                assert_eq!(<$signed>::max_value().to_string(), $max);
                assert_eq!(<$signed>::min_value().to_string(), $min);
            };
        }
        test_identities!(
            I96,
            "39614081257132168796771975167",
            "-39614081257132168796771975168"
        );
        test_identities!(
            I128,
            "170141183460469231731687303715884105727",
            "-170141183460469231731687303715884105728"
        );
        test_identities!(
            I192,
            "3138550867693340381917894711603833208051177722232017256447",
            "-3138550867693340381917894711603833208051177722232017256448"
        );
        test_identities!(
            I256,
            "57896044618658097711785492504343953926634992332820282019728792003956564819967",
            "-57896044618658097711785492504343953926634992332820282019728792003956564819968"
        );
    }

    #[test]
    // #[allow(clippy::cognitive_complexity)]
    fn std_num_conversion() {
        // test conversion from basic types

        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty, $i:ty, $u:ty) => {
                // Test a specific number
                assert_eq!(<$i_struct>::try_from(-42 as $i).unwrap().to_string(), "-42");
                assert_eq!(<$i_struct>::try_from(42 as $i).unwrap().to_string(), "42");
                assert_eq!(<$i_struct>::try_from(42 as $u).unwrap().to_string(), "42");

                if <$u_struct>::BITS as u32 >= <$u>::BITS {
                    assert_eq!(
                        <$i_struct>::try_from(<$i>::MAX).unwrap().to_string(),
                        <$i>::MAX.to_string(),
                    );
                    assert_eq!(
                        <$i_struct>::try_from(<$i>::MIN).unwrap().to_string(),
                        <$i>::MIN.to_string(),
                    );
                } else {
                    assert_eq!(
                        <$i_struct>::try_from(<$i>::MAX).unwrap_err(),
                        BigIntConversionError,
                    );
                }
            };

            ($i_struct:ty, $u_struct:ty) => {
                run_test!($i_struct, $u_struct, i8, u8);
                run_test!($i_struct, $u_struct, i16, u16);
                run_test!($i_struct, $u_struct, i32, u32);
                run_test!($i_struct, $u_struct, i64, u64);
                run_test!($i_struct, $u_struct, i128, u128);
                run_test!($i_struct, $u_struct, isize, usize);
            };
        }

        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn parse_dec_str() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let min_abs: $u_struct = <$i_struct>::MIN.0;
                let unsigned = <$u_struct>::from_str_radix("3141592653589793", 10).unwrap();

                let value = <$i_struct>::from_dec_str(&format!("-{unsigned}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Negative, unsigned));

                let value = <$i_struct>::from_dec_str(&format!("{unsigned}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Positive, unsigned));

                let value = <$i_struct>::from_dec_str(&format!("+{unsigned}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Positive, unsigned));

                let err = <$i_struct>::from_dec_str("invalid string").unwrap_err();
                assert_eq!(
                    err,
                    ParseSignedError::Ruint(ParseError::BaseConvertError(
                        BaseConvertError::InvalidDigit(18, 10)
                    ))
                );

                let err = <$i_struct>::from_dec_str(&format!("1{}", <$u_struct>::MAX)).unwrap_err();
                assert_eq!(err, ParseSignedError::IntegerOverflow);

                let err = <$i_struct>::from_dec_str(&format!("-{}", <$u_struct>::MAX)).unwrap_err();
                assert_eq!(err, ParseSignedError::IntegerOverflow);

                let value = <$i_struct>::from_dec_str(&format!("-{}", min_abs)).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Negative, min_abs));

                let err = <$i_struct>::from_dec_str(&format!("{}", min_abs)).unwrap_err();
                assert_eq!(err, ParseSignedError::IntegerOverflow);
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn parse_hex_str() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let min_abs = <$i_struct>::MIN.0;
                let unsigned = <$u_struct>::from_str_radix("3141592653589793", 10).unwrap();

                let value = <$i_struct>::from_hex_str(&format!("-{unsigned:x}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Negative, unsigned));

                let value = <$i_struct>::from_hex_str(&format!("-0x{unsigned:x}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Negative, unsigned));

                let value = <$i_struct>::from_hex_str(&format!("{unsigned:x}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Positive, unsigned));

                let value = <$i_struct>::from_hex_str(&format!("0x{unsigned:x}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Positive, unsigned));

                let value = <$i_struct>::from_hex_str(&format!("+0x{unsigned:x}")).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Positive, unsigned));

                let err = <$i_struct>::from_hex_str("invalid string").unwrap_err();
                assert!(matches!(err, ParseSignedError::Ruint(_)));

                let err =
                    <$i_struct>::from_hex_str(&format!("1{:x}", <$u_struct>::MAX)).unwrap_err();
                assert!(matches!(err, ParseSignedError::IntegerOverflow));

                let err =
                    <$i_struct>::from_hex_str(&format!("-{:x}", <$u_struct>::MAX)).unwrap_err();
                assert!(matches!(err, ParseSignedError::IntegerOverflow));

                let value = <$i_struct>::from_hex_str(&format!("-{:x}", min_abs)).unwrap();
                assert_eq!(value.into_sign_and_abs(), (Sign::Negative, min_abs));

                let err = <$i_struct>::from_hex_str(&format!("{:x}", min_abs)).unwrap_err();
                assert!(matches!(err, ParseSignedError::IntegerOverflow));
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn formatting() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let unsigned = <$u_struct>::from_str_radix("3141592653589793", 10).unwrap();
                let positive = <$i_struct>::try_from(unsigned).unwrap();
                let negative = -positive;

                assert_eq!(format!("{positive}"), format!("{unsigned}"));
                assert_eq!(format!("{negative}"), format!("-{unsigned}"));
                assert_eq!(format!("{positive:+}"), format!("+{unsigned}"));
                assert_eq!(format!("{negative:+}"), format!("-{unsigned}"));

                assert_eq!(format!("{positive:x}"), format!("{unsigned:x}"));
                assert_eq!(format!("{negative:x}"), format!("-{unsigned:x}"));
                assert_eq!(format!("{positive:+x}"), format!("+{unsigned:x}"));
                assert_eq!(format!("{negative:+x}"), format!("-{unsigned:x}"));

                assert_eq!(
                    format!("{positive:X}"),
                    format!("{unsigned:x}").to_uppercase()
                );
                assert_eq!(
                    format!("{negative:X}"),
                    format!("-{unsigned:x}").to_uppercase()
                );
                assert_eq!(
                    format!("{positive:+X}"),
                    format!("+{unsigned:x}").to_uppercase()
                );
                assert_eq!(
                    format!("{negative:+X}"),
                    format!("-{unsigned:x}").to_uppercase()
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn signs() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(<$i_struct>::MAX.sign(), Sign::Positive);
                assert!(<$i_struct>::MAX.is_positive());
                assert!(!<$i_struct>::MAX.is_negative());
                assert!(!<$i_struct>::MAX.is_zero());

                assert_eq!(<$i_struct>::one().sign(), Sign::Positive);
                assert!(<$i_struct>::one().is_positive());
                assert!(!<$i_struct>::one().is_negative());
                assert!(!<$i_struct>::one().is_zero());

                assert_eq!(<$i_struct>::MIN.sign(), Sign::Negative);
                assert!(!<$i_struct>::MIN.is_positive());
                assert!(<$i_struct>::MIN.is_negative());
                assert!(!<$i_struct>::MIN.is_zero());

                assert_eq!(<$i_struct>::minus_one().sign(), Sign::Negative);
                assert!(!<$i_struct>::minus_one().is_positive());
                assert!(<$i_struct>::minus_one().is_negative());
                assert!(!<$i_struct>::minus_one().is_zero());

                assert_eq!(<$i_struct>::zero().sign(), Sign::Positive);
                assert!(!<$i_struct>::zero().is_positive());
                assert!(!<$i_struct>::zero().is_negative());
                assert!(<$i_struct>::zero().is_zero());
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn abs() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let positive = <$i_struct>::from_dec_str("3141592653589793").unwrap();
                let negative = <$i_struct>::from_dec_str("-27182818284590").unwrap();

                assert_eq!(positive.sign(), Sign::Positive);
                assert_eq!(positive.abs().sign(), Sign::Positive);
                assert_eq!(positive, positive.abs());
                assert_ne!(negative, negative.abs());
                assert_eq!(negative.sign(), Sign::Negative);
                assert_eq!(negative.abs().sign(), Sign::Positive);
                assert_eq!(<$i_struct>::zero().abs(), <$i_struct>::zero());
                assert_eq!(<$i_struct>::MAX.abs(), <$i_struct>::MAX);
                assert_eq!((-<$i_struct>::MAX).abs(), <$i_struct>::MAX);
                assert_eq!(<$i_struct>::MIN.checked_abs(), None);
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn neg() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let positive = <$i_struct>::from_dec_str("3141592653589793")
                    .unwrap()
                    .sign();
                let negative = -positive;

                assert_eq!(-positive, negative);
                assert_eq!(-negative, positive);

                assert_eq!(-<$i_struct>::zero(), <$i_struct>::zero());
                assert_eq!(-(-<$i_struct>::MAX), <$i_struct>::MAX);
                assert_eq!(<$i_struct>::MIN.checked_neg(), None);
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn bits() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(<$i_struct>::try_from(0b1000).unwrap().bits(), 5);
                assert_eq!(<$i_struct>::try_from(-0b1000).unwrap().bits(), 4);

                assert_eq!(<$i_struct>::try_from(i64::MAX).unwrap().bits(), 64);
                assert_eq!(<$i_struct>::try_from(i64::MIN).unwrap().bits(), 64);

                assert_eq!(<$i_struct>::MAX.bits(), <$i_struct>::BITS as u32);
                assert_eq!(<$i_struct>::MIN.bits(), <$i_struct>::BITS as u32);

                assert_eq!(<$i_struct>::zero().bits(), 0);
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn bit_shift() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(
                    <$i_struct>::one() << <$i_struct>::BITS - 1,
                    <$i_struct>::MIN
                );
                assert_eq!(
                    <$i_struct>::MIN >> <$i_struct>::BITS - 1,
                    <$i_struct>::one()
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn arithmetic_shift_right() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let exp = <$i_struct>::BITS - 2;
                let shift = <$i_struct>::BITS - 3;

                let value =
                    <$i_struct>::from_raw(<$u_struct>::from(2u8).pow(<$u_struct>::from(exp))).neg();

                let expected_result =
                    <$i_struct>::from_raw(<$u_struct>::MAX - <$u_struct>::from(1u8));
                assert_eq!(
                    value.asr(shift),
                    expected_result,
                    "1011...1111 >> 253 was not 1111...1110"
                );

                let value = <$i_struct>::minus_one();
                let expected_result = <$i_struct>::minus_one();
                assert_eq!(
                    value.asr(250),
                    expected_result,
                    "-1 >> any_amount was not -1"
                );

                let value = <$i_struct>::from_raw(
                    <$u_struct>::from(2u8).pow(<$u_struct>::from(<$i_struct>::BITS - 2)),
                )
                .neg();
                let expected_result = <$i_struct>::minus_one();
                assert_eq!(
                    value.asr(<$i_struct>::BITS - 1),
                    expected_result,
                    "1011...1111 >> 255 was not -1"
                );

                let value = <$i_struct>::from_raw(
                    <$u_struct>::from(2u8).pow(<$u_struct>::from(<$i_struct>::BITS - 2)),
                )
                .neg();
                let expected_result = <$i_struct>::minus_one();
                assert_eq!(
                    value.asr(1024),
                    expected_result,
                    "1011...1111 >> 1024 was not -1"
                );

                let value = <$i_struct>::try_from(1024i32).unwrap();
                let expected_result = <$i_struct>::try_from(32i32).unwrap();
                assert_eq!(value.asr(5), expected_result, "1024 >> 5 was not 32");

                let value = <$i_struct>::MAX;
                let expected_result = <$i_struct>::zero();
                assert_eq!(
                    value.asr(255),
                    expected_result,
                    "<$i_struct>::MAX >> 255 was not 0"
                );

                let value =
                    <$i_struct>::from_raw(<$u_struct>::from(2u8).pow(<$u_struct>::from(exp))).neg();
                let expected_result = value;
                assert_eq!(
                    value.asr(0),
                    expected_result,
                    "1011...1111 >> 0 was not 1011...111"
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn arithmetic_shift_left() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let value = <$i_struct>::minus_one();
                let expected_result = Some(value);
                assert_eq!(value.asl(0), expected_result, "-1 << 0 was not -1");

                let value = <$i_struct>::minus_one();
                let expected_result = None;
                assert_eq!(
                    value.asl(256),
                    expected_result,
                    "-1 << 256 did not overflow (result should be 0000...0000)"
                );

                let value = <$i_struct>::minus_one();
                let expected_result = Some(<$i_struct>::from_raw(
                    <$u_struct>::from(2u8).pow(<$u_struct>::from(<$i_struct>::BITS - 1)),
                ));
                assert_eq!(
                    value.asl(<$i_struct>::BITS - 1),
                    expected_result,
                    "-1 << 255 was not 1000...0000"
                );

                let value = <$i_struct>::try_from(-1024i32).unwrap();
                let expected_result = Some(<$i_struct>::try_from(-32768i32).unwrap());
                assert_eq!(value.asl(5), expected_result, "-1024 << 5 was not -32768");

                let value = <$i_struct>::try_from(1024i32).unwrap();
                let expected_result = Some(<$i_struct>::try_from(32768i32).unwrap());
                assert_eq!(value.asl(5), expected_result, "1024 << 5 was not 32768");

                let value = <$i_struct>::try_from(1024i32).unwrap();
                let expected_result = None;
                assert_eq!(
                    value.asl(<$i_struct>::BITS - 11),
                    expected_result,
                    "1024 << 245 did not overflow (result should be 1000...0000)"
                );

                let value = <$i_struct>::zero();
                let expected_result = Some(value);
                assert_eq!(value.asl(1024), expected_result, "0 << anything was not 0");
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn addition() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(
                    <$i_struct>::MIN.overflowing_add(<$i_struct>::MIN),
                    (<$i_struct>::zero(), true)
                );
                assert_eq!(
                    <$i_struct>::MAX.overflowing_add(<$i_struct>::MAX),
                    (<$i_struct>::try_from(-2).unwrap(), true)
                );

                assert_eq!(
                    <$i_struct>::MIN.overflowing_add(<$i_struct>::minus_one()),
                    (<$i_struct>::MAX, true)
                );
                assert_eq!(
                    <$i_struct>::MAX.overflowing_add(<$i_struct>::one()),
                    (<$i_struct>::MIN, true)
                );

                assert_eq!(
                    <$i_struct>::MAX + <$i_struct>::MIN,
                    <$i_struct>::minus_one()
                );
                assert_eq!(
                    <$i_struct>::try_from(2).unwrap() + <$i_struct>::try_from(40).unwrap(),
                    <$i_struct>::try_from(42).unwrap()
                );

                assert_eq!(
                    <$i_struct>::zero() + <$i_struct>::zero(),
                    <$i_struct>::zero()
                );

                assert_eq!(
                    <$i_struct>::MAX.saturating_add(<$i_struct>::MAX),
                    <$i_struct>::MAX
                );
                assert_eq!(
                    <$i_struct>::MIN.saturating_add(<$i_struct>::minus_one()),
                    <$i_struct>::MIN
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn subtraction() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(
                    <$i_struct>::MIN.overflowing_sub(<$i_struct>::MAX),
                    (<$i_struct>::one(), true)
                );
                assert_eq!(
                    <$i_struct>::MAX.overflowing_sub(<$i_struct>::MIN),
                    (<$i_struct>::minus_one(), true)
                );

                assert_eq!(
                    <$i_struct>::MIN.overflowing_sub(<$i_struct>::one()),
                    (<$i_struct>::MAX, true)
                );
                assert_eq!(
                    <$i_struct>::MAX.overflowing_sub(<$i_struct>::minus_one()),
                    (<$i_struct>::MIN, true)
                );

                assert_eq!(
                    <$i_struct>::zero().overflowing_sub(<$i_struct>::MIN),
                    (<$i_struct>::MIN, true)
                );

                assert_eq!(<$i_struct>::MAX - <$i_struct>::MAX, <$i_struct>::zero());
                assert_eq!(
                    <$i_struct>::try_from(2).unwrap() - <$i_struct>::try_from(44).unwrap(),
                    <$i_struct>::try_from(-42).unwrap()
                );

                assert_eq!(
                    <$i_struct>::zero() - <$i_struct>::zero(),
                    <$i_struct>::zero()
                );

                assert_eq!(
                    <$i_struct>::MAX.saturating_sub(<$i_struct>::MIN),
                    <$i_struct>::MAX
                );
                assert_eq!(
                    <$i_struct>::MIN.saturating_sub(<$i_struct>::one()),
                    <$i_struct>::MIN
                );
            };
        }

        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn multiplication() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(
                    <$i_struct>::MIN.overflowing_mul(<$i_struct>::MAX),
                    (<$i_struct>::MIN, true)
                );
                assert_eq!(
                    <$i_struct>::MAX.overflowing_mul(<$i_struct>::MIN),
                    (<$i_struct>::MIN, true)
                );

                assert_eq!(<$i_struct>::MIN * <$i_struct>::one(), <$i_struct>::MIN);
                assert_eq!(
                    <$i_struct>::try_from(2).unwrap() * <$i_struct>::try_from(-21).unwrap(),
                    <$i_struct>::try_from(-42).unwrap()
                );

                assert_eq!(
                    <$i_struct>::MAX.saturating_mul(<$i_struct>::MAX),
                    <$i_struct>::MAX
                );
                assert_eq!(
                    <$i_struct>::MAX.saturating_mul(<$i_struct>::try_from(2).unwrap()),
                    <$i_struct>::MAX
                );
                assert_eq!(
                    <$i_struct>::MIN.saturating_mul(<$i_struct>::try_from(-2).unwrap()),
                    <$i_struct>::MAX
                );

                assert_eq!(
                    <$i_struct>::MIN.saturating_mul(<$i_struct>::MAX),
                    <$i_struct>::MIN
                );
                assert_eq!(
                    <$i_struct>::MIN.saturating_mul(<$i_struct>::try_from(2).unwrap()),
                    <$i_struct>::MIN
                );
                assert_eq!(
                    <$i_struct>::MAX.saturating_mul(<$i_struct>::try_from(-2).unwrap()),
                    <$i_struct>::MIN
                );

                assert_eq!(
                    <$i_struct>::zero() * <$i_struct>::zero(),
                    <$i_struct>::zero()
                );
                assert_eq!(
                    <$i_struct>::one() * <$i_struct>::zero(),
                    <$i_struct>::zero()
                );
                assert_eq!(<$i_struct>::MAX * <$i_struct>::zero(), <$i_struct>::zero());
                assert_eq!(<$i_struct>::MIN * <$i_struct>::zero(), <$i_struct>::zero());
            };
        }

        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn division() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                // The only case for overflow.
                assert_eq!(
                    <$i_struct>::MIN.overflowing_div(<$i_struct>::try_from(-1).unwrap()),
                    (<$i_struct>::MIN, true)
                );

                assert_eq!(
                    <$i_struct>::MIN / <$i_struct>::MAX,
                    <$i_struct>::try_from(-1).unwrap()
                );
                assert_eq!(<$i_struct>::MAX / <$i_struct>::MIN, <$i_struct>::zero());

                assert_eq!(<$i_struct>::MIN / <$i_struct>::one(), <$i_struct>::MIN);
                assert_eq!(
                    <$i_struct>::try_from(-42).unwrap() / <$i_struct>::try_from(-21).unwrap(),
                    <$i_struct>::try_from(2).unwrap()
                );
                assert_eq!(
                    <$i_struct>::try_from(-42).unwrap() / <$i_struct>::try_from(2).unwrap(),
                    <$i_struct>::try_from(-21).unwrap()
                );
                assert_eq!(
                    <$i_struct>::try_from(42).unwrap() / <$i_struct>::try_from(-21).unwrap(),
                    <$i_struct>::try_from(-2).unwrap()
                );
                assert_eq!(
                    <$i_struct>::try_from(42).unwrap() / <$i_struct>::try_from(21).unwrap(),
                    <$i_struct>::try_from(2).unwrap()
                );

                // The only saturating corner case.
                assert_eq!(
                    <$i_struct>::MIN.saturating_div(<$i_struct>::try_from(-1).unwrap()),
                    <$i_struct>::MAX
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    #[should_panic]
    fn division_by_zero() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let _ = <$i_struct>::one() / <$i_struct>::zero();
            };
        }

        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn div_euclid() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let a = <$i_struct>::try_from(7).unwrap();
                let b = <$i_struct>::try_from(4).unwrap();

                assert_eq!(a.div_euclid(b), <$i_struct>::one()); // 7 >= 4 * 1
                assert_eq!(a.div_euclid(-b), <$i_struct>::minus_one()); // 7 >= -4 * -1
                assert_eq!((-a).div_euclid(b), -<$i_struct>::try_from(2).unwrap()); // -7 >= 4 * -2
                assert_eq!((-a).div_euclid(-b), <$i_struct>::try_from(2).unwrap()); // -7 >= -4 * 2

                // Overflowing
                assert_eq!(
                    <$i_struct>::MIN.overflowing_div_euclid(<$i_struct>::minus_one()),
                    (<$i_struct>::MIN, true)
                );
                // Wrapping
                assert_eq!(
                    <$i_struct>::MIN.wrapping_div_euclid(<$i_struct>::minus_one()),
                    <$i_struct>::MIN
                );
                // // Checked
                assert_eq!(
                    <$i_struct>::MIN.checked_div_euclid(<$i_struct>::minus_one()),
                    None
                );
                assert_eq!(
                    <$i_struct>::one().checked_div_euclid(<$i_struct>::zero()),
                    None
                );
            };
        }

        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn rem_euclid() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let a = <$i_struct>::try_from(7).unwrap(); // or any other integer type
                let b = <$i_struct>::try_from(4).unwrap();

                assert_eq!(a.rem_euclid(b), <$i_struct>::try_from(3).unwrap());
                assert_eq!((-a).rem_euclid(b), <$i_struct>::one());
                assert_eq!(a.rem_euclid(-b), <$i_struct>::try_from(3).unwrap());
                assert_eq!((-a).rem_euclid(-b), <$i_struct>::one());

                // Overflowing
                assert_eq!(
                    a.overflowing_rem_euclid(b),
                    (<$i_struct>::try_from(3).unwrap(), false)
                );
                assert_eq!(
                    <$i_struct>::min_value().overflowing_rem_euclid(<$i_struct>::minus_one()),
                    (<$i_struct>::zero(), true)
                );

                // Wrapping
                assert_eq!(
                    <$i_struct>::try_from(100)
                        .unwrap()
                        .wrapping_rem_euclid(<$i_struct>::try_from(10).unwrap()),
                    <$i_struct>::zero()
                );
                assert_eq!(
                    <$i_struct>::min_value().wrapping_rem_euclid(<$i_struct>::minus_one()),
                    <$i_struct>::zero()
                );

                // Checked
                assert_eq!(
                    a.checked_rem_euclid(b),
                    Some(<$i_struct>::try_from(3).unwrap())
                );
                assert_eq!(a.checked_rem_euclid(<$i_struct>::zero()), None);
                assert_eq!(
                    <$i_struct>::min_value().checked_rem_euclid(<$i_struct>::minus_one()),
                    None
                );
            };
        }

        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    #[should_panic]
    fn div_euclid_by_zero() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let _ = <$i_struct>::one().div_euclid(<$i_struct>::zero());
                assert_eq!(
                    <$i_struct>::MIN.div_euclid(<$i_struct>::minus_one()),
                    <$i_struct>::MAX
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    #[should_panic]
    fn div_euclid_overflow() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let _ = <$i_struct>::MIN.div_euclid(<$i_struct>::minus_one());
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    #[should_panic]
    fn mod_by_zero() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                let _ = <$i_struct>::one() % <$i_struct>::zero();
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn remainder() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                // The only case for overflow.
                assert_eq!(
                    <$i_struct>::MIN.overflowing_rem(<$i_struct>::try_from(-1).unwrap()),
                    (<$i_struct>::zero(), true)
                );
                assert_eq!(
                    <$i_struct>::try_from(-5).unwrap() % <$i_struct>::try_from(-2).unwrap(),
                    <$i_struct>::try_from(-1).unwrap()
                );
                assert_eq!(
                    <$i_struct>::try_from(5).unwrap() % <$i_struct>::try_from(-2).unwrap(),
                    <$i_struct>::one()
                );
                assert_eq!(
                    <$i_struct>::try_from(-5).unwrap() % <$i_struct>::try_from(2).unwrap(),
                    <$i_struct>::try_from(-1).unwrap()
                );
                assert_eq!(
                    <$i_struct>::try_from(5).unwrap() % <$i_struct>::try_from(2).unwrap(),
                    <$i_struct>::one()
                );

                assert_eq!(
                    <$i_struct>::MIN.checked_rem(<$i_struct>::try_from(-1).unwrap()),
                    None
                );
                assert_eq!(
                    <$i_struct>::one().checked_rem(<$i_struct>::one()),
                    Some(<$i_struct>::zero())
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn exponentiation() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(
                    <$i_struct>::try_from(1000).unwrap().saturating_pow(1000),
                    <$i_struct>::MAX
                );
                assert_eq!(
                    <$i_struct>::try_from(-1000).unwrap().saturating_pow(1001),
                    <$i_struct>::MIN
                );

                assert_eq!(
                    <$i_struct>::try_from(2).unwrap().pow(64),
                    <$i_struct>::try_from(1u128 << 64).unwrap()
                );
                assert_eq!(
                    <$i_struct>::try_from(-2).unwrap().pow(63),
                    <$i_struct>::try_from(i64::MIN).unwrap()
                );

                assert_eq!(<$i_struct>::zero().pow(42), <$i_struct>::zero());
                assert_eq!(<$i_struct>::exp10(18).to_string(), "1000000000000000000");
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn iterators() {
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_eq!(
                    (1..=5)
                        .map(<$i_struct>::try_from)
                        .map(Result::unwrap)
                        .sum::<$i_struct>(),
                    <$i_struct>::try_from(15).unwrap()
                );
                assert_eq!(
                    (1..=5)
                        .map(<$i_struct>::try_from)
                        .map(Result::unwrap)
                        .product::<$i_struct>(),
                    <$i_struct>::try_from(120).unwrap()
                );
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }

    #[test]
    fn twos_complement() {
        macro_rules! assert_twos_complement {
            ($i_struct:ty, $u_struct:ty, $signed:ty, $unsigned:ty) => {
                if <$u_struct>::BITS as u32 >= <$unsigned>::BITS {
                    assert_eq!(
                        <$i_struct>::try_from(<$signed>::MAX)
                            .unwrap()
                            .twos_complement(),
                        <$u_struct>::try_from(<$signed>::MAX).unwrap()
                    );
                    assert_eq!(
                        <$i_struct>::try_from(<$signed>::MIN)
                            .unwrap()
                            .twos_complement(),
                        <$u_struct>::try_from(<$signed>::MIN.unsigned_abs()).unwrap()
                    );
                }

                assert_eq!(
                    <$i_struct>::try_from(0 as $signed)
                        .unwrap()
                        .twos_complement(),
                    <$u_struct>::try_from(0 as $signed).unwrap()
                );

                assert_eq!(
                    <$i_struct>::try_from(0 as $unsigned)
                        .unwrap()
                        .twos_complement(),
                    <$u_struct>::try_from(0 as $unsigned).unwrap()
                );
            };
        }
        macro_rules! run_test {
            ($i_struct:ty, $u_struct:ty) => {
                assert_twos_complement!($i_struct, $u_struct, i8, u8);
                assert_twos_complement!($i_struct, $u_struct, i16, u16);
                assert_twos_complement!($i_struct, $u_struct, i32, u32);
                assert_twos_complement!($i_struct, $u_struct, i64, u64);
                assert_twos_complement!($i_struct, $u_struct, i128, u128);
                assert_twos_complement!($i_struct, $u_struct, isize, usize);
            };
        }
        run_test!(I96, U96);
        run_test!(I128, U128);
        run_test!(I160, U160);
        run_test!(I192, U192);
        run_test!(I256, U256);
    }
}
