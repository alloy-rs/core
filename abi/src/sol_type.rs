use core::marker::PhantomData;

use ethers_primitives::{B160, U256};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::{String as RustString, ToOwned, ToString, Vec};
#[cfg(feature = "std")]
use std::string::String as RustString;

use crate::{
    decoder::*,
    token::{DynSeqToken, FixedSeqToken, PackedSeqToken, TokenSeq, TokenType, WordToken},
    AbiResult, Error, Word,
};

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
///
/// ```
/// # use ethers_abi_enc::AbiResult;
/// use ethers_abi_enc::sol_type::*;
/// use ethers_primitives::U256;
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
///     fn sol_type_name() -> std::string::String {
///         "MySolidityStruct".to_owned()
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
///     // Convert from the token to the rust type. We cheat here again by
///     // delegating.
///     fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
///         let (a, b) = UnderlyingTuple::detokenize(token)?;
///         Ok(MyRustStruct{ a, b })
///     }
///
///     // Convert from the rust type to the token type. We cheat here AGAIN
///     // by delegating.
///     fn tokenize(rust: Self::RustType) -> Self::TokenType {
///         let MyRustStruct { a, b } = rust;
///         UnderlyingTuple::tokenize((a, b))
///     }
///
///     fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
///        let MyRustStruct { a, b } = rust;
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
    /// The corresponding Rust type.
    type RustType: Sized;

    /// The corresponding ABI token type.
    ///
    /// See implementers of [`TokenType`].
    ///
    type TokenType: TokenType;

    /// The name of the type in solidity
    fn sol_type_name() -> RustString;
    /// True if the type is dynamic according to ABI rules
    fn is_dynamic() -> bool;
    /// Check a token to see if it can be detokenized with this type
    fn type_check(token: &Self::TokenType) -> AbiResult<()>;
    /// Detokenize
    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType>;
    /// Tokenize
    fn tokenize(rust: Self::RustType) -> Self::TokenType;

    /// Implemens solidity's encodePacked() function, writing to the target to
    /// avoid allocations
    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType);

    /// Implements solidity's encodePacked() function
    fn encode_packed(rust: Self::RustType) -> Vec<u8> {
        let mut res = Vec::new();
        Self::encode_packed_to(&mut res, rust);
        res
    }

    #[doc(hidden)]
    fn type_check_fail(data: &[u8]) -> Error {
        Error::TypeCheckFail {
            data: hex::encode(data),
            expected_type: Self::sol_type_name(),
        }
    }

    /// Encode a single ABI token by wrapping it in a 1-length sequence
    fn encode_single(rust: Self::RustType) -> Vec<u8> {
        let token = Self::tokenize(rust);
        crate::encode_single(token)
    }

    /// Encode an ABI sequence
    fn encode(rust: Self::RustType) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
    {
        let token = Self::tokenize(rust);
        crate::encode(token)
    }

    /// Encode an ABI sequence suitable for function params
    fn encode_params(rust: Self::RustType) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
    {
        let token = Self::tokenize(rust);
        crate::encode_params(token)
    }

    /// Hex output of encode
    fn hex_encode(rust: Self::RustType) -> RustString
    where
        Self::TokenType: TokenSeq,
    {
        format!("0x{}", hex::encode(Self::encode(rust)))
    }

    /// Hex output of encode_single
    fn hex_encode_single(rust: Self::RustType) -> RustString {
        format!("0x{}", hex::encode(Self::encode_single(rust)))
    }

    /// Hex output of encode_params
    fn hex_encode_params(rust: Self::RustType) -> RustString
    where
        Self::TokenType: TokenSeq,
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
pub struct Address;

impl SolType for Address {
    type RustType = B160;
    type TokenType = WordToken;

    fn sol_type_name() -> RustString {
        "address".to_string()
    }

    fn is_dynamic() -> bool {
        false
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if !check_zeroes(&token.inner()[..12]) {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        let sli = &token.as_slice()[12..];
        Ok(B160::from_slice(sli))
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        // push the last 20 bytes of the word to the target
        target.extend_from_slice(&rust.as_bytes()[12..]);
    }
}

/// Bytes - `bytes`
pub struct Bytes;

impl SolType for Bytes {
    type RustType = Vec<u8>;
    type TokenType = PackedSeqToken;

    fn is_dynamic() -> bool {
        true
    }

    fn sol_type_name() -> RustString {
        "bytes".to_string()
    }

    fn type_check(_token: &Self::TokenType) -> AbiResult<()> {
        Ok(())
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.take_vec())
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        // push the buf to the vec
        target.extend_from_slice(&rust);
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

