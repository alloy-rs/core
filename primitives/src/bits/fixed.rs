use core::{fmt, ops};

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Index, IndexMut};

/// A bytearray of fixed length.
#[cfg_attr(feature = "arbitrary", derive(Arbitrary, PropTestArbitrary))]
#[derive(
    AsRef,
    AsMut,
    Deref,
    DerefMut,
    From,
    Hash,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Index,
    IndexMut,
)]
pub struct FixedBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for FixedBytes<N> {
    fn default() -> Self {
        FixedBytes([0; N])
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
    #[track_caller]
    fn from(bytes: &'a [u8; N]) -> Self {
        FixedBytes(*bytes)
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
    #[track_caller]
    fn from(bytes: &'a mut [u8; N]) -> Self {
        FixedBytes(*bytes)
    }
}

impl<const N: usize> From<FixedBytes<N>> for [u8; N] {
    #[inline]
    #[track_caller]
    fn from(s: FixedBytes<N>) -> Self {
        s.0
    }
}

impl<const N: usize> AsRef<[u8]> for FixedBytes<N> {
    #[inline]
    #[track_caller]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: usize> AsMut<[u8]> for FixedBytes<N> {
    #[inline]
    #[track_caller]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl<const N: usize> FixedBytes<N> {
    /// Instantiates a new fixed hash from the given bytes array.
    pub const fn new(bytes: [u8; N]) -> Self {
        FixedBytes(bytes)
    }

    /// Concatenate two fixed hashes. Due to rust constraints, the user must
    /// specify Z. Incorrect specification will result in a panic.
    pub const fn concat<const M: usize, const Z: usize>(
        self,
        other: FixedBytes<M>,
    ) -> FixedBytes<Z> {
        assert!(N + M == Z, "Output size must be sum of input sizes");

        let mut result = [0u8; Z];

        let i = 0;
        loop {
            result[i] = if i >= N { other.0[i - N] } else { self.0[i] };
            if i == Z {
                break;
            }
        }

        FixedBytes(result)
    }

    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    #[track_caller]
    pub const fn repeat_byte(byte: u8) -> FixedBytes<N> {
        FixedBytes([byte; N])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    #[track_caller]
    pub const fn zero() -> FixedBytes<N> {
        FixedBytes::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    #[track_caller]
    pub const fn len_bytes() -> usize {
        32
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    #[track_caller]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    #[track_caller]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    #[track_caller]
    pub const fn as_fixed_bytes(&self) -> &[u8; N] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    #[track_caller]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    #[track_caller]
    pub const fn to_fixed_bytes(self) -> [u8; N] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    #[track_caller]
    pub const fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    #[track_caller]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        assert_eq!(src.len(), 32);
        self.as_bytes_mut().copy_from_slice(src);
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
    pub fn from_slice(src: &[u8]) -> Self {
        assert_eq!(src.len(), 32);
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    #[track_caller]
    pub fn covers(&self, b: &Self) -> bool {
        &(*b & *self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    #[track_caller]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}

impl<const N: usize> fmt::Debug for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl<const N: usize> fmt::Display for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            return write!(f, "{:#x}", self);
        }

        write!(f, "0x")?;
        for i in &self.0[0..2] {
            write!(f, "{:02x}", i)?;
        }
        write!(f, "…")?;
        for i in &self.0[32 - 2..32] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl<const N: usize> fmt::LowerHex for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl<const N: usize> fmt::UpperHex for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0X")?;
        }
        for i in &self.0[..] {
            write!(f, "{:02X}", i)?;
        }
        Ok(())
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
    type Err = super::hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0u8; N];
        super::hex::from_hex_raw(s, &mut buf)?;
        Ok(Self(buf))
    }
}

#[cfg(test)]
mod test {}
