use alloc::borrow::Cow;
use core::marker::PhantomData;
use ethers_primitives::{keccak256, Address as RustAddress, I256, U256};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::{Borrow, String as RustString, ToOwned, Vec};
#[cfg(feature = "std")]
use std::{borrow::Borrow, string::String as RustString};

use crate::{decode, decode_params, decode_single, token::*, util, AbiResult, Error, Word};

/// A Solidity Type, for ABI enc/decoding
///
/// This trait is implemented by types that contain ABI enc/decoding info for
/// solidity types. Types may be combined to express arbitrarily complex
/// solidity types.
///
/// Future work will add derive for this trait :)
///
/// ```
/// use ethers_abi_enc::sol_type::*;
///
/// // uint256[]
/// type DynUint256Array = Array<Uint<256>>;
/// assert_eq!(&DynUint256Array::sol_type_name(), "uint256[]");
///
/// type Erc20FunctionArgs = (Address, Uint<256>);
/// assert_eq!(&Erc20FunctionArgs::sol_type_name(), "tuple(address,uint256)");
///
/// type LargeComplexType = (FixedArray<Array<Bool>, 2>, (FixedBytes<13>, String));
/// assert_eq!(&LargeComplexType::sol_type_name(), "tuple(bool[][2],tuple(bytes13,string))");
/// ```
///
/// These types are zero cost representations of Solidity types. They do not
/// exist at runtime. They ONLY information about the type, they do not carry
/// data
///
/// ### Implementer's Guide
///
/// You may want to implement this on your own struct, for example, to encode a
/// named solidity struct.
///
/// Overall, implementing this trait is straightforward.
///
/// ```
/// # use ethers_abi_enc::{AbiResult, Word, no_std_prelude::Borrow};
/// # use ethers_abi_enc::sol_type::*;
/// # use ethers_primitives::U256;
///
/// // This is the solidity type:
/// //
/// // struct MySolidityStruct {
/// //    uint256 a;
/// //    uint256 b;
/// // }
///
/// // This should be a ZST. See note.
/// pub struct MySolidityStruct;
///
/// // This will be the data type in rust.
/// pub struct MyRustStruct {
///    a: U256,
///    b: U256,
/// }
///
/// // We're going to get really cute here.
/// //
/// // Structs are encoded as Tuples. So we can entirely define this trait by
/// // delegating to a tuple type!
/// type UnderlyingTuple = (Uint<256>, Uint<256>);
///
/// impl SolType for MySolidityStruct {
///     type RustType = MyRustStruct;
///     type TokenType = <UnderlyingTuple as SolType>::TokenType;
///
///     // The name in solidity
///     fn sol_type_name() -> std::borrow::Cow<'static, str> {
///         "MySolidityStruct".into()
///     }
///
///     // True if your type has a dynamic encoding length. This is dynamic
///     // arrays, strings, bytes, dynamic tuple etc.
///     //
///     // Of course, we can cheat here by delegating to the tuple
///     fn is_dynamic() -> bool {
///         UnderlyingTuple::is_dynamic()
///     }
///
///     // This function should check the data in the token and enforce any
///     // type rules. For example, a bool should ONLY be 0 or 1. This function
///     // should check the data, and return false if the bool is 2 or 3 or
///     // whatever.
///     //
///     // It will be ignored if the decoder runs without validation
///     fn type_check(token: &Self::TokenType) -> AbiResult<()> {
///         UnderlyingTuple::type_check(token)
///     }
///
///     // This function defines the EIP-712 encoding of the type. This is
///     // used to encode types for EIP-712 typed data signing. For value types
///     // it is equal to the ABI encoding. For compound types, it is the
///     // keccak256 hash of the encoding of the components.
///     //
///     // Our implementation is easy, we just delegate :)
///     fn eip712_data_word<B>(rust: B) -> Word
///     where
///         B: Borrow<Self::RustType>
///     {
///         let rust = rust.borrow();
///         UnderlyingTuple::eip712_data_word((rust.a, rust.b))
///     }
///
///     // Convert from the token to the rust type. We cheat here again by
///     // delegating.
///     fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
///         let (a, b) = UnderlyingTuple::detokenize(token)?;
///         Ok(MyRustStruct{ a, b })
///     }
///
///     // Convert from the rust type to the token type. We cheat here AGAIN
///     // by delegating.
///     fn tokenize<B>(rust: B) -> Self::TokenType
///     where
///        B: std::borrow::Borrow<Self::RustType>,
///     {
///         let MyRustStruct { a, b } = *rust.borrow();
///         UnderlyingTuple::tokenize((a, b))
///     }
///
///     // Implement packed encoding
///     fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B)
///     where
///        B: std::borrow::Borrow<Self::RustType>,
///     {
///        let MyRustStruct { a, b } = *rust.borrow();
///        UnderlyingTuple::encode_packed_to(target, (a, b))
///     }
///
/// }
/// ```
///
/// As you can see, because any NEW soltype corresponds to some combination of
/// OLD sol types, it's really easy to implement [`SolType`] for anything you
/// want!
///
/// #### Note on implementing type size
///
/// Any type implementing this should be 0-sized. This trait exposes only
/// associated functions and types, and not methods.
///
/// ```ignore
/// // Bad - This type is sized.
/// pub struct MyStruct(usize);
///
/// impl SolType for MyStruct { ... }
///
/// // Good - This type is 0 sized.
/// pub struct MyStruct;
///
/// impl SolType for MyStruct { ... }
/// ```
pub trait SolType {
    /// The corresponding Rust type. This type may be borrowed (e.g. `str`)
    type RustType;

