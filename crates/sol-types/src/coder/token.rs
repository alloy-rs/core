// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors

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

use crate::{no_std_prelude::*, Decoder, Encoder, Result, Word};
use alloy_primitives::{Address, U256};
use core::fmt;

mod sealed {
    use super::*;

    pub trait Sealed {}
    impl Sealed for WordToken {}
    impl Sealed for () {}
    impl<T, const N: usize> Sealed for FixedSeqToken<T, N> {}
    impl<T> Sealed for DynSeqToken<T> {}
    impl Sealed for PackedSeqToken {}
}

use sealed::Sealed;

/// Abi-Encoding Tokens. This is a sealed trait. It contains the type
/// information necessary to encode & decode data. Tokens are an intermediate
/// state between abi-encoded blobs, and rust types.
pub trait TokenType: Sealed + Sized {
    /// True if the token represents a dynamically-sized type.
    fn is_dynamic() -> bool;

    /// Decode a token from a decoder.
    fn decode_from(dec: &mut Decoder<'_>) -> Result<Self>;

    /// Calculate the number of head words.
    fn head_words(&self) -> usize;

    /// Calculate the number of tail words.
    fn tail_words(&self) -> usize;

    /// Calculate the total number of head and tail words.
    #[inline]
    fn total_words(&self) -> usize {
        self.head_words() + self.tail_words()
    }

    /// Append head words to the encoder.
    fn head_append(&self, enc: &mut Encoder);

    /// Append tail words to the encoder.
    fn tail_append(&self, enc: &mut Encoder);
}

/// A token composed of a sequence of other tokens
///
/// This functions as an extension trait for [`TokenType`], and may only be
/// implemented by [`FixedSeqToken`], [`DynSeqToken`], and [`PackedSeqToken`].
pub trait TokenSeq: TokenType {
    /// True for tuples only.
    const IS_TUPLE: bool = false;

    /// ABI-encode the token sequence into the encoder.
    fn encode_sequence(&self, enc: &mut Encoder);

    /// ABI-decode the token sequence from the encoder.
    fn decode_sequence(dec: &mut Decoder<'_>) -> Result<Self>;
}

/// A single EVM word - T for any value type.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WordToken(pub Word);

impl From<Word> for WordToken {
    #[inline]
    fn from(value: Word) -> Self {
        Self(value)
    }
}

impl From<bool> for WordToken {
    #[inline]
    fn from(value: bool) -> Self {
        U256::from(value as usize).into()
    }
}

impl From<U256> for WordToken {
    #[inline]
    fn from(value: U256) -> Self {
        Self(value.to_be_bytes().into())
    }
}

impl From<Address> for WordToken {
    #[inline]
    fn from(value: Address) -> Self {
        Self(value.into())
    }
}

impl From<[u8; 20]> for WordToken {
    #[inline]
    fn from(value: [u8; 20]) -> Self {
        Self(Address::from(value).into())
    }
}

impl From<WordToken> for [u8; 32] {
    #[inline]
    fn from(value: WordToken) -> [u8; 32] {
        value.0.into()
    }
}

impl From<[u8; 32]> for WordToken {
    #[inline]
    fn from(value: [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl AsRef<Word> for WordToken {
    #[inline]
    fn as_ref(&self) -> &Word {
        &self.0
    }
}

impl AsRef<[u8]> for WordToken {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
}

impl TokenType for WordToken {
    #[inline]
    fn is_dynamic() -> bool {
        false
    }

    #[inline]
    fn decode_from(dec: &mut Decoder<'_>) -> Result<Self> {
        dec.take_word().map(Self)
    }

    #[inline]
    fn head_words(&self) -> usize {
        1
    }

    #[inline]
    fn tail_words(&self) -> usize {
        0
    }

    #[inline]
    fn head_append(&self, enc: &mut Encoder) {
        enc.append_word(self.inner());
    }

    #[inline]
    fn tail_append(&self, _enc: &mut Encoder) {}
}

impl WordToken {
    /// Returns a reference to the word as a slice.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.0 .0
    }

    /// Copy the inner word.
    #[inline]
    pub const fn inner(self) -> Word {
        self.0
    }
}

/// A Fixed Sequence - `T[N]`
#[derive(Clone, Debug, PartialEq)]
pub struct FixedSeqToken<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> TryFrom<Vec<T>> for FixedSeqToken<T, N> {
    type Error = <[T; N] as TryFrom<Vec<T>>>::Error;

    #[inline]
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        <[T; N]>::try_from(value).map(Self)
    }
}

impl<T, const N: usize> From<[T; N]> for FixedSeqToken<T, N> {
    #[inline]
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> AsRef<[T; N]> for FixedSeqToken<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

impl<T: TokenType, const N: usize> TokenType for FixedSeqToken<T, N> {
    #[inline]
    fn is_dynamic() -> bool {
        T::is_dynamic()
    }

