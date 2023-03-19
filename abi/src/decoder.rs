// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//! ABI decoder.

use core::ops::Range;

use crate::{AbiResult, Error, SolType, TokenType, Word};

fn round_up_nearest_multiple(value: usize, padding: usize) -> usize {
    (value + padding - 1) / padding * padding
}

pub(crate) fn check_fixed_bytes(word: Word, len: usize) -> AbiResult<()> {
    if word == Word::default() {
        return Ok(());
    }
    match len {
        0 => Err(Error::InvalidData),
        1..=31 => check_zeroes(&word[len..]),
        32 => Ok(()),
        33.. => Err(Error::InvalidData),
        _ => unreachable!(),
    }
}

pub(crate) fn as_usize(slice: Word) -> AbiResult<usize> {
    check_zeroes(&slice[..28])?;

    let result = ((slice[28] as usize) << 24)
        + ((slice[29] as usize) << 16)
        + ((slice[30] as usize) << 8)
        + (slice[31] as usize);

    Ok(result)
}

pub(crate) fn check_bool(slice: Word) -> AbiResult<()> {
    check_zeroes(&slice[..31])?;
    Ok(())
}

/// The [`Decoder`] wraps a byte slice with necessary info to progressively
/// deserialize the bytes into a sequence of tokens.
///
/// # Usage Note
///
/// While the Decoder contains the necessary info, the actual deserialization
/// is done in the [`crate::SolType`] trait.
#[derive(Clone, Copy)]
pub struct Decoder<'a> {
    // the underlying buffer
    buf: &'a [u8],
    // the current offset in the buffer
    offset: usize,
    // true if encoding a root-level tuple
    is_params: bool,
    // true if we validate type correctness and blob re-encoding
    validate: bool,
}

impl core::fmt::Debug for Decoder<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Decoder")
            .field("buf", &format!("0x{}", hex::encode(self.buf)))
            .field("offset", &self.offset)
            .field("is_params", &self.is_params)
            .field("validate", &self.validate)
            .finish()
    }
}

impl<'a> Decoder<'a> {
    pub fn new(buf: &'a [u8], is_params: bool, validate: bool) -> Self {
        Self {
            buf,
            offset: 0,
            is_params,
            validate,
        }
    }

    fn child(&self, offset: usize) -> Result<Decoder<'a>, Error> {
        if offset > self.buf.len() {
            return Err(Error::Overrun);
        }
        Ok(Self {
            buf: &self.buf[offset..],
            offset: 0,
            is_params: false,
            validate: self.validate,
        })
    }

    pub fn raw_child(&self) -> Decoder<'a> {
        self.child(self.offset).unwrap()
    }

    fn increase_offset(&mut self, len: usize) {
        self.offset += len;
        self.is_params = false;
    }

    pub fn peek(&self, range: Range<usize>) -> Result<&'a [u8], Error> {
        (self.buf.len() >= range.end)
            .then(|| &self.buf[range])
            .ok_or(Error::Overrun)
    }

    pub fn peek_len_at(&self, offset: usize, len: usize) -> Result<&'a [u8], Error> {
        self.peek(offset..offset + len)
    }

    pub fn peek_len(&self, len: usize) -> Result<&'a [u8], Error> {
        self.peek_len_at(self.offset, len)
    }

    pub fn peek_word_at(&self, offset: usize) -> Result<Word, Error> {
        Ok(Word::from_slice(
            self.peek_len_at(offset, Word::len_bytes())?,
        ))
    }

    pub fn peek_word(&self) -> Result<Word, Error> {
        self.peek_word_at(self.offset)
    }

    pub fn peek_usize_at(&self, offset: usize) -> AbiResult<usize> {
        as_usize(self.peek_word_at(offset)?)
    }

    pub fn peek_usize(&self) -> AbiResult<usize> {
        as_usize(self.peek_word()?)
    }

    pub fn take_word(&mut self) -> Result<Word, Error> {
        let contents = self.peek_word()?;
        self.increase_offset(Word::len_bytes());
        Ok(contents)
    }

    pub fn take_indirection(&mut self) -> Result<Decoder<'a>, Error> {
        let ptr = self.take_usize()?;
        self.child(ptr)
    }

    pub fn take_usize(&mut self) -> AbiResult<usize> {
        as_usize(self.take_word()?)
    }

    pub fn take_slice(&mut self, len: usize) -> Result<&[u8], Error> {
        if self.validate {
            let padded_len = round_up_nearest_multiple(len, 32);
            if self.offset + padded_len > self.buf.len() {
                return Err(Error::Overrun);
            }
            check_zeroes(self.peek(self.offset + len..self.offset + padded_len)?)?;
        }
        let res = self.peek_len(len)?;
        self.increase_offset(len);
        Ok(res)
    }

    pub fn validate(&self) -> bool {
        self.validate
    }

    pub fn is_params(&self) -> bool {
        self.is_params
    }

    pub fn take_offset(&mut self, child: Decoder<'a>) {
        self.set_offset(child.offset + (self.buf.len() - child.buf.len()))
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn decode<T>(&mut self, data: &[u8]) -> AbiResult<T::TokenType>
    where
        T: SolType,
    {
        if data.is_empty() {
            return Err(Error::InvalidData);
        }

        let token = T::TokenType::decode_from(self)?;

        if self.validate && data.len() != token.total_words() * 32 {
            return Err(Error::ExtraData);
        }

        Ok(token)
    }
}