    /// The corresponding ABI token type.
    ///
    /// See implementers of [`TokenType`].
    type TokenType: TokenType;

    /// The name of the type in solidity
    fn sol_type_name() -> Cow<'static, str>;

    /// True if the type is dynamic according to ABI rules
    fn is_dynamic() -> bool;

    /// True if the type is a user defined type. These include structs, enums,
    /// and user defined value types
    fn is_user_defined() -> bool {
        false
    }

    /// The encoded struct type (as EIP-712), if any. None for non-structs
    fn eip712_encode_type() -> Option<Cow<'static, str>> {
        None
    }

    /// Check a token to see if it can be detokenized with this type
    fn type_check(token: &Self::TokenType) -> AbiResult<()>;

    /// Detokenize
    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType>;

    /// Tokenize
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType;

    /// Implemens Solidity's `encodePacked()` function, writing into the given buffer.
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B);

    /// Implements Solidity's `encodePacked()` function.
    fn encode_packed<B: Borrow<Self::RustType>>(rust: B) -> Vec<u8> {
        let mut res = Vec::new();
        Self::encode_packed_to(&mut res, rust);
        res
    }

    #[doc(hidden)]
    fn type_check_fail(data: &[u8]) -> Error {
        Error::type_check_fail(hex::encode(data), Self::sol_type_name())
    }

    /// Encode this data according to EIP-712 `encodeData` rules, and hash it
    /// if necessary.
    ///
    /// Implementer's note: All single-word types are encoded as their word.
    /// All multi-word types are encoded as the hash the concatenated data
    /// words for each element
    ///
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word;

    /// Encode a single ABI token by wrapping it in a 1-length sequence
    fn encode_single<B: Borrow<Self::RustType>>(rust: B) -> Vec<u8> {
        let token = Self::tokenize(rust);
        crate::encode_single(token)
    }

    /// Encode an ABI sequence
    fn encode<B>(rust: B) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        let token = Self::tokenize(rust);
        crate::encode(token)
    }

    /// Encode an ABI sequence suitable for function params
    fn encode_params<B>(rust: B) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        let token = Self::tokenize(rust);
        crate::encode_params(token)
    }

    /// Hex output of encode
    fn hex_encode<B>(rust: B) -> RustString
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        format!("0x{}", hex::encode(Self::encode(rust)))
    }

    /// Hex output of encode_single
    fn hex_encode_single<B: Borrow<Self::RustType>>(rust: B) -> RustString {
        format!("0x{}", hex::encode(Self::encode_single(rust)))
    }

    /// Hex output of encode_params
    fn hex_encode_params<B>(rust: B) -> RustString
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        format!("0x{}", hex::encode(Self::encode_params(rust)))
    }

    /// Decode a Rust type from an ABI blob
    fn decode(data: &[u8], validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let decoded = decode::<Self::TokenType>(data, validate)?;
        if validate {
            Self::type_check(&decoded)?;
        }
        Self::detokenize(decoded)
    }

    /// Decode a Rust type from an ABI blob
    fn decode_params(data: &[u8], validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let decoded = decode_params::<Self::TokenType>(data, validate)?;
        if validate {
            Self::type_check(&decoded)?;
        }
        Self::detokenize(decoded)
    }

    /// Decode a Rust type from an ABI blob
    fn decode_single(data: &[u8], validate: bool) -> AbiResult<Self::RustType> {
        let decoded = decode_single::<Self::TokenType>(data, validate)?;
        if validate {
            Self::type_check(&decoded)?;
        }
        Self::detokenize(decoded)
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode(data: &str, validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(Into::into)
            .and_then(|buf| Self::decode(&buf, validate))
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode_single(data: &str, validate: bool) -> AbiResult<Self::RustType> {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(Into::into)
            .and_then(|buf| Self::decode_single(&buf, validate))
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode_params(data: &str, validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(Into::into)
            .and_then(|buf| Self::decode_params(&buf, validate))
    }
}

/// Address - `address`
#[derive(Copy, Clone, Debug)]
pub struct Address;

impl SolType for Address {
    type RustType = RustAddress;
    type TokenType = WordToken;

    fn sol_type_name() -> Cow<'static, str> {
        "address".into()
    }

    fn is_dynamic() -> bool {
        false
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if !util::check_zeroes(&token.inner()[..12]) {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        Self::tokenize(rust).inner()
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        let sli = &token.as_slice()[12..];
        Ok(RustAddress::from_slice(sli))
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        WordToken::from(*rust.borrow())
    }

    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
        // push the last 20 bytes of the word to the target
        target.extend_from_slice(&rust.borrow().as_bytes()[12..]);
    }
}

/// Bytes - `bytes`
#[derive(Copy, Clone, Debug)]
pub struct Bytes;

impl SolType for Bytes {
    type RustType = Vec<u8>;
    type TokenType = PackedSeqToken;

    fn is_dynamic() -> bool {
        true
    }

    fn sol_type_name() -> Cow<'static, str> {
        "bytes".into()
    }

    fn type_check(_token: &Self::TokenType) -> AbiResult<()> {
        Ok(())
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        keccak256(Self::encode_packed(rust.borrow()))
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.take_vec())
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        rust.borrow().to_owned().into()
    }

    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
        // push the buf to the vec
        target.extend_from_slice(rust.borrow());
    }
}