    #[inline]
    fn decode_from(dec: &mut Decoder<'_>) -> Result<Self> {
        let is_dynamic = Self::is_dynamic();

        let mut child = if is_dynamic {
            dec.take_indirection()?
        } else {
            dec.raw_child()
        };

        Self::decode_sequence(&mut child)
    }

    #[inline]
    fn head_words(&self) -> usize {
        if Self::is_dynamic() {
            1
        } else {
            self.0.iter().map(TokenType::head_words).sum()
        }
    }

    #[inline]
    fn tail_words(&self) -> usize {
        if Self::is_dynamic() {
            N
        } else {
            0
        }
    }

    #[inline]
    fn head_append(&self, enc: &mut Encoder) {
        if Self::is_dynamic() {
            enc.append_indirection();
        } else {
            self.0.iter().for_each(|inner| inner.head_append(enc))
        }
    }

    #[inline]
    fn tail_append(&self, enc: &mut Encoder) {
        if Self::is_dynamic() {
            self.encode_sequence(enc)
        }
    }
}

impl<T: TokenType, const N: usize> TokenSeq for FixedSeqToken<T, N> {
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

    fn decode_sequence(dec: &mut Decoder<'_>) -> Result<Self> {
        // TODO: None of this is necessary if `core::array::try_from_fn` is stabilized.
        // core::array::try_from_fn(|_| T::decode_from(dec)).map(Self)
        super::impl_core::try_from_fn(|_| T::decode_from(dec)).map(Self)
    }
}

impl<T, const N: usize> FixedSeqToken<T, N> {
    /// Take the backing array, consuming the token.
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn into_array(self) -> [T; N] {
        self.0
    }

    /// Returns a reference to the array.
    #[inline]
    pub const fn as_array(&self) -> &[T; N] {
        &self.0
    }

    /// Returns a reference to the array as a slice.
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        &self.0
    }
}

/// A Dynamic Sequence - `T[]`
#[derive(Clone, Debug, PartialEq)]
pub struct DynSeqToken<T>(pub Vec<T>);

impl<T> From<Vec<T>> for DynSeqToken<T> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> AsRef<[T]> for DynSeqToken<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T: TokenType> TokenType for DynSeqToken<T> {
    #[inline]
    fn is_dynamic() -> bool {
        true
    }

    fn decode_from(dec: &mut Decoder<'_>) -> Result<Self> {
        let mut child = dec.take_indirection()?;
        let len = child.take_u32()? as usize;
        (0..len)
            .map(|_| T::decode_from(&mut child))
            .collect::<Result<Vec<T>>>()
            .map(DynSeqToken)
    }

    #[inline]
    fn head_words(&self) -> usize {
        1
    }

    #[inline]
    fn tail_words(&self) -> usize {
        1 + self.0.iter().map(TokenType::total_words).sum::<usize>()
    }

    #[inline]
    fn head_append(&self, enc: &mut Encoder) {
        enc.append_indirection();
    }

    #[inline]
    fn tail_append(&self, enc: &mut Encoder) {
        enc.append_seq_len(&self.0);
        self.encode_sequence(enc);
    }
}

impl<T: TokenType> TokenSeq for DynSeqToken<T> {
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

    #[inline]
    fn decode_sequence(dec: &mut Decoder<'_>) -> Result<Self> {
        Self::decode_from(dec)
    }
}

impl<T> DynSeqToken<T> {
    /// Converts the sequence into the vector.
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Returns a reference to the backing slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
}

/// A Packed Sequence - `bytes` or `string`
#[derive(Clone, PartialEq)]
pub struct PackedSeqToken(pub Vec<u8>);

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

impl TokenType for PackedSeqToken {
    #[inline]
    fn is_dynamic() -> bool {
        true
    }

    #[inline]
    fn decode_from(dec: &mut Decoder<'_>) -> Result<Self> {
        let mut child = dec.take_indirection()?;
        let len = child.take_u32()? as usize;
        let bytes = child.peek_len(len)?;
        Ok(PackedSeqToken(bytes.to_vec()))
    }

    #[inline]
    fn head_words(&self) -> usize {
        1
    }

