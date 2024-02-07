//! Solidity types.
//!
//! These are the types that are [built into Solidity][ref].
//!
//! See [`SolType`] for more details.
//!
//! [ref]: https://docs.soliditylang.org/en/latest/types.html

#![allow(missing_copy_implementations, missing_debug_implementations)]

use crate::{abi::token::*, private::SolTypeValue, utils, SolType, Word};
use alloc::{borrow::Cow, string::String as RustString, vec::Vec};
use alloy_primitives::{
    keccak256, Address as RustAddress, FixedBytes as RustFixedBytes, Function as RustFunction,
    I256, U256,
};
use core::{borrow::Borrow, fmt::*, hash::Hash, marker::PhantomData, ops::*};

// IMPORTANT: Keep in sync with `rec_expand_rust_type` in
// `sol-macro/src/expand/ty.rs`

/// Bool - `bool`
pub struct Bool;

impl SolTypeValue<Bool> for bool {
    #[inline]
    fn stv_to_tokens(&self) -> WordToken {
        WordToken(Word::with_last_byte(*self as u8))
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        out.push(*self as u8);
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<Bool>::stv_to_tokens(self).0
    }
}

impl SolType for Bool {
    type RustType = bool;
    type Token<'a> = WordToken;

    const ENCODED_SIZE: Option<usize> = Some(32);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        utils::check_zeroes(&token.0[..31])
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        token.0 != Word::ZERO
    }
}

/// Int - `intX`
pub struct Int<const BITS: usize>;

impl<T, const BITS: usize> SolTypeValue<Int<BITS>> for T
where
    T: Borrow<<IntBitCount<BITS> as SupportedInt>::Int>,
    IntBitCount<BITS>: SupportedInt,
{
    #[inline]
    fn stv_to_tokens(&self) -> WordToken {
        IntBitCount::<BITS>::tokenize_int(*self.borrow())
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        IntBitCount::<BITS>::encode_packed_to_int(*self.borrow(), out);
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<Int<BITS>>::stv_to_tokens(self).0
    }
}

impl<const BITS: usize> SolType for Int<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    type RustType = <IntBitCount<BITS> as SupportedInt>::Int;
    type Token<'a> = WordToken;

    const ENCODED_SIZE: Option<usize> = Some(32);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        IntBitCount::<BITS>::INT_NAME.into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        if BITS == 256 {
            return true;
        }

        let is_negative = token.0[IntBitCount::<BITS>::WORD_MSB] & 0x80 == 0x80;
        let sign_extension = is_negative as u8 * 0xff;

        // check that all upper bytes are an extension of the sign bit
        token.0[..IntBitCount::<BITS>::WORD_MSB].iter().all(|byte| *byte == sign_extension)
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        IntBitCount::<BITS>::detokenize_int(token)
    }
}

/// Uint - `uintX`
pub struct Uint<const BITS: usize>;

impl<const BITS: usize, T> SolTypeValue<Uint<BITS>> for T
where
    T: Borrow<<IntBitCount<BITS> as SupportedInt>::Uint>,
    IntBitCount<BITS>: SupportedInt,
{
    #[inline]
    fn stv_to_tokens(&self) -> WordToken {
        IntBitCount::<BITS>::tokenize_uint(*self.borrow())
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        IntBitCount::<BITS>::encode_packed_to_uint(*self.borrow(), out);
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<Uint<BITS>>::stv_to_tokens(self).0
    }
}

impl<const BITS: usize> SolType for Uint<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    type RustType = <IntBitCount<BITS> as SupportedInt>::Uint;
    type Token<'a> = WordToken;

    const ENCODED_SIZE: Option<usize> = Some(32);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        IntBitCount::<BITS>::UINT_NAME.into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        utils::check_zeroes(&token.0[..<IntBitCount<BITS> as SupportedInt>::WORD_MSB])
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        IntBitCount::<BITS>::detokenize_uint(token)
    }
}

/// FixedBytes - `bytesX`
#[derive(Clone, Copy, Debug)]
pub struct FixedBytes<const N: usize>;

impl<T: Borrow<[u8; N]>, const N: usize> SolTypeValue<FixedBytes<N>> for T
where
    ByteCount<N>: SupportedFixedBytes,
{
    #[inline]
    fn stv_to_tokens(&self) -> <FixedBytes<N> as SolType>::Token<'_> {
        let mut word = Word::ZERO;
        word[..N].copy_from_slice(self.borrow());
        word.into()
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<FixedBytes<N>>::stv_to_tokens(self).0
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.borrow().as_slice());
    }
}

impl<const N: usize> SolType for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    type RustType = RustFixedBytes<N>;
    type Token<'a> = WordToken;

    const ENCODED_SIZE: Option<usize> = Some(32);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        <ByteCount<N>>::NAME.into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        utils::check_zeroes(&token.0[N..])
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        token.0[..N].try_into().unwrap()
    }
}

/// Address - `address`
pub struct Address;

impl<T: Borrow<[u8; 20]>> SolTypeValue<Address> for T {
    #[inline]
    fn stv_to_tokens(&self) -> WordToken {
        WordToken(RustAddress::new(*self.borrow()).into_word())
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.borrow());
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<Address>::stv_to_tokens(self).0
    }
}

impl SolType for Address {
    type RustType = RustAddress;
    type Token<'a> = WordToken;

    const ENCODED_SIZE: Option<usize> = Some(32);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "address".into()
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        RustAddress::from_word(token.0)
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        utils::check_zeroes(&token.0[..12])
    }
}