macro_rules! impl_int_sol_type {
    ($ity:ty, $bits:literal) => {
        impl SolType for Int<$bits> {
            type RustType = $ity;
            type TokenType = WordToken;

            fn is_dynamic() -> bool {
                false
            }

            fn sol_type_name() -> Cow<'static, str> {
                concat!("int", $bits).into()
            }

            fn type_check(_token: &Self::TokenType) -> AbiResult<()> {
                Ok(())
            }

            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let bytes = (<$ity>::BITS / 8) as usize;
                let sli = &token.as_slice()[32 - bytes..];
                Ok(<$ity>::from_be_bytes(sli.try_into().unwrap()))
            }

            fn tokenize<B>(rust: B) -> Self::TokenType
            where
                B: Borrow<Self::RustType>
            {
                let rust = rust.borrow();
                let bytes = (<$ity>::BITS / 8) as usize;
                let mut word = if *rust < 0 {
                    // account for negative
                    Word::repeat_byte(0xff)
                } else {
                    Word::default()
                };
                let slice = rust.to_be_bytes();
                word[32 - bytes..].copy_from_slice(&slice);
                word.into()
            }

            fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B)
            where
                B: Borrow<Self::RustType>
            {
                let rust = rust.borrow();
                if rust.is_negative(){
                    let bytes = rust.to_be_bytes();
                    target.extend(bytes);
                } else {
                    Uint::<$bits>::encode_packed_to(target, *rust as <Uint::<$bits> as SolType>::RustType);
                }
            }
        }
    };

    ($bits:literal) => {
        impl SolType for Int<$bits> {
            type RustType = I256;
            type TokenType = WordToken;

            fn is_dynamic() -> bool {
                false
            }

            fn sol_type_name() -> Cow<'static, str> {
                concat!("int", $bits).into()
            }

            fn type_check(_token: &Self::TokenType) -> AbiResult<()> {
                Ok(())
            }

            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok(I256::from_be_bytes::<32>(token.into()))
            }

            fn tokenize<B>(rust: B) -> Self::TokenType where B: Borrow<Self::RustType>{
                rust.borrow().to_be_bytes().into()
            }

            fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B) where B: Borrow<Self::RustType>{
                let rust = rust.borrow();
                if rust.is_negative(){
                    let bytes = rust.to_be_bytes();
                    target.extend(bytes);
                } else {
                    Uint::<$bits>::encode_packed_to(
                        target,
                        rust.into_raw(),
                    )
                }
            }
        }
    };

    ($($bits:literal,)+) => {
        $(
            impl_int_sol_type!($bits);
        )+
    };
}

