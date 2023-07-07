use crate::{Header, EMPTY_STRING_CODE};
use arrayvec::ArrayVec;
use bytes::{BufMut, Bytes, BytesMut};
use core::borrow::Borrow;

macro_rules! to_be_bytes_trimmed {
    ($be:ident, $x:expr) => {{
        $be = $x.to_be_bytes();
        &$be[($x.leading_zeros() / 8) as usize..]
    }};
}
pub(crate) use to_be_bytes_trimmed;

/// Determine the length in bytes of the length prefix of an RLP item.
#[inline]
pub const fn length_of_length(payload_length: usize) -> usize {
    if payload_length < 56 {
        1
    } else {
        1 + 8 - payload_length.leading_zeros() as usize / 8
    }
}

#[doc(hidden)]
#[inline]
pub const fn const_add(a: usize, b: usize) -> usize {
    a + b
}

#[doc(hidden)]
pub unsafe trait MaxEncodedLen<const LEN: usize>: Encodable {}

#[doc(hidden)]
pub unsafe trait MaxEncodedLenAssoc: Encodable {
    const LEN: usize;
}

/// Use this to define length of an encoded entity
///
/// # Safety
///
/// An invalid value can cause the encoder to crash.
#[macro_export]
macro_rules! impl_max_encoded_len {
    ($t:ty, $len:expr) => {
        unsafe impl $crate::MaxEncodedLen<{ $len }> for $t {}
        unsafe impl $crate::MaxEncodedLenAssoc for $t {
            const LEN: usize = $len;
        }
    };
}

/// A type that can be encoded via RLP.
pub trait Encodable {
    /// Encode the type into the `out` buffer.
    fn encode(&self, out: &mut dyn BufMut);

    /// Return the length of the type in bytes
    ///
    /// The default implementation computes this by encoding the type. If
    /// feasible, we recommender implementers override this default impl.
    #[inline]
    fn length(&self) -> usize {
        let mut out = BytesMut::new();
        self.encode(&mut out);
        out.len()
    }
}

impl Encodable for [u8] {
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        if self.len() != 1 || self[0] >= EMPTY_STRING_CODE {
            Header {
                list: false,
                payload_length: self.len(),
            }
            .encode(out);
        }
        out.put_slice(self);
    }

    #[inline]
    fn length(&self) -> usize {
        let mut len = self.len();
        if self.len() != 1 || self[0] >= EMPTY_STRING_CODE {
            len += length_of_length(self.len());
        }
        len
    }
}

impl<const N: usize> Encodable for [u8; N] {
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        self[..].encode(out);
    }

    #[inline]
    fn length(&self) -> usize {
        self[..].length()
    }
}

unsafe impl<const N: usize> MaxEncodedLenAssoc for [u8; N] {
    const LEN: usize = N + length_of_length(N);
}

impl Encodable for str {
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        self.as_bytes().encode(out)
    }

    #[inline]
    fn length(&self) -> usize {
        self.as_bytes().length()
    }
}

impl Encodable for bool {
    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        // inlined `(*self as u8).encode(out)`
        out.put_u8(if *self { 1 } else { EMPTY_STRING_CODE });
    }

    #[inline]
    fn length(&self) -> usize {
        // a `bool` is always `< EMPTY_STRING_CODE`
        1
    }
}

impl_max_encoded_len!(bool, <u8 as MaxEncodedLenAssoc>::LEN);

macro_rules! uint_impl {
    ($($t:ty),+ $(,)?) => {$(
        impl Encodable for $t {
            #[inline]
            fn length(&self) -> usize {
                let x = *self;
                if x < EMPTY_STRING_CODE as $t {
                    1
                } else {
                    1 + (<$t>::BITS as usize / 8) - (x.leading_zeros() as usize / 8)
                }
            }

            #[inline]
            fn encode(&self, out: &mut dyn BufMut) {
                let x = *self;
                if x == 0 {
                    out.put_u8(EMPTY_STRING_CODE);
                } else if x < EMPTY_STRING_CODE as $t {
                    out.put_u8(x as u8);
                } else {
                    let be;
                    let be = to_be_bytes_trimmed!(be, x);
                    out.put_u8(EMPTY_STRING_CODE + be.len() as u8);
                    out.put_slice(be);
                }
            }
        }

        impl_max_encoded_len!($t, {
            length_of_length(<$t>::MAX.to_be_bytes().len()) + <$t>::MAX.to_be_bytes().len()
        });
    )+};
}

