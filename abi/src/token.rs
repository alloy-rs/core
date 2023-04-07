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

use ethers_primitives::{B160, U256};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{decoder::Decoder, encoder::Encoder, AbiResult, Word};

/// Abi-Encoding Tokens. This is a sealed trait. It contains the type
/// information necessary to encode & decode data. Tokens are an intermediate
/// state between abi-encoded blobs, and rust types.
pub trait TokenType: sealed::Sealed + Sized + Clone {
    /// True if the token represents a dynamically-sized type
    fn is_dynamic() -> bool;

    /// Decode a token from a decoder
    fn decode_from(dec: &mut Decoder) -> AbiResult<Self>;

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

pub trait TokenSeq: TokenType {
    /// True for tuples only.
    fn can_be_params() -> bool {
        false
    }
    fn encode_sequence(&self, enc: &mut Encoder);

    fn decode_sequence(dec: &mut Decoder) -> AbiResult<Self>;
}

/// A single EVM word - T for any value type
#[derive(Debug, Clone, PartialEq, Default)]
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
        Self(value.into())
    }
}

impl From<B160> for WordToken {
    fn from(value: B160) -> Self {
        Self(value.into())
    }
}

impl From<[u8; 20]> for WordToken {
    fn from(value: [u8; 20]) -> Self {
        Self(B160::from(value).into())
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
    pub fn inner(&self) -> Word {
        self.0
    }
}

impl TokenType for WordToken {
    fn is_dynamic() -> bool {
        false
    }

    fn decode_from(dec: &mut Decoder) -> AbiResult<Self> {
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

/// A Fixed Sequence - T\[M\]
#[derive(Debug, Clone, PartialEq)]
pub struct FixedSeqToken<T, const N: usize>([T; N]);

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

    fn decode_from(dec: &mut Decoder) -> AbiResult<Self> {
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

    fn decode_sequence(dec: &mut Decoder) -> AbiResult<Self> {
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

/// A Dynamic Sequence - T[]
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

    fn decode_from(dec: &mut Decoder) -> AbiResult<Self> {
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

    fn decode_sequence(dec: &mut Decoder) -> AbiResult<Self> {
        Self::decode_from(dec)
    }
}

/// A Packed Sequence - bytes or string
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
    pub fn take_vec(self) -> Vec<u8> {
        self.0
    }
}

impl TokenType for PackedSeqToken {
    fn is_dynamic() -> bool {
        true
    }

    fn decode_from(dec: &mut Decoder) -> AbiResult<Self> {
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

macro_rules! impl_tuple_token_type {
    ($num:expr, $( $ty:ident : $no:tt ),+ $(,)?) => {
        impl<$($ty,)+> sealed::Sealed for ($( $ty, )+)
        where
        $(
            $ty: TokenType,
        )+
        {}

        impl<$($ty,)+> TokenType for ($( $ty, )+)
        where
            $(
                $ty: TokenType,
            )+
        {
            fn is_dynamic() -> bool {
                $(
                    if $ty::is_dynamic() {
                        return true;
                    }
                )+
                false
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
                    let mut sum = 0;
                    $(
                        sum += self.$no.head_words();
                    )+
                    sum
                }
            }

            fn tail_words(&self) -> usize {
                let mut sum = 0;
                if Self::is_dynamic() {
                    $(
                        sum += self.$no.total_words();
                    )+
                }
                sum
            }

            fn total_words(&self) -> usize {
                let mut sum = 0;
                $(
                    sum += self.$no.total_words();
                )+
                sum
            }

            fn head_append(&self, enc: &mut Encoder) {
                if Self::is_dynamic() {
                    enc.append_indirection();
                } else {
                    $(
                        self.$no.head_append(enc);
                    )+
                }
            }

            fn tail_append(&self, enc: &mut Encoder) {
                if Self::is_dynamic() {
                    let mut head_words = 0;
                    $(
                        head_words += self.$no.head_words();
                    )+
                    enc.push_offset(head_words as u32);
                    $(
                        self.$no.head_append(enc);
                        enc.bump_offset(self.$no.tail_words() as u32);
                    )+
                    $(
                        self.$no.tail_append(enc);
                    )+
                    enc.pop_offset();
                }
            }
        }

        impl<$($ty,)+> TokenSeq for ($( $ty, )+)
        where
            $(
                $ty: TokenType,
            )+
        {

            fn can_be_params() -> bool {
                true
            }

            fn encode_sequence(&self, enc: &mut Encoder) {
                let mut head_words = 0;
                $(
                    head_words += self.$no.head_words();
                )+
                enc.push_offset(head_words as u32);
                $(
                    self.$no.head_append(enc);
                    enc.bump_offset(self.$no.tail_words() as u32);
                )+
                $(
                    self.$no.tail_append(enc);
                )+
                enc.pop_offset();
            }

            fn decode_sequence(dec: &mut Decoder) -> AbiResult<Self> {
                let res = (
                    $($ty::decode_from(dec)?,)+
                );
                Ok(res)
            }
        }
    }
}

impl_tuple_token_type!(1, A:0, );
impl_tuple_token_type!(2, A:0, B:1, );
impl_tuple_token_type!(3, A:0, B:1, C:2, );
impl_tuple_token_type!(4, A:0, B:1, C:2, D:3, );
impl_tuple_token_type!(5, A:0, B:1, C:2, D:3, E:4, );
impl_tuple_token_type!(6, A:0, B:1, C:2, D:3, E:4, F:5, );
impl_tuple_token_type!(7, A:0, B:1, C:2, D:3, E:4, F:5, G:6, );
impl_tuple_token_type!(8, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, );
impl_tuple_token_type!(9, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, );
impl_tuple_token_type!(10, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, );
impl_tuple_token_type!(11, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, );
impl_tuple_token_type!(12, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, );
impl_tuple_token_type!(13, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, );
impl_tuple_token_type!(14, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, );
impl_tuple_token_type!(15, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, );
impl_tuple_token_type!(16, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, );
impl_tuple_token_type!(17, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16,);
impl_tuple_token_type!(18, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17,);
impl_tuple_token_type!(19, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18,);
impl_tuple_token_type!(20, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19,);
impl_tuple_token_type!(21, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19, U:20,);

#[cfg(test)]
mod tests {
    use ethers_primitives::B256;

    use super::*;
    #[cfg(not(feature = "std"))]
    use crate::no_std_prelude::*;
    use crate::{sol_type, SolType};

    macro_rules! assert_type_check {
        ($sol:ty, $token:expr) => {
            assert!(<$sol>::type_check($token).is_ok())
        };
        ($sol:ty, $token:expr,) => {
            assert_type_check!($sol, $token)
        };
    }

    macro_rules! assert_not_type_check {
        ($sol:ty, $token:expr) => {
            assert!(<$sol>::type_check($token).is_err())
        };
        ($sol:ty, $token:expr,) => {
            assert_not_type_check!($sol, $token)
        };
    }

    #[test]
    fn test_type_check() {
        assert_type_check!(
            (sol_type::Uint<256>, sol_type::Bool),
            &(WordToken(B256::default()), WordToken(B256::default())),
        );

        // TODO: more like this where we test type check internal logic
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
