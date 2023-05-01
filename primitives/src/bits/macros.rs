/// Wrap a fixed-size byte array in a newtype, delegating all methods to the
/// underlying fixed
#[macro_export]
macro_rules! wrap_fixed_bytes {
    ($name:ident<$n:literal>, $sname:expr, $sn:expr) => {
        #[doc = "A fixed byte array representing a "]
        #[doc = $sname]
        #[doc = " and containing "]
        #[doc = $sn]
        #[doc = " bytes"]
        #[derive(
            $crate::derive_more::AsRef,
            $crate::derive_more::AsMut,
            $crate::derive_more::Deref,
            $crate::derive_more::DerefMut,
            $crate::derive_more::From,
            Hash,
            Copy,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Default,
            $crate::derive_more::Index,
            $crate::derive_more::IndexMut,
            $crate::derive_more::BitAnd,
            $crate::derive_more::BitOr,
            $crate::derive_more::BitXor,
            $crate::derive_more::BitAndAssign,
            $crate::derive_more::BitOrAssign,
            $crate::derive_more::BitXorAssign,
            $crate::derive_more::FromStr,
            $crate::derive_more::LowerHex,
            $crate::derive_more::UpperHex,
        )]
        pub struct $name($crate::FixedBytes<$n>);

        impl<'a> From<[u8; $n]> for $name {
            #[inline]
            #[track_caller]
            fn from(bytes: [u8; $n]) -> Self {
                Self(bytes.into())
            }
        }

        impl<'a> From<&'a [u8; $n]> for $name {
            #[inline]
            #[track_caller]
            fn from(bytes: &'a [u8; $n]) -> Self {
                Self(bytes.into())
            }
        }

        impl AsRef<[u8]> for $name {
            #[inline]
            #[track_caller]
            fn as_ref(&self) -> &[u8] {
                self.as_bytes()
            }
        }

        impl AsMut<[u8]> for $name {
            #[inline]
            #[track_caller]
            fn as_mut(&mut self) -> &mut [u8] {
                self.as_bytes_mut()
            }
        }

        impl $name {
            /// Returns a new fixed hash from the given bytes array.
            pub const fn new(bytes: [u8; $n]) -> Self {
                Self(FixedBytes(bytes))
            }
            /// Returns a new fixed hash where all bits are set to the given byte.
            #[inline]
            #[track_caller]
            pub const fn repeat_byte(byte: u8) -> Self {
                Self($crate::FixedBytes::repeat_byte(byte))
            }
            /// Returns a new zero-initialized fixed hash.
            #[inline]
            #[track_caller]
            pub const fn zero() -> Self {
                Self($crate::FixedBytes::repeat_byte(0u8))
            }
            /// Returns the size of this hash in bytes.
            #[inline]
            #[track_caller]
            pub const fn len_bytes() -> usize {
                $n
            }
            /// Extracts a byte slice containing the entire fixed hash.
            #[inline]
            #[track_caller]
            pub const fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }
            /// Extracts a mutable byte slice containing the entire fixed hash.
            #[inline]
            #[track_caller]
            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                self.0.as_bytes_mut()
            }
            /// Extracts a reference to the byte array containing the entire fixed hash.
            #[inline]
            #[track_caller]
            pub const fn as_fixed_bytes(&self) -> &[u8; $n] {
                self.0.as_fixed_bytes()
            }
            /// Extracts a reference to the byte array containing the entire fixed hash.
            #[inline]
            #[track_caller]
            pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; $n] {
                self.0.as_fixed_bytes_mut()
            }
            /// Returns the inner bytes array.
            #[inline]
            #[track_caller]
            pub const fn to_fixed_bytes(self) -> [u8; $n] {
                self.0.to_fixed_bytes()
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
                assert_eq!(src.len(), $n);
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
                assert_eq!(src.len(), $n);
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

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }

        $crate::impl_rlp!($name);
        $crate::impl_serde!($name);
    };

    ($name:ident<$n:literal>) => {
        wrap_fixed_bytes!($name<$n>, stringify!($name), stringify!($n));
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "rlp")]
macro_rules! impl_rlp {
    ($t:ty) => {
        impl ethers_rlp::Decodable for $t {
            fn decode(buf: &mut &[u8]) -> Result<Self, ethers_rlp::DecodeError> {
                ethers_rlp::Decodable::decode(buf).map(Self)
            }
        }

        impl ethers_rlp::Encodable for $t {
            fn length(&self) -> usize {
                self.0.length()
            }

            fn encode(&self, out: &mut dyn bytes::BufMut) {
                self.0.encode(out)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "rlp"))]
macro_rules! impl_rlp {
    ($t:ty) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! impl_serde {
    ($t:ty) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serde::Serialize::serialize(&self.0, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer).map(Self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! impl_serde {
    ($t:ty) => {};
}
