/// Wrap a fixed-size byte array in a newtype, delegating all methods to the
/// underlying [`crate::FixedBytes`].
///
/// This functionally creates a new named FixedBytes that cannot be
/// type-confused for another named FixedBytes.
///
/// # Example
///
/// ```
/// use alloy_primitives::wrap_fixed_bytes;
///
/// // These hashes are the same length, and have the same functionality, but
/// // are distinct types
/// wrap_fixed_bytes!(KeccakOutput<32>);
/// wrap_fixed_bytes!(MerkleTreeItem<32>);
/// ```
#[macro_export]
macro_rules! wrap_fixed_bytes {
    (
        $(#[$attrs:meta])*
        $name:ident<$n:literal>
    ) => {
        $crate::wrap_fixed_bytes!(
            name_str: stringify!($name),
            num_str: stringify!($n),
            $(#[$attrs])*
            $name<$n>,
        );
    };

    (
        name_str: $sname:expr,
        num_str: $sn:expr,
        $(#[$attrs:meta])*
        $name:ident<$n:literal>,
    ) => {
        $(#[$attrs])*
        #[doc = "A fixed byte array representing a "]
        #[doc = $sname]
        #[doc = " and containing "]
        #[doc = $sn]
        #[doc = " bytes."]
        #[derive(
            $crate::private::derive_more::AsRef,
            $crate::private::derive_more::AsMut,
            $crate::private::derive_more::Deref,
            $crate::private::derive_more::DerefMut,
            $crate::private::derive_more::From,
            Hash,
            Copy,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Default,
            $crate::private::derive_more::Index,
            $crate::private::derive_more::IndexMut,
            $crate::private::derive_more::BitAnd,
            $crate::private::derive_more::BitOr,
            $crate::private::derive_more::BitXor,
            $crate::private::derive_more::BitAndAssign,
            $crate::private::derive_more::BitOrAssign,
            $crate::private::derive_more::BitXorAssign,
            $crate::private::derive_more::FromStr,
            $crate::private::derive_more::LowerHex,
            $crate::private::derive_more::UpperHex,
        )]
        pub struct $name(pub $crate::FixedBytes<$n>);

        impl From<[u8; $n]> for $name {
            #[inline]
            fn from(value: [u8; $n]) -> Self {
                Self(value.into())
            }
        }

        impl From<$name> for [u8; $n] {
            #[inline]
            fn from(value: $name) -> Self {
                value.0.0
            }
        }

        impl<'a> From<&'a [u8; $n]> for $name {
            #[inline]
            fn from(value: &'a [u8; $n]) -> Self {
                Self(value.into())
            }
        }

        impl AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                &self.0.0
            }
        }

        impl AsMut<[u8]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0.0
            }
        }

        impl $name {
            /// Array of Zero bytes.
            pub const ZERO: Self = Self($crate::FixedBytes::ZERO);

            /// Returns a new fixed hash from the given bytes array.
            #[inline]
            pub const fn new(bytes: [u8; $n]) -> Self {
                Self($crate::FixedBytes(bytes))
            }

            /// Utility function to create a fixed hash with the last byte set to `x`.
            #[inline]
            pub const fn with_last_byte(x: u8) -> Self {
                Self($crate::FixedBytes::with_last_byte(x))
            }

            /// Returns a new fixed hash where all bits are set to the given byte.
            #[inline]
            pub const fn repeat_byte(byte: u8) -> Self {
                Self($crate::FixedBytes::repeat_byte(byte))
            }

            /// Returns the size of this hash in bytes.
            #[inline]
            pub const fn len_bytes() -> usize {
                $n
            }

            /// Extracts a byte slice containing the entire fixed hash.
            #[inline]
            pub const fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            /// Extracts a mutable byte slice containing the entire fixed hash.
            #[inline]
            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                self.0.as_bytes_mut()
            }

            /// Extracts a reference to the byte array containing the entire fixed hash.
            #[inline]
            pub const fn as_fixed_bytes(&self) -> &[u8; $n] {
                self.0.as_fixed_bytes()
            }

            /// Extracts a reference to the byte array containing the entire fixed hash.
            #[inline]
            pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; $n] {
                self.0.as_fixed_bytes_mut()
            }

            /// Returns the inner bytes array.
            #[inline]
            pub const fn to_fixed_bytes(self) -> [u8; $n] {
                self.0.to_fixed_bytes()
            }

            /// Returns a constant raw pointer to the value.
            #[inline]
            pub const fn as_ptr(&self) -> *const u8 {
                self.as_bytes().as_ptr()
            }

            /// Returns a mutable raw pointer to the value.
            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut u8 {
                self.as_bytes_mut().as_mut_ptr()
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
            pub fn from_slice(src: &[u8]) -> Self {
                Self($crate::FixedBytes::from_slice(src))
            }

            /// Returns `true` if all bits set in `b` are also set in `self`.
            #[inline]
            pub fn covers(&self, b: &Self) -> bool {
                &(*b & *self) == b
            }

            /// Returns `true` if no bits are set.
            #[inline]
            pub fn is_zero(&self) -> bool {
                self.as_bytes().iter().all(|&byte| byte == 0u8)
            }

            /// Compile-time equality. NOT constant-time equality.
            pub const fn const_eq(&self, other: &Self) -> bool {
                self.0.const_eq(&other.0)
            }

            /// Returns `true` if no bits are set.
            #[inline]
            pub const fn const_is_zero(&self) -> bool {
                self.0.const_is_zero()
            }

            /// Computes the bitwise AND of two `FixedBytes`.
            pub const fn bit_and(self, rhs: Self) -> Self {
                Self(self.0.bit_and(rhs.0))
            }

            /// Computes the bitwise OR of two `FixedBytes`.
            pub const fn bit_or(self, rhs: Self) -> Self {
                Self(self.0.bit_or(rhs.0))
            }

            /// Computes the bitwise XOR of two `FixedBytes`.
            pub const fn bit_xor(self, rhs: Self) -> Self {
                Self(self.0.bit_xor(rhs.0))
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        $crate::impl_getrandom!($name);
        $crate::impl_rlp!($name);
        $crate::impl_serde!($name);
        $crate::impl_arbitrary!($name, $n);
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "getrandom")]
macro_rules! impl_getrandom {
    ($t:ty) => {
        impl $t {
            /// Instantiates a new fixed hash with cryptographically random content.
            #[inline]
            pub fn random() -> Self {
                Self($crate::FixedBytes::random())
            }

            /// Instantiates a new fixed hash with cryptographically random content.
            #[inline]
            pub fn try_random() -> ::core::result::Result<Self, $crate::private::getrandom::Error> {
                $crate::FixedBytes::try_random().map(Self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "getrandom"))]
macro_rules! impl_getrandom {
    ($t:ty) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "rlp")]
macro_rules! impl_rlp {
    ($t:ty) => {
        impl $crate::private::alloy_rlp::Decodable for $t {
            #[inline]
            fn decode(buf: &mut &[u8]) -> Result<Self, $crate::private::alloy_rlp::DecodeError> {
                $crate::private::alloy_rlp::Decodable::decode(buf).map(Self)
            }
        }

        impl $crate::private::alloy_rlp::Encodable for $t {
            #[inline]
            fn length(&self) -> usize {
                $crate::private::alloy_rlp::Encodable::length(&self.0)
            }

            #[inline]
            fn encode(&self, out: &mut dyn bytes::BufMut) {
                $crate::private::alloy_rlp::Encodable::encode(&self.0, out)
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
        impl $crate::private::serde::Serialize for $t {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                $crate::private::serde::Serialize::serialize(&self.0, serializer)
            }
        }

        impl<'de> $crate::private::serde::Deserialize<'de> for $t {
            #[inline]
            fn deserialize<D: $crate::private::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                $crate::private::serde::Deserialize::deserialize(deserializer).map(Self)
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

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "arbitrary")]
macro_rules! impl_arbitrary {
    ($t:ty, $n:literal) => {
        impl<'a> $crate::private::arbitrary::Arbitrary<'a> for $t {
            #[inline]
            fn arbitrary(u: &mut $crate::private::arbitrary::Unstructured<'a>) -> $crate::private::arbitrary::Result<Self> {
                <$crate::FixedBytes<$n> as $crate::private::arbitrary::Arbitrary>::arbitrary(u).map(Self)
            }

            #[inline]
            fn arbitrary_take_rest(u: $crate::private::arbitrary::Unstructured<'a>) -> $crate::private::arbitrary::Result<Self> {
                <$crate::FixedBytes<$n> as $crate::private::arbitrary::Arbitrary>::arbitrary_take_rest(u).map(Self)
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <$crate::FixedBytes<$n> as $crate::private::arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        impl $crate::private::proptest::arbitrary::Arbitrary for $t {
            type Parameters = <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::Parameters;
            type Strategy = $crate::private::proptest::strategy::Map<
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::Strategy,
                fn($crate::FixedBytes<$n>) -> Self,
            >;

            #[inline]
            fn arbitrary() -> Self::Strategy {
                use $crate::private::proptest::strategy::Strategy;
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::arbitrary().prop_map(Self)
            }

            #[inline]
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                use $crate::private::proptest::strategy::Strategy;
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::arbitrary_with(args).prop_map(Self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "arbitrary"))]
macro_rules! impl_arbitrary {
    ($t:ty, $n:literal) => {};
}