/// Function - `function`
pub struct Function;

impl<T: Borrow<[u8; 24]>> SolTypeValue<Function> for T {
    #[inline]
    fn stv_to_tokens(&self) -> WordToken {
        WordToken(RustFunction::new(*self.borrow()).into_word())
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.borrow());
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<Function>::stv_to_tokens(self).0
    }
}

impl SolType for Function {
    type RustType = RustFunction;
    type Token<'a> = WordToken;

    const ENCODED_SIZE: Option<usize> = Some(32);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "function".into()
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        RustFunction::from_word(token.0)
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        utils::check_zeroes(&token.0[24..])
    }
}

/// Bytes - `bytes`
pub struct Bytes;

impl<T: ?Sized + AsRef<[u8]>> SolTypeValue<Bytes> for T {
    #[inline]
    fn stv_to_tokens(&self) -> PackedSeqToken<'_> {
        PackedSeqToken(self.as_ref())
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        let s = self.as_ref();
        if s.is_empty() {
            64
        } else {
            64 + utils::padded_len(s)
        }
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        keccak256(Bytes::abi_encode_packed(self))
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.as_ref());
    }
}

impl SolType for Bytes {
    type RustType = Vec<u8>;
    type Token<'a> = PackedSeqToken<'a>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bytes".into()
    }

    #[inline]
    fn valid_token(_token: &Self::Token<'_>) -> bool {
        true
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        token.into_vec()
    }
}

/// String - `string`
pub struct String;

impl<T: ?Sized + AsRef<str>> SolTypeValue<String> for T {
    #[inline]
    fn stv_to_tokens(&self) -> PackedSeqToken<'_> {
        PackedSeqToken(self.as_ref().as_bytes())
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        let s = self.as_ref();
        if s.is_empty() {
            64
        } else {
            64 + utils::padded_len(s.as_bytes())
        }
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        keccak256(String::abi_encode_packed(self))
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.as_ref().as_ref());
    }
}

impl SolType for String {
    type RustType = RustString;
    type Token<'a> = PackedSeqToken<'a>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "string".into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        core::str::from_utf8(token.as_slice()).is_ok()
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        // NOTE: We're decoding strings using lossy UTF-8 decoding to
        // prevent invalid strings written into contracts by either users or
        // Solidity bugs from causing graph-node to fail decoding event
        // data.
        RustString::from_utf8_lossy(token.as_slice()).into_owned()
    }
}

/// Array - `T[]`
pub struct Array<T: SolType>(PhantomData<T>);

impl<T, U> SolTypeValue<Array<U>> for [T]
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> DynSeqToken<U::Token<'_>> {
        DynSeqToken(self.iter().map(T::stv_to_tokens).collect())
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        if let Some(size) = Array::<U>::ENCODED_SIZE {
            return size;
        }

        64 + self.iter().map(T::stv_abi_encoded_size).sum::<usize>()
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        let mut encoded = Vec::new();
        for item in self {
            encoded.extend_from_slice(T::stv_eip712_data_word(item).as_slice());
        }
        keccak256(encoded)
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        for item in self {
            T::stv_abi_encode_packed_to(item, out);
        }
    }
}

impl<T, U> SolTypeValue<Array<U>> for &[T]
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> DynSeqToken<U::Token<'_>> {
        (**self).stv_to_tokens()
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        (**self).stv_abi_encoded_size()
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        (**self).stv_eip712_data_word()
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        (**self).stv_abi_encode_packed_to(out)
    }
}

impl<T, U> SolTypeValue<Array<U>> for &mut [T]
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> DynSeqToken<U::Token<'_>> {
        (**self).stv_to_tokens()
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        (**self).stv_abi_encoded_size()
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        (**self).stv_eip712_data_word()
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        (**self).stv_abi_encode_packed_to(out)
    }
}

impl<T, U> SolTypeValue<Array<U>> for Vec<T>
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> DynSeqToken<U::Token<'_>> {
        <[T] as SolTypeValue<Array<U>>>::stv_to_tokens(self)
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        (**self).stv_abi_encoded_size()
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        (**self).stv_eip712_data_word()
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        (**self).stv_abi_encode_packed_to(out)
    }
}

impl<T: SolType> SolType for Array<T> {
    type RustType = Vec<T::RustType>;
    type Token<'a> = DynSeqToken<T::Token<'a>>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[]", T::sol_type_name()).into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        token.0.iter().all(T::valid_token)
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        token.0.into_iter().map(T::detokenize).collect()
    }
}

/// FixedArray - `T[M]`
pub struct FixedArray<T, const N: usize>(PhantomData<T>);

impl<T, U, const N: usize> SolTypeValue<FixedArray<U, N>> for [T; N]
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> <FixedArray<U, N> as SolType>::Token<'_> {
        FixedSeqToken(core::array::from_fn(|i| self[i].stv_to_tokens()))
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        if let Some(size) = FixedArray::<U, N>::ENCODED_SIZE {
            return size;
        }

        let sum = self.iter().map(T::stv_abi_encoded_size).sum::<usize>();
        if FixedArray::<U, N>::DYNAMIC {
            32 + sum
        } else {
            sum
        }
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        let mut encoded = crate::impl_core::uninit_array::<[u8; 32], N>();
        for (i, item) in self.iter().enumerate() {
            encoded[i].write(T::stv_eip712_data_word(item).0);
        }
        // SAFETY: Flattening [[u8; 32]; N] to [u8; N * 32] is valid
        let encoded: &[u8] =
            unsafe { core::slice::from_raw_parts(encoded.as_ptr().cast(), N * 32) };
        keccak256(encoded)
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        for item in self {
            T::stv_abi_encode_packed_to(item, out);
        }
    }
}

