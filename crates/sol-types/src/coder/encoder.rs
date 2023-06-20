// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    no_std_prelude::*,
    token::TokenSeq,
    util::{pad_u32, words_for},
    TokenType, Word,
};
use core::mem;

/// An ABI encoder.
///
/// This is not intended for public consumption. It should be used only by the
/// token types. If you have found yourself here, you probably want to use the
/// high-level [`crate::SolType`] interface (or its dynamic equivalent) instead.
#[derive(Default, Clone, Debug)]
pub struct Encoder {
    buf: Vec<Word>,
    suffix_offset: Vec<u32>,
}

impl Encoder {
    /// Instantiate a new empty encoder.
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: Vec::new(),
            suffix_offset: Vec::new(),
        }
    }

    /// Instantiate a new encoder with a given capacity in words.
    #[inline]
    pub fn with_capacity(size: usize) -> Self {
        Self {
            buf: Vec::with_capacity(size + 1),
            suffix_offset: vec![],
        }
    }

    /// Finish the encoding process, returning the encoded words.
    ///
    /// Use `into_bytes` instead to flatten the words into bytes.
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn finish(self) -> Vec<Word> {
        self.buf
    }

    /// Finish the encoding process, returning the encoded bytes.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        // TODO: remove once `Vec::into_flattened` is stabilized.
        // unsafe { mem::transmute::<Vec<_>, Vec<[u8; 32]>>(self.buf).into_flattened() }

        // SAFETY: `#[repr(transparent)] FixedBytes<N>([u8; N])`
        unsafe { super::impl_core::into_flattened::<_, 32>(mem::transmute(self.buf)) }
    }

    /// Determine the current suffix offset.
    #[inline]
    pub fn suffix_offset(&self) -> u32 {
        *self.suffix_offset.last().unwrap()
    }

    /// Appends a suffix offset.
    #[inline]
    pub fn push_offset(&mut self, words: u32) {
        self.suffix_offset.push(words * 32);
    }

    /// Removes the last offset and returns it.
    #[inline]
    pub fn pop_offset(&mut self) -> Option<u32> {
        self.suffix_offset.pop()
    }

    /// Bump the suffix offset by a given number of words.
    #[inline]
    pub fn bump_offset(&mut self, words: u32) {
        if let Some(last) = self.suffix_offset.last_mut() {
            *last += words * 32;
        }
    }

    /// Append a word to the encoder.
    #[inline]
    pub fn append_word(&mut self, word: Word) {
        self.buf.push(word);
    }

    /// Append a pointer to the current suffix offset.
    #[inline]
    pub fn append_indirection(&mut self) {
        self.append_word(pad_u32(self.suffix_offset()));
    }

    /// Append a sequence length.
    #[inline]
    pub fn append_seq_len<T>(&mut self, seq: &[T]) {
        self.append_word(pad_u32(seq.len() as u32));
    }

    /// Append a sequence of bytes, padding to the next word.
    #[inline]
    fn append_bytes(&mut self, bytes: &[u8]) {
        let len = words_for(bytes);
        for i in 0..len {
            let mut padded = Word::ZERO;

            let to_copy = match i == len - 1 {
                false => 32,
                true => match bytes.len() % 32 {
                    0 => 32,
                    x => x,
                },
            };

            let offset = 32 * i;
            padded[..to_copy].copy_from_slice(&bytes[offset..offset + to_copy]);
            self.append_word(padded);
        }
    }

    /// Append a sequence of bytes as a packed sequence with a length prefix.
    #[inline]
    pub fn append_packed_seq(&mut self, bytes: &[u8]) {
        self.append_seq_len(bytes);
        self.append_bytes(bytes);
    }

    /// Shortcut for appending a token sequence.
    #[inline]
    pub fn append_head_tail<'a, T: TokenSeq<'a>>(&mut self, token: &T) {
        token.encode_sequence(self);
    }
}