pub(crate) fn decode_params_impl<T>(data: &[u8], validate: bool) -> AbiResult<T::TokenType>
where
    T: SolType,
{
    let mut decoder = Decoder::new(data, true, validate);
    decoder.decode::<T>(data)
}

pub(crate) fn decode_impl<T>(data: &[u8], validate: bool) -> AbiResult<T::TokenType>
where
    T: SolType,
{
    let mut decoder = Decoder::new(data, false, validate);
    decoder.decode::<T>(data)
}

/// Decodes ABI compliant vector of bytes into vector of tokens described by types param.
/// Checks, that decoded data is exact as input provided
pub fn decode_validate<T>(data: &[u8]) -> AbiResult<T::TokenType>
where
    T: SolType,
{
    decode_impl::<T>(data, true)
}

/// Decode top-level function args and validate
pub fn decode_params_validate<T>(data: &[u8]) -> AbiResult<T::TokenType>
where
    T: SolType,
{
    decode_params_impl::<T>(data, true)
}

/// Decodes ABI compliant vector of bytes into vector of tokens described by types param.
pub fn decode<T>(data: &[u8]) -> AbiResult<T::TokenType>
where
    T: SolType,
{
    decode_impl::<T>(data, false)
}

/// Decode top-level function args
pub fn decode_params<T>(data: &[u8]) -> AbiResult<T::TokenType>
where
    T: SolType,
{
    decode_params_impl::<T>(data, false)
}

pub(crate) fn check_zeroes(data: &[u8]) -> AbiResult<()> {
    if data.iter().all(|b| *b == 0) {
        Ok(())
    } else {
        Err(Error::InvalidData)
    }
}

#[cfg(test)]
mod tests {
    // TODO

    // use ethers_primitives::{B160, B256};
    // use hex_literal::hex;

    // #[cfg(not(feature = "std"))]
    // use crate::no_std_prelude::*;
    // use crate::{decode, decode_params, decode_validate, sol_type, util::pad_u32, SolType};

    // #[test]
    // fn decode_static_tuple_of_addresses_and_uints() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000001111111111111111111111111111111111111111
    // 		0000000000000000000000002222222222222222222222222222222222222222
    // 		1111111111111111111111111111111111111111111111111111111111111111
    // 	"
    //     );
    //     let address1 = sol_type::Address::tokenize(B160([0x11u8; 20]));
    //     let address2 = sol_type::Address::tokenize(B160([0x22u8; 20]));
    //     let uint = Token::Word([0x11u8; 32].into());
    //     let expected = Token::FixedSeq(vec![address1, address2, uint]);
    //     let decoded =
    //         decode::<(sol_type::Address, sol_type::Address, sol_type::Uint<32>)>(&encoded).unwrap();
    //     assert_eq!(decoded, expected);
    // }

    // #[test]
    // fn decode_dynamic_tuple() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000000000000000000000000000000000000000000020
    // 		0000000000000000000000000000000000000000000000000000000000000040
    // 		0000000000000000000000000000000000000000000000000000000000000080
    // 		0000000000000000000000000000000000000000000000000000000000000009
    // 		6761766f66796f726b0000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000009
    // 		6761766f66796f726b0000000000000000000000000000000000000000000000
    // 	"
    //     );
    //     let string1 = Token::PackedSeq(b"gavofyork".to_vec());
    //     let string2 = Token::PackedSeq(b"gavofyork".to_vec());
    //     let expected = Token::FixedSeq(vec![string1, string2]);