impl<T, U, const N: usize> SolTypeValue<FixedArray<U, N>> for &[T; N]
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> <FixedArray<U, N> as SolType>::Token<'_> {
        <[T; N] as SolTypeValue<FixedArray<U, N>>>::stv_to_tokens(&**self)
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        SolTypeValue::<FixedArray<U, N>>::stv_abi_encoded_size(&**self)
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<FixedArray<U, N>>::stv_eip712_data_word(&**self)
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        SolTypeValue::<FixedArray<U, N>>::stv_abi_encode_packed_to(&**self, out)
    }
}

impl<T, U, const N: usize> SolTypeValue<FixedArray<U, N>> for &mut [T; N]
where
    T: SolTypeValue<U>,
    U: SolType,
{
    #[inline]
    fn stv_to_tokens(&self) -> <FixedArray<U, N> as SolType>::Token<'_> {
        <[T; N] as SolTypeValue<FixedArray<U, N>>>::stv_to_tokens(&**self)
    }

    #[inline]
    fn stv_abi_encoded_size(&self) -> usize {
        SolTypeValue::<FixedArray<U, N>>::stv_abi_encoded_size(&**self)
    }

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        SolTypeValue::<FixedArray<U, N>>::stv_eip712_data_word(&**self)
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
        SolTypeValue::<FixedArray<U, N>>::stv_abi_encode_packed_to(&**self, out)
    }
}

impl<T: SolType, const N: usize> SolType for FixedArray<T, N> {
    type RustType = [T::RustType; N];
    type Token<'a> = FixedSeqToken<T::Token<'a>, N>;

    const ENCODED_SIZE: Option<usize> = {
        match T::ENCODED_SIZE {
            Some(size) => Some(size * N),
            None => None,
        }
    };

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[{}]", T::sol_type_name(), N).into()
    }

    #[inline]
    fn valid_token(token: &Self::Token<'_>) -> bool {
        token.as_array().iter().all(T::valid_token)
    }

    #[inline]
    fn detokenize(token: Self::Token<'_>) -> Self::RustType {
        token.0.map(T::detokenize)
    }
}

macro_rules! tuple_encodable_impls {
    ($count:literal $(($ty:ident $uty:ident)),+) => {
        #[allow(non_snake_case)]
        impl<$($ty: SolTypeValue<$uty>, $uty: SolType),+> SolTypeValue<($($uty,)+)> for ($($ty,)+) {
            #[inline]
            fn stv_to_tokens(&self) -> <($($uty,)+) as SolType>::Token<'_> {
                let ($($ty,)+) = self;
                ($(SolTypeValue::<$uty>::stv_to_tokens($ty),)+)
            }

            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <($($uty,)+) as SolType>::ENCODED_SIZE {
                    return size
                }

                let ($($ty,)+) = self;
                let sum = 0 $( + $ty.stv_abi_encoded_size() )+;
                if <($($uty,)+) as SolType>::DYNAMIC {
                    32 + sum
                } else {
                    sum
                }
            }

            fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
                let ($($ty,)+) = self;
                // TODO: Reserve
                $(
                    $ty.stv_abi_encode_packed_to(out);
                )+
            }

            fn stv_eip712_data_word(&self) -> Word {
                let ($($ty,)+) = self;
                let encoding: [[u8; 32]; $count] = [$(
                    <$uty as SolType>::eip712_data_word($ty).0,
                )+];
                // SAFETY: Flattening [[u8; 32]; $count] to [u8; $count * 32] is valid
                let encoding: &[u8] = unsafe { core::slice::from_raw_parts(encoding.as_ptr().cast(), $count * 32) };
                keccak256(encoding).into()
            }
        }
    };
}

macro_rules! tuple_impls {
    // Push 1 element, push a comma if we're not done yet, recurse
    (@fmt $s:ident; ) => {};
    (@fmt $s:ident; $first:ident $(, $rest:ident)*) => {
        $s.extend_from_slice(<$first as SolType>::sol_type_name().as_bytes());
        tuple_impls!(@fmt_comma $s; $($rest),*);
        tuple_impls!(@fmt $s; $($rest),*);
    };

    (@fmt_comma $s:ident; ) => {};
    (@fmt_comma $s:ident; $($t:tt)+) => { $s.push(b',') };

    ($count:literal $($ty:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($ty: SolType,)+> SolType for ($($ty,)+) {
            type RustType = ($( $ty::RustType, )+);
            type Token<'a> = ($( $ty::Token<'a>, )+);

            const ENCODED_SIZE: Option<usize> = 'l: {
                let mut acc = 0;
                $(
                    match <$ty as SolType>::ENCODED_SIZE {
                        Some(size) => acc += size,
                        None => break 'l None,
                    }
                )+
                Some(acc)
            };

            fn sol_type_name() -> Cow<'static, str> {
                let mut s = Vec::<u8>::with_capacity(2 + $count * 8);
                s.push(b'(');
                tuple_impls!(@fmt s; $($ty),+);
                s.push(b')');
                // SAFETY: we're pushing only other `str`s and ASCII characters
                Cow::Owned(unsafe { RustString::from_utf8_unchecked(s) })
            }

            fn valid_token(token: &Self::Token<'_>) -> bool {
                let ($($ty,)+) = token;
                $(<$ty as SolType>::valid_token($ty))&&+
            }

            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let ($($ty,)+) = token;
                ($(
                    <$ty as SolType>::detokenize($ty),
                )+)
            }
        }
    };
}

