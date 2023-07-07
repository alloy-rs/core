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
/// wrap_fixed_bytes!(pub struct KeccakOutput<32>;);
/// wrap_fixed_bytes!(pub struct MerkleTreeItem<32>;);
/// ```
#[macro_export]
macro_rules! wrap_fixed_bytes {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident<$n:literal>;
    ) => {
        $crate::wrap_fixed_bytes!(
            extra_derives: [$crate::private::derive_more::Display],
            $(#[$attrs])*
            $vis struct $name<$n>;
        );
    };

    (
        extra_derives: [$($extra_derives:path),* $(,)?],
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident<$n:literal>;
    ) => {
        $(#[$attrs])*
        #[derive(
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            $crate::private::derive_more::AsMut,
            $crate::private::derive_more::AsRef,
            $crate::private::derive_more::BitAnd,
            $crate::private::derive_more::BitAndAssign,
            $crate::private::derive_more::BitOr,
            $crate::private::derive_more::BitOrAssign,
            $crate::private::derive_more::BitXor,
            $crate::private::derive_more::BitXorAssign,
            $crate::private::derive_more::Deref,
            $crate::private::derive_more::DerefMut,
            $crate::private::derive_more::From,
            $crate::private::derive_more::FromStr,
            $crate::private::derive_more::Index,
            $crate::private::derive_more::IndexMut,
            $crate::private::derive_more::Into,
            $crate::private::derive_more::IntoIterator,
            $crate::private::derive_more::LowerHex,
            $crate::private::derive_more::UpperHex,
            $(
                $extra_derives,
            )*
        )]
        #[repr(transparent)]
        $vis struct $name(#[into_iterator(owned, ref, ref_mut)] pub $crate::FixedBytes<$n>);

        impl ::core::convert::From<[u8; $n]> for $name {
            #[inline]
            fn from(value: [u8; $n]) -> Self {
                Self($crate::FixedBytes(value))
            }
        }

        impl ::core::convert::From<$name> for [u8; $n] {
            #[inline]
            fn from(value: $name) -> Self {
                value.0 .0
            }
        }

        impl<'a> ::core::convert::From<&'a [u8; $n]> for $name {
            #[inline]
            fn from(value: &'a [u8; $n]) -> Self {
                Self($crate::FixedBytes(*value))
            }
        }

        impl ::core::convert::TryFrom<&[u8]> for $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
                <&Self>::try_from(slice).map(|this| *this)
            }
        }

        impl ::core::convert::TryFrom<&mut [u8]> for $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(slice: &mut [u8]) -> Result<Self, Self::Error> {
                Self::try_from(&*slice)
            }
        }

        impl<'a> ::core::convert::TryFrom<&'a [u8]> for &'a $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            #[allow(unsafe_code)]
            fn try_from(slice: &'a [u8]) -> Result<&'a $name, Self::Error> {
                // SAFETY: `$name` is `repr(transparent)` for `FixedBytes<$n>`
                // and consequently `[u8; $n]`
                <&[u8; $n] as ::core::convert::TryFrom<&[u8]>>::try_from(slice)
                    .map(|array_ref| unsafe { core::mem::transmute(array_ref) })
            }
        }

        impl<'a> ::core::convert::TryFrom<&'a mut [u8]> for &'a mut $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            #[allow(unsafe_code)]
            fn try_from(slice: &'a mut [u8]) -> Result<&'a mut $name, Self::Error> {
                // SAFETY: `$name` is `repr(transparent)` for `FixedBytes<$n>`
                // and consequently `[u8; $n]`
                <&mut [u8; $n] as ::core::convert::TryFrom<&mut [u8]>>::try_from(slice)
                    .map(|array_ref| unsafe { core::mem::transmute(array_ref) })
            }
        }

        impl ::core::convert::AsRef<[u8; $n]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8; $n] {
                &self.0 .0
            }
        }

        impl ::core::convert::AsMut<[u8; $n]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8; $n] {
                &mut self.0 .0
            }
        }

        impl ::core::convert::AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                &self.0 .0
            }
        }

        impl ::core::convert::AsMut<[u8]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0 .0
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }

        $crate::impl_fixed_bytes_traits!($name, $n);
        $crate::impl_getrandom!($name);
        $crate::impl_rlp!($name);
        $crate::impl_serde!($name);
        $crate::impl_arbitrary!($name, $n);

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

            /// Create a new fixed-hash from the given slice `src`.
            ///
            /// # Note
            ///
            /// The given bytes are interpreted in big endian order.
            ///
            /// # Panics
            ///
            /// If the length of `src` and the number of bytes in `Self` do not match.
            #[inline]
            pub fn from_slice(src: &[u8]) -> Self {
                Self($crate::FixedBytes::from_slice(src))
            }

            /// Returns the inner bytes array.
            #[inline]
            pub const fn into_array(self) -> [u8; $n] {
                self.0 .0
            }

            /// Returns `true` if all bits set in `b` are also set in `self`.
            #[inline]
            pub fn covers(&self, b: &Self) -> bool {
                &(*b & *self) == b
            }

            /// Compile-time equality. NOT constant-time equality.
            pub const fn const_eq(&self, other: &Self) -> bool {
                self.0.const_eq(&other.0)
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
    };
}

