use core::{fmt, ops, str};
use derive_more::{Deref, DerefMut, From, Index, IndexMut, IntoIterator};

/// A byte array of fixed length (`[u8; N]`).
///
/// This type allows us to more tightly control serialization, deserialization.
/// rlp encoding, decoding, and other type-level attributes for fixed-length
/// byte arrays.
///
/// Users looking to prevent type-confusion between byte arrays of different
/// lengths should use the [`wrap_fixed_bytes!`](crate::wrap_fixed_bytes) macro
/// to create a new fixed-length byte array type.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    DerefMut,
    From,
    Index,
    IndexMut,
    IntoIterator,
)]
#[cfg_attr(
    feature = "arbitrary",
    derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
#[repr(transparent)]
pub struct FixedBytes<const N: usize>(#[into_iterator(owned, ref, ref_mut)] pub [u8; N]);

crate::impl_fixed_bytes_traits!(FixedBytes<N>, N, const);

impl<const N: usize> Default for FixedBytes<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for FixedBytes<N> {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; N]) -> Self {
        Self(*bytes)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for FixedBytes<N> {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; N]) -> Self {
        Self(*bytes)
    }
}

impl<const N: usize> From<FixedBytes<N>> for [u8; N] {
    #[inline]
    fn from(s: FixedBytes<N>) -> Self {
        s.0
    }
}

impl<const N: usize> AsRef<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for FixedBytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for FixedBytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> fmt::Debug for FixedBytes<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<false>(f, true)
    }
}

impl<const N: usize> fmt::Display for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If the alternate flag is NOT set, we write the full hex.
        if N <= 4 || !f.alternate() {
            return self.fmt_hex::<false>(f, true)
        }

        // If the alternate flag is set, we use middle-out compression.
        const SEP_LEN: usize = '…'.len_utf8();
        let mut buf = [0; 2 + 4 + SEP_LEN + 4];
        buf[0] = b'0';
        buf[1] = b'x';
        hex::encode_to_slice(&self.0[0..2], &mut buf[2..6]).unwrap();
        '…'.encode_utf8(&mut buf[6..]);
        hex::encode_to_slice(&self.0[N - 2..N], &mut buf[6 + SEP_LEN..]).unwrap();

        // SAFETY: always valid UTF-8
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}

impl<const N: usize> fmt::LowerHex for FixedBytes<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<false>(f, f.alternate())
    }
}

impl<const N: usize> fmt::UpperHex for FixedBytes<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_hex::<true>(f, f.alternate())
    }
}

impl<const N: usize> ops::BitAnd for FixedBytes<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut other = self;
        other.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a &= *b);
        other
    }
}

impl<const N: usize> ops::BitAndAssign for FixedBytes<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a &= *b);
    }
}

impl<const N: usize> ops::BitOr for FixedBytes<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut other = self;
        other.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a |= *b);
        other
    }
}

impl<const N: usize> ops::BitOrAssign for FixedBytes<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a |= *b);
    }
}

impl<const N: usize> ops::BitXor for FixedBytes<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut other = self;
        other.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a ^= *b);
        other
    }
}

impl<const N: usize> ops::BitXorAssign for FixedBytes<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a ^= *b);
    }
}

impl<const N: usize> core::str::FromStr for FixedBytes<N> {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0u8; N];
        hex::decode_to_slice(s, &mut buf)?;
        Ok(Self(buf))
    }
}

impl<const N: usize> FixedBytes<N> {
    /// Array of Zero bytes.
    pub const ZERO: Self = Self([0u8; N]);

    /// Instantiates a new fixed hash from the given bytes array.
    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Utility function to create a fixed hash with the last byte set to `x`.
    #[inline]
    pub const fn with_last_byte(x: u8) -> Self {
        let mut bytes = [0u8; N];
        bytes[N - 1] = x;
        Self(bytes)
    }

    /// Instantiates a new fixed hash with cryptographically random content.
    #[cfg(feature = "getrandom")]
    #[inline]
    pub fn random() -> Self {
        Self::try_random().unwrap()
    }

    /// Instantiates a new fixed hash with cryptographically random content.
    #[cfg(feature = "getrandom")]
    pub fn try_random() -> Result<Self, getrandom::Error> {
        let mut bytes: [_; N] = super::impl_core::uninit_array();
        getrandom::getrandom_uninit(&mut bytes)?;
        // SAFETY: The array is initialized by getrandom_uninit.
        Ok(Self(unsafe { super::impl_core::array_assume_init(bytes) }))
    }