            fn sol_type_name() -> RustString {
                format!("int{}", $bits)
            }

            fn type_check(_token: &Self::TokenType) -> AbiResult<()> {
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let bytes = (<$ity>::BITS / 8) as usize;
                let sli = &token.as_slice()[32 - bytes..];
                Ok(<$ity>::from_be_bytes(sli.try_into().unwrap()))
            }

            fn tokenize(rust: Self::RustType) -> Self::TokenType {
                let bytes = (<$ity>::BITS / 8) as usize;
                let mut word = if rust < 0 {
                    // account for negative
                    Word::repeat_byte(0xff)
                } else {
                    Word::default()
                };
                let slice = rust.to_be_bytes();
                word[32 - bytes..].copy_from_slice(&slice);
                word.into()
            }

            fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
                // encode the rust to be bytes, strip leading zeroes, then push to the target
                let bytes = rust.to_be_bytes();
                target.extend(bytes);
            }
        }
    };
}

/// Int - `intX`
pub struct Int<const BITS: usize>;
impl_int_sol_type!(i8, 8);
impl_int_sol_type!(i16, 16);
impl_int_sol_type!(i32, 24);
impl_int_sol_type!(i32, 32);
impl_int_sol_type!(i64, 40);
impl_int_sol_type!(i64, 48);
impl_int_sol_type!(i64, 56);
impl_int_sol_type!(i64, 64);
// TODO: larger

macro_rules! impl_uint_sol_type {
    ($uty:ty, $bits:literal) => {
        impl SolType for Uint<$bits> {
            type RustType = $uty;
            type TokenType = WordToken;

            fn is_dynamic() -> bool {
                false
            }

            fn sol_type_name() -> RustString {
                format!("uint{}", $bits)
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[..32 - bytes];
                if !check_zeroes(sli) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[32 - bytes..];
                Ok(<$uty>::from_be_bytes(sli.try_into().unwrap()))
            }

            fn tokenize(rust: Self::RustType) -> Self::TokenType {
                let bytes = (<$uty>::BITS / 8) as usize;
                let mut word = Word::default();
                let slice = rust.to_be_bytes();
                word[32 - bytes..].copy_from_slice(&slice);
                word.into()
            }

            fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
                // encode the rust to be bytes, strip leading zeroes, then push to the target
                let bytes = rust.to_be_bytes();
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

            fn sol_type_name() -> RustString {
                format!("uint{}", $bits)
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                let bytes = $bits / 8 as usize;
                let sli = &token.as_slice()[..32 - bytes];
                if !check_zeroes(sli) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok(U256::from_be_bytes::<32>(*token.inner()))
            }

            fn tokenize(rust: Self::RustType) -> Self::TokenType {
                rust.into()
            }

            fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
                // encode the rust to be bytes, strip leading zeroes, then push to the target
                let bytes: [u8; $bits / 8] = rust.to_be_bytes();
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
pub struct Bool;

impl SolType for Bool {
    type RustType = bool;
    type TokenType = WordToken;

    fn is_dynamic() -> bool {
        false
    }

    fn sol_type_name() -> RustString {
        "bool".into()
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if !check_bool(token.inner()) {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.inner() != Word::repeat_byte(0))
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        let mut word = Word::default();
        word[31..32].copy_from_slice(&[rust as u8]);
        word.into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        // write the bool as a u8
        target.push(rust as u8);
    }
}

/// Array - `T[]`
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

    fn sol_type_name() -> RustString {
        format!("{}[]", T::sol_type_name())
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        for token in token.as_slice() {
            T::type_check(token)?;
        }
        Ok(())
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        token.take_vec().into_iter().map(T::detokenize).collect()
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.into_iter()
            .map(|r| T::tokenize(r))
            .collect::<Vec<_>>()
            .into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        for item in rust {
            T::encode_packed_to(target, item);
        }
    }
}

/// String - `string`
pub struct String;

impl SolType for String {
    type RustType = RustString;
    type TokenType = PackedSeqToken;

    fn is_dynamic() -> bool {
        true
    }

    fn sol_type_name() -> RustString {
        "string".to_owned()
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if core::str::from_utf8(token.as_slice()).is_err() {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        // NOTE: We're decoding strings using lossy UTF-8 decoding to
        // prevent invalid strings written into contracts by either users or
        // Solidity bugs from causing graph-node to fail decoding event
        // data.
        Ok(RustString::from_utf8_lossy(&Bytes::detokenize(token)?).into())
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.into_bytes().into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        target.extend(rust.as_bytes());
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

            fn sol_type_name() -> RustString {
                format!("bytes{}", $bytes)
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                if !check_fixed_bytes(token.inner(), $bytes) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                let word = token.as_slice();
                let mut res = Self::RustType::default();
                res[..$bytes].copy_from_slice(&word[..$bytes]);
                Ok(res)
            }

            fn tokenize(rust: Self::RustType) -> Self::TokenType {
                let mut word = Word::default();
                word[..$bytes].copy_from_slice(&rust[..]);
                word.into()
            }

            fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
                // write only the first n bytes
                target.extend_from_slice(&rust[..$bytes]);
            }
        }
    };

    ($($bytes:literal,)+) => {
        $(impl_fixed_bytes_sol_type!($bytes);)+
    };
}

/// FixedBytes - `bytesX`
pub struct FixedBytes<const N: usize>;
impl_fixed_bytes_sol_type!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32,
);