/// Int - `intX`
#[derive(Copy, Clone, Debug)]
pub struct Int<const BITS: usize>;
impl_int_sol_type!(i8, 8);
impl_int_sol_type!(i16, 16);
impl_int_sol_type!(i32, 24);
impl_int_sol_type!(i32, 32);
impl_int_sol_type!(i64, 40);
impl_int_sol_type!(i64, 48);
impl_int_sol_type!(i64, 56);
impl_int_sol_type!(i64, 64);
impl_int_sol_type!(
    72, 80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160, 168, 176, 184, 192, 200, 208, 216, 224,
    232, 240, 248, 256,
);

macro_rules! impl_uint_sol_type {
    ($uty:ty, $bits:literal) => {
        impl SolType for Uint<$bits> {
            type RustType = $uty;
            type TokenType = WordToken;

            fn is_dynamic() -> bool {
                false
            }

            fn sol_type_name() -> Cow<'static, str> {
                concat!("uint", $bits).into()
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[..32 - bytes];
                if !util::check_zeroes(sli) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[32 - bytes..];
                Ok(<$uty>::from_be_bytes(sli.try_into().unwrap()))
            }

            fn tokenize<B>(rust: B) -> Self::TokenType where B: Borrow<Self::RustType>{
                let bytes = (<$uty>::BITS / 8) as usize;
                let mut word = Word::default();
                let slice = rust.borrow().to_be_bytes();
                word[32 - bytes..].copy_from_slice(&slice);
                word.into()
            }

            fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B) where B: Borrow<Self::RustType>{
                // encode the rust to be bytes, strip leading zeroes, then push to the target
                let bytes = rust.borrow().to_be_bytes();
                target.extend(bytes);
            }
        }
    };

    ($bits:literal) => {
        impl SolType for Uint<$bits> {
            type RustType = U256;
            type TokenType = WordToken;

            fn is_dynamic() -> bool {
                false
            }

            fn sol_type_name() -> Cow<'static, str> {
                concat!("uint", $bits).into()
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                let bytes = $bits / 8 as usize;
                let sli = &token.as_slice()[..32 - bytes];
                if !util::check_zeroes(sli) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok(U256::from_be_bytes::<32>(*token.inner()))
            }

            fn tokenize<B>(rust: B) -> Self::TokenType where B: Borrow<Self::RustType>{
                (*rust.borrow()).into()
            }

            fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B) where B: Borrow<Self::RustType>{
                // encode the rust to be bytes, strip leading zeroes, then push to the target
                let bytes: [u8; $bits / 8] = rust.borrow().to_be_bytes();
                target.extend(bytes);
            }
        }
    };

    ($($bits:literal,)+) => {
        $(
            impl_uint_sol_type!($bits);
        )+
    }
}

/// Uint - `uintX`
#[derive(Copy, Clone, Debug)]
pub struct Uint<const BITS: usize>;