// Extra traits that cannot be derived automatically
#[doc(hidden)]
#[macro_export]
macro_rules! impl_fixed_bytes_traits {
    (impl<$($const:ident)?> Borrow<$t:ty> for $b:ty) => {
        impl<$($const N: usize)?> ::core::borrow::Borrow<$t> for $b {
            #[inline]
            fn borrow(&self) -> &$t {
                ::core::borrow::Borrow::borrow(&self.0)
            }
        }
    };

    (impl<$($const:ident)?> BorrowMut<$t:ty> for $b:ty) => {
        impl<$($const N: usize)?> ::core::borrow::BorrowMut<$t> for $b {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $t {
                ::core::borrow::BorrowMut::borrow_mut(&mut self.0)
            }
        }
    };

    (impl<$($const:ident)?> cmp::$tr:ident<$a:ty> for $b:ty where fn $fn:ident -> $ret:ty $(, [$e:expr])?) => {
        impl<$($const N: usize)?> ::core::cmp::$tr<$a> for $b {
            #[inline]
            fn $fn(&self, other: &$a) -> $ret {
                ::core::cmp::$tr::$fn(&self.0 $([$e])?, other)
            }
        }

        impl<$($const N: usize)?> ::core::cmp::$tr<$b> for $a {
            #[inline]
            fn $fn(&self, other: &$b) -> $ret {
                ::core::cmp::$tr::$fn(self, &other.0 $([$e])?)
            }
        }

        impl<$($const N: usize)?> ::core::cmp::$tr<&$a> for $b {
            #[inline]
            fn $fn(&self, other: &&$a) -> $ret {
                ::core::cmp::$tr::$fn(&self.0 $([$e])?, *other)
            }
        }

        impl<$($const N: usize)?> ::core::cmp::$tr<$b> for &$a {
            #[inline]
            fn $fn(&self, other: &$b) -> $ret {
                ::core::cmp::$tr::$fn(*self, &other.0 $([$e])?)
            }
        }

        impl<$($const N: usize)?> ::core::cmp::$tr<$a> for &$b {
            #[inline]
            fn $fn(&self, other: &$a) -> $ret {
                ::core::cmp::$tr::$fn(&self.0 $([$e])?, other)
            }
        }

        impl<$($const N: usize)?> ::core::cmp::$tr<&$b> for $a {
            #[inline]
            fn $fn(&self, other: &&$b) -> $ret {
                ::core::cmp::$tr::$fn(self, &other.0 $([$e])?)
            }
        }
    };

    ($t:ty, $n:tt $(, $const:ident)?) => {
        // Borrow is not automatically implemented for references
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> Borrow<[u8]>        for $t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> Borrow<[u8]>        for &$t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> Borrow<[u8]>        for &mut $t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> Borrow<[u8; $n]>    for $t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> Borrow<[u8; $n]>    for &$t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> Borrow<[u8; $n]>    for &mut $t);

        $crate::impl_fixed_bytes_traits!(impl<$($const)?> BorrowMut<[u8]>     for $t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> BorrowMut<[u8]>     for &mut $t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> BorrowMut<[u8; $n]> for $t);
        $crate::impl_fixed_bytes_traits!(impl<$($const)?> BorrowMut<[u8; $n]> for &mut $t);

        // Implement PartialEq, PartialOrd, with slice and array
        $crate::impl_fixed_bytes_traits!(
            impl<$($const)?> cmp::PartialEq<[u8]> for $t where fn eq -> bool
        );
        $crate::impl_fixed_bytes_traits!(
            impl<$($const)?> cmp::PartialEq<[u8; $n]> for $t where fn eq -> bool
        );
        $crate::impl_fixed_bytes_traits!(
            impl<$($const)?> cmp::PartialOrd<[u8]> for $t
            where
                fn partial_cmp -> ::core::option::Option<::core::cmp::Ordering>,
                [..] // slices $t
        );
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
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::arbitrary()
                    .prop_map(Self)
            }

            #[inline]
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                use $crate::private::proptest::strategy::Strategy;
                <$crate::FixedBytes<$n> as $crate::private::proptest::arbitrary::Arbitrary>::arbitrary_with(args)
                    .prop_map(Self)
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

macro_rules! fixed_bytes_macros {
    ($d:tt $($(#[$attr:meta])* macro $name:ident($ty:ident);)*) => {$(
        /// Converts a sequence of string literals containing hex-encoded data
        #[doc = concat!(
            "into a new [`", stringify!($ty), "`][crate::", stringify!($ty), "].\n",
        )]
        ///
        /// Note that the strings cannot be prefixed with `0x`.
        ///
        /// See [`hex_literal::hex!`] for more information.
        $(#[$attr])*
        #[macro_export]
        macro_rules! $name {
            ($d ($d s:literal)*) => {
                $crate::$ty::new($crate::hex!($d ($d s)*))
            };
        }
    )*};
}

fixed_bytes_macros! { $
    macro address(Address);

    macro b64(B64);

    macro b128(B128);

    macro b256(B256);

    macro b512(B512);

    macro bloom(Bloom);

    macro fixed_bytes(FixedBytes);
}

/// Converts a sequence of string literals containing hex-encoded data into a
/// new [`Bytes`][crate::Bytes].
///
/// Note that the strings cannot be prefixed with `0x`.
///
/// See [`hex_literal::hex!`] for more information.
#[macro_export]
macro_rules! bytes {
    ($($s:literal)*) => {
        $crate::Bytes::from_static(&$crate::hex!($($s)*))
    };
}

#[cfg(test)]
mod tests {
    use crate::{Address, Bytes};
    use hex_literal::hex;

    #[test]
    fn fixed_byte_macros() {
        const A1: Address = address!("0102030405060708090a0b0c0d0e0f1011121314");
        const A2: Address = Address(fixed_bytes!("0102030405060708090a0b0c0d0e0f1011121314"));
        assert_eq!(A1, A2);
        assert_eq!(A1, hex!("0102030405060708090a0b0c0d0e0f1011121314"));

        static B: Bytes = bytes!("112233");
        assert_eq!(B[..], [0x11, 0x22, 0x33]);
    }
}
