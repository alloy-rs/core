// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ABI encoder.

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{util::pad_u32, Bytes, Token, Word};

fn pad_bytes_len(bytes: &[u8]) -> u32 {
    // "+ 1" because len is also appended
    ((bytes.len() + 31) / 32) as u32 + 1
}

fn pad_bytes_append(data: &mut Vec<Word>, bytes: &[u8]) {
    data.push(pad_u32(bytes.len() as u32));
    fixed_bytes_append(data, bytes);
}

fn fixed_bytes_append(result: &mut Vec<Word>, bytes: &[u8]) {
    let len = (bytes.len() + 31) / 32;
    for i in 0..len {
        let mut padded = Word::default();

        let to_copy = match i == len - 1 {
            false => 32,
            true => match bytes.len() % 32 {
                0 => 32,
                x => x,
            },
        };

        let offset = 32 * i;
        padded[..to_copy].copy_from_slice(&bytes[offset..offset + to_copy]);
        result.push(padded);
    }
}

fn encode_head_tail(mediates: &[Mediate]) -> Vec<Word> {
    let (heads_len, tails_len) = mediates.iter().fold((0, 0), |(head_acc, tail_acc), m| {
        (head_acc + m.head_len(), tail_acc + m.tail_len())
    });

    let mut result = Vec::with_capacity((heads_len + tails_len) as usize);
    encode_head_tail_append(&mut result, mediates);

    result
}

fn encode_head_tail_append(acc: &mut Vec<Word>, mediates: &[Mediate]) {
    let heads_len = mediates
        .iter()
        .fold(0, |head_acc, m| head_acc + m.head_len());

    let mut offset = heads_len;
    for mediate in mediates {
        mediate.head_append(acc, offset);
        offset += mediate.tail_len();
    }

    mediates.iter().for_each(|m| m.tail_append(acc));
}

fn encode_token_append(data: &mut Vec<Word>, token: &Token) {
    match token {
        Token::Word(word) => data.push(*word),
        Token::PackedSeq(bytes) => pad_bytes_append(data, bytes),
        _ => panic!("Unhandled nested token: {:?}", token),
    };
}