impl_uint_sol_type!(u8, 8);
impl_uint_sol_type!(u16, 16);
impl_uint_sol_type!(u32, 24);
impl_uint_sol_type!(u32, 32);
impl_uint_sol_type!(u64, 40);
impl_uint_sol_type!(u64, 48);
impl_uint_sol_type!(u64, 56);
impl_uint_sol_type!(u64, 64);
impl_uint_sol_type!(
    72, 80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160, 168, 176, 184, 192, 200, 208, 216, 224,
    232, 240, 248, 256,
);

/// Bool - `bool`
#[derive(Copy, Clone, Debug)]
pub struct Bool;

impl SolType for Bool {
    type RustType = bool;
    type TokenType = WordToken;

    fn is_dynamic() -> bool {
        false
    }

    fn sol_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if !util::check_bool(token.inner()) {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        Self::tokenize(rust).inner()
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.inner() != Word::repeat_byte(0))
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        let mut word = Word::default();
        word[31..32].copy_from_slice(&[*rust.borrow() as u8]);
        word.into()
    }

    fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B)
    where
        B: Borrow<Self::RustType>,
    {
        // write the bool as a u8
        target.push(*rust.borrow() as u8);
    }
}

/// Array - `T[]`
#[derive(Copy, Clone, Debug)]
pub struct Array<T: SolType>(PhantomData<T>);

impl<T> SolType for Array<T>
where
    T: SolType,
{
    type RustType = Vec<T::RustType>;
    type TokenType = DynSeqToken<T::TokenType>;

    fn is_dynamic() -> bool {
        true
    }

    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[]", T::sol_type_name()).into()
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        for token in token.as_slice() {
            T::type_check(token)?;
        }
        Ok(())
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        let mut encoded = Vec::new();
        for item in rust.borrow() {
            encoded.extend(T::eip712_data_word(item).as_slice());
        }
        keccak256(encoded)
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        token.take_vec().into_iter().map(T::detokenize).collect()
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        rust.borrow()
            .iter()
            .map(|r| T::tokenize(r))
            .collect::<Vec<_>>()
            .into()
    }

    fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B)
    where
        B: Borrow<Self::RustType>,
    {
        for item in rust.borrow() {
            T::encode_packed_to(target, item);
        }
    }
}

/// String - `string`
#[derive(Copy, Clone, Debug)]
pub struct String;

impl SolType for String {
    type RustType = RustString;
    type TokenType = PackedSeqToken;

    fn is_dynamic() -> bool {
        true
    }

    fn sol_type_name() -> Cow<'static, str> {
        "string".into()
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if core::str::from_utf8(token.as_slice()).is_err() {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        keccak256(Self::encode_packed(rust.borrow()))
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        // NOTE: We're decoding strings using lossy UTF-8 decoding to
        // prevent invalid strings written into contracts by either users or
        // Solidity bugs from causing graph-node to fail decoding event
        // data.
        Ok(RustString::from_utf8_lossy(&Bytes::detokenize(token)?).into_owned())
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        rust.borrow().as_bytes().to_vec().into()
    }

    fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B)
    where
        B: Borrow<Self::RustType>,
    {
        target.extend(rust.borrow().as_bytes());
    }
}

macro_rules! impl_fixed_bytes_sol_type {
    ($bytes:literal) => {
        impl SolType for FixedBytes<$bytes> {
            type RustType = [u8; $bytes];
            type TokenType = WordToken;

            fn is_dynamic() -> bool {
                false
            }

            fn sol_type_name() -> Cow<'static, str> {
                concat!("bytes", $bytes).into()
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                if !util::check_fixed_bytes(token.inner(), $bytes) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            fn eip712_data_word<B>(rust: B) -> Word where B: Borrow<Self::RustType> {
                Self::tokenize(rust).inner()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let word = token.as_slice();
                let mut res = Self::RustType::default();
                res[..$bytes].copy_from_slice(&word[..$bytes]);
                Ok(res)
            }

            fn tokenize<B>(rust: B) -> Self::TokenType where B: Borrow<Self::RustType>{
                let mut word = Word::default();
                word[..$bytes].copy_from_slice(&rust.borrow()[..]);
                word.into()
            }

            fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B) where B: Borrow<Self::RustType> {
                // write only the first n bytes
                target.extend_from_slice(&rust.borrow()[..$bytes]);
            }
        }
    };

    ($($bytes:literal,)+) => {
        $(impl_fixed_bytes_sol_type!($bytes);)+
    };
}

