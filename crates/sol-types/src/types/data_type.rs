//! Solidity Primitives. These are the types that are built into Solidity.

use crate::{
    no_std_prelude::{String as RustString, *},
    token::*,
    util, Result, SolType, Word,
};
use alloc::borrow::Cow;
use alloy_primitives::{keccak256, Address as RustAddress, I256, U256};
use core::marker::PhantomData;

/// Address - `address`
#[derive(Copy, Clone, Debug)]
pub struct Address;

impl SolType for Address {
    type RustType = RustAddress;
    type TokenType = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "address".into()
    }

    #[inline]
    fn is_dynamic() -> bool {
        false
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        let sli = &token.as_slice()[12..];
        RustAddress::from_slice(sli)
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        WordToken::from(*rust.borrow())
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if !util::check_zeroes(&token.inner()[..12]) {
            return Err(Self::type_check_fail(token.as_slice()))
        }
        Ok(())
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).inner()
    }

    #[inline]
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

    #[inline]
    fn is_dynamic() -> bool {
        true
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bytes".into()
    }

    #[inline]
    fn type_check(_token: &Self::TokenType) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.into_vec()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        rust.borrow().to_owned().into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        keccak256(Self::encode_packed(rust.borrow()))
    }

    #[inline]
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

            #[inline]
            fn is_dynamic() -> bool {
                false
            }

            #[inline]
            fn sol_type_name() -> Cow<'static, str> {
                concat!("int", $bits).into()
            }

            #[inline]
            fn type_check(token: &Self::TokenType) -> Result<()> {
                // check for 256 can be omitted as this macro expansion is not used for 256 bits

                let bytes = token.as_slice();
                let meaningful_idx = 32 - ($bits / 8);

                let sign_extension = if bytes[meaningful_idx] & 0x80 == 0x80 {
                    0xff
                } else {
                    0
                };

                // check that all upper bytes are an extension of the sign bit
                bytes
                    .iter()
                    .take(meaningful_idx)
                    .all(|byte| *byte == sign_extension)
                    .then(|| ())
                    .ok_or_else(|| Self::type_check_fail(bytes))
            }

            #[inline]
            fn detokenize(token: Self::TokenType) -> Self::RustType {
                let bytes = (<$ity>::BITS / 8) as usize;
                let sli = &token.as_slice()[32 - bytes..];
                <$ity>::from_be_bytes(sli.try_into().unwrap())
            }

            #[inline]
            fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
                let rust = *rust.borrow();
                let bytes = (<$ity>::BITS / 8) as usize;
                let mut word = if rust.is_negative() {
                    Word::repeat_byte(0xff)
                } else {
                    Word::default()
                };
                let slice = rust.to_be_bytes();
                word[32 - bytes..].copy_from_slice(&slice);
                word.into()
            }

            #[inline]
            fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
                Self::tokenize(rust).inner().into()
            }

            #[inline]
            fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
                target.extend(rust.borrow().to_be_bytes());
            }
        }
    };

    ($($bits:literal),+ $(,)?) => {$(
        impl SolType for Int<$bits> {
            type RustType = I256;
            type TokenType = WordToken;

            #[inline]
            fn is_dynamic() -> bool {
                false
            }

            #[inline]
            fn sol_type_name() -> Cow<'static, str> {
                concat!("int", $bits).into()
            }

            #[inline]
            fn type_check(token: &Self::TokenType) -> Result<()> {
                if $bits == 256 {
                    return Ok(())
                }

                let bytes = token.as_slice();
                let meaningful_idx = 32 - ($bits / 8);

                let sign_extension = if bytes[meaningful_idx] & 0x80 == 0x80 {
                    0xff
                } else {
                    0
                };

                // check that all upper bytes are an extension of the sign bit
                bytes
                    .iter()
                    .take(meaningful_idx)
                    .all(|byte| *byte == sign_extension)
                    .then(|| ())
                    .ok_or_else(|| Self::type_check_fail(bytes))
            }

            #[inline]
            fn detokenize(token: Self::TokenType) -> Self::RustType {
                I256::from_be_bytes::<32>(token.into())
            }

            #[inline]
            fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
                rust.borrow().to_be_bytes::<32>().into()
            }

            #[inline]
            fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
                Self::tokenize(rust).inner().into()
            }

            #[inline]
            fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
                target.extend(rust.borrow().to_be_bytes::<32>());
            }
        }
    )+};
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

            #[inline]
            fn is_dynamic() -> bool {
                false
            }

            #[inline]
            fn sol_type_name() -> Cow<'static, str> {
                concat!("uint", $bits).into()
            }

            #[inline]
            fn type_check(token: &Self::TokenType) -> Result<()> {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[..32 - bytes];
                if !util::check_zeroes(sli) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            #[inline]
            fn detokenize(token: Self::TokenType) -> Self::RustType {
                let bytes = (<$uty>::BITS / 8) as usize;
                let sli = &token.as_slice()[32 - bytes..];
                <$uty>::from_be_bytes(sli.try_into().unwrap())
            }

            #[inline]
            fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
                let bytes = (<$uty>::BITS / 8) as usize;
                let mut word = Word::default();
                let slice = rust.borrow().to_be_bytes();
                word[32 - bytes..].copy_from_slice(&slice);
                word.into()
            }

            #[inline]
            fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
                Self::tokenize(rust).inner().into()
            }

            #[inline]
            fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
                // TODO: encode the rust to be bytes, strip leading zeroes, then push to the target
                target.extend(rust.borrow().to_be_bytes());
            }
        }
    };

    ($($bits:literal),+ $(,)?) => {$(
        impl SolType for Uint<$bits> {
            type RustType = U256;
            type TokenType = WordToken;

            #[inline]
            fn is_dynamic() -> bool {
                false
            }

            #[inline]
            fn sol_type_name() -> Cow<'static, str> {
                concat!("uint", $bits).into()
            }

            #[inline]
            fn type_check(token: &Self::TokenType) -> Result<()> {
                let bytes = $bits / 8 as usize;
                let sli = &token.as_slice()[..32 - bytes];
                if !util::check_zeroes(sli) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            #[inline]
            fn detokenize(token: Self::TokenType) -> Self::RustType {
                U256::from_be_bytes::<32>(*token.inner())
            }

            #[inline]
            fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
                (*rust.borrow()).into()
            }

            #[inline]
            fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
                Self::tokenize(rust).inner().into()
            }

            #[inline]
            fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
                // TODO: encode the rust to be bytes, strip leading zeroes, then push to the target
                target.extend(rust.borrow().to_be_bytes::<{ $bits / 8 }>());
            }
        }
    )+};
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

    #[inline]
    fn is_dynamic() -> bool {
        false
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if !util::check_bool(token.inner()) {
            return Err(Self::type_check_fail(token.as_slice()))
        }
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.inner() != Word::repeat_byte(0)
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        let mut word = Word::default();
        word[31] = *rust.borrow() as u8;
        word.into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).inner()
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
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

    #[inline]
    fn is_dynamic() -> bool {
        true
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[]", T::sol_type_name()).into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        for token in token.as_slice() {
            T::type_check(token)?;
        }
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.into_vec().into_iter().map(T::detokenize).collect()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        rust.borrow()
            .iter()
            .map(|r| T::tokenize(r))
            .collect::<Vec<_>>()
            .into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        let mut encoded = Vec::new();
        for item in rust.borrow() {
            encoded.extend(T::eip712_data_word(item).as_slice());
        }
        keccak256(encoded)
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
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

    #[inline]
    fn is_dynamic() -> bool {
        true
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "string".into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if core::str::from_utf8(token.as_slice()).is_err() {
            return Err(Self::type_check_fail(token.as_slice()))
        }
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        // NOTE: We're decoding strings using lossy UTF-8 decoding to
        // prevent invalid strings written into contracts by either users or
        // Solidity bugs from causing graph-node to fail decoding event
        // data.
        RustString::from_utf8_lossy(&Bytes::detokenize(token)).into_owned()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        rust.borrow().as_bytes().to_vec().into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        keccak256(Self::encode_packed(rust.borrow()))
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
        target.extend(rust.borrow().as_bytes());
    }
}