    //     let decoded = decode::<(sol_type::String, sol_type::String)>(&encoded).unwrap();
    //     assert_eq!(decoded, expected);
    // }

    // #[test]
    // fn decode_nested_tuple() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000000000000000000000000000000000000000000020
    // 		0000000000000000000000000000000000000000000000000000000000000080
    // 		0000000000000000000000000000000000000000000000000000000000000001
    // 		00000000000000000000000000000000000000000000000000000000000000c0
    // 		0000000000000000000000000000000000000000000000000000000000000100
    // 		0000000000000000000000000000000000000000000000000000000000000004
    // 		7465737400000000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000006
    // 		6379626f72670000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000060
    // 		00000000000000000000000000000000000000000000000000000000000000a0
    // 		00000000000000000000000000000000000000000000000000000000000000e0
    // 		0000000000000000000000000000000000000000000000000000000000000005
    // 		6e69676874000000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000003
    // 		6461790000000000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000040
    // 		0000000000000000000000000000000000000000000000000000000000000080
    // 		0000000000000000000000000000000000000000000000000000000000000004
    // 		7765656500000000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000008
    // 		66756e7465737473000000000000000000000000000000000000000000000000
    // 	"
    //     );
    //     let string1 = Token::PackedSeq(b"test".to_vec());
    //     let string2 = Token::PackedSeq(b"cyborg".to_vec());
    //     let string3 = Token::PackedSeq(b"night".to_vec());
    //     let string4 = Token::PackedSeq(b"day".to_vec());
    //     let string5 = Token::PackedSeq(b"weee".to_vec());
    //     let string6 = Token::PackedSeq(b"funtests".to_vec());
    //     let bool = sol_type::Bool::tokenize(true);
    //     let deep_tuple = Token::FixedSeq(vec![string5, string6]);
    //     let inner_tuple = Token::FixedSeq(vec![string3, string4, deep_tuple]);
    //     let expected = Token::FixedSeq(vec![string1, bool, string2, inner_tuple]);

    //     type MyTy = (
    //         sol_type::String,
    //         sol_type::Bool,
    //         sol_type::String,
    //         (
    //             sol_type::String,
    //             sol_type::String,
    //             (sol_type::String, sol_type::String),
    //         ),
    //     );

    //     let decoded = decode::<MyTy>(&encoded).unwrap();
    //     assert_eq!(decoded, expected);
    // }

    // #[test]
    // fn decode_complex_tuple_of_dynamic_and_static_types() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000000000000000000000000000000000000000000020
    // 		1111111111111111111111111111111111111111111111111111111111111111
    // 		0000000000000000000000000000000000000000000000000000000000000080
    // 		0000000000000000000000001111111111111111111111111111111111111111
    // 		0000000000000000000000002222222222222222222222222222222222222222
    // 		0000000000000000000000000000000000000000000000000000000000000009
    // 		6761766f66796f726b0000000000000000000000000000000000000000000000
    // 	"
    //     );
    //     let uint = Token::Word([0x11u8; 32].into());
    //     let string = Token::PackedSeq(b"gavofyork".to_vec());
    //     let address1 = sol_type::Address::tokenize(B160([0x11u8; 20]));
    //     let address2 = sol_type::Address::tokenize(B160([0x22u8; 20]));
    //     let expected = Token::FixedSeq(vec![uint, string, address1, address2]);

    //     type MyTy = (
    //         sol_type::Uint<32>,
    //         sol_type::String,
    //         sol_type::Address,
    //         sol_type::Address,
    //     );

    //     let decoded = decode::<MyTy>(&encoded).unwrap();
    //     assert_eq!(decoded, expected);
    // }

