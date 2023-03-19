use core::marker::PhantomData;

use ethers_primitives::{B160, B256, U256};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::{String as RustString, ToOwned, ToString, Vec};
#[cfg(feature = "std")]
use std::string::String as RustString;

use crate::{
    decoder::*,
    token::{DynSeqToken, FixedSeqToken, PackedSeqToken, TokenSeq, TokenType, WordToken},
    AbiResult,
    Error::InvalidData,
    Word,
};

/// A Solidity Type, for ABI enc/decoding
pub trait SolType {
    /// The corresponding Rust type
    type RustType;

    /// The corresponding token type
    type TokenType: TokenType;

    /// The name of the type in solidity
    fn sol_type_name() -> RustString;
    /// True if the type is dynamic according to ABI rules
    fn is_dynamic() -> bool;
    /// Check a token to see if it can be detokenized with this type
    fn type_check(token: &Self::TokenType) -> bool;
    /// Detokenize
    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType>;
    /// Tokenize
    fn tokenize(rust: Self::RustType) -> Self::TokenType;

    /// Encode a Rust type to an ABI blob
    fn encode(rust: Self::RustType) -> Vec<u8> {
        let token = Self::tokenize(rust);
        crate::encode((token,))
    }

    /// Encode a Rust type
    fn encode_params(rust: Self::RustType) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
    {
        let token = Self::tokenize(rust);
        crate::encode_params(token)
    }

    /// Encode a Rust type to an ABI blob, then hex encode the blob
    fn hex_encode(rust: Self::RustType) -> RustString {
        format!("0x{}", hex::encode(Self::encode(rust)))
    }

    /// Decode a Rust type from an ABI blob
    fn decode(data: &[u8]) -> AbiResult<Self::RustType> {
        Self::detokenize(Self::TokenType::decode_from(&mut Decoder::new(
            data, false, false,
        ))?)
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode(data: &str) -> AbiResult<Self::RustType> {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(|_| InvalidData)
            .and_then(|buf| Self::decode(&buf))
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

    fn type_check(token: &Self::TokenType) -> bool {
        check_zeroes(&token.inner()[..12]).is_ok()
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        let sli = &token.as_slice()[12..];
        Ok(B160::from_slice(sli))
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        let mut word = Word::default();
        word[12..].copy_from_slice(&rust[..]);
        word.into()
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

    fn type_check(_token: &Self::TokenType) -> bool {
        true
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.take_vec())
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.into()
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

            fn type_check(_token: &Self::TokenType) -> bool {
                true
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

            fn type_check(token: &Self::TokenType) -> bool {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[..32 - bytes];
                check_zeroes(sli).is_ok()
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

            fn type_check(token: &Self::TokenType) -> bool {
                let bytes = $bits / 8 as usize;
                let sli = &token.as_slice()[..32 - bytes];
                check_zeroes(sli).is_ok()
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok(U256::from_be_bytes::<32>(*token.inner()))
            }

            fn tokenize(rust: Self::RustType) -> Self::TokenType {
                B256(rust.to_be_bytes::<32>()).into()
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

    fn type_check(token: &Self::TokenType) -> bool {
        check_bool(token.inner()).is_ok()
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.as_slice()[31] < 2)
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        let mut word = Word::default();
        word[31..32].copy_from_slice(&[rust as u8]);
        word.into()
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

    fn type_check(token: &Self::TokenType) -> bool {
        token.as_slice().iter().all(|inner| T::type_check(inner))
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

    fn type_check(token: &Self::TokenType) -> bool {
        core::str::from_utf8(token.as_slice()).is_ok()
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        RustString::from_utf8(Bytes::detokenize(token)?).map_err(|_| InvalidData)
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.into_bytes().into()
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

            fn type_check(token: &Self::TokenType) -> bool {
                check_fixed_bytes(token.inner(), $bytes).is_ok()
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

    fn type_check(token: &Self::TokenType) -> bool {
        token.as_array().iter().all(|token| T::type_check(token))
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        token
            .take_array()
            .into_iter()
            .map(|t| T::detokenize(t))
            .collect::<AbiResult<Vec<_>>>()?
            .try_into()
            .map_err(|_| InvalidData)
    }

    fn tokenize(rust: Self::RustType) -> Self::TokenType {
        rust.map(|r| T::tokenize(r)).into()
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

            fn type_check(token: &Self::TokenType) -> bool {
                $(
                    if !$ty::type_check(&token.$no) {
                        return false
                    }
                )+
                true
            }

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                if !Self::type_check(&token) {
                    return Err(InvalidData)
                }

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

    fn type_check(token: &Self::TokenType) -> bool {
        crate::decoder::check_fixed_bytes(token.inner(), 24).is_ok()
    }

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        if !Self::type_check(&token) {
            return Err(InvalidData);
        }
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
}