/// ABI-encode a token sequence.
pub fn encode<'a, T: TokenSeq<'a>>(tokens: &T) -> Vec<u8> {
    let mut enc = Encoder::with_capacity(tokens.total_words());
    enc.append_head_tail(tokens);
    enc.into_bytes()
}

/// ABI-encode a single token.
#[inline]
pub fn encode_single<'a, T: TokenType<'a>>(token: &T) -> Vec<u8> {
    // Same as [`core::array::from_ref`].
    // SAFETY: Converting `&T` to `&(T,)` is sound.
    encode::<(T,)>(unsafe { &*(token as *const T).cast::<(T,)>() })
}

/// Encode a tuple as ABI function params, suitable for passing to a function.
#[inline]
pub fn encode_params<'a, T: TokenSeq<'a>>(token: &T) -> Vec<u8> {
    if T::IS_TUPLE {
        encode(token)
    } else {
        encode_single(token)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, U256};
    use hex_literal::hex;

    #[cfg(not(feature = "std"))]
    use crate::no_std_prelude::*;
    use crate::{sol_data, SolType};

    #[test]
    fn encode_address() {
        let address = Address::from([0x11u8; 20]);
        let expected = hex!("0000000000000000000000001111111111111111111111111111111111111111");
        let encoded = sol_data::Address::encode_single(&address);
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), sol_data::Address::encoded_size(&address));
    }

    #[test]
    fn encode_dynamic_array_of_addresses() {
        type MyTy = sol_data::Array<sol_data::Address>;
        let data = vec![Address::from([0x11u8; 20]), Address::from([0x22u8; 20])];
        let encoded = MyTy::encode_single(&data);
        let expected = hex!(
            "
			0000000000000000000000000000000000000000000000000000000000000020
			0000000000000000000000000000000000000000000000000000000000000002
			0000000000000000000000001111111111111111111111111111111111111111
			0000000000000000000000002222222222222222222222222222222222222222
		"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_fixed_array_of_addresses() {
        type MyTy = sol_data::FixedArray<sol_data::Address, 2>;

        let addresses = [Address::from([0x11u8; 20]), Address::from([0x22u8; 20])];

        let encoded = MyTy::encode_single(&addresses);
        let encoded_params = MyTy::encode_params(&addresses);
        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), MyTy::encoded_size(&addresses));
    }

    #[test]
    fn encode_two_addresses() {
        type MyTy = (sol_data::Address, sol_data::Address);
        let addresses = (Address::from([0x11u8; 20]), Address::from([0x22u8; 20]));

        let encoded = MyTy::encode(&addresses);
        let encoded_params = MyTy::encode_params(&addresses);
        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&addresses));
    }

    #[test]
    fn encode_fixed_array_of_dynamic_array_of_addresses() {
        type MyTy = sol_data::FixedArray<sol_data::Array<sol_data::Address>, 2>;
        let data = [
            vec![Address::from([0x11u8; 20]), Address::from([0x22u8; 20])],
            vec![Address::from([0x33u8; 20]), Address::from([0x44u8; 20])],
        ];

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        )
        .to_vec();
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);

        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_dynamic_array_of_fixed_array_of_addresses() {
        type TwoAddrs = sol_data::FixedArray<sol_data::Address, 2>;
        type MyTy = sol_data::Array<TwoAddrs>;

        let data = vec![
            [Address::from([0x11u8; 20]), Address::from([0x22u8; 20])],
            [Address::from([0x33u8; 20]), Address::from([0x44u8; 20])],
        ];

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        )
        .to_vec();
        // a DynSeq at top level ALWAYS has extra indirection
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_dynamic_array_of_dynamic_arrays() {
        type MyTy = sol_data::Array<sol_data::Array<sol_data::Address>>;

        let data = vec![
            vec![Address::from([0x11u8; 20])],
            vec![Address::from([0x22u8; 20])],
        ];

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        // a DynSeq at top level ALWAYS has extra indirection
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_dynamic_array_of_dynamic_arrays2() {
        type MyTy = sol_data::Array<sol_data::Array<sol_data::Address>>;

        let data = vec![
            vec![Address::from([0x11u8; 20]), Address::from([0x22u8; 20])],
            vec![Address::from([0x33u8; 20]), Address::from([0x44u8; 20])],
        ];
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        )
        .to_vec();
        // a DynSeq at top level ALWAYS has extra indirection
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_fixed_array_of_fixed_arrays() {
        type MyTy = sol_data::FixedArray<sol_data::FixedArray<sol_data::Address, 2>, 2>;

        let fixed = [
            [Address::from([0x11u8; 20]), Address::from([0x22u8; 20])],
            [Address::from([0x33u8; 20]), Address::from([0x44u8; 20])],
        ];

        let encoded = MyTy::encode(&fixed);
        let encoded_params = MyTy::encode_params(&fixed);
        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        )
        .to_vec();
        // a non-dynamic FixedSeq at top level NEVER has extra indirection
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded.len(), MyTy::encoded_size(&fixed));
    }

    #[test]
    fn encode_fixed_array_of_static_tuples_followed_by_dynamic_type() {
        type Tup = (sol_data::Uint<256>, sol_data::Uint<256>, sol_data::Address);
        type Fixed = sol_data::FixedArray<Tup, 2>;
        type MyTy = (Fixed, sol_data::String);

        let data = (
            [
                (
                    U256::from(93523141),
                    U256::from(352332135),
                    Address::from([0x44u8; 20]),
                ),
                (
                    U256::from(12411),
                    U256::from(451),
                    Address::from([0x22u8; 20]),
                ),
            ],
            "gavofyork".to_string(),
        );

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000005930cc5
    		0000000000000000000000000000000000000000000000000000000015002967
    		0000000000000000000000004444444444444444444444444444444444444444
    		000000000000000000000000000000000000000000000000000000000000307b
    		00000000000000000000000000000000000000000000000000000000000001c3
    		0000000000000000000000002222222222222222222222222222222222222222
    		00000000000000000000000000000000000000000000000000000000000000e0
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();

        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_empty_array() {
        type MyTy0 = sol_data::Array<sol_data::Address>;

        let data = vec![];

        // Empty arrays
        let encoded = MyTy0::encode_params(&data);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000000
    	    "
        )
        .to_vec();

        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), <(MyTy0,)>::encoded_size(&(data,)));

        type MyTy = (
            sol_data::Array<sol_data::Address>,
            sol_data::Array<sol_data::Address>,
        );
        let data = (vec![], vec![]);

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000060
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000000
    	    "
        )
        .to_vec();

        // Empty arrays
        let encoded = MyTy::encode_single(&data);
        assert_ne!(encoded, expected);

        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));

        type MyTy2 = (
            sol_data::Array<sol_data::Array<sol_data::Address>>,
            sol_data::Array<sol_data::Array<sol_data::Address>>,
        );

        let data = (vec![vec![]], vec![vec![]]);

        // Nested empty arrays
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // A Dynamic FixedSeq may be a top-level sequence to `encode` or may
        // itself be an item in a top-level sequence. Which is to say, it could
        // be (as `encode(T)` or `encode((T,))`). This test was `encode(T)`
        let encoded = MyTy2::encode_single(&data);
        assert_ne!(encoded, expected);
        let encoded_params = MyTy2::encode_params(&data);

        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy2::encoded_size(&data));
    }

    #[test]
    fn encode_bytes() {
        type MyTy = sol_data::Bytes;
        let bytes = vec![0x12, 0x34];

        let encoded = MyTy::encode_single(&bytes);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		1234000000000000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(bytes,)));
    }

    #[test]
    fn encode_fixed_bytes() {
        let encoded = sol_data::FixedBytes::<2>::encode_single(&[0x12, 0x34]);
        let expected = hex!("1234000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(encoded, expected);
        assert_eq!(
            encoded.len(),
            sol_data::FixedBytes::<2>::encoded_size(&[0x12, 0x34])
        );
    }

    #[test]
    fn encode_string() {
        let s = "gavofyork".to_string();
        let encoded = sol_data::String::encode_single(&s);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), <(sol_data::String,)>::encoded_size(&(s,)));
    }

    #[test]
    fn encode_bytes2() {
        let bytes = hex!("10000000000000000000000000000000000000000000000000000000000002").to_vec();
        let encoded = sol_data::Bytes::encode_single(&bytes);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		000000000000000000000000000000000000000000000000000000000000001f
    		1000000000000000000000000000000000000000000000000000000000000200
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), <(sol_data::Bytes,)>::encoded_size(&(bytes,)));
    }

    #[test]
    fn encode_bytes3() {
        let bytes = hex!(
            "
    		1000000000000000000000000000000000000000000000000000000000000000
    		1000000000000000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        let encoded = sol_data::Bytes::encode_single(&bytes);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		1000000000000000000000000000000000000000000000000000000000000000
    		1000000000000000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), <(sol_data::Bytes,)>::encoded_size(&(bytes,)));
    }

    #[test]
    fn encode_two_bytes() {
        type MyTy = (sol_data::Bytes, sol_data::Bytes);

        let bytes = (
            hex!("10000000000000000000000000000000000000000000000000000000000002").to_vec(),
            hex!("0010000000000000000000000000000000000000000000000000000000000002").to_vec(),
        );
        let encoded = MyTy::encode_single(&bytes);
        let encoded_params = MyTy::encode_params(&bytes);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		000000000000000000000000000000000000000000000000000000000000001f
    		1000000000000000000000000000000000000000000000000000000000000200
    		0000000000000000000000000000000000000000000000000000000000000020
    		0010000000000000000000000000000000000000000000000000000000000002
    	"
        )
        .to_vec();
        // A Dynamic FixedSeq may be a top-level sequence to `encode` or may
        // itself be an item in a top-level sequence. Which is to say, it could
        // be (as `encode(T)` or `encode((T,))`). This test was `encode(T)`
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&bytes));
    }

    #[test]
    fn encode_uint() {
        let uint = 4;
        let encoded = sol_data::Uint::<8>::encode_single(&uint);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000004");
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), sol_data::Uint::<8>::encoded_size(&uint));
    }

    #[test]
    fn encode_int() {
        let int = 4;
        let encoded = sol_data::Int::<8>::encode_single(&int);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000004");
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), sol_data::Int::<8>::encoded_size(&int));
    }

    #[test]
    fn encode_bool() {
        let encoded = sol_data::Bool::encode_single(&true);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), sol_data::Bool::encoded_size(&true));
    }

    #[test]
    fn encode_bool2() {
        let encoded = sol_data::Bool::encode_single(&false);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), sol_data::Bool::encoded_size(&false));
    }

    #[test]
    fn comprehensive_test() {
        type MyTy = (
            sol_data::Uint<8>,
            sol_data::Bytes,
            sol_data::Uint<8>,
            sol_data::Bytes,
        );

        let bytes = hex!(
            "
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    	"
        )
        .to_vec();

        let data = (5, bytes.clone(), 3, bytes);

        let encoded = MyTy::encode_single(&data);
        let encoded_params = MyTy::encode_params(&data);

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000005
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000003
    		00000000000000000000000000000000000000000000000000000000000000e0
    		0000000000000000000000000000000000000000000000000000000000000040
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    		0000000000000000000000000000000000000000000000000000000000000040
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    	"
        )
        .to_vec();
        // A Dynamic FixedSeq may be a top-level sequence to `encode` or may
        // itself be an item in a top-level sequence. Which is to say, it could
        // be (as `encode(T)` or `encode((T,))`). This test was `encode(T)`
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn comprehensive_test2() {
        type MyTy = (
            sol_data::Bool,
            sol_data::String,
            sol_data::Uint<8>,
            sol_data::Uint<8>,
            sol_data::Uint<8>,
            sol_data::Array<sol_data::Uint<8>>,
        );

        let data = (true, "gavofyork".to_string(), 2, 3, 4, vec![5, 6, 7]);

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000001
    		00000000000000000000000000000000000000000000000000000000000000c0
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000003
    		0000000000000000000000000000000000000000000000000000000000000004
    		0000000000000000000000000000000000000000000000000000000000000100
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000003
    		0000000000000000000000000000000000000000000000000000000000000005
    		0000000000000000000000000000000000000000000000000000000000000006
    		0000000000000000000000000000000000000000000000000000000000000007
    	"
        )
        .to_vec();
        // A Dynamic FixedSeq may be a top-level sequence to `encode` or may
        // itself be an item in a top-level sequence. Which is to say, it could
        // be (as `encode(T)` or `encode((T,))`). This test was `encode(T)`
        let encoded = MyTy::encode_single(&data);
        assert_ne!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_dynamic_array_of_bytes() {
        type MyTy = sol_data::Array<sol_data::Bytes>;
        let data = vec![hex!(
            "019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a4218a274f00d607"
        )
        .to_vec()];

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000026
    		019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a421
    		8a274f00d6070000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a DynSeq at top level ALWAYS has extra indirection
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_dynamic_array_of_bytes2() {
        type MyTy = sol_data::Array<sol_data::Bytes>;

        let data = vec![
            hex!("4444444444444444444444444444444444444444444444444444444444444444444444444444")
                .to_vec(),
            hex!("6666666666666666666666666666666666666666666666666666666666666666666666666666")
                .to_vec(),
        ];

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000026
    		4444444444444444444444444444444444444444444444444444444444444444
    		4444444444440000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000026
    		6666666666666666666666666666666666666666666666666666666666666666
    		6666666666660000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a DynSeq at top level ALWAYS has extra indirection
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded.len(), <(MyTy,)>::encoded_size(&(data,)));
    }

    #[test]
    fn encode_static_tuple_of_addresses() {
        type MyTy = (sol_data::Address, sol_data::Address);
        let data = (Address::from([0x11u8; 20]), Address::from([0x22u8; 20]));

        let encoded = MyTy::encode(&data);
        let encoded_params = MyTy::encode_params(&data);

        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_dynamic_tuple() {
        type MyTy = (sol_data::String, sol_data::String);
        let data = ("gavofyork".to_string(), "gavofyork".to_string());

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded.
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_dynamic_tuple_of_bytes2() {
        type MyTy = (sol_data::Bytes, sol_data::Bytes);

        let data = (
            hex!("4444444444444444444444444444444444444444444444444444444444444444444444444444")
                .to_vec(),
            hex!("6666666666666666666666666666666666666666666666666666666666666666666666666666")
                .to_vec(),
        );

        let encoded = MyTy::encode_single(&data);
        let encoded_params = MyTy::encode_params(&data);

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000026
    		4444444444444444444444444444444444444444444444444444444444444444
    		4444444444440000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000026
    		6666666666666666666666666666666666666666666666666666666666666666
    		6666666666660000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded.
        assert_eq!(encoded, expected);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_complex_tuple() {
        type MyTy = (
            sol_data::Uint<256>,
            sol_data::String,
            sol_data::Address,
            sol_data::Address,
        );

        let data = (
            U256::from_be_bytes::<32>([0x11u8; 32]),
            "gavofyork".to_owned(),
            Address::from([0x11u8; 20]),
            Address::from([0x22u8; 20]),
        );

        let expected = hex!(
            "
            0000000000000000000000000000000000000000000000000000000000000020
            1111111111111111111111111111111111111111111111111111111111111111
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded.
        let encoded = MyTy::encode_single(&data);
        assert_eq!(encoded, expected);
        let encoded_params = MyTy::encode_params(&data);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_nested_tuple() {
        type MyTy = (
            sol_data::String,
            sol_data::Bool,
            sol_data::String,
            (
                sol_data::String,
                sol_data::String,
                (sol_data::String, sol_data::String),
            ),
        );

        let data = (
            "test".to_string(),
            true,
            "cyborg".to_string(),
            (
                "night".to_string(),
                "day".to_string(),
                ("weee".to_string(), "funtests".to_string()),
            ),
        );

        let encoded = MyTy::encode_single(&data);
        let encoded_params = MyTy::encode(&data);

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000001
    		00000000000000000000000000000000000000000000000000000000000000c0
    		0000000000000000000000000000000000000000000000000000000000000100
    		0000000000000000000000000000000000000000000000000000000000000004
    		7465737400000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000006
    		6379626f72670000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000060
    		00000000000000000000000000000000000000000000000000000000000000a0
    		00000000000000000000000000000000000000000000000000000000000000e0
    		0000000000000000000000000000000000000000000000000000000000000005
    		6e69676874000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000003
    		6461790000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000004
    		7765656500000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000008
    		66756e7465737473000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded
        assert_eq!(encoded, expected);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_params_containing_dynamic_tuple() {
        type MyTy = (
            sol_data::Address,
            (sol_data::Bool, sol_data::String, sol_data::String),
            sol_data::Address,
            sol_data::Address,
            sol_data::Bool,
        );
        let data = (
            Address::from([0x22u8; 20]),
            (true, "spaceship".to_owned(), "cyborg".to_owned()),
            Address::from([0x33u8; 20]),
            Address::from([0x44u8; 20]),
            false,
        );

        let encoded_single = MyTy::encode_single(&data);
        let encoded = MyTy::encode(&data);

        let expected = hex!(
            "
    		0000000000000000000000002222222222222222222222222222222222222222
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000060
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000009
    		7370616365736869700000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000006
    		6379626f72670000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // A Dynamic FixedSeq may be a top-level sequence to `encode` or may
        // itself be an item in a top-level sequence. Which is to say, it could
        // be (as `encode(T)` or `encode((T,))`). This test was `encode(T)`
        assert_ne!(encoded_single, expected);
        assert_eq!(encoded, expected);
        assert_eq!(encoded.len() + 32, encoded_single.len());
        assert_eq!(encoded.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_params_containing_static_tuple() {
        type MyTy = (
            sol_data::Address,
            (sol_data::Address, sol_data::Bool, sol_data::Bool),
            sol_data::Address,
            sol_data::Address,
        );

        let data = (
            Address::from([0x11u8; 20]),
            (Address::from([0x22u8; 20]), true, false),
            Address::from([0x33u8; 20]),
            Address::from([0x44u8; 20]),
        );

        let encoded = MyTy::encode(&data);
        let encoded_params = MyTy::encode_params(&data);

        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        )
        .to_vec();

        // a static FixedSeq should NEVER indirect
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }

    #[test]
    fn encode_dynamic_tuple_with_nested_static_tuples() {
        type MyTy = (
            ((sol_data::Bool, sol_data::Uint<16>),),
            sol_data::Array<sol_data::Uint<16>>,
        );

        let data = (((false, 0x777),), vec![0x42, 0x1337]);

        let encoded = MyTy::encode_single(&data);
        let encoded_params = MyTy::encode_params(&data);

        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000777
    		0000000000000000000000000000000000000000000000000000000000000060
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000042
    		0000000000000000000000000000000000000000000000000000000000001337
    	"
        )
        .to_vec();
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded
        assert_eq!(encoded, expected);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
        assert_eq!(encoded_params.len(), MyTy::encoded_size(&data));
    }
}
