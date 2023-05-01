// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Ethereum ABI Tokens.
//!
//! ABI encoding uses 5 types:
//! - Single EVM words (a 32-byte string)
//! - Sequences with a fixed length `T[M]`
//! - Sequences with a dynamic length `T[]`
//! - Tuples (T, U, V, ...)
//! - Dynamic-length byte arrays `u8[]`

use core::fmt;

use ethers_primitives::{Address, U256};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{AbiResult, Decoder, Encoder, Word};

/// Abi-Encoding Tokens. This is a sealed trait. It contains the type
/// information necessary to encode & decode data. Tokens are an intermediate
/// state between abi-encoded blobs, and rust types.
pub trait TokenType: sealed::Sealed + Sized + Clone {
    /// True if the token represents a dynamically-sized type
    fn is_dynamic() -> bool;

    /// Decode a token from a decoder
    fn decode_from(dec: &mut Decoder<'_>) -> AbiResult<Self>;

    /// Calculate the number of head words
    fn head_words(&self) -> usize;

    /// Calculate the number of tail words
    fn tail_words(&self) -> usize;

    /// Calculate the total number of head and tail words
    fn total_words(&self) -> usize {
        self.head_words() + self.tail_words()
    }

    /// Append head words to the encoder
    fn head_append(&self, enc: &mut Encoder);

    /// Append tail words to the encoder
    fn tail_append(&self, enc: &mut Encoder);
}

/// A token composed of a sequence of other tokens
///
/// This functions as an extension trait for [`TokenType`], and may only be
/// implemented by [`FixedSeqToken`], [`DynSeqToken`], and [`PackedSeqToken`].
pub trait TokenSeq: TokenType {
    /// True for tuples only.
    fn can_be_params() -> bool {
        false
    }

    /// Encode the token sequence to the encoder
    fn encode_sequence(&self, enc: &mut Encoder);

