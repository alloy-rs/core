use alloc::borrow::Cow;
use core::marker::PhantomData;
use ethers_primitives::{keccak256, Address as RustAddress, I256, U256};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::{Borrow, String as RustString, ToOwned, Vec};
#[cfg(feature = "std")]
use std::{borrow::Borrow, string::String as RustString};

use crate::{token::*, util, AbiResult, SolType, Word};

/// This trait describes types that exist in normal Solidity operation
/// (i.e. NOT events, errors, function calls)
pub trait SolDataType: SolType {
    /// The encoded struct type (as EIP-712), if any. None for non-structs
    fn eip712_encode_type() -> Option<Cow<'static, str>> {
        None
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

    /// Implemens Solidity's `encodePacked()` function, writing into the given buffer.
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B);

    /// Implements Solidity's `encodePacked()` function.
    fn encode_packed<B: Borrow<Self::RustType>>(rust: B) -> Vec<u8> {
        let mut res = Vec::new();
        Self::encode_packed_to(&mut res, rust);
        res
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

    fn type_check(token: &Self::TokenType) -> AbiResult<()> {
        if !util::check_zeroes(&token.inner()[..12]) {
            return Err(Self::type_check_fail(token.as_slice()));
        }
        Ok(())
    }
}

impl SolDataType for Address {
    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        Self::tokenize(rust).inner()
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

    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
        Ok(token.take_vec())
    }

    fn tokenize<B>(rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
        rust.borrow().to_owned().into()
    }
}

impl SolDataType for Bytes {
    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        keccak256(Self::encode_packed(rust.borrow()))
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
        }

        impl SolDataType for Int<$bits> {
            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
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

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok(I256::from_be_bytes::<32>(token.into()))
            }

            fn tokenize<B>(rust: B) -> Self::TokenType where B: Borrow<Self::RustType>{
                rust.borrow().to_be_bytes().into()
            }
        }
        impl SolDataType for Int<$bits> {
            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
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

        }
        impl SolDataType for Uint<$bits> {
            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
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

            fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
                Ok(U256::from_be_bytes::<32>(*token.inner()))
            }

            fn tokenize<B>(rust: B) -> Self::TokenType where B: Borrow<Self::RustType>{
                (*rust.borrow()).into()
            }
        }
        impl SolDataType for Uint<$bits> {
            fn eip712_data_word<B>(rust: B) -> Word
            where
                B: Borrow<Self::RustType>
            {
                Self::tokenize(rust).inner().into()
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
}

impl SolDataType for Bool {
    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        Self::tokenize(rust).inner()
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
}

impl<T> SolDataType for Array<T>
where
    T: SolDataType,
{
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
}

impl SolDataType for String {
    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        keccak256(Self::encode_packed(rust.borrow()))
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
        }

        impl SolDataType for FixedBytes<$bytes> {
            fn eip712_data_word<B>(rust: B) -> Word where B: Borrow<Self::RustType> {
                Self::tokenize(rust).inner()
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
}

impl<T, const N: usize> SolDataType for FixedArray<T, N>
where
    T: SolDataType,
{
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
        }
        #[allow(non_snake_case)]
        impl<$($ty: SolDataType,)+> SolDataType for ($($ty,)+) {
            fn eip712_data_word<B_: Borrow<Self::RustType>>(rust: B_) -> Word {
                let ($(ref $ty,)+) = *rust.borrow();
                let encoding: Vec<u8> = [$(
                    <$ty as SolDataType>::eip712_data_word($ty).0,
                )+].concat();
                keccak256(&encoding).into()
            }

            fn encode_packed_to<B_: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B_) {
                let ($(ref $ty,)+) = *rust.borrow();
                // TODO: Reserve
                $(
                    <$ty as SolDataType>::encode_packed_to(target, $ty);
                )+
            }
        }

        tuple_impls! { @peel $($ty,)+ }
    };
}

tuple_impls! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, }

impl SolType for () {
    type RustType = ();
    type TokenType = ();

    fn is_dynamic() -> bool {
        false
    }

    fn sol_type_name() -> Cow<'static, str> {
        "tuple()".into()
    }

    fn type_check(_token: &Self::TokenType) -> AbiResult<()> {
        Err(crate::Error::type_check_fail(b"", "tuple()"))
    }

    fn detokenize(_token: Self::TokenType) -> AbiResult<Self::RustType> {
        Err(crate::Error::type_check_fail(b"", "tuple()"))
    }

    fn tokenize<B>(_rust: B) -> Self::TokenType
    where
        B: Borrow<Self::RustType>,
    {
    }
}

impl SolDataType for () {
    fn eip712_data_word<B>(_rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        Word::zero()
    }

    fn encode_packed_to<B>(_target: &mut Vec<u8>, _rust: B)
    where
        B: Borrow<Self::RustType>,
    {
    }
}