    // #[test]
    // fn decode_params_containing_dynamic_tuple() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000002222222222222222222222222222222222222222
    // 		00000000000000000000000000000000000000000000000000000000000000a0
    // 		0000000000000000000000003333333333333333333333333333333333333333
    // 		0000000000000000000000004444444444444444444444444444444444444444
    // 		0000000000000000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000001
    // 		0000000000000000000000000000000000000000000000000000000000000060
    // 		00000000000000000000000000000000000000000000000000000000000000a0
    // 		0000000000000000000000000000000000000000000000000000000000000009
    // 		7370616365736869700000000000000000000000000000000000000000000000
    // 		0000000000000000000000000000000000000000000000000000000000000006
    // 		6379626f72670000000000000000000000000000000000000000000000000000
    // 	"
    //     );
    //     let address1 = sol_type::Address::tokenize(B160([0x22u8; 20]));
    //     let bool1 = sol_type::Bool::tokenize(true);
    //     let string1 = Token::PackedSeq(b"spaceship".to_vec());
    //     let string2 = Token::PackedSeq(b"cyborg".to_vec());
    //     let tuple = Token::FixedSeq(vec![bool1, string1, string2]);
    //     let address2 = sol_type::Address::tokenize(B160([0x33u8; 20]));
    //     let address3 = sol_type::Address::tokenize(B160([0x44u8; 20]));
    //     let bool2 = sol_type::Bool::tokenize(false);
    //     let expected = Token::FixedSeq(vec![address1, tuple, address2, address3, bool2]);

    //     type MyTy = (
    //         sol_type::Address,
    //         (sol_type::Bool, sol_type::String, sol_type::String),
    //         sol_type::Address,
    //         sol_type::Address,
    //         sol_type::Bool,
    //     );

    //     let decoded = decode_params::<MyTy>(&encoded).unwrap();
    //     assert_eq!(decoded, expected);
    // }

    // #[test]
    // fn decode_params_containing_static_tuple() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000001111111111111111111111111111111111111111
    // 		0000000000000000000000002222222222222222222222222222222222222222
    // 		0000000000000000000000000000000000000000000000000000000000000001
    // 		0000000000000000000000000000000000000000000000000000000000000000
    // 		0000000000000000000000003333333333333333333333333333333333333333
    // 		0000000000000000000000004444444444444444444444444444444444444444
    // 	"
    //     );
    //     let address1 = sol_type::Address::tokenize(B160([0x11u8; 20]));
    //     let address2 = sol_type::Address::tokenize(B160([0x22u8; 20]));
    //     let bool1 = sol_type::Bool::tokenize(true);
    //     let bool2 = sol_type::Bool::tokenize(false);
    //     let tuple = Token::FixedSeq(vec![address2, bool1, bool2]);
    //     let address3 = sol_type::Address::tokenize(B160([0x33u8; 20]));
    //     let address4 = sol_type::Address::tokenize(B160([0x44u8; 20]));

    //     let expected = Token::FixedSeq(vec![address1, tuple, address3, address4]);

    //     type MyTy = (
    //         sol_type::Address,
    //         (sol_type::Address, sol_type::Bool, sol_type::Bool),
    //         sol_type::Address,
    //         sol_type::Address,
    //     );

    //     let decoded = decode_params::<MyTy>(&encoded).unwrap();
    //     assert_eq!(decoded, expected);
    // }

    // #[test]
    // fn decode_data_with_size_that_is_not_a_multiple_of_32() {
    //     let encoded = hex!(
    //         "
    //         0000000000000000000000000000000000000000000000000000000000000000
    //         00000000000000000000000000000000000000000000000000000000000000a0
    //         0000000000000000000000000000000000000000000000000000000000000152
    //         0000000000000000000000000000000000000000000000000000000000000001
    //         000000000000000000000000000000000000000000000000000000000054840d
    //         0000000000000000000000000000000000000000000000000000000000000092
    //         3132323033393637623533326130633134633938306235616566666231373034
    //         3862646661656632633239336139353039663038656233633662306635663866
    //         3039343265376239636337366361353163636132366365353436393230343438
    //         6533303866646136383730623565326165313261323430396439343264653432
    //         3831313350373230703330667073313678390000000000000000000000000000
    //         0000000000000000000000000000000000103933633731376537633061363531
    //         3761
    //     "
    //     );

    //     type MyTy = (
    //         sol_type::Uint<256>,
    //         sol_type::String,
    //         sol_type::String,
    //         sol_type::Uint<256>,
    //         sol_type::Uint<256>,
    //     );