impl SolTypeValue<()> for () {
    #[inline]
    fn stv_to_tokens(&self) {}

    #[inline]
    fn stv_eip712_data_word(&self) -> Word {
        Word::ZERO
    }

    #[inline]
    fn stv_abi_encode_packed_to(&self, _out: &mut Vec<u8>) {}
}

all_the_tuples!(@double tuple_encodable_impls);

impl SolType for () {
    type RustType = ();
    type Token<'a> = ();

    const ENCODED_SIZE: Option<usize> = Some(0);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "()".into()
    }

    #[inline]
    fn valid_token((): &()) -> bool {
        true
    }

    #[inline]
    fn detokenize((): ()) -> Self::RustType {}
}

all_the_tuples!(tuple_impls);

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

/// Specifies the number of bytes in a [`FixedBytes`] array as a type.
pub struct ByteCount<const N: usize>;

impl<const N: usize> Sealed for ByteCount<N> {}

/// Statically guarantees that a `FixedBytes` byte count is marked as supported.
///
/// This trait is *sealed*: the list of implementors below is total.
///
/// Users do not have the ability to mark additional [`ByteCount<N>`] values as
/// supported. Only `FixedBytes` with supported byte counts are constructable.
pub trait SupportedFixedBytes: Sealed {
    /// The name of the `FixedBytes` type: `bytes<N>`
    const NAME: &'static str;
}

macro_rules! supported_fixed_bytes {
    ($($n:literal),+) => {$(
        impl SupportedFixedBytes for ByteCount<$n> {
            const NAME: &'static str = concat!("bytes", $n);
        }
    )+};
}

supported_fixed_bytes!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);

/// Specifies the number of bits in an [`Int`] or [`Uint`] as a type.
pub struct IntBitCount<const N: usize>;

impl<const N: usize> Sealed for IntBitCount<N> {}