/// FixedArray - `T[M]`
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

    fn sol_type_name() -> RustString {
        format!("{}[{}]", T::sol_type_name(), N)
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        for token in token.as_array().iter() {
            T::type_check(token)?;
        }
        Ok(())
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

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.map(|r| T::tokenize(r)).into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        for item in rust {
            T::encode_packed_to(target, item);
        }
    }
}

macro_rules! impl_tuple_sol_type {
    ($num:expr, $( $ty:ident : $no:tt ),+ $(,)?) => {
        impl<$($ty,)+> SolType for ($( $ty, )+)
        where
            $(
                $ty: SolType,
            )+
        {
            type RustType = ($( $ty::RustType, )+);
            type TokenType = ($( $ty::TokenType, )+);

            fn is_dynamic() -> bool {
                $(
                    if $ty::is_dynamic() {
                        return true;
                    }
                )+
                false
            }

            fn sol_type_name() -> RustString {
                let mut types = Vec::with_capacity($num);
                $(
                    types.push($ty::sol_type_name());
                )+

                format!("tuple({})", types.join(","))
            }

            fn type_check(token: &Self::TokenType) -> AbiResult<()> {
                $(
                    $ty::type_check(&token.$no)?;
                )+
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok((
                    $(
                        $ty::detokenize(token.$no)?,
                    )+
                ))
            }

            fn tokenize(rust: Self::RustType) -> Self::TokenType {
                ($(
                    $ty::tokenize(rust.$no),
                )+)
            }

            fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
                $(
                    $ty::encode_packed_to(target, rust.$no);
                )+
            }
        }
    };
}
impl_tuple_sol_type!(1, A:0, );
impl_tuple_sol_type!(2, A:0, B:1, );
impl_tuple_sol_type!(3, A:0, B:1, C:2, );
impl_tuple_sol_type!(4, A:0, B:1, C:2, D:3, );
impl_tuple_sol_type!(5, A:0, B:1, C:2, D:3, E:4, );
impl_tuple_sol_type!(6, A:0, B:1, C:2, D:3, E:4, F:5, );
impl_tuple_sol_type!(7, A:0, B:1, C:2, D:3, E:4, F:5, G:6, );
impl_tuple_sol_type!(8, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, );
impl_tuple_sol_type!(9, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, );
impl_tuple_sol_type!(10, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, );
impl_tuple_sol_type!(11, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, );
impl_tuple_sol_type!(12, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, );
impl_tuple_sol_type!(13, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, );
impl_tuple_sol_type!(14, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, );
impl_tuple_sol_type!(15, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, );
impl_tuple_sol_type!(16, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, );
impl_tuple_sol_type!(17, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16,);
impl_tuple_sol_type!(18, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17,);
impl_tuple_sol_type!(19, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18,);
impl_tuple_sol_type!(20, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19,);
impl_tuple_sol_type!(21, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19, U:20,);

/// Function - `function`
pub struct Function;

impl SolType for Function {
    type RustType = (B160, [u8; 4]);
    type TokenType = WordToken;

    fn sol_type_name() -> RustString {
        "function".to_string()
    }

    fn is_dynamic() -> bool {
        false
    }

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if !crate::decoder::check_fixed_bytes(token.inner(), 24) {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        let t = token.as_slice();

        let mut address = [0u8; 20];
        let mut selector = [0u8; 4];
        address.copy_from_slice(&t[..20]);
        selector.copy_from_slice(&t[20..24]);
        Ok((B160(address), selector))
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        let mut word = Word::default();
        word[..20].copy_from_slice(&rust.0[..]);
        word[20..24].copy_from_slice(&rust.1[..]);
        word.into()
    }

    fn encode_packed_to(target: &mut Vec<u8>, rust: Self::RustType) {
        target.extend_from_slice(&rust.0[..]);
        target.extend_from_slice(&rust.1[..]);
    }
}
