use alloc::{string::String, vec::Vec};
use core::{borrow::Borrow, fmt, ops::Deref};

#[cfg(feature = "rlp")]
mod rlp;

/// Wrapper type around Bytes to deserialize/serialize "0x" prefixed ethereum
/// hex strings.
#[derive(Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bytes(#[cfg_attr(serde, serde(with = "hex"))] pub bytes::Bytes);

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Bytes(")?;
        f.write_str(&self.hex_encode())?;
        f.write_str(")")
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex_encode())
    }
}

impl fmt::LowerHex for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex_encode())
    }
}

impl Deref for Bytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl FromIterator<u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<bytes::Bytes>())
    }
}

impl<'a> FromIterator<&'a u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
        Self(iter.into_iter().copied().collect::<bytes::Bytes>())
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = bytes::buf::IntoIter<bytes::Bytes>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Bytes {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

impl From<bytes::Bytes> for Bytes {
    fn from(src: bytes::Bytes) -> Self {
        Self(src)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(src: Vec<u8>) -> Self {
        Self(src.into())
    }
}

impl<const N: usize> From<[u8; N]> for Bytes {
    fn from(src: [u8; N]) -> Self {
        src.to_vec().into()
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for Bytes {
    fn from(src: &'a [u8; N]) -> Self {
        src.to_vec().into()
    }
}

impl From<&'static [u8]> for Bytes {
    fn from(value: &'static [u8]) -> Self {
        Self(value.into())
    }
}

impl From<&'static str> for Bytes {
    fn from(value: &'static str) -> Self {
        Self(value.into())
    }
}

impl PartialEq<[u8]> for Bytes {
    fn eq(&self, other: &[u8]) -> bool {
        self[..] == *other
    }
}

impl PartialEq<Bytes> for [u8] {
    fn eq(&self, other: &Bytes) -> bool {
        *self == other[..]
    }
}

impl PartialEq<Vec<u8>> for Bytes {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self[..] == other[..]
    }
}

impl PartialEq<Bytes> for Vec<u8> {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl PartialEq<bytes::Bytes> for Bytes {
    fn eq(&self, other: &bytes::Bytes) -> bool {
        other == self.as_ref()
    }
}

impl core::str::FromStr for Bytes {
    type Err = hex::FromHexError;

    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        hex::decode(value).map(Into::into)
    }
}

impl Bytes {
    /// Creates a new empty `Bytes`.
    ///
    /// This will not allocate and the returned `Bytes` handle will be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_primitives::Bytes;
    ///
    /// let b = Bytes::new();
    /// assert_eq!(&b[..], b"");
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(bytes::Bytes::new())
    }

    /// Creates a new `Bytes` from a static slice.
    ///
    /// The returned `Bytes` will point directly to the static slice. There is
    /// no allocating or copying.
    ///
    /// # Examples
    ///
    /// ```
    /// use ethers_primitives::Bytes;
    ///
    /// let b = Bytes::from_static(b"hello");
    /// assert_eq!(&b[..], b"hello");
    /// ```
    #[inline]
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Self(bytes::Bytes::from_static(bytes))
    }

    fn hex_encode(&self) -> String {
        hex::encode_prefixed(self.0.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            "1213".parse::<Bytes>().unwrap(),
            hex::decode("1213").unwrap()
        );
        assert_eq!(
            "0x1213".parse::<Bytes>().unwrap(),
            hex::decode("0x1213").unwrap()
        );
    }

    #[test]
    fn hex() {
        let b = Bytes::from_static(&[1, 35, 69, 103, 137, 171, 205, 239]);
        let expected = "0x0123456789abcdef";
        assert_eq!(format!("{b:x}"), expected);
        assert_eq!(format!("{b}"), expected);
    }

    #[test]
    fn debug() {
        let b = Bytes::from(vec![1, 35, 69, 103, 137, 171, 205, 239]);
        assert_eq!(format!("{b:?}"), "Bytes(0x0123456789abcdef)");
        assert_eq!(format!("{b:#?}"), "Bytes(0x0123456789abcdef)");
    }
}