    //     assert_eq!(
    // 		decode::<MyTy>(
    // 			&encoded,
    // 		).unwrap(),
    // 		Token::FixedSeq(
    //             vec![
    //                 Token::Word(pad_u32(0)),
    //                 Token::PackedSeq(b"12203967b532a0c14c980b5aeffb17048bdfaef2c293a9509f08eb3c6b0f5f8f0942e7b9cc76ca51cca26ce546920448e308fda6870b5e2ae12a2409d942de428113P720p30fps16x9".to_vec()),
    //                 Token::PackedSeq(b"93c717e7c0a6517a".to_vec()),
    //                 Token::Word(pad_u32(1)),
    //                 Token::Word(pad_u32(5538829))
    //             ]
    //         )
    // 	);
    // }

    // #[test]
    // fn decode_after_fixed_bytes_with_less_than_32_bytes() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000008497afefdc5ac170a664a231f6efb25526ef813f
    // 		0101010101010101010101010101010101010101010101010101010101010101
    // 		0202020202020202020202020202020202020202020202020202020202020202
    // 		0000000000000000000000000000000000000000000000000000000000000080
    // 		000000000000000000000000000000000000000000000000000000000000000a
    // 		3078303030303030314600000000000000000000000000000000000000000000
    // 	    "
    //     );

    //     type MyTy = (
    //         sol_type::Address,
    //         sol_type::FixedBytes<32>,
    //         sol_type::FixedBytes<4>,
    //         sol_type::String,
    //     );

    //     assert_eq!(
    //         decode_params::<MyTy>(&encoded).unwrap(),
    //         Token::FixedSeq(vec![
    //             sol_type::Address::tokenize(B160(hex!("8497afefdc5ac170a664a231f6efb25526ef813f"))),
    //             Token::Word(B256::repeat_byte(0x01)),
    //             Token::Word(B256::repeat_byte(0x02)),
    //             Token::PackedSeq("0x0000001F".into()),
    //         ])
    //     )
    // }

    // #[test]
    // fn decode_broken_utf8() {
    //     let encoded = hex!(
    //         "
    // 		0000000000000000000000000000000000000000000000000000000000000020
    // 		0000000000000000000000000000000000000000000000000000000000000004
    // 		e4b88de500000000000000000000000000000000000000000000000000000000
    //         "
    //     );

    //     assert_eq!(
    //         decode::<sol_type::String>(&encoded).unwrap(),
    //         Token::PackedSeq([0xe4, 0xb8, 0x8d, 0xe5].to_vec())
    //     );
    // }

    // #[test]
    // fn decode_corrupted_dynamic_array() {
    //     // line 1 at 0x00 =   0: tail offset of array
    //     // line 2 at 0x20 =  32: length of array
    //     // line 3 at 0x40 =  64: first word
    //     // line 4 at 0x60 =  96: second word
    //     let encoded = hex!(
    //         "
    // 	0000000000000000000000000000000000000000000000000000000000000020
    // 	00000000000000000000000000000000000000000000000000000000ffffffff
    // 	0000000000000000000000000000000000000000000000000000000000000001
    // 	0000000000000000000000000000000000000000000000000000000000000002
    //     "
    //     );

    //     type MyTy = sol_type::Array<sol_type::Uint<32>>;
    //     assert!(decode::<MyTy>(&encoded).is_err());
    // }

    // #[test]
    // fn decode_verify_addresses() {
    //     let input = hex!(
    //         "
    // 	0000000000000000000000000000000000000000000000000000000000012345
    // 	0000000000000000000000000000000000000000000000000000000000054321
    // 	"
    //     );
    //     assert!(decode::<sol_type::Address>(&input).is_ok());
    //     assert!(decode_validate::<sol_type::Address>(&input).is_err());
    //     assert!(decode_validate::<(sol_type::Address, sol_type::Address)>(&input).is_ok());
    // }

    // #[test]
    // fn decode_verify_bytes() {
    //     let input = hex!(
    //         "
    // 	0000000000000000000000001234500000000000000000000000000000012345
    // 	0000000000000000000000005432100000000000000000000000000000054321
    // 	"
    //     );
    //     assert!(decode_validate::<(sol_type::Address, sol_type::FixedBytes<20>)>(&input).is_err());
    //     assert!(decode_validate::<(sol_type::Address, sol_type::Address)>(&input).is_ok());
    // }
}
