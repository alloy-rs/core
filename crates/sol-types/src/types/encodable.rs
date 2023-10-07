use super::SolType;
use crate::{
    abi::TokenSeq,
    private::SolTypeEncodable,
    sol_data::{self, ByteCount, SupportedFixedBytes},
    Result, Word,
};
use alloc::{borrow::Cow, string::String, vec::Vec};
use alloy_primitives::{Address, Bytes, FixedBytes, Function, I256, U256};

/// Solidity values.
///
/// This is a convenience trait that re-exports the logic in [`SolType`] with
/// less generic implementations so that they can be used as methods with `self`
/// receivers.
///
/// See [`SolType`] for more information.
///
/// # Implementer's Guide
///
/// It should not be necessary to implement this trait manually. Instead, use
/// the [`sol!`](crate::sol!) procedural macro to parse Solidity syntax into
/// types that implement this trait.
///
/// # Examples
///
/// ```
/// use alloy_sol_types::SolValue;
///
/// let my_values = ("hello", 0xdeadbeef_u32, true, [0x42_u8; 24]);
/// let _ = my_values.abi_encode();
/// let _ = my_values.abi_encode_packed();
/// assert_eq!(my_values.sol_type_name(), "(string,uint32,bool,bytes24)");
/// ```
pub trait SolValue: SolTypeEncodable<Self::SolType> {
    /// The Solidity type that this type corresponds to.
    type SolType: SolType;

    /// The name of the associated Solidity type.
    ///
    /// See [`SolType::sol_type_name`] for more information.
    #[inline]
    fn sol_type_name(&self) -> Cow<'static, str> {
        Self::SolType::sol_type_name()
    }

    /// Tokenizes this value into this type's token.
    ///
    /// See [`SolType::tokenize`] for more information.
    #[inline]
    fn tokenize(&self) -> <Self::SolType as SolType>::TokenType<'_> {
        <Self as SolTypeEncodable<Self::SolType>>::to_tokens(self)
    }

    /// Detokenize the given token into this type.
    ///
    /// See [`SolType::detokenize`] for more information.
    #[inline]
    fn detokenize(token: <Self::SolType as SolType>::TokenType<'_>) -> Self
    where
        Self: From<<Self::SolType as SolType>::RustType>,
    {
        Self::from(<Self::SolType as SolType>::detokenize(token))
    }

    /// Calculate the ABI-encoded size of the data.
    ///
    /// See [`SolType::abi_encoded_size`] for more information.
    #[inline]
    fn abi_encoded_size(&self) -> usize {
        <Self as SolTypeEncodable<Self::SolType>>::abi_encoded_size(self)
    }

    /// Encode this data according to EIP-712 `encodeData` rules, and hash it
    /// if necessary.
    ///
    /// See [`SolType::eip712_data_word`] for more information.
    #[inline]
    fn eip712_data_word(&self) -> Word {
        <Self as SolTypeEncodable<Self::SolType>>::eip712_data_word(self)
    }

    /// Non-standard Packed Mode ABI encoding.
    ///
    /// See [`SolType::abi_encode_packed_to`] for more information.
    #[inline]
    fn abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        <Self as SolTypeEncodable<Self::SolType>>::abi_encode_packed_to(self, out)
    }

    /// Non-standard Packed Mode ABI encoding.
    ///
    /// See [`SolType::abi_encode_packed`] for more information.
    #[inline]
    fn abi_encode_packed(&self) -> Vec<u8> {
        let mut out = Vec::new();
        <Self as SolTypeEncodable<Self::SolType>>::abi_encode_packed_to(self, &mut out);
        out
    }

    /// ABI-encodes the value.
    ///
    /// See [`SolType::abi_encode`] for more information.
    #[inline]
    fn abi_encode(&self) -> Vec<u8> {
        Self::SolType::abi_encode(self)
    }

    /// Encodes an ABI sequence.
    ///
    /// See [`SolType::abi_encode_sequence`] for more information.
    #[inline]
    fn abi_encode_sequence(&self) -> Vec<u8>
    where
        for<'a> <Self::SolType as SolType>::TokenType<'a>: TokenSeq<'a>,
    {
        Self::SolType::abi_encode_sequence(self)
    }

    /// Encodes an ABI sequence suitable for function parameters.
    ///
    /// See [`SolType::abi_encode_params`] for more information.
    #[inline]
    fn abi_encode_params(&self) -> Vec<u8>
    where
        for<'a> <Self::SolType as SolType>::TokenType<'a>: TokenSeq<'a>,
    {
        Self::SolType::abi_encode_params(self)
    }

    /// ABI-decode this type from the given data.
    ///
    /// See [`SolType::abi_decode`] for more information.
    fn abi_decode(data: &[u8], validate: bool) -> Result<Self>
    where
        Self: From<<Self::SolType as SolType>::RustType>,
    {
        Self::SolType::abi_decode(data, validate).map(Self::from)
    }

    /// ABI-decode this type from the given data.
    ///
    /// See [`SolType::abi_decode_params`] for more information.
    #[inline]
    fn abi_decode_params<'de>(data: &'de [u8], validate: bool) -> Result<Self>
    where
        Self: From<<Self::SolType as SolType>::RustType>,
        <Self::SolType as SolType>::TokenType<'de>: TokenSeq<'de>,
    {
        Self::SolType::abi_decode_params(data, validate).map(Self::from)
    }

    /// ABI-decode this type from the given data.
    ///
    /// See [`SolType::abi_decode_sequence`] for more information.
    #[inline]
    fn abi_decode_sequence<'de>(data: &'de [u8], validate: bool) -> Result<Self>
    where
        Self: From<<Self::SolType as SolType>::RustType>,
        <Self::SolType as SolType>::TokenType<'de>: TokenSeq<'de>,
    {
        Self::SolType::abi_decode_sequence(data, validate).map(Self::from)
    }
}