    /// Decode the token sequence from the encoder
    fn decode_sequence(dec: &mut Decoder<'_>) -> AbiResult<Self>;
}

/// A single EVM word - T for any value type
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct WordToken(Word);

impl From<Word> for WordToken {
    fn from(value: Word) -> Self {
        Self(value)
    }
}

impl From<bool> for WordToken {
    fn from(value: bool) -> Self {
        U256::from(value as usize).into()
    }
}

impl From<U256> for WordToken {
    fn from(value: U256) -> Self {
        Self(value.to_be_bytes().into())
    }
}

impl From<Address> for WordToken {
    fn from(value: Address) -> Self {
        Self(value.into())
    }
}

impl From<[u8; 20]> for WordToken {
    fn from(value: [u8; 20]) -> Self {
        Self(Address::from(value).into())
    }
}

impl From<WordToken> for [u8; 32] {
    fn from(value: WordToken) -> [u8; 32] {
        value.0.into()
    }
}

impl From<[u8; 32]> for WordToken {
    fn from(value: [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl AsRef<Word> for WordToken {
    fn as_ref(&self) -> &Word {
        &self.0
    }
}

impl AsRef<[u8]> for WordToken {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl WordToken {
    /// Get a reference to the word as a slice
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Copy the inner word
    pub const fn inner(&self) -> Word {
        self.0
    }
}

impl TokenType for WordToken {
    fn is_dynamic() -> bool {
        false
    }

    fn decode_from(dec: &mut Decoder<'_>) -> AbiResult<Self> {
        dec.take_word().map(Into::into)
    }

    fn head_words(&self) -> usize {
        1
    }

    fn tail_words(&self) -> usize {
        0
    }

    fn head_append(&self, enc: &mut Encoder) {
        enc.append_word(self.inner());
    }

    fn tail_append(&self, _enc: &mut Encoder) {}
}

/// A Fixed Sequence - `T[N]`
#[derive(Debug, Clone, PartialEq)]
pub struct FixedSeqToken<T, const N: usize>([T; N]);

impl<T, const N: usize> TryFrom<Vec<T>> for FixedSeqToken<T, N> {
    type Error = <[T; N] as TryFrom<Vec<T>>>::Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        <[T; N]>::try_from(value).map(Self)
    }
}

impl<T, const N: usize> From<[T; N]> for FixedSeqToken<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> AsRef<[T; N]> for FixedSeqToken<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

impl<T, const N: usize> TokenType for FixedSeqToken<T, N>
where
    T: TokenType,
{
    fn is_dynamic() -> bool {
        T::is_dynamic()
    }

    fn decode_from(dec: &mut Decoder<'_>) -> AbiResult<Self> {
        let is_dynamic = Self::is_dynamic();

        let mut child = if is_dynamic {
            dec.take_indirection()?
        } else {
            dec.raw_child()
        };

        Self::decode_sequence(&mut child)
    }

    fn head_words(&self) -> usize {
        if Self::is_dynamic() {
            1
        } else {
            self.0.iter().map(TokenType::head_words).sum()
        }
    }

    fn tail_words(&self) -> usize {
        if Self::is_dynamic() {
            N
        } else {
            0
        }
    }

    fn head_append(&self, enc: &mut Encoder) {
        if Self::is_dynamic() {
            enc.append_indirection();
        } else {
            self.0.iter().for_each(|inner| inner.head_append(enc))
        }
    }

    fn tail_append(&self, enc: &mut Encoder) {
        if Self::is_dynamic() {
            self.encode_sequence(enc)
        }
    }
}

impl<T, const N: usize> TokenSeq for FixedSeqToken<T, N>
where
    T: TokenType,
{
    fn encode_sequence(&self, enc: &mut Encoder) {
        let head_words = self.0.iter().map(TokenType::head_words).sum::<usize>();
        enc.push_offset(head_words as u32);

        for t in self.0.iter() {
            t.head_append(enc);
            enc.bump_offset(t.tail_words() as u32);
        }
        for t in self.0.iter() {
            t.tail_append(enc);
        }
        enc.pop_offset();
    }

    fn decode_sequence(dec: &mut Decoder<'_>) -> AbiResult<Self> {
        let mut tokens = Vec::with_capacity(N);

        for _ in 0..N {
            let token = T::decode_from(dec)?;
            tokens.push(token);
        }

        match tokens.try_into() {
            Ok(tokens) => Ok(Self(tokens)),
            _ => panic!("vec has size n exactly"),
        }
    }
}

impl<T, const N: usize> FixedSeqToken<T, N> {
    /// Take the backing array, consuming the token
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    pub fn take_array(self) -> [T; N] {
        self.0
    }

    /// Get a reference to the array
    pub fn as_array(&self) -> &[T; N] {
        self.as_ref()
    }

    /// Get a reference to the backing array as a slice
    pub fn as_slice(&self) -> &[T] {
        self.as_array().as_slice()
    }
}

/// A Dynamic Sequence - `T[]`
#[derive(Debug, Clone, PartialEq)]
pub struct DynSeqToken<T>(Vec<T>);

impl<T> From<Vec<T>> for DynSeqToken<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> AsRef<[T]> for DynSeqToken<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> DynSeqToken<T> {
    /// Take the backing vec, consuming the tokey
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    pub fn take_vec(self) -> Vec<T> {
        self.0
    }

    /// Get a reference to the backing slice
    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T> AsRef<Vec<T>> for DynSeqToken<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T: TokenType> TokenType for DynSeqToken<T> {
    fn is_dynamic() -> bool {
        true
    }

    fn decode_from(dec: &mut Decoder<'_>) -> AbiResult<Self> {
        let mut child = dec.take_indirection()?;
        let len = child.take_u32()? as usize;

        let mut tokens = vec![];

        for _ in 0..len {
            let token = T::decode_from(&mut child)?;
            tokens.push(token);
        }

        Ok(DynSeqToken(tokens))
    }

    fn head_words(&self) -> usize {
        1
    }

    fn tail_words(&self) -> usize {
        1 + self.0.iter().map(TokenType::total_words).sum::<usize>()
    }

    fn head_append(&self, enc: &mut Encoder) {
        enc.append_indirection();
    }

    fn tail_append(&self, enc: &mut Encoder) {
        enc.append_seq_len(&self.0);
        self.encode_sequence(enc);
    }
}

impl<T> TokenSeq for DynSeqToken<T>
where
    T: TokenType,
{
    fn encode_sequence(&self, enc: &mut Encoder) {
        let head_words = self.0.iter().map(TokenType::head_words).sum::<usize>();
        enc.push_offset(head_words as u32);
        for t in self.0.iter() {
            t.head_append(enc);
            enc.bump_offset(t.tail_words() as u32);
        }
        for t in self.0.iter() {
            t.tail_append(enc);
        }
        enc.pop_offset();
    }

    fn decode_sequence(dec: &mut Decoder<'_>) -> AbiResult<Self> {
        Self::decode_from(dec)
    }
}

/// A Packed Sequence - `bytes` or `string`
#[derive(Clone, PartialEq)]
pub struct PackedSeqToken(Vec<u8>);

impl From<Vec<u8>> for PackedSeqToken {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for PackedSeqToken {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl PackedSeqToken {
    /// Get a reference to the backing slice
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Take the backing vec, consuming the token
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    pub fn take_vec(self) -> Vec<u8> {
        self.0
    }
}

impl TokenType for PackedSeqToken {
    fn is_dynamic() -> bool {
        true
    }

    fn decode_from(dec: &mut Decoder<'_>) -> AbiResult<Self> {
        let mut child = dec.take_indirection()?;
        let len = child.take_u32()? as usize;
        let bytes = child.peek_len(len)?;
        Ok(PackedSeqToken(bytes.to_vec()))
    }

    fn head_words(&self) -> usize {
        1
    }

    fn tail_words(&self) -> usize {
        // "1 +" because len is also appended
        1 + ((self.0.len() + 31) / 32)
    }

    fn head_append(&self, enc: &mut Encoder) {
        enc.append_indirection();
    }

    fn tail_append(&self, enc: &mut Encoder) {
        enc.append_packed_seq(&self.0)
    }
}

impl fmt::Debug for PackedSeqToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PackedSeq")
            .field(&hex::encode(&self.0))
            .finish()
    }
}

macro_rules! tuple_impls {
    () => {};
    (@peel $_:ident, $($other:ident,)*) => { tuple_impls! { $($other,)* } };
    ($($ty:ident,)+) => {
        impl<$($ty: TokenType,)+> sealed::Sealed for ($($ty,)+) {}

        #[allow(non_snake_case)]
        impl<$($ty: TokenType,)+> TokenType for ($($ty,)+) {
            fn is_dynamic() -> bool {
                $( <$ty as TokenType>::is_dynamic() )||+
            }

            fn decode_from(dec: &mut Decoder<'_>) -> AbiResult<Self> {
                let is_dynamic = Self::is_dynamic();
                // The first element in a dynamic Tuple is an offset to the Tuple's data
                // For a static Tuple the data begins right away
                let mut child = if is_dynamic {
                    dec.take_indirection()?
                } else {
                    dec.raw_child()
                };

                let res = Self::decode_sequence(&mut child)?;

                if !is_dynamic {
                    dec.take_offset(child);
                }

                Ok(res)
            }

            fn head_words(&self) -> usize {
                if Self::is_dynamic() {
                    1
                } else {
                    let ($(ref $ty,)+) = *self;
                    0 $( + $ty.head_words() )+
                }
            }

            fn tail_words(&self) -> usize {
                if Self::is_dynamic() {
                    let ($(ref $ty,)+) = *self;
                    0 $( + $ty.total_words() )+
                } else {
                    0
                }
            }

            fn total_words(&self) -> usize {
                let ($(ref $ty,)+) = *self;
                0 $( + $ty.total_words() )+
            }

            fn head_append(&self, enc: &mut Encoder) {
                if Self::is_dynamic() {
                    enc.append_indirection();
                } else {
                    let ($(ref $ty,)+) = *self;
                    $(
                        $ty.head_append(enc);
                    )+
                }
            }

            fn tail_append(&self, enc: &mut Encoder) {
                if Self::is_dynamic() {
                    let ($(ref $ty,)+) = *self;
                    let head_words = 0 $( + $ty.head_words() )+;

                    enc.push_offset(head_words as u32);
                    $(
                        $ty.head_append(enc);
                        enc.bump_offset($ty.tail_words() as u32);
                    )+
                    $(
                        $ty.tail_append(enc);
                    )+
                    enc.pop_offset();
                }
            }
        }

        #[allow(non_snake_case)]
        impl<$($ty: TokenType,)+> TokenSeq for ($($ty,)+) {
            fn can_be_params() -> bool {
                true
            }

            fn encode_sequence(&self, enc: &mut Encoder) {
                let ($(ref $ty,)+) = *self;
                let head_words = 0 $( + $ty.head_words() )+;
                enc.push_offset(head_words as u32);
                $(
                    $ty.head_append(enc);
                    enc.bump_offset($ty.tail_words() as u32);
                )+
                $(
                    $ty.tail_append(enc);
                )+
                enc.pop_offset();
            }

            fn decode_sequence(dec: &mut Decoder<'_>) -> AbiResult<Self> {
                Ok(($(
                    <$ty as TokenType>::decode_from(dec)?,
                )+))
            }
        }

        tuple_impls! { @peel $($ty,)+ }
    };
}

tuple_impls! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, }

#[cfg(test)]
mod tests {
    use ethers_primitives::B256;

    use super::*;
    #[cfg(not(feature = "std"))]
    use crate::no_std_prelude::*;
    use crate::{sol_type, SolType};

    macro_rules! assert_type_check {
        ($sol:ty, $token:expr $(,)?) => {
            assert!(<$sol>::type_check($token).is_ok())
        };
    }

    macro_rules! assert_not_type_check {
        ($sol:ty, $token:expr $(,)?) => {
            assert!(<$sol>::type_check($token).is_err())
        };
    }

    #[test]
    fn test_type_check() {
        assert_type_check!(
            (sol_type::Uint<256>, sol_type::Bool),
            &(WordToken(B256::default()), WordToken(B256::default())),
        );

        // TODO(tests): more like this where we test type check internal logic
        assert_not_type_check!(sol_type::Uint<8>, &Word::repeat_byte(0x11).into());
        assert_not_type_check!(sol_type::Bool, &B256::repeat_byte(0x11).into());
        assert_not_type_check!(sol_type::FixedBytes<31>, &B256::repeat_byte(0x11).into());

        assert_type_check!(
            (sol_type::Uint<32>, sol_type::Bool),
            &(WordToken(B256::default()), WordToken(B256::default())),
        );

        assert_type_check!(
            sol_type::Array<sol_type::Bool>,
            &DynSeqToken(vec![WordToken(B256::default()), WordToken(B256::default()),]),
        );

        assert_type_check!(
            sol_type::Array<sol_type::Bool>,
            &DynSeqToken(vec![WordToken(B256::default()), WordToken(B256::default()),]),
        );
        assert_type_check!(
            sol_type::Array<sol_type::Address>,
            &DynSeqToken(vec![WordToken(B256::default()), WordToken(B256::default()),]),
        );

        assert_type_check!(
            sol_type::FixedArray<sol_type::Bool, 2>,
            &FixedSeqToken::<_, 2>([
                WordToken(B256::default()),
                WordToken(B256::default()),
            ]),
        );

        assert_type_check!(
            sol_type::FixedArray<sol_type::Address, 2>,
            &FixedSeqToken::<_, 2>([
                WordToken(B256::default()),
                WordToken(B256::default()),
            ]),
        );
    }
}

mod sealed {
    use super::*;
    pub trait Sealed {}
    impl Sealed for WordToken {}
    impl<T, const N: usize> Sealed for FixedSeqToken<T, N> {}
    impl<T> Sealed for DynSeqToken<T> {}
    impl Sealed for PackedSeqToken {}
}