// Declares types with the same traits
// TODO: Add more traits
// TODO: Integrate `num_traits` (needs `ruint`)
macro_rules! declare_int_types {
    ($($(#[$attr:meta])* type $name:ident;)*) => {$(
        $(#[$attr])*
        type $name: Sized + Copy + PartialOrd + Ord + Eq + Hash
            + Not + BitAnd + BitOr + BitXor
            + Add + Sub + Mul + Div + Rem
            + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
            + Debug + Display + LowerHex + UpperHex + Octal + Binary;
    )*};
}

/// Statically guarantees that a [`Int`] or [`Uint`] bit count is marked as
/// supported.
///
/// This trait is *sealed*: the list of implementors below is total.
///
/// Users do not have the ability to mark additional [`IntBitCount<N>`] values
/// as supported. Only `Int` and `Uint` with supported byte counts are
/// constructable.
pub trait SupportedInt: Sealed {
    declare_int_types! {
        /// The signed integer Rust representation.
        type Int;

        /// The unsigned integer Rust representation.
        type Uint;
    }

    /// The name of the `Int` type: `int<N>`
    const INT_NAME: &'static str;

    /// The name of the `Uint` type: `uint<N>`
    const UINT_NAME: &'static str;

    /// The number of bits in the integer: `BITS`
    ///
    /// Note that this is not equal to `Self::Int::BITS`.
    const BITS: usize;

    /// The number of bytes in the integer: `BITS / 8`
    const BYTES: usize = Self::BITS / 8;

    /// The difference between the representation's and this integer's bytes:
    /// `(Self::Int::BITS - Self::BITS) / 8`
    ///
    /// E.g.: `word[Self::WORD_MSB - Self::SKIP_BYTES..] == int.to_be_bytes()`
    const SKIP_BYTES: usize;

    /// The index of the most significant byte in the Word type.
    ///
    /// E.g.: `word[Self::WORD_MSB..] == int.to_be_bytes()[Self::SKIP_BYTES..]`
    const WORD_MSB: usize = 32 - Self::BYTES;

    /// Tokenizes a signed integer.
    fn tokenize_int(int: Self::Int) -> WordToken;
    /// Detokenizes a signed integer.
    fn detokenize_int(token: WordToken) -> Self::Int;
    /// ABI-encode a signed integer in packed mode.
    fn encode_packed_to_int(int: Self::Int, out: &mut Vec<u8>);

    /// Tokenizes an unsigned integer.
    fn tokenize_uint(uint: Self::Uint) -> WordToken;
    /// Detokenizes an unsigned integer.
    fn detokenize_uint(token: WordToken) -> Self::Uint;
    /// ABI-encode an unsigned integer in packed mode.
    fn encode_packed_to_uint(uint: Self::Uint, out: &mut Vec<u8>);
}

macro_rules! supported_int {
    ($($n:literal => $i:ident, $u:ident;)+) => {$(
        impl SupportedInt for IntBitCount<$n> {
            type Int = $i;
            type Uint = $u;

            const UINT_NAME: &'static str = concat!("uint", $n);
            const INT_NAME: &'static str = concat!("int", $n);

            const BITS: usize = $n;
            const SKIP_BYTES: usize = (<$i>::BITS as usize - <Self as SupportedInt>::BITS) / 8;

            int_impls2!($i);
            int_impls2!($u);
        }
    )+};
}

macro_rules! int_impls {
    (@primitive_int $ity:ident) => {
        #[inline]
        fn tokenize_int(int: $ity) -> WordToken {
            let mut word = [int.is_negative() as u8 * 0xff; 32];
            word[Self::WORD_MSB..].copy_from_slice(&int.to_be_bytes()[Self::SKIP_BYTES..]);
            WordToken::new(word)
        }

        #[inline]
        fn detokenize_int(mut token: WordToken) -> $ity {
            // sign extend bits to ignore
            let is_negative = token.0[Self::WORD_MSB] & 0x80 == 0x80;
            let sign_extension = is_negative as u8 * 0xff;
            token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(sign_extension);

            let s = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
            <$ity>::from_be_bytes(s.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_int(int: $ity, out: &mut Vec<u8>) {
            out.extend_from_slice(&int.to_be_bytes()[Self::SKIP_BYTES..]);
        }
    };
    (@primitive_uint $uty:ident) => {
        #[inline]
        fn tokenize_uint(uint: $uty) -> WordToken {
            let mut word = Word::ZERO;
            word[Self::WORD_MSB..].copy_from_slice(&uint.to_be_bytes()[Self::SKIP_BYTES..]);
            WordToken(word)
        }

        #[inline]
        fn detokenize_uint(mut token: WordToken) -> $uty {
            // zero out bits to ignore (u24):
            // mov   byte ptr [rdi + 28], 0
            // movbe eax, dword ptr [rdi + 28]
            token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(0);
            let s = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
            <$uty>::from_be_bytes(s.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_uint(uint: $uty, out: &mut Vec<u8>) {
            out.extend_from_slice(&uint.to_be_bytes()[Self::SKIP_BYTES..]);
        }
    };

    (@big_int $ity:ident) => {
        #[inline]
        fn tokenize_int(int: $ity) -> WordToken {
            let mut word = [int.is_negative() as u8 * 0xff; 32];
            word[Self::WORD_MSB..].copy_from_slice(&int.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
            WordToken::new(word)
        }

        #[inline]
        fn detokenize_int(mut token: WordToken) -> $ity {
            // sign extend bits to ignore
            let is_negative = token.0[Self::WORD_MSB] & 0x80 == 0x80;
            let sign_extension = is_negative as u8 * 0xff;
            token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(sign_extension);

            let s = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
            <$ity>::from_be_bytes::<32>(s.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_int(int: $ity, out: &mut Vec<u8>) {
            out.extend_from_slice(&int.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
        }
    };
    (@big_uint $uty:ident) => {
        #[inline]
        fn tokenize_uint(uint: $uty) -> WordToken {
            let mut word = Word::ZERO;
            word[Self::WORD_MSB..].copy_from_slice(&uint.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
            WordToken(word)
        }

        #[inline]
        fn detokenize_uint(mut token: WordToken) -> $uty {
            // zero out bits to ignore
            token.0[..Self::SKIP_BYTES].fill(0);
            <$uty>::from_be_bytes::<32>(token.0 .0)
        }

        #[inline]
        fn encode_packed_to_uint(uint: $uty, out: &mut Vec<u8>) {
            out.extend_from_slice(&uint.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
        }
    };
}

#[rustfmt::skip]
macro_rules! int_impls2 {
    (  i8) => { int_impls! { @primitive_int    i8 } };
    ( i16) => { int_impls! { @primitive_int   i16 } };
    ( i32) => { int_impls! { @primitive_int   i32 } };
    ( i64) => { int_impls! { @primitive_int   i64 } };
    (i128) => { int_impls! { @primitive_int  i128 } };

    (  u8) => { int_impls! { @primitive_uint   u8 } };
    ( u16) => { int_impls! { @primitive_uint  u16 } };
    ( u32) => { int_impls! { @primitive_uint  u32 } };
    ( u64) => { int_impls! { @primitive_uint  u64 } };
    (u128) => { int_impls! { @primitive_uint u128 } };

    (I256) => { int_impls! { @big_int  I256 } };
    (U256) => { int_impls! { @big_uint U256 } };
}

supported_int!(
      8 => i8, u8;
     16 => i16, u16;
     24 => i32, u32;
     32 => i32, u32;
     40 => i64, u64;
     48 => i64, u64;
     56 => i64, u64;
     64 => i64, u64;
     72 => i128, u128;
     80 => i128, u128;
     88 => i128, u128;
     96 => i128, u128;
    104 => i128, u128;
    112 => i128, u128;
    120 => i128, u128;
    128 => i128, u128;
    136 => I256, U256;
    144 => I256, U256;
    152 => I256, U256;
    160 => I256, U256;
    168 => I256, U256;
    176 => I256, U256;
    184 => I256, U256;
    192 => I256, U256;
    200 => I256, U256;
    208 => I256, U256;
    216 => I256, U256;
    224 => I256, U256;
    232 => I256, U256;
    240 => I256, U256;
    248 => I256, U256;
    256 => I256, U256;
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sol, SolValue};
    use alloy_primitives::hex;

    macro_rules! assert_encoded_size {
        ($t:ty, $sz:expr) => {
            let sz = $sz;
            assert_eq!(<$t as SolType>::ENCODED_SIZE, sz);
            assert_eq!(<$t as SolType>::DYNAMIC, sz.is_none());
        };
    }

    #[test]
    fn primitive_encoded_sizes() {
        assert_encoded_size!(Bool, Some(32));

        assert_encoded_size!(Uint<8>, Some(32));
        assert_encoded_size!(Int<8>, Some(32));
        assert_encoded_size!(Uint<16>, Some(32));
        assert_encoded_size!(Int<16>, Some(32));
        assert_encoded_size!(Uint<32>, Some(32));
        assert_encoded_size!(Int<32>, Some(32));
        assert_encoded_size!(Uint<64>, Some(32));
        assert_encoded_size!(Int<64>, Some(32));
        assert_encoded_size!(Uint<128>, Some(32));
        assert_encoded_size!(Int<128>, Some(32));
        assert_encoded_size!(Uint<256>, Some(32));
        assert_encoded_size!(Int<256>, Some(32));

        assert_encoded_size!(Address, Some(32));
        assert_encoded_size!(Function, Some(32));
        assert_encoded_size!(FixedBytes<1>, Some(32));
        assert_encoded_size!(FixedBytes<16>, Some(32));
        assert_encoded_size!(FixedBytes<32>, Some(32));

        assert_encoded_size!(Bytes, None);
        assert_encoded_size!(String, None);

        assert_encoded_size!(Array<()>, None);
        assert_encoded_size!(Array<Uint<8>>, None);
        assert_encoded_size!(Array<Bytes>, None);

        assert_encoded_size!(FixedArray<(), 0>, Some(0));
        assert_encoded_size!(FixedArray<(), 1>, Some(0));
        assert_encoded_size!(FixedArray<(), 2>, Some(0));
        assert_encoded_size!(FixedArray<Uint<8>, 0>, Some(0));
        assert_encoded_size!(FixedArray<Uint<8>, 1>, Some(32));
        assert_encoded_size!(FixedArray<Uint<8>, 2>, Some(64));
        assert_encoded_size!(FixedArray<Bytes, 0>, None);
        assert_encoded_size!(FixedArray<Bytes, 1>, None);
        assert_encoded_size!(FixedArray<Bytes, 2>, None);

        assert_encoded_size!((), Some(0));
        assert_encoded_size!(((),), Some(0));
        assert_encoded_size!(((), ()), Some(0));
        assert_encoded_size!((Uint<8>,), Some(32));
        assert_encoded_size!((Uint<8>, Bool), Some(64));
        assert_encoded_size!((Uint<8>, Bool, FixedArray<Address, 4>), Some(6 * 32));
        assert_encoded_size!((Bytes,), None);
        assert_encoded_size!((Uint<8>, Bytes), None);
    }

    #[test]
    fn udvt_encoded_sizes() {
        macro_rules! udvt_and_assert {
            ([$($t:tt)*], $e:expr) => {{
                type Alias = sol!($($t)*);
                sol!(type Udvt is $($t)*;);
                assert_encoded_size!(Alias, $e);
                assert_encoded_size!(Udvt, $e);
            }};
        }
        udvt_and_assert!([bool], Some(32));

        udvt_and_assert!([uint8], Some(32));
        udvt_and_assert!([int8], Some(32));
        udvt_and_assert!([uint16], Some(32));
        udvt_and_assert!([int16], Some(32));
        udvt_and_assert!([uint32], Some(32));
        udvt_and_assert!([int32], Some(32));
        udvt_and_assert!([uint64], Some(32));
        udvt_and_assert!([int64], Some(32));
        udvt_and_assert!([uint128], Some(32));
        udvt_and_assert!([int128], Some(32));
        udvt_and_assert!([uint256], Some(32));
        udvt_and_assert!([int256], Some(32));

        udvt_and_assert!([address], Some(32));
        udvt_and_assert!([function()], Some(32));
        udvt_and_assert!([bytes1], Some(32));
        udvt_and_assert!([bytes16], Some(32));
        udvt_and_assert!([bytes32], Some(32));
    }

    #[test]
    fn custom_encoded_sizes() {
        macro_rules! custom_and_assert {
            ($block:tt, $e:expr) => {{
                sol! {
                    struct Struct $block
                }
                assert_encoded_size!(Struct, $e);
            }};
        }
        custom_and_assert!({ bool a; }, Some(32));
        custom_and_assert!({ bool a; address b; }, Some(64));
        custom_and_assert!({ bool a; bytes1[69] b; uint8 c; }, Some(71 * 32));
        custom_and_assert!({ bytes a; }, None);
        custom_and_assert!({ bytes a; bytes24 b; }, None);
        custom_and_assert!({ bool a; bytes2[42] b; uint8 c; bytes d; }, None);
    }

    #[test]
    fn tuple_of_refs() {
        let a = (1u8,);
        let b = (&1u8,);

        type MyTy = (Uint<8>,);

        MyTy::tokenize(&a);
        MyTy::tokenize(&b);
    }

    macro_rules! roundtrip {
        ($($name:ident($st:ty : $t:ty);)+) => {
            proptest::proptest! {$(
                #[test]
                #[cfg_attr(miri, ignore = "doesn't run in isolation and would take too long")]
                fn $name(i: $t) {
                    let token = <$st>::tokenize(&i);
                    proptest::prop_assert_eq!(token.total_words() * 32, <$st>::abi_encoded_size(&i));
                    proptest::prop_assert_eq!(<$st>::detokenize(token), i);
                }
            )+}
        };
    }

    roundtrip! {
        roundtrip_address(Address: RustAddress);
        roundtrip_bool(Bool: bool);
        roundtrip_bytes(Bytes: Vec<u8>);
        roundtrip_string(String: RustString);
        roundtrip_fixed_bytes_16(FixedBytes<16>: [u8; 16]);
        roundtrip_fixed_bytes_32(FixedBytes<32>: [u8; 32]);

        // can only test corresponding integers
        roundtrip_u8(Uint<8>: u8);
        roundtrip_i8(Int<8>: i8);
        roundtrip_u16(Uint<16>: u16);
        roundtrip_i16(Int<16>: i16);
        roundtrip_u32(Uint<32>: u32);
        roundtrip_i32(Int<32>: i32);
        roundtrip_u64(Uint<64>: u64);
        roundtrip_i64(Int<64>: i64);
        roundtrip_u128(Uint<128>: u128);
        roundtrip_i128(Int<128>: i128);
        roundtrip_u256(Uint<256>: U256);
        roundtrip_i256(Int<256>: I256);
    }

    #[test]
    fn tokenize_uint() {
        macro_rules! test {
            ($($n:literal: $x:expr => $l:literal),+ $(,)?) => {$(
                let uint: <Uint<$n> as SolType>::RustType = $x.into();
                let int = <Int<$n> as SolType>::RustType::try_from(uint).unwrap();

                assert_eq!(
                    <Uint<$n>>::tokenize(&uint),
                    WordToken::new(alloy_primitives::hex!($l))
                );
                assert_eq!(
                    <Int<$n>>::tokenize(&int),
                    WordToken::new(alloy_primitives::hex!($l))
                );
            )+};
        }

        let word: Word = Word::new(core::array::from_fn(|i| i as u8 + 1));

        test! {
             8: 0x00u8 => "0000000000000000000000000000000000000000000000000000000000000000",
             8: 0x01u8 => "0000000000000000000000000000000000000000000000000000000000000001",
            24: 0x01020304u32 => "0000000000000000000000000000000000000000000000000000000000020304",
            32: 0x01020304u32 => "0000000000000000000000000000000000000000000000000000000001020304",
            56: 0x0102030405060708u64 => "0000000000000000000000000000000000000000000000000002030405060708",
            64: 0x0102030405060708u64 => "0000000000000000000000000000000000000000000000000102030405060708",

            160: word => "0000000000000000000000000d0e0f101112131415161718191a1b1c1d1e1f20",
            200: word => "0000000000000008090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
            256: word => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
        }
    }

    #[test]
    fn detokenize_ints() {
        /*
        for i in range(1, 32 + 1):
            n = "0x"
            for j in range(32, 0, -1):
                if j <= i:
                    n += hex(33 - j)[2:].zfill(2)
                else:
                    n += "00"
            if i > 16:
                n = f'"{n}".parse().unwrap()'
            else:
                n = f" {n}"
            print(f"{i * 8:4} => {n},")
        */
        let word = core::array::from_fn(|i| i as u8 + 1);
        let token = WordToken::new(word);
        macro_rules! test {
            ($($n:literal => $x:expr),+ $(,)?) => {$(
                assert_eq!(<Uint<$n>>::detokenize(token), $x);
                assert_eq!(<Int<$n>>::detokenize(token), $x);
            )+};
        }
        #[rustfmt::skip]
        test! {
             8 =>  0x0000000000000000000000000000000000000000000000000000000000000020,
            16 =>  0x0000000000000000000000000000000000000000000000000000000000001f20,
            24 =>  0x00000000000000000000000000000000000000000000000000000000001e1f20,
            32 =>  0x000000000000000000000000000000000000000000000000000000001d1e1f20,
            40 =>  0x0000000000000000000000000000000000000000000000000000001c1d1e1f20,
            48 =>  0x00000000000000000000000000000000000000000000000000001b1c1d1e1f20,
            56 =>  0x000000000000000000000000000000000000000000000000001a1b1c1d1e1f20,
            64 =>  0x000000000000000000000000000000000000000000000000191a1b1c1d1e1f20,
            72 =>  0x000000000000000000000000000000000000000000000018191a1b1c1d1e1f20,
            80 =>  0x000000000000000000000000000000000000000000001718191a1b1c1d1e1f20,
            88 =>  0x000000000000000000000000000000000000000000161718191a1b1c1d1e1f20,
            96 =>  0x000000000000000000000000000000000000000015161718191a1b1c1d1e1f20,
           104 =>  0x000000000000000000000000000000000000001415161718191a1b1c1d1e1f20,
           112 =>  0x000000000000000000000000000000000000131415161718191a1b1c1d1e1f20,
           120 =>  0x000000000000000000000000000000000012131415161718191a1b1c1d1e1f20,
           128 =>  0x000000000000000000000000000000001112131415161718191a1b1c1d1e1f20,
           136 => "0x000000000000000000000000000000101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           144 => "0x00000000000000000000000000000f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           152 => "0x000000000000000000000000000e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           160 => "0x0000000000000000000000000d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           168 => "0x00000000000000000000000c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           176 => "0x000000000000000000000b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           184 => "0x0000000000000000000a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           192 => "0x0000000000000000090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           200 => "0x0000000000000008090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           208 => "0x0000000000000708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           216 => "0x0000000000060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           224 => "0x0000000005060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           232 => "0x0000000405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           240 => "0x0000030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           248 => "0x0002030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           256 => "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
        };
    }

    #[test]
    fn detokenize_negative_int() {
        let word = [0xff; 32];
        let token = WordToken::new(word);
        assert_eq!(<Int<8>>::detokenize(token), -1);
        assert_eq!(<Int<16>>::detokenize(token), -1);
        assert_eq!(<Int<24>>::detokenize(token), -1);
        assert_eq!(<Int<32>>::detokenize(token), -1);
        assert_eq!(<Int<40>>::detokenize(token), -1);
        assert_eq!(<Int<48>>::detokenize(token), -1);
        assert_eq!(<Int<56>>::detokenize(token), -1);
        assert_eq!(<Int<64>>::detokenize(token), -1);
        assert_eq!(<Int<72>>::detokenize(token), -1);
        assert_eq!(<Int<80>>::detokenize(token), -1);
        assert_eq!(<Int<88>>::detokenize(token), -1);
        assert_eq!(<Int<96>>::detokenize(token), -1);
        assert_eq!(<Int<104>>::detokenize(token), -1);
        assert_eq!(<Int<112>>::detokenize(token), -1);
        assert_eq!(<Int<120>>::detokenize(token), -1);
        assert_eq!(<Int<128>>::detokenize(token), -1);
        assert_eq!(<Int<136>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<144>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<152>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<160>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<168>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<176>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<184>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<192>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<200>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<208>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<216>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<224>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<232>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<240>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<248>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<256>>::detokenize(token), I256::MINUS_ONE);
    }

    #[test]
    #[rustfmt::skip]
    fn detokenize_int() {
        let word =
            core::array::from_fn(|i| (i | (0x80 * (i % 2 == 1) as usize)) as u8 + 1);
        let token = WordToken::new(word);
        trait Conv {
            fn as_u256_as_i256(&self) -> I256;
        }
        impl Conv for str {
            fn as_u256_as_i256(&self) -> I256 {
                I256::from_raw(self.parse::<U256>().unwrap())
            }
        }
        assert_eq!(<Int<8>>::detokenize(token),    0x00000000000000000000000000000000000000000000000000000000000000a0_u8 as i8);
        assert_eq!(<Int<16>>::detokenize(token),   0x0000000000000000000000000000000000000000000000000000000000001fa0_u16 as i16);
        assert_eq!(<Int<24>>::detokenize(token),   0x00000000000000000000000000000000000000000000000000000000ff9e1fa0_u32 as i32);
        assert_eq!(<Int<32>>::detokenize(token),   0x000000000000000000000000000000000000000000000000000000001d9e1fa0_u32 as i32);
        assert_eq!(<Int<40>>::detokenize(token),   0x000000000000000000000000000000000000000000000000ffffff9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<48>>::detokenize(token),   0x00000000000000000000000000000000000000000000000000001b9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<56>>::detokenize(token),   0x000000000000000000000000000000000000000000000000ff9a1b9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<64>>::detokenize(token),   0x000000000000000000000000000000000000000000000000199a1b9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<72>>::detokenize(token),   0x00000000000000000000000000000000ffffffffffffff98199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<80>>::detokenize(token),   0x000000000000000000000000000000000000000000001798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<88>>::detokenize(token),   0x00000000000000000000000000000000ffffffffff961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<96>>::detokenize(token),   0x000000000000000000000000000000000000000015961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<104>>::detokenize(token),  0x00000000000000000000000000000000ffffff9415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<112>>::detokenize(token),  0x000000000000000000000000000000000000139415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<120>>::detokenize(token),  0x00000000000000000000000000000000ff92139415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<128>>::detokenize(token),  0x000000000000000000000000000000001192139415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<136>>::detokenize(token), "0xffffffffffffffffffffffffffffff901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<144>>::detokenize(token), "0x00000000000000000000000000000f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<152>>::detokenize(token), "0xffffffffffffffffffffffffff8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<160>>::detokenize(token), "0x0000000000000000000000000d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<168>>::detokenize(token), "0xffffffffffffffffffffff8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<176>>::detokenize(token), "0x000000000000000000000b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<184>>::detokenize(token), "0xffffffffffffffffff8a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<192>>::detokenize(token), "0x0000000000000000098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<200>>::detokenize(token), "0xffffffffffffff88098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<208>>::detokenize(token), "0x0000000000000788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<216>>::detokenize(token), "0xffffffffff860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<224>>::detokenize(token), "0x0000000005860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<232>>::detokenize(token), "0xffffff8405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<240>>::detokenize(token), "0x0000038405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<248>>::detokenize(token), "0xff82038405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<256>>::detokenize(token), "0x0182038405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
    }

    #[test]
    fn encode_packed() {
        let value = (RustAddress::with_last_byte(1), U256::from(2), 3u32, -3i32, 3u32, -3i32);

        let res =
            <sol! { (address, uint160, uint24, int24, uint32, int32) }>::abi_encode_packed(&value);
        let expected = hex!(
            "0000000000000000000000000000000000000001"
            "0000000000000000000000000000000000000002"
            "000003"
            "fffffd"
            "00000003"
            "fffffffd"
        );
        assert_eq!(hex::encode(res), hex::encode(expected));

        // Type is `(address, uint256, uint32, int32, uint32, int32)`
        let expected = hex!(
            "0000000000000000000000000000000000000001"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "00000003"
            "fffffffd"
            "00000003"
            "fffffffd"
        );
        assert_eq!(hex::encode(value.abi_encode_packed()), hex::encode(expected));
    }
}