#[derive(Debug)]
enum Mediate<'a> {
    // head
    // Head-only

    // Raw: head words + token
    Raw(u32, &'a Token),
    // RawArray: tokens
    RawArray(Vec<Mediate<'a>>),

    // head + tail

    // Prefixed: tail words, token
    Prefixed(u32, &'a Token),
    //
    PrefixedArray(Vec<Mediate<'a>>),
    PrefixedArrayWithLength(Vec<Mediate<'a>>),
}

impl Mediate<'_> {
    fn from_token(token: &Token) -> Mediate<'_> {
        match token {
            Token::Word(_) => Mediate::Raw(1, token),
            Token::FixedSeq(tokens) => {
                let mediates = tokens.iter().map(Mediate::from_token).collect();

                if token.is_dynamic() {
                    Mediate::PrefixedArray(mediates)
                } else {
                    Mediate::RawArray(mediates)
                }
            }
            Token::DynSeq(tokens) => {
                let mediates = tokens.iter().map(Mediate::from_token).collect();

                Mediate::PrefixedArrayWithLength(mediates)
            }
            Token::PackedSeq(seq) => Mediate::Prefixed(pad_bytes_len(seq), token),
        }
    }

    fn head_len(&self) -> u32 {
        match self {
            Mediate::Raw(len, _) => 32 * len,
            Mediate::RawArray(ref mediates) => {
                mediates.iter().map(|mediate| mediate.head_len()).sum()
            }
            Mediate::Prefixed(_, _)
            | Mediate::PrefixedArray(_)
            | Mediate::PrefixedArrayWithLength(_) => 32,
        }
    }

    fn tail_len(&self) -> u32 {
        match self {
            Mediate::Raw(_, _) | Mediate::RawArray(_) => 0,
            Mediate::Prefixed(len, _) => 32 * len,
            Mediate::PrefixedArray(ref mediates) => mediates
                .iter()
                .fold(0, |acc, m| acc + m.head_len() + m.tail_len()),
            Mediate::PrefixedArrayWithLength(ref mediates) => mediates
                .iter()
                .fold(32, |acc, m| acc + m.head_len() + m.tail_len()),
        }
    }

    fn head_append(&self, acc: &mut Vec<Word>, suffix_offset: u32) {
        match *self {
            Mediate::Raw(_, raw) => encode_token_append(acc, raw),
            Mediate::RawArray(ref raw) => {
                raw.iter().for_each(|mediate| mediate.head_append(acc, 0))
            }
            Mediate::Prefixed(_, _)
            | Mediate::PrefixedArray(_)
            | Mediate::PrefixedArrayWithLength(_) => acc.push(pad_u32(suffix_offset)),
        }
    }

    fn tail_append(&self, acc: &mut Vec<Word>) {
        match *self {
            Mediate::Raw(_, _) | Mediate::RawArray(_) => {}
            Mediate::Prefixed(_, raw) => encode_token_append(acc, raw),
            Mediate::PrefixedArray(ref mediates) => encode_head_tail_append(acc, mediates),
            Mediate::PrefixedArrayWithLength(ref mediates) => {
                // + 32 added to offset represents len of the array prepended to tail
                acc.push(pad_u32(mediates.len() as u32));
                encode_head_tail_append(acc, mediates);
            }
        };
    }
}

/// Encodes vector of tokens into ABI compliant vector of bytes.
fn encode_impl<'a>(tokens: impl IntoIterator<Item = &'a Token>) -> Bytes {
    let mediates = &tokens
        .into_iter()
        .map(Mediate::from_token)
        .collect::<Vec<_>>();

    encode_head_tail(mediates)
        .into_iter()
        .flat_map(Into::<[u8; 32]>::into)
        .collect()
}

/// Encode a token to a bytearray.
pub fn encode(token: &Token) -> Bytes {
    match token {
        Token::FixedSeq(v) => encode_impl(v),
        _ => encode_impl([token]),
    }
}

/// Encode a token into a bytearray suitable for use INTERNAL to an abi blob.
/// Typically.
pub fn encode_raw(token: &Token) -> Bytes {
    encode_impl([token])
}

#[cfg(test)]
mod tests {
    use ethers_primitives::{B160, U256};
    use hex_literal::hex;

    #[cfg(not(feature = "std"))]
    use crate::no_std_prelude::*;
    use crate::{sol_type, util::pad_u32, SolType};

    #[test]
    fn encode_address() {
        let address = B160([0x11u8; 20]);
        let expected = hex!("0000000000000000000000001111111111111111111111111111111111111111");
        let encoded = sol_type::Address::encode(address);
        assert_eq!(encoded, expected);
        assert_eq!(sol_type::Address::encode_params(address), expected);
    }

    #[test]
    fn encode_dynamic_array_of_addresses() {
        type MyTy = sol_type::Array<sol_type::Address>;
        let rust = vec![B160([0x11u8; 20]), B160([0x22u8; 20])];
        let encoded = MyTy::encode(rust);
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
    }

    #[test]
    fn encode_fixed_array_of_addresses() {
        type MyTy = sol_type::FixedArray<sol_type::Address, 2>;

        let addresses = [B160([0x11u8; 20]), B160([0x22u8; 20])];

        let encoded = MyTy::encode(addresses);
        let encoded_params = MyTy::encode_params(addresses);
        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_two_addresses() {
        type MyTy = (sol_type::Address, sol_type::Address);
        let addresses = (B160([0x11u8; 20]), B160([0x22u8; 20]));

        let encoded = MyTy::encode(addresses);
        let encoded_params = MyTy::encode_params(addresses);
        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_fixed_array_of_dynamic_array_of_addresses() {
        type MyTy = sol_type::FixedArray<sol_type::Array<sol_type::Address>, 2>;

        let fixed = [
            vec![B160([0x11u8; 20]), B160([0x22u8; 20])],
            vec![B160([0x33u8; 20]), B160([0x44u8; 20])],
        ];

        let encoded = MyTy::encode(fixed.clone());
        let encoded_params = MyTy::encode_params(fixed);

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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded
        assert_eq!(encoded, expected);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_dynamic_array_of_fixed_array_of_addresses() {
        type TwoAddrs = sol_type::FixedArray<sol_type::Address, 2>;
        type MyTy = sol_type::Array<TwoAddrs>;

        let dynamic = vec![
            [B160([0x11u8; 20]), B160([0x22u8; 20])],
            [B160([0x33u8; 20]), B160([0x44u8; 20])],
        ];

        let encoded = MyTy::encode(dynamic.clone());
        let encoded_params = MyTy::encode_params(dynamic);
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
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_dynamic_array_of_dynamic_arrays() {
        type MyTy = sol_type::Array<sol_type::Array<sol_type::Address>>;

        let dynamic = vec![vec![B160([0x11u8; 20])], vec![B160([0x22u8; 20])]];

        let encoded = MyTy::encode(dynamic.clone());
        let encoded_params = MyTy::encode_params(dynamic);
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
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_dynamic_array_of_dynamic_arrays2() {
        type MyTy = sol_type::Array<sol_type::Array<sol_type::Address>>;

        let dynamic = vec![
            vec![B160([0x11u8; 20]), B160([0x22u8; 20])],
            vec![B160([0x33u8; 20]), B160([0x44u8; 20])],
        ];
        let encoded = MyTy::encode(dynamic.clone());
        let encoded_params = MyTy::encode_params(dynamic);
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
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_fixed_array_of_fixed_arrays() {
        type MyTy = sol_type::FixedArray<sol_type::FixedArray<sol_type::Address, 2>, 2>;

        let fixed = [
            [B160([0x11u8; 20]), B160([0x22u8; 20])],
            [B160([0x33u8; 20]), B160([0x44u8; 20])],
        ];

        let encoded = MyTy::encode(fixed);
        let encoded_params = MyTy::encode_params(fixed);
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
    }

    #[test]
    fn encode_fixed_array_of_static_tuples_followed_by_dynamic_type() {
        type Tup = (sol_type::Uint<256>, sol_type::Uint<256>, sol_type::Address);
        type Fixed = sol_type::FixedArray<Tup, 2>;
        type MyTy = (Fixed, sol_type::String);

        let data = (
            [
                (
                    U256::from(93523141),
                    U256::from(352332135),
                    B160([0x44u8; 20]),
                ),
                (U256::from(12411), U256::from(451), B160([0x22u8; 20])),
            ],
            "gavofyork".to_string(),
        );

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);
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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_empty_array() {
        type MyTy = (
            sol_type::Array<sol_type::Address>,
            sol_type::Array<sol_type::Address>,
        );
        let data = (vec![], vec![]);

        // Empty arrays
        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000060
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());

        type MyTy2 = (
            sol_type::Array<sol_type::Array<sol_type::Address>>,
            sol_type::Array<sol_type::Array<sol_type::Address>>,
        );
        let data = (vec![vec![]], vec![vec![]]);

        // Nested empty arrays
        let encoded = MyTy2::encode(data.clone());
        let encoded_params = MyTy2::encode_params(data);
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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_bytes() {
        type MyTy = sol_type::Bytes;
        let bytes = vec![0x12, 0x34];

        let encoded = MyTy::encode(bytes.clone());
        let encoded_params = MyTy::encode_params(bytes);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		1234000000000000000000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_fixed_bytes() {
        let encoded = sol_type::FixedBytes::<2>::encode([0x12, 0x34]);
        let encoded_params = sol_type::FixedBytes::<2>::encode_params([0x12, 0x34]);
        let expected = hex!("1234000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_string() {
        let s = "gavofyork".to_string();
        let encoded = sol_type::String::encode(s.clone());
        let encoded_params = sol_type::String::encode_params(s);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_bytes2() {
        let bytes = hex!("10000000000000000000000000000000000000000000000000000000000002").to_vec();
        let encoded = sol_type::Bytes::encode(bytes.clone());
        let encoded_params = sol_type::Bytes::encode_params(bytes);
        let expected = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		000000000000000000000000000000000000000000000000000000000000001f
    		1000000000000000000000000000000000000000000000000000000000000200
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
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
        let encoded = sol_type::Bytes::encode(bytes.clone());
        let encoded_params = sol_type::Bytes::encode_params(bytes);
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
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_two_bytes() {
        type MyTy = (sol_type::Bytes, sol_type::Bytes);

        let bytes = (
            hex!("10000000000000000000000000000000000000000000000000000000000002").to_vec(),
            hex!("0010000000000000000000000000000000000000000000000000000000000002").to_vec(),
        );
        let encoded = MyTy::encode(bytes.clone());
        let encoded_params = MyTy::encode_params(bytes);
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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_uint() {
        let uint = 4;
        let encoded = sol_type::Uint::<8>::encode(uint);
        let encoded_params = sol_type::Uint::<8>::encode_params(uint);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000004");
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_int() {
        let int = 4;
        let encoded = sol_type::Int::<8>::encode(int);
        let encoded_params = sol_type::Int::<8>::encode_params(int);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000004");
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_bool() {
        let encoded = sol_type::Bool::encode(true);
        let encoded_params = sol_type::Bool::encode_params(true);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_bool2() {
        let encoded = sol_type::Bool::encode(false);
        let encoded_params = sol_type::Bool::encode_params(false);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn comprehensive_test() {
        type MyTy = (
            sol_type::Uint<8>,
            sol_type::Bytes,
            sol_type::Uint<8>,
            sol_type::Bytes,
        );

        let bytes = hex!(
            "
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
    	"
        )
        .to_vec();

        let data = (5, bytes.clone(), 3, bytes);

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn test_pad_u32() {
        // this will fail if endianess is not supported
        assert_eq!(pad_u32(0x1)[31], 1);
        assert_eq!(pad_u32(0x100)[30], 1);
    }

    #[test]
    fn comprehensive_test2() {
        type MyTy = (
            sol_type::Bool,
            sol_type::String,
            sol_type::Uint<8>,
            sol_type::Uint<8>,
            sol_type::Uint<8>,
            sol_type::Array<sol_type::Uint<8>>,
        );

        let data = (true, "gavofyork".to_string(), 2, 3, 4, vec![5, 6, 7]);

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_dynamic_array_of_bytes() {
        type MyTy = sol_type::Array<sol_type::Bytes>;
        let data = vec![hex!(
            "019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a4218a274f00d607"
        )
        .to_vec()];

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_dynamic_array_of_bytes2() {
        type MyTy = sol_type::Array<sol_type::Bytes>;

        let data = vec![
            hex!("4444444444444444444444444444444444444444444444444444444444444444444444444444")
                .to_vec(),
            hex!("6666666666666666666666666666666666666666666666666666666666666666666666666666")
                .to_vec(),
        ];

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_static_tuple_of_addresses() {
        type MyTy = (sol_type::Address, sol_type::Address);
        let data = (B160([0x11u8; 20]), B160([0x22u8; 20]));

        let encoded = MyTy::encode(data);
        let encoded_params = MyTy::encode_params(data);

        let expected = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        )
        .to_vec();
        assert_eq!(encoded, expected);
        assert_eq!(encoded_params, expected);
    }

    #[test]
    fn encode_dynamic_tuple() {
        type MyTy = (sol_type::String, sol_type::String);
        let data = ("gavofyork".to_string(), "gavofyork".to_string());

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        assert_eq!(encoded, expected);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_dynamic_tuple_of_bytes2() {
        type MyTy = (sol_type::Bytes, sol_type::Bytes);

        let data = (
            hex!("4444444444444444444444444444444444444444444444444444444444444444444444444444")
                .to_vec(),
            hex!("6666666666666666666666666666666666666666666666666666666666666666666666666666")
                .to_vec(),
        );

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
    }

    #[test]
    fn encode_complex_tuple() {
        type MyTy = (
            sol_type::Uint<256>,
            sol_type::String,
            sol_type::Address,
            sol_type::Address,
        );

        let data = (
            U256::from_be_bytes::<32>([0x11u8; 32]),
            "gavofyork".to_owned(),
            B160([0x11u8; 20]),
            B160([0x22u8; 20]),
        );

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        assert_eq!(encoded, expected);
        assert_ne!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_nested_tuple() {
        type MyTy = (
            sol_type::String,
            sol_type::Bool,
            sol_type::String,
            (
                sol_type::String,
                sol_type::String,
                (sol_type::String, sol_type::String),
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

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
    }

    #[test]
    fn encode_params_containing_dynamic_tuple() {
        type MyTy = (
            sol_type::Address,
            (sol_type::Bool, sol_type::String, sol_type::String),
            sol_type::Address,
            sol_type::Address,
            sol_type::Bool,
        );
        let data = (
            B160([0x22u8; 20]),
            (true, "spaceship".to_owned(), "cyborg".to_owned()),
            B160([0x33u8; 20]),
            B160([0x44u8; 20]),
            false,
        );

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
        // a dynamic FixedSeq at top level should start with indirection
        // when not param encoded. For this particular test, there was an
        // implicit param incoding
        assert_ne!(encoded, expected);
        assert_eq!(encoded_params, expected);
        assert_eq!(encoded_params.len() + 32, encoded.len());
    }

    #[test]
    fn encode_params_containing_static_tuple() {
        type MyTy = (
            sol_type::Address,
            (sol_type::Address, sol_type::Bool, sol_type::Bool),
            sol_type::Address,
            sol_type::Address,
        );

        let data = (
            B160([0x11u8; 20]),
            (B160([0x22u8; 20]), true, false),
            B160([0x33u8; 20]),
            B160([0x44u8; 20]),
        );

        let encoded = MyTy::encode(data);
        let encoded_params = MyTy::encode_params(data);

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
    }

    #[test]
    fn encode_dynamic_tuple_with_nested_static_tuples() {
        type MyTy = (
            ((sol_type::Bool, sol_type::Uint<16>),),
            sol_type::Array<sol_type::Uint<16>>,
        );

        let data = (((false, 0x777),), vec![0x42, 0x1337]);

        let encoded = MyTy::encode(data.clone());
        let encoded_params = MyTy::encode_params(data);

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
    }
}