    /// Concatenate two `FixedBytes`.
    ///
    /// Due to constraints in the language, the user must specify the value of
    /// the output size `Z`.
    ///
    /// # Panics
    ///
    /// This function panics if `Z` is not equal to `N + M`.
    pub const fn concat_const<const M: usize, const Z: usize>(
        self,
        other: FixedBytes<M>,
    ) -> FixedBytes<Z> {
        assert!(
            N + M == Z,
            "Output size `Z` must equal the sum of the input sizes `N` and `M`"
        );

        let mut result = [0u8; Z];
        let mut i = 0;
        while i < Z {
            result[i] = if i >= N { other.0[i - N] } else { self.0[i] };
            i += 1;
        }
        FixedBytes(result)
    }

    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> Self {
        Self([byte; N])
    }

    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        N
    }

    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    #[track_caller]
    #[inline]
    pub fn from_slice(src: &[u8]) -> Self {
        Self(src.try_into().unwrap())
    }

    /// Returns a slice containing the entire array. Equivalent to `&s[..]`.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Returns a mutable slice containing the entire array. Equivalent to
    /// `&mut s[..]`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(*b & *self) == b
    }

    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    /// Compile-time equality. NOT constant-time equality.
    #[inline]
    pub const fn const_eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < N {
            if self.0[i] != other.0[i] {
                return false
            }
            i += 1;
        }

        true
    }

    /// Returns `true` if no bits are set.
    #[inline]
    pub const fn const_is_zero(&self) -> bool {
        self.const_eq(&Self::ZERO)
    }

    /// Computes the bitwise AND of two `FixedBytes`.
    pub const fn bit_and(self, rhs: Self) -> Self {
        let mut ret = Self::ZERO;
        let mut i = 0;
        while i < N {
            ret.0[i] = self.0[i] & rhs.0[i];
            i += 1;
        }
        ret
    }

    /// Computes the bitwise OR of two `FixedBytes`.
    pub const fn bit_or(self, rhs: Self) -> Self {
        let mut ret = Self::ZERO;
        let mut i = 0;
        while i < N {
            ret.0[i] = self.0[i] | rhs.0[i];
            i += 1;
        }
        ret
    }

    /// Computes the bitwise XOR of two `FixedBytes`.
    pub const fn bit_xor(self, rhs: Self) -> Self {
        let mut ret = Self::ZERO;
        let mut i = 0;
        while i < N {
            ret.0[i] = self.0[i] ^ rhs.0[i];
            i += 1;
        }
        ret
    }

    fn fmt_hex<const UPPER: bool>(&self, f: &mut fmt::Formatter<'_>, prefix: bool) -> fmt::Result {
        let mut buf = hex::Buffer::<N, true>::new();
        let s = if UPPER {
            buf.format_upper(self)
        } else {
            buf.format(self)
        };
        f.write_str(&s[(!prefix as usize) * 2..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    macro_rules! test_fmt {
        ($($fmt:literal, $hex:literal => $expected:literal;)+) => {$(
            assert_eq!(
                format!($fmt, FixedBytes::from(hex!($hex))),
                $expected
            );
        )+};
    }

    #[test]
    fn concat_const() {
        const A: FixedBytes<2> = FixedBytes(hex!("0123"));
        const B: FixedBytes<2> = FixedBytes(hex!("4567"));
        const EXPECTED: FixedBytes<4> = FixedBytes(hex!("01234567"));
        const ACTUAL: FixedBytes<4> = A.concat_const(B);

        assert_eq!(ACTUAL, EXPECTED);
    }

    #[test]
    fn display() {
        test_fmt! {
            "{}", "0123456789abcdef" => "0x0123456789abcdef";
            "{:#}", "0123" => "0x0123";
            "{:#}", "01234567" => "0x01234567";
            "{:#}", "0123456789" => "0x0123…6789";
        }
    }

    #[test]
    fn debug() {
        test_fmt! {
            "{:?}", "0123456789abcdef" => "0x0123456789abcdef";
            "{:#?}", "0123456789abcdef" => "0x0123456789abcdef";
        }
    }

    #[test]
    fn lower_hex() {
        test_fmt! {
            "{:x}", "0123456789abcdef" => "0123456789abcdef";
            "{:#x}", "0123456789abcdef" => "0x0123456789abcdef";
        }
    }

    #[test]
    fn upper_hex() {
        test_fmt! {
            "{:X}", "0123456789abcdef" => "0123456789ABCDEF";
            "{:#X}", "0123456789abcdef" => "0x0123456789ABCDEF";
        }
    }
}