macro_rules! impl_fixed_bytes_sol_type {
    ($($bytes:literal),+ $(,)?) => {$(
        impl SolType for FixedBytes<$bytes> {
            type RustType = [u8; $bytes];
            type TokenType = WordToken;

            #[inline]
            fn is_dynamic() -> bool {
                false
            }

            #[inline]
            fn sol_type_name() -> Cow<'static, str> {
                concat!("bytes", $bytes).into()
            }

            #[inline]
            fn type_check(token: &Self::TokenType) -> Result<()> {
                if !util::check_fixed_bytes(token.inner(), $bytes) {
                    return Err(Self::type_check_fail(token.as_slice()));
                }
                Ok(())
            }

            #[inline]
            fn detokenize(token: Self::TokenType) -> Self::RustType {
                let word = token.as_slice();
                let mut res = [0; $bytes];
                res[..].copy_from_slice(&word[..$bytes]);
                res
            }

            #[inline]
            fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
                let mut word = Word::default();
                word[..$bytes].copy_from_slice(rust.borrow());
                word.into()
            }

            #[inline]
            fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
                Self::tokenize(rust).inner()
            }

            #[inline]
            fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
                // write only the first n bytes
                target.extend_from_slice(rust.borrow());
            }
        }
    )+};
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

    #[inline]
    fn is_dynamic() -> bool {
        T::is_dynamic()
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[{}]", T::sol_type_name(), N).into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        for token in token.as_array().iter() {
            T::type_check(token)?;
        }
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        let res = token
            .into_array()
            .into_iter()
            .map(|t| T::detokenize(t))
            .collect::<Vec<_>>()
            .try_into();
        match res {
            Ok(tokens) => tokens,
            Err(_) => unreachable!("input is exact len"),
        }
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
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

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        let rust = rust.borrow();
        let encoded = rust
            .iter()
            .flat_map(|element| T::eip712_data_word(element).to_fixed_bytes())
            .collect::<Vec<u8>>();
        keccak256(encoded)
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
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

            fn type_check(token: &Self::TokenType) -> Result<()> {
                let ($(ref $ty,)+) = *token;
                $(
                    <$ty as SolType>::type_check($ty)?;
                )+
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> Self::RustType {
                let ($($ty,)+) = token;
                ($(
                    <$ty as SolType>::detokenize($ty),
                )+)
            }

            fn tokenize<B_: Borrow<Self::RustType>>(rust: B_) -> Self::TokenType {
                let ($(ref $ty,)+) = *rust.borrow();
                ($(
                    <$ty as SolType>::tokenize($ty),
                )+)
            }

            fn eip712_data_word<B_: Borrow<Self::RustType>>(rust: B_) -> Word {
                let ($(ref $ty,)+) = *rust.borrow();
                let encoding: Vec<u8> = [$(
                    <$ty as SolType>::eip712_data_word($ty).0,
                )+].concat();
                keccak256(&encoding).into()
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

impl SolType for () {
    type RustType = ();
    type TokenType = FixedSeqToken<(), 0>;

    #[inline]
    fn is_dynamic() -> bool {
        false
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "tuple()".into()
    }

    #[inline]
    fn type_check(_token: &Self::TokenType) -> Result<()> {
        Err(crate::Error::type_check_fail(b"", "tuple()"))
    }

    #[inline]
    fn detokenize(_token: Self::TokenType) -> Self::RustType {}

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(_rust: B) -> Self::TokenType {
        [].into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(_rust: B) -> Word {
        Word::ZERO
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(_target: &mut Vec<u8>, _rust: B) {}
}