macro_rules! impl_encodable {
    ($($(#[$attr:meta])* [$($gen:tt)*] $rust:ty => $sol:ty [$($where:tt)*];)+) => {$(
        $(#[$attr])*
        impl<$($gen)*> SolValue for $rust $($where)* {
            type SolType = $sol;
        }
    )*};
}

impl_encodable! {
    // Basic
    [] bool => sol_data::Bool [];

    [] i8 => sol_data::Int::<8> [];
    [] i16 => sol_data::Int::<16> [];
    [] i32 => sol_data::Int::<32> [];
    [] i64 => sol_data::Int::<64> [];
    [] i128 => sol_data::Int::<128> [];
    [] I256 => sol_data::Int::<256> [];
    #[cfg(pointer_width = "32")]
    [] isize => sol_data::Int::<32> [];
    #[cfg(pointer_width = "64")]
    [] isize => sol_data::Int::<64> [];

    // TODO: Array<u8> is specialized to encode as `bytes[N]`
    // [] u8 => sol_data::Uint::<8> [];
    [] u16 => sol_data::Uint::<16> [];
    [] u32 => sol_data::Uint::<32> [];
    [] u64 => sol_data::Uint::<64> [];
    [] u128 => sol_data::Uint::<128> [];
    [] U256 => sol_data::Uint::<256> [];
    #[cfg(pointer_width = "32")]
    [] usize => sol_data::Uint::<32> [];
    #[cfg(pointer_width = "64")]
    [] usize => sol_data::Uint::<64> [];

    [] Address => sol_data::Address [];
    [] Function => sol_data::Function [];
    [const N: usize] FixedBytes<N> => sol_data::FixedBytes<N> [where ByteCount<N>: SupportedFixedBytes];
    [const N: usize] [u8; N] => sol_data::FixedBytes<N> [where ByteCount<N>: SupportedFixedBytes];
    [] String => sol_data::String [];
    [] str => sol_data::String [];
    [] Bytes => sol_data::Bytes [];

    [] Vec<u8> => sol_data::Bytes [];
    [] [u8] => sol_data::Bytes [];

    // Generic
    [T: SolValue] Vec<T> => sol_data::Array<T::SolType> [];
    [T: SolValue] [T] => sol_data::Array<T::SolType> [];
    [T: SolValue, const N: usize] [T; N] => sol_data::FixedArray<T::SolType, N> [];

    ['a, T: ?Sized + SolValue] &'a T => T::SolType [where &'a T: SolTypeEncodable<T::SolType>];
    ['a, T: ?Sized + SolValue] &'a mut T => T::SolType [where &'a mut T: SolTypeEncodable<T::SolType>];
}

macro_rules! tuple_impls {
    ($count:literal $($ty:ident),+) => {
        impl<$($ty: SolValue,)+> SolValue for ($($ty,)+) {
            type SolType = ($($ty::SolType,)+);
        }
    };
}

impl SolValue for () {
    type SolType = ();
}

all_the_tuples!(tuple_impls);

#[cfg(test)]
#[allow(clippy::type_complexity)]
mod tests {
    use super::*;

    // Make sure these are in scope
    #[allow(unused_imports)]
    use crate::{private::SolTypeEncodable as _, SolType as _};

    #[test]
    fn inference() {
        false.sol_type_name();
        // false.abi_encoded_size();
        // false.eip712_data_word();
        // false.abi_encode_packed_to(&mut vec![]);
        false.abi_encode_packed();
        false.abi_encode();
        (false,).abi_encode_sequence();
        (false,).abi_encode_params();

        "".sol_type_name();
        // "".abi_encoded_size();
        // "".eip712_data_word();
        // "".abi_encode_packed_to(&mut vec![]);
        "".abi_encode_packed();
        "".abi_encode();
        ("",).abi_encode_sequence();
        ("",).abi_encode_params();
    }

    #[test]
    fn basic() {
        assert_eq!(false.abi_encode(), Word::ZERO[..]);
        assert_eq!(true.abi_encode(), Word::with_last_byte(1)[..]);

        assert_eq!(0i8.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i16.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i32.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i64.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i128.abi_encode(), Word::ZERO[..]);
        assert_eq!(I256::ZERO.abi_encode(), Word::ZERO[..]);

        assert_eq!(0u16.abi_encode(), Word::ZERO[..]);
        assert_eq!(0u32.abi_encode(), Word::ZERO[..]);
        assert_eq!(0u64.abi_encode(), Word::ZERO[..]);
        assert_eq!(0u128.abi_encode(), Word::ZERO[..]);
        assert_eq!(U256::ZERO.abi_encode(), Word::ZERO[..]);

        assert_eq!(Address::ZERO.abi_encode(), Word::ZERO[..]);
        assert_eq!(Function::ZERO.abi_encode(), Word::ZERO[..]);

        let encode_bytes = |b: &[u8]| {
            let last = Word::new({
                let mut buf = [0u8; 32];
                buf[..b.len()].copy_from_slice(b);
                buf
            });
            [
                &Word::with_last_byte(0x20)[..],
                &Word::with_last_byte(b.len() as u8)[..],
                if b.is_empty() { b } else { &last[..] },
            ]
            .concat()
        };

        // empty `bytes`
        assert_eq!(b"".abi_encode(), encode_bytes(b""));
        assert_eq!((b"" as &[_]).abi_encode(), encode_bytes(b""));
        // `bytes1`
        assert_eq!(b"a".abi_encode()[0], b'a');
        assert_eq!(b"a".abi_encode()[1..], Word::ZERO[1..]);
        // `bytes`
        assert_eq!((b"a" as &[_]).abi_encode(), encode_bytes(b"a"));

        assert_eq!("".abi_encode(), encode_bytes(b""));
        assert_eq!("a".abi_encode(), encode_bytes(b"a"));
        assert_eq!(String::new().abi_encode(), encode_bytes(b""));
        assert_eq!(String::from("a").abi_encode(), encode_bytes(b"a"));
        assert_eq!(Vec::<u8>::new().abi_encode(), encode_bytes(b""));
        assert_eq!(Vec::<u8>::from(&b"a"[..]).abi_encode(), encode_bytes(b"a"));
    }

    #[test]
    fn big() {
        let tuple = (
            false,
            0i8,
            0i16,
            0i32,
            0i64,
            0i128,
            I256::ZERO,
            // 0u8,
            0u16,
            0u32,
            0u64,
            0u128,
            U256::ZERO,
            Address::ZERO,
            Function::ZERO,
        );
        let encoded = tuple.abi_encode();
        assert_eq!(encoded.len(), 32 * 14);
        assert!(encoded.iter().all(|&b| b == 0));
    }

    #[test]
    fn complex() {
        let tuple = ((((((false,),),),),),);
        assert_eq!(tuple.abi_encode(), Word::ZERO[..]);
        assert_eq!(tuple.sol_type_name(), "((((((bool))))))");

        let tuple = (
            42u64,
            "hello world",
            true,
            (
                String::from("aaaa"),
                Address::with_last_byte(69),
                b"bbbb".to_vec(),
                b"cccc",
                &b"dddd"[..],
            ),
        );
        assert_eq!(
            tuple.sol_type_name(),
            "(uint64,string,bool,(string,address,bytes,bytes4,bytes))"
        );
    }

    #[test]
    fn derefs() {
        let x: &[Address; 0] = &[];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[0]");

        let x = &[Address::ZERO];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[1]");

        let x = &[Address::ZERO, Address::ZERO];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[2]");

        let x = &[Address::ZERO][..];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[]");

        let mut x = *b"0";
        let x = (&mut x, *b"aaaa", b"00");
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "(bytes1,bytes4,bytes2)");

        let tuple = &(&0u16, &"", b"0", &mut [Address::ZERO][..]);
        tuple.abi_encode();
        assert_eq!(tuple.sol_type_name(), "(uint16,string,bytes1,address[])");
    }

    #[test]
    fn decode() {
        let _: Result<String> = String::abi_decode(b"", false);

        let _: Result<Vec<String>> = Vec::<String>::abi_decode(b"", false);

        let _: Result<(u64, String, U256)> = <(u64, String, U256)>::abi_decode(b"", false);
        let _: Result<(i64, Vec<(u32, String, Vec<FixedBytes<4>>)>, U256)> =
            <(i64, Vec<(u32, String, Vec<FixedBytes<4>>)>, U256)>::abi_decode(b"", false);
    }
}