uint_impl!(u8, u16, u32, u64, usize, u128);

impl<T: Encodable> Encodable for alloc::vec::Vec<T> {
    #[inline]
    fn length(&self) -> usize {
        list_length(self)
    }

    #[inline]
    fn encode(&self, out: &mut dyn BufMut) {
        encode_list(self, out)
    }
}

macro_rules! deref_impl {
    ($([$($gen:tt)*] $t:ty),+ $(,)?) => {$(
        impl<$($gen)*> Encodable for $t {
            #[inline]
            fn encode(&self, out: &mut dyn BufMut) {
                (**self).encode(out)
            }

            #[inline]
            fn length(&self) -> usize {
                (**self).length()
            }
        }
    )+};
}

deref_impl! {
    [] alloc::string::String,
    [] Bytes,
    [] BytesMut,
    [T: ?Sized + Encodable] &T,
    [T: ?Sized + Encodable] &mut T,
    [T: ?Sized + Encodable] alloc::boxed::Box<T>,
    [T: ?Sized + alloc::borrow::ToOwned + Encodable] alloc::borrow::Cow<'_, T>,
    [T: ?Sized + Encodable] alloc::rc::Rc<T>,
    [T: ?Sized + Encodable] alloc::sync::Arc<T>,
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    impl Encodable for IpAddr {
        fn encode(&self, out: &mut dyn BufMut) {
            match self {
                IpAddr::V4(ip) => ip.encode(out),
                IpAddr::V6(ip) => ip.encode(out),
            }
        }

        fn length(&self) -> usize {
            match self {
                IpAddr::V4(ip) => ip.length(),
                IpAddr::V6(ip) => ip.length(),
            }
        }
    }

    impl Encodable for Ipv4Addr {
        fn encode(&self, out: &mut dyn BufMut) {
            Encodable::encode(&self.octets()[..], out)
        }

        fn length(&self) -> usize {
            Encodable::length(&self.octets()[..])
        }
    }

    impl Encodable for Ipv6Addr {
        fn encode(&self, out: &mut dyn BufMut) {
            Encodable::encode(&self.octets()[..], out)
        }

        fn length(&self) -> usize {
            Encodable::length(&self.octets()[..])
        }
    }
}

fn rlp_list_header<E, K>(v: &[K]) -> Header
where
    E: Encodable + ?Sized,
    K: Borrow<E>,
{
    let mut h = Header {
        list: true,
        payload_length: 0,
    };
    for x in v {
        h.payload_length += x.borrow().length();
    }
    h
}

/// Calculate the length of a list.
pub fn list_length<E, K>(v: &[K]) -> usize
where
    E: Encodable,
    K: Borrow<E>,
{
    let payload_length = rlp_list_header(v).payload_length;
    length_of_length(payload_length) + payload_length
}

/// Encode a list of items.
pub fn encode_list<E: Encodable>(v: &[E], out: &mut dyn BufMut) {
    let h = rlp_list_header(v);
    h.encode(out);
    for x in v {
        x.borrow().encode(out);
    }
}

/// Encode all items from an iterator.
///
/// This clones the iterator. Prefer [`encode_list`] if possible.
pub fn encode_iter<K, I>(i: I, out: &mut dyn BufMut)
where
    K: Encodable,
    I: Iterator<Item = K> + Clone,
{
    let mut h = Header {
        list: true,
        payload_length: 0,
    };
    for x in i.clone() {
        h.payload_length += x.length();
    }

    h.encode(out);
    for x in i {
        x.encode(out);
    }
}