    #[inline]
    fn tail_words(&self) -> usize {
        // "1 +" because len is also appended
        1 + ((self.0.len() + 31) / 32)
    }

    #[inline]
    fn head_append(&self, enc: &mut Encoder) {
        enc.append_indirection();
    }

    #[inline]
    fn tail_append(&self, enc: &mut Encoder) {
        enc.append_packed_seq(&self.0)
    }
}

impl fmt::Debug for PackedSeqToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PackedSeq")
            .field(&hex::encode_prefixed(&self.0))
            .finish()
    }
}

impl PackedSeqToken {
    /// Consumes `self` to return the underlying vector.
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    /// Returns a reference to the slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

macro_rules! tuple_impls {
    () => {};
    (@peel $_:ident, $($other:ident,)*) => { tuple_impls! { $($other,)* } };
    ($($ty:ident,)+) => {
        impl<$($ty: TokenType,)+> Sealed for ($($ty,)+) {}

        #[allow(non_snake_case)]
        impl<$($ty: TokenType,)+> TokenType for ($($ty,)+) {
            #[inline]
            fn is_dynamic() -> bool {
                $( <$ty as TokenType>::is_dynamic() )||+
            }

            #[inline]
            fn decode_from(dec: &mut Decoder<'_>) -> Result<Self> {
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

            #[inline]
            fn head_words(&self) -> usize {
                if Self::is_dynamic() {
                    1
                } else {
                    let ($(ref $ty,)+) = *self;
                    0 $( + $ty.head_words() )+
                }
            }

            #[inline]
            fn tail_words(&self) -> usize {
                if Self::is_dynamic() {
                    self.total_words()
                } else {
                    0
                }
            }

            #[inline]
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
            const IS_TUPLE: bool = true;

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

            fn decode_sequence(dec: &mut Decoder<'_>) -> Result<Self> {
                Ok(($(
                    <$ty as TokenType>::decode_from(dec)?,
                )+))
            }
        }

        tuple_impls! { @peel $($ty,)+ }
    };
}

tuple_impls! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, }

impl TokenType for () {
    #[inline]
    fn is_dynamic() -> bool {
        false
    }

    #[inline]
    fn decode_from(_dec: &mut Decoder<'_>) -> Result<Self> {
        Ok(())
    }

    #[inline]
    fn head_words(&self) -> usize {
        0
    }

    #[inline]
    fn tail_words(&self) -> usize {
        0
    }

    #[inline]
    fn head_append(&self, _enc: &mut Encoder) {}

    #[inline]
    fn tail_append(&self, _enc: &mut Encoder) {}
}

impl TokenSeq for () {
    const IS_TUPLE: bool = true;

    #[inline]
    fn encode_sequence(&self, _enc: &mut Encoder) {}

    #[inline]
    fn decode_sequence(_dec: &mut Decoder<'_>) -> Result<Self> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sol_data, SolType};
    use alloy_primitives::B256;

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
            (sol_data::Uint<256>, sol_data::Bool),
            &(WordToken(B256::default()), WordToken(B256::default())),
        );

        // TODO(tests): more like this where we test type check internal logic
        assert_not_type_check!(sol_data::Uint<8>, &Word::repeat_byte(0x11).into());
        assert_not_type_check!(sol_data::Bool, &B256::repeat_byte(0x11).into());
        assert_not_type_check!(sol_data::FixedBytes<31>, &B256::repeat_byte(0x11).into());

        assert_type_check!(
            (sol_data::Uint<32>, sol_data::Bool),
            &(WordToken(B256::default()), WordToken(B256::default())),
        );

        assert_type_check!(
            sol_data::Array<sol_data::Bool>,
            &DynSeqToken(vec![WordToken(B256::default()), WordToken(B256::default()),]),
        );

        assert_type_check!(
            sol_data::Array<sol_data::Bool>,
            &DynSeqToken(vec![WordToken(B256::default()), WordToken(B256::default()),]),
        );
        assert_type_check!(
            sol_data::Array<sol_data::Address>,
            &DynSeqToken(vec![WordToken(B256::default()), WordToken(B256::default()),]),
        );

        assert_type_check!(
            sol_data::FixedArray<sol_data::Bool, 2>,
            &FixedSeqToken::<_, 2>([
                WordToken(B256::default()),
                WordToken(B256::default()),
            ]),
        );

        assert_type_check!(
            sol_data::FixedArray<sol_data::Address, 2>,
            &FixedSeqToken::<_, 2>([
                WordToken(B256::default()),
                WordToken(B256::default()),
            ]),
        );
    }
}