/// FixedBytes - `bytesX`
#[derive(Copy, Clone, Debug)]
pub struct FixedBytes<const N: usize>;
impl_fixed_bytes_sol_type!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32,
);

/// FixedArray - `T[M]`
#[derive(Copy, Clone, Debug)]
pub struct FixedArray<T, const N: usize>(PhantomData<T>);

impl<T, const N: usize> SolType for FixedArray<T, N>
where
    T: SolType,
{
    type RustType = [T::RustType; N];
    type TokenType = FixedSeqToken<T::TokenType, N>;

    fn is_dynamic() -> bool {
        T::is_dynamic()
    }

    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[{}]", T::sol_type_name(), N).into()
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        for token in token.as_array().iter() {
            T::type_check(token)?;
        }
        Ok(())
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        let rust = rust.borrow();
        let encoded = rust
            .iter()
            .flat_map(|element| T::eip712_data_word(element).to_fixed_bytes())
            .collect::<Vec<u8>>();
        keccak256(encoded)
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        let res = token
            .take_array()
            .into_iter()
            .map(|t| T::detokenize(t))
            .collect::<AbiResult<Vec<_>>>()?
            .try_into();
        match res {
            Ok(tokens) => Ok(tokens),
            Err(_) => panic!("input is exact len"),
        }
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        match rust
            .borrow()
            .iter()
            .map(|r| T::tokenize(r))
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(tokens) => tokens,
            Err(_) => unreachable!(),
        }
    }

    fn encode_packed_to<B>(target: &mut Vec<u8>, rust: B)
    where
        B: Borrow<Self::RustType>,
    {
        for item in rust.borrow() {
            T::encode_packed_to(target, item);
        }
    }
}

macro_rules! tuple_impls {
    () => {};

    (@peel $_:ident, $($other:ident,)*) => { tuple_impls! { $($other,)* } };

    // compile time `join(",")` format string
    (@fmt $other:ident) => { ",{}" };
    (@fmt $first:ident, $($other:ident,)*) => {
        concat!(
            "{}",
            $(tuple_impls! { @fmt $other }),*
        )
    };

    ($($ty:ident,)+) => {
        #[allow(non_snake_case)]
        impl<$($ty: SolType,)+> SolType for ($($ty,)+) {
            type RustType = ($( $ty::RustType, )+);
            type TokenType = ($( $ty::TokenType, )+);

            fn is_dynamic() -> bool {
                $( <$ty as SolType>::is_dynamic() )||+
            }

            fn sol_type_name() -> Cow<'static, str> {
                format!(
                    concat!(
                        "tuple(",
                        tuple_impls! { @fmt $($ty,)+ },
                        ")",
                    ),
                    $(<$ty as SolType>::sol_type_name(),)+
                ).into()
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                let ($(ref $ty,)+) = *token;
                $(
                    <$ty as SolType>::type_check($ty)?;
                )+
                Ok(())
            }

            fn eip712_data_word<B_: Borrow<Self::RustType>>(rust: B_) -> Word {
                let ($(ref $ty,)+) = *rust.borrow();
                let encoding: Vec<u8> = [$(
                    <$ty as SolType>::eip712_data_word($ty).0,
                )+].concat();
                keccak256(&encoding).into()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let ($($ty,)+) = token;
                Ok(($(
                    <$ty as SolType>::detokenize($ty)?,
                )+))
            }

            fn tokenize<B_: Borrow<Self::RustType>>(rust: B_) -> Self::TokenType {
                let ($(ref $ty,)+) = *rust.borrow();
                ($(
                    <$ty as SolType>::tokenize($ty),
                )+)
            }

            fn encode_packed_to<B_: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B_) {
                let ($(ref $ty,)+) = *rust.borrow();
                // TODO: Reserve
                $(
                    <$ty as SolType>::encode_packed_to(target, $ty);
                )+
            }
        }

        tuple_impls! { @peel $($ty,)+ }
    };
}

tuple_impls! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, }