/// Encode a type with a known maximum size.
pub fn encode_fixed_size<E: MaxEncodedLen<LEN>, const LEN: usize>(v: &E) -> ArrayVec<u8, LEN> {
    let mut out = ArrayVec::from([0_u8; LEN]);

    let mut s = out.as_mut_slice();

    v.encode(&mut s);

    let final_len = LEN - s.len();
    out.truncate(final_len);

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use hex_literal::hex;

    fn encoded<T: Encodable>(t: T) -> BytesMut {
        let mut out = BytesMut::new();
        t.encode(&mut out);
        out
    }

    fn encoded_list<T: Encodable + Clone>(t: &[T]) -> BytesMut {
        let mut out1 = BytesMut::new();
        encode_list(t, &mut out1);

        let v = t.to_vec();
        assert_eq!(out1.len(), v.length());

        let mut out2 = BytesMut::new();
        v.encode(&mut out2);
        assert_eq!(out1, out2);

        out1
    }

    fn encoded_iter<'a, T: Encodable + 'a>(iter: impl Iterator<Item = &'a T> + Clone) -> BytesMut {
        let mut out = BytesMut::new();
        encode_iter(iter, &mut out);
        out
    }

    #[test]
    fn rlp_str() {
        assert_eq!(encoded("")[..], hex!("80")[..]);
        assert_eq!(encoded("{")[..], hex!("7b")[..]);
        assert_eq!(encoded("test str")[..], hex!("887465737420737472")[..]);
    }

    #[test]
    fn rlp_strings() {
        assert_eq!(encoded(hex!(""))[..], hex!("80")[..]);
        assert_eq!(encoded(hex!("7B"))[..], hex!("7b")[..]);
        assert_eq!(encoded(hex!("80"))[..], hex!("8180")[..]);
        assert_eq!(encoded(hex!("ABBA"))[..], hex!("82abba")[..]);
    }

    fn c<T, U: From<T>>(
        it: impl IntoIterator<Item = (T, &'static [u8])>,
    ) -> impl Iterator<Item = (U, &'static [u8])> {
        it.into_iter().map(|(k, v)| (k.into(), v))
    }

    fn u8_fixtures() -> impl IntoIterator<Item = (u8, &'static [u8])> {
        vec![
            (0, &hex!("80")[..]),
            (1, &hex!("01")[..]),
            (0x7F, &hex!("7F")[..]),
            (0x80, &hex!("8180")[..]),
        ]
    }

    fn u16_fixtures() -> impl IntoIterator<Item = (u16, &'static [u8])> {
        c(u8_fixtures()).chain(vec![(0x400, &hex!("820400")[..])])
    }

    fn u32_fixtures() -> impl IntoIterator<Item = (u32, &'static [u8])> {
        c(u16_fixtures()).chain(vec![
            (0xFFCCB5, &hex!("83ffccb5")[..]),
            (0xFFCCB5DD, &hex!("84ffccb5dd")[..]),
        ])
    }

    fn u64_fixtures() -> impl IntoIterator<Item = (u64, &'static [u8])> {
        c(u32_fixtures()).chain(vec![
            (0xFFCCB5DDFF, &hex!("85ffccb5ddff")[..]),
            (0xFFCCB5DDFFEE, &hex!("86ffccb5ddffee")[..]),
            (0xFFCCB5DDFFEE14, &hex!("87ffccb5ddffee14")[..]),
            (0xFFCCB5DDFFEE1483, &hex!("88ffccb5ddffee1483")[..]),
        ])
    }

    fn u128_fixtures() -> impl IntoIterator<Item = (u128, &'static [u8])> {
        c(u64_fixtures()).chain(vec![(
            0x10203E405060708090A0B0C0D0E0F2,
            &hex!("8f10203e405060708090a0b0c0d0e0f2")[..],
        )])
    }

    macro_rules! uint_rlp_test {
        ($fixtures:expr) => {
            for (input, output) in $fixtures {
                assert_eq!(encoded(input), output);
            }
        };
    }

    #[test]
    fn rlp_uints() {
        uint_rlp_test!(u8_fixtures());
        uint_rlp_test!(u16_fixtures());
        uint_rlp_test!(u32_fixtures());
        uint_rlp_test!(u64_fixtures());
        uint_rlp_test!(u128_fixtures());
        // #[cfg(feature = "ethnum")]
        // uint_rlp_test!(u256_fixtures());
    }

    /*
    #[cfg(feature = "ethnum")]
    fn u256_fixtures() -> impl IntoIterator<Item = (ethnum::U256, &'static [u8])> {
        c(u128_fixtures()).chain(vec![(
            ethnum::U256::from_str_radix(
                "0100020003000400050006000700080009000A0B4B000C000D000E01",
                16,
            )
            .unwrap(),
            &hex!("9c0100020003000400050006000700080009000a0b4b000c000d000e01")[..],
        )])
    }

    #[cfg(feature = "ethereum-types")]
    fn eth_u64_fixtures() -> impl IntoIterator<Item = (ethereum_types::U64, &'static [u8])> {
        c(u64_fixtures()).chain(vec![
            (
                ethereum_types::U64::from_str_radix("FFCCB5DDFF", 16).unwrap(),
                &hex!("85ffccb5ddff")[..],
            ),
            (
                ethereum_types::U64::from_str_radix("FFCCB5DDFFEE", 16).unwrap(),
                &hex!("86ffccb5ddffee")[..],
            ),
            (
                ethereum_types::U64::from_str_radix("FFCCB5DDFFEE14", 16).unwrap(),
                &hex!("87ffccb5ddffee14")[..],
            ),
            (
                ethereum_types::U64::from_str_radix("FFCCB5DDFFEE1483", 16).unwrap(),
                &hex!("88ffccb5ddffee1483")[..],
            ),
        ])
    }

    fn eth_u128_fixtures() -> impl IntoIterator<Item = (ethereum_types::U128, &'static [u8])> {
        c(u128_fixtures()).chain(vec![(
            ethereum_types::U128::from_str_radix("10203E405060708090A0B0C0D0E0F2", 16).unwrap(),
            &hex!("8f10203e405060708090a0b0c0d0e0f2")[..],
        )])
    }

    fn eth_u256_fixtures() -> impl IntoIterator<Item = (ethereum_types::U256, &'static [u8])> {
        c(u128_fixtures()).chain(vec![(
            ethereum_types::U256::from_str_radix(
                "0100020003000400050006000700080009000A0B4B000C000D000E01",
                16,
            )
            .unwrap(),
            &hex!("9c0100020003000400050006000700080009000a0b4b000c000d000e01")[..],
        )])
    }

    #[cfg(feature = "ethereum-types")]
    fn eth_u512_fixtures() -> impl IntoIterator<Item = (ethereum_types::U512, &'static [u8])> {
        c(eth_u256_fixtures()).chain(vec![(
            ethereum_types::U512::from_str_radix(
                "0100020003000400050006000700080009000A0B4B000C000D000E010100020003000400050006000700080009000A0B4B000C000D000E01",
                16,
            )
            .unwrap(),
            &hex!("b8380100020003000400050006000700080009000A0B4B000C000D000E010100020003000400050006000700080009000A0B4B000C000D000E01")[..],
        )])
    }

    #[cfg(feature = "ethereum-types")]
    #[test]
    fn rlp_eth_uints() {
        uint_rlp_test!(eth_u64_fixtures());
        uint_rlp_test!(eth_u128_fixtures());
        uint_rlp_test!(eth_u256_fixtures());
        uint_rlp_test!(eth_u512_fixtures());
    }
    */

    #[test]
    fn rlp_list() {
        assert_eq!(encoded_list::<u64>(&[]), &hex!("c0")[..]);
        assert_eq!(encoded_list::<u8>(&[0x00u8]), &hex!("c180")[..]);
        assert_eq!(
            encoded_list(&[0xFFCCB5_u64, 0xFFC0B5_u64]),
            &hex!("c883ffccb583ffc0b5")[..]
        );
    }

    #[test]
    fn rlp_iter() {
        assert_eq!(encoded_iter::<u64>([].iter()), &hex!("c0")[..]);
        assert_eq!(
            encoded_iter([0xFFCCB5_u64, 0xFFC0B5_u64].iter()),
            &hex!("c883ffccb583ffc0b5")[..]
        );
    }

    #[test]
    fn to_be_bytes_trimmed() {
        macro_rules! test_to_be_bytes_trimmed {
            ($($x:expr => $expected:expr),+ $(,)?) => {$(
                let be;
                assert_eq!(to_be_bytes_trimmed!(be, $x), $expected);
            )+};
        }

        test_to_be_bytes_trimmed! {
            0u8 => [],
            0u16 => [],
            0u32 => [],
            0u64 => [],
            0usize => [],
            0u128 => [],

            1u8 => [1],
            1u16 => [1],
            1u32 => [1],
            1u64 => [1],
            1usize => [1],
            1u128 => [1],

            u8::MAX => [0xff],
            u16::MAX => [0xff, 0xff],
            u32::MAX => [0xff, 0xff, 0xff, 0xff],
            u64::MAX => [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            u128::MAX => [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],

            1u8 => [1],
            255u8 => [255],
            256u16 => [1, 0],
            65535u16 => [255, 255],
            65536u32 => [1, 0, 0],
            65536u64 => [1, 0, 0],
        }
    }
}
