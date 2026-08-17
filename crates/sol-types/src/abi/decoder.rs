// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//

use crate::{
    Error, Result, Word,
    abi::{Token, token::TokenSeq},
    utils,
};
use alloc::vec::Vec;
use alloy_primitives::hex;
use core::{fmt, mem, slice::SliceIndex};

/// The decoder recursion limit.
///
/// This is the default value used by [`AbiDecoderConfig`].
#[deprecated(note = "use `AbiDecoderConfig` to configure the recursion limit")]
pub const RECURSION_LIMIT: usize = 16;

const DEFAULT_MEMORY_LIMIT: usize = 1 << 30;

/// Configuration for ABI decoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbiDecoderConfig {
    recursion_limit: usize,
    memory_limit: usize,
    strict: bool,
}

impl Default for AbiDecoderConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl AbiDecoderConfig {
    /// Creates a decoder configuration with the default limits.
    #[inline]
    pub const fn new() -> Self {
        Self { recursion_limit: 16, memory_limit: DEFAULT_MEMORY_LIMIT, strict: false }
    }

    /// Returns the maximum recursion depth.
    #[inline]
    pub const fn get_recursion_limit(&self) -> usize {
        self.recursion_limit
    }

    /// Returns the maximum amount of memory that decoding may allocate.
    #[inline]
    pub const fn get_memory_limit(&self) -> usize {
        self.memory_limit
    }

    /// Returns whether strict validation is enabled.
    #[inline]
    pub const fn get_strict(&self) -> bool {
        self.strict
    }

    /// Sets the maximum recursion depth.
    #[inline]
    pub const fn recursion_limit(mut self, limit: usize) -> Self {
        self.recursion_limit = limit;
        self
    }

    /// Sets the maximum amount of memory that decoding may allocate.
    #[inline]
    pub const fn memory_limit(mut self, limit: usize) -> Self {
        self.memory_limit = limit;
        self
    }

    /// Enables or disables strict validation.
    #[inline]
    pub const fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Sets the maximum recursion depth in place.
    #[inline]
    pub const fn set_recursion_limit(&mut self, limit: usize) {
        self.recursion_limit = limit;
    }

    /// Sets the maximum amount of memory that decoding may allocate in place.
    #[inline]
    pub const fn set_memory_limit(&mut self, limit: usize) {
        self.memory_limit = limit;
    }

    /// Enables or disables strict validation in place.
    #[inline]
    pub const fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }
}

/// The [`Decoder`] wraps a byte slice with necessary info to progressively
/// deserialize the bytes into a sequence of tokens.
///
/// # Usage Note
///
/// While the Decoder contains the necessary info, the actual deserialization
/// is done in the [`crate::SolType`] trait.
#[derive(Clone, Copy)]
pub struct Decoder<'de> {
    // The underlying buffer.
    buf: &'de [u8],
    // The current offset in the buffer.
    offset: usize,
    /// The current recursion depth.
    depth: usize,
    /// The decoder configuration.
    config: AbiDecoderConfig,
    /// Memory allocated by this decoder and its children.
    memory_used: usize,
}

impl fmt::Debug for Decoder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut body = self.buf.chunks(32).map(hex::encode_prefixed).collect::<Vec<_>>();
        body[self.offset / 32].push_str(" <-- Next Word");

        f.debug_struct("Decoder")
            .field("buf", &body)
            .field("offset", &self.offset)
            .field("depth", &self.depth)
            .field("config", &self.config)
            .finish()
    }
}

impl fmt::Display for Decoder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Abi Decode Buffer")?;

        for (i, chunk) in self.buf.chunks(32).enumerate() {
            let idx = i * 32;
            writeln!(
                f,
                "0x{idx:04x}: {}{}",
                hex::encode_prefixed(chunk),
                if idx == self.offset { " <-- Next Word" } else { "" }
            )?;
        }
        Ok(())
    }
}

impl<'de> Decoder<'de> {
    /// Instantiates a new decoder from a byte slice.
    #[inline]
    pub const fn new(buf: &'de [u8]) -> Self {
        Self { buf, offset: 0, depth: 0, config: AbiDecoderConfig::new(), memory_used: 0 }
    }

    #[inline]
    const fn with_config(buf: &'de [u8], config: AbiDecoderConfig) -> Self {
        Self { buf, offset: 0, depth: 0, config, memory_used: 0 }
    }

    /// Returns the current offset in the buffer.
    #[inline]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of bytes in the remaining buffer.
    #[inline]
    pub const fn remaining(&self) -> Option<usize> {
        self.buf.len().checked_sub(self.offset)
    }

    /// Returns the number of words in the remaining buffer.
    #[inline]
    pub const fn remaining_words(&self) -> usize {
        if let Some(remaining) = self.remaining() { remaining / Word::len_bytes() } else { 0 }
    }

    /// Returns a reference to the remaining bytes in the buffer.
    #[inline]
    pub fn remaining_buf(&self) -> Option<&'de [u8]> {
        self.buf.get(self.offset..)
    }

    /// Returns whether the remaining buffer is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        match self.remaining() {
            Some(0) | None => true,
            Some(_) => false,
        }
    }

    /// Create a child decoder, starting at `offset` bytes from the current
    /// decoder's offset.
    ///
    /// See [`child`](Self::child).
    #[inline]
    pub fn raw_child(&self) -> Result<Self> {
        self.child(self.offset)
    }

    /// Create a child decoder, starting at `offset` bytes from the current
    /// decoder's offset.
    /// The child decoder shares the buffer.
    #[inline]
    pub fn child(&self, offset: usize) -> Result<Self, Error> {
        let recursion_limit = self.config.get_recursion_limit();
        if self.depth >= recursion_limit {
            return Err(Error::RecursionLimitExceeded(recursion_limit));
        }
        match self.buf.get(offset..) {
            Some(buf) => Ok(Decoder {
                buf,
                offset: 0,
                depth: self.depth + 1,
                config: self.config,
                memory_used: self.memory_used,
            }),
            None => Err(Error::Overrun),
        }
    }

    /// Registers an allocation against the configured memory limit.
    #[inline]
    pub(crate) fn reserve(&mut self, bytes: usize) -> Result<()> {
        let used = self
            .memory_used
            .checked_add(bytes)
            .ok_or(Error::MemoryLimitExceeded(self.config.get_memory_limit()))?;
        if used > self.config.get_memory_limit() {
            return Err(Error::MemoryLimitExceeded(self.config.get_memory_limit()));
        }
        self.memory_used = used;
        Ok(())
    }

    #[inline]
    pub(crate) fn reserve_elements<T>(&mut self, len: usize) -> Result<()> {
        let bytes = len
            .checked_mul(mem::size_of::<T>())
            .ok_or(Error::MemoryLimitExceeded(self.config.get_memory_limit()))?;
        self.reserve(bytes)
    }

    /// Advance the offset by `len` bytes.
    #[inline]
    const fn increase_offset(&mut self, len: usize) {
        self.offset += len;
    }

    /// Peek into the buffer.
    #[inline]
    pub fn peek<I: SliceIndex<[u8]>>(&self, index: I) -> Result<&'de I::Output, Error> {
        self.buf.get(index).ok_or(Error::Overrun)
    }

    /// Peek a slice of size `len` from the buffer at a specific offset, without
    /// advancing the offset.
    #[inline]
    pub fn peek_len_at(&self, offset: usize, len: usize) -> Result<&'de [u8], Error> {
        let end = offset.checked_add(len).ok_or(Error::Overrun)?;
        self.peek(offset..end)
    }

    /// Peek a slice of size `len` from the buffer without advancing the offset.
    #[inline]
    pub fn peek_len(&self, len: usize) -> Result<&'de [u8], Error> {
        self.peek_len_at(self.offset, len)
    }

    /// Peek a word from the buffer at a specific offset, without advancing the
    /// offset.
    #[inline]
    pub fn peek_word_at(&self, offset: usize) -> Result<&'de Word, Error> {
        self.peek_len_at(offset, Word::len_bytes()).map(|w| <&Word>::try_from(w).unwrap())
    }

    /// Peek the next word from the buffer without advancing the offset.
    #[inline]
    pub fn peek_word(&self) -> Result<&'de Word, Error> {
        self.peek_word_at(self.offset)
    }

    /// Peek a `usize` from the buffer at a specific offset, without advancing
    /// the offset.
    #[inline]
    pub fn peek_offset_at(&self, offset: usize) -> Result<usize> {
        self.peek_word_at(offset).and_then(utils::as_offset)
    }

    /// Peek a `usize` from the buffer, without advancing the offset.
    #[inline]
    pub fn peek_offset(&self) -> Result<usize> {
        self.peek_word().and_then(utils::as_offset)
    }

    /// Take a word from the buffer, advancing the offset.
    #[inline]
    pub fn take_word(&mut self) -> Result<&'de Word, Error> {
        let contents = self.peek_word()?;
        self.increase_offset(Word::len_bytes());
        Ok(contents)
    }

    /// Return a child decoder by consuming a word, interpreting it as a
    /// pointer, and following it.
    #[inline]
    pub fn take_indirection(&mut self) -> Result<Self, Error> {
        self.take_offset().and_then(|offset| self.child(offset))
    }

    #[inline]
    pub(crate) fn take_indirection_with<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let mut child = self.take_indirection()?;
        let result = f(&mut child)?;
        self.take_memory_from(&child);
        Ok(result)
    }

    #[inline]
    pub(crate) fn raw_child_with<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let mut child = self.raw_child()?;
        let result = f(&mut child)?;
        self.take_memory_from(&child);
        Ok(result)
    }

    /// Takes a `usize` offset from the buffer by consuming a word.
    #[inline]
    pub fn take_offset(&mut self) -> Result<usize> {
        self.take_word().and_then(utils::as_offset)
    }

    /// Takes a slice of bytes of the given length.
    #[inline]
    pub fn take_slice(&mut self, len: usize) -> Result<&'de [u8]> {
        self.peek_len(len).inspect(|_| self.increase_offset(len))
    }

    /// Takes the offset from the child decoder and sets it as the current
    /// offset.
    #[inline]
    pub const fn take_offset_from(&mut self, child: &Self) {
        self.set_offset(child.offset + (self.buf.len() - child.buf.len()));
        self.memory_used = child.memory_used;
    }

    /// Takes the allocation count from a child decoder.
    #[inline]
    const fn take_memory_from(&mut self, child: &Self) {
        self.memory_used = child.memory_used;
    }

    /// Sets the current offset in the buffer.
    #[inline]
    pub const fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    /// Decodes a single token from the underlying buffer.
    #[inline]
    pub fn decode<T: Token<'de>>(&mut self) -> Result<T> {
        T::decode_from(self)
    }

    /// Decodes a sequence of tokens from the underlying buffer.
    #[inline]
    pub fn decode_sequence<T: Token<'de> + TokenSeq<'de>>(&mut self) -> Result<T> {
        T::decode_sequence(self)
    }
}

/// ABI-decodes a token by wrapping it in a single-element tuple.
///
/// You are probably looking for
/// [`SolValue::abi_decode`](crate::SolValue::abi_decode) if you are not
/// intending to use raw tokens.
///
/// See the [`abi`](super) module for more information.
#[inline(always)]
pub fn decode<'de, T: Token<'de>>(data: &'de [u8]) -> Result<T> {
    decode_with_config(data, AbiDecoderConfig::default())
}

/// ABI-decodes a token with the given configuration.
#[inline(always)]
pub fn decode_with_config<'de, T: Token<'de>>(
    data: &'de [u8],
    config: AbiDecoderConfig,
) -> Result<T> {
    decode_sequence_with_config::<(T,)>(data, config).map(|(t,)| t)
}

/// ABI-decodes top-level function args.
///
/// Decodes as function parameters if [`T` is a tuple](TokenSeq::IS_TUPLE).
/// Otherwise, decodes it as a single-element tuple.
///
/// You are probably looking for
/// [`SolValue::abi_decode_params`](crate::SolValue::abi_decode_params) if
/// you are not intending to use raw tokens.
///
/// See the [`abi`](super) module for more information.
#[inline(always)]
pub fn decode_params<'de, T: TokenSeq<'de>>(data: &'de [u8]) -> Result<T> {
    decode_params_with_config(data, AbiDecoderConfig::default())
}

/// ABI-decodes top-level function args with the given configuration.
#[inline(always)]
pub fn decode_params_with_config<'de, T: TokenSeq<'de>>(
    data: &'de [u8],
    config: AbiDecoderConfig,
) -> Result<T> {
    if T::IS_TUPLE {
        decode_sequence_with_config(data, config)
    } else {
        decode_with_config(data, config)
    }
}

/// Decodes ABI compliant vector of bytes into vector of tokens described by
/// types param.
///
/// You are probably looking for
/// [`SolValue::abi_decode_sequence`](crate::SolValue::abi_decode_sequence) if
/// you are not intending to use raw tokens.
///
/// See the [`abi`](super) module for more information.
#[inline]
pub fn decode_sequence<'de, T: TokenSeq<'de>>(data: &'de [u8]) -> Result<T> {
    decode_sequence_with_config(data, AbiDecoderConfig::default())
}

/// Decodes an ABI-compliant sequence into tokens with the given configuration.
#[inline]
pub fn decode_sequence_with_config<'de, T: TokenSeq<'de>>(
    data: &'de [u8],
    config: AbiDecoderConfig,
) -> Result<T> {
    let mut decoder = Decoder::with_config(data, config);
    let result = decoder.decode_sequence::<T>()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SolCall, SolType, SolValue, sol, sol_data, utils::pad_usize};
    use alloc::string::ToString;
    use alloy_primitives::{Address, B256, U256, address, bytes, hex};

    sol! {
        struct Rule {
            bytes4 selector;
            address[] recipients;
        }

        struct Scope {
            address target;
            Rule[] rules;
        }

        function apply(address account, Scope[] scopes);
    }

    fn word(value: usize) -> [u8; 32] {
        let mut out = [0_u8; 32];
        out[24..].copy_from_slice(&(value as u64).to_be_bytes());
        out
    }

    fn aliased_call_data(width: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(292 + 96 * width);
        data.extend(applyCall::SELECTOR);

        data.extend(word(0));
        data.extend(word(64));

        data.extend(word(width));
        for _ in 0..width {
            data.extend(word(width * 32));
        }

        data.extend(word(1));
        data.extend(word(64));

        data.extend(word(width));
        for _ in 0..width {
            data.extend(word(width * 32));
        }

        let mut selector = [0_u8; 32];
        selector[..4].copy_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
        data.extend(selector);
        data.extend(word(64));

        data.extend(word(width));
        for i in 0..width {
            data.extend(word(i + 1));
        }

        assert_eq!(data.len(), 292 + 96 * width);
        data
    }

    #[test]
    fn dynamic_array_of_dynamic_arrays() {
        type MyTy = sol_data::Array<sol_data::Array<sol_data::Address>>;
        let encoded = hex!(
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
        );

        let ty = vec![vec![Address::repeat_byte(0x11)], vec![Address::repeat_byte(0x22)]];
        assert_eq!(MyTy::abi_encode_params(&ty), encoded);

        let decoded = MyTy::abi_decode_params(&encoded).unwrap();
        assert_eq!(decoded, ty);
        assert_eq!(decoded.abi_encode_params(), encoded);
        assert_eq!(decoded.abi_encoded_size(), encoded.len());
    }

    #[test]
    fn decode_static_tuple_of_addresses_and_uints() {
        type MyTy = (sol_data::Address, sol_data::Address, sol_data::Uint<256>);

        let encoded = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		1111111111111111111111111111111111111111111111111111111111111111
    	"
        );
        let address1 = Address::from([0x11u8; 20]);
        let address2 = Address::from([0x22u8; 20]);
        let uint = U256::from_be_bytes::<32>([0x11u8; 32]);
        let expected = (address1, address2, uint);
        let decoded = MyTy::abi_decode_sequence(&encoded).unwrap();
        assert_eq!(decoded, expected);
        assert_eq!(decoded.abi_encode_params(), encoded);
        assert_eq!(decoded.abi_encoded_size(), encoded.len());
    }

    #[test]
    fn decode_dynamic_tuple() {
        type MyTy = (sol_data::String, sol_data::String);
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        );
        let string1 = "gavofyork".to_string();
        let string2 = "gavofyork".to_string();
        let expected = (string1, string2);

        // this test vector contains a top-level indirect
        let decoded = MyTy::abi_decode(&encoded).unwrap();
        assert_eq!(decoded, expected);
        assert_eq!(decoded.abi_encode(), encoded);
        assert_eq!(decoded.abi_encoded_size(), encoded.len());
    }

    #[test]
    fn decode_nested_tuple() {
        type MyTy = (
            sol_data::String,
            sol_data::Bool,
            sol_data::String,
            (sol_data::String, sol_data::String, (sol_data::String, sol_data::String)),
        );

        let encoded = hex!(
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
        );
        let string1 = "test".into();
        let string2 = "cyborg".into();
        let string3 = "night".into();
        let string4 = "day".into();
        let string5 = "weee".into();
        let string6 = "funtests".into();
        let bool = true;
        let deep_tuple = (string5, string6);
        let inner_tuple = (string3, string4, deep_tuple);
        let expected = (string1, bool, string2, inner_tuple);

        let decoded = MyTy::abi_decode(&encoded).unwrap();
        assert_eq!(decoded, expected);
        assert_eq!(decoded.abi_encode(), encoded);
        assert_eq!(decoded.abi_encoded_size(), encoded.len());
    }

    #[test]
    fn decode_complex_tuple_of_dynamic_and_static_types() {
        type MyTy = (sol_data::Uint<256>, sol_data::String, sol_data::Address, sol_data::Address);

        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		1111111111111111111111111111111111111111111111111111111111111111
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        );
        let uint = U256::from_be_bytes::<32>([0x11u8; 32]);
        let string = "gavofyork".to_string();
        let address1 = Address::from([0x11u8; 20]);
        let address2 = Address::from([0x22u8; 20]);
        let expected = (uint, string, address1, address2);

        let decoded = MyTy::abi_decode(&encoded).unwrap();
        assert_eq!(decoded, expected);
        assert_eq!(decoded.abi_encode(), encoded);
        assert_eq!(decoded.abi_encoded_size(), encoded.len());
    }

    #[test]
    fn decode_params_containing_dynamic_tuple() {
        type MyTy = (
            sol_data::Address,
            (sol_data::Bool, sol_data::String, sol_data::String),
            sol_data::Address,
            sol_data::Address,
            sol_data::Bool,
        );

        let encoded = hex!(
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
        );
        let address1 = Address::from([0x22u8; 20]);
        let bool1 = true;
        let string1 = "spaceship".to_string();
        let string2 = "cyborg".to_string();
        let tuple = (bool1, string1, string2);
        let address2 = Address::from([0x33u8; 20]);
        let address3 = Address::from([0x44u8; 20]);
        let bool2 = false;
        let expected = (address1, tuple, address2, address3, bool2);

        let decoded = MyTy::abi_decode_params(&encoded).unwrap();
        assert_eq!(decoded, expected);
        assert_eq!(decoded.abi_encode_params(), encoded);
        assert_eq!(decoded.abi_encoded_size(), encoded.len() + 32);
    }

    #[test]
    fn decode_params_containing_static_tuple() {
        type MyTy = (
            sol_data::Address,
            (sol_data::Address, sol_data::Bool, sol_data::Bool),
            sol_data::Address,
            sol_data::Address,
        );

        let encoded = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        );
        let address1 = Address::from([0x11u8; 20]);
        let address2 = Address::from([0x22u8; 20]);
        let bool1 = true;
        let bool2 = false;
        let tuple = (address2, bool1, bool2);
        let address3 = Address::from([0x33u8; 20]);
        let address4 = Address::from([0x44u8; 20]);

        let expected = (address1, tuple, address3, address4);

        let decoded = MyTy::abi_decode_params(&encoded).unwrap();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn decode_data_with_size_that_is_not_a_multiple_of_32() {
        type MyTy = (
            sol_data::Uint<256>,
            sol_data::String,
            sol_data::String,
            sol_data::Uint<256>,
            sol_data::Uint<256>,
        );

        let data = (
            pad_usize(0).into(),
            "12203967b532a0c14c980b5aeffb17048bdfaef2c293a9509f08eb3c6b0f5f8f0942e7b9cc76ca51cca26ce546920448e308fda6870b5e2ae12a2409d942de428113P720p30fps16x9".to_string(),
            "93c717e7c0a6517a".to_string(),
            pad_usize(1).into(),
            pad_usize(5538829).into()
        );

        let encoded = hex!(
            "
            0000000000000000000000000000000000000000000000000000000000000000
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000152
            0000000000000000000000000000000000000000000000000000000000000001
            000000000000000000000000000000000000000000000000000000000054840d
            0000000000000000000000000000000000000000000000000000000000000092
            3132323033393637623533326130633134633938306235616566666231373034
            3862646661656632633239336139353039663038656233633662306635663866
            3039343265376239636337366361353163636132366365353436393230343438
            6533303866646136383730623565326165313261323430396439343264653432
            3831313350373230703330667073313678390000000000000000000000000000
            0000000000000000000000000000000000103933633731376537633061363531
            3761
        "
        );

        assert_eq!(MyTy::abi_decode_sequence(&encoded).unwrap(), data);
    }

    #[test]
    fn decode_after_fixed_bytes_with_less_than_32_bytes() {
        type MyTy = (
            sol_data::Address,
            sol_data::FixedBytes<32>,
            sol_data::FixedBytes<4>,
            sol_data::String,
        );

        let encoded = hex!(
            "
    		0000000000000000000000008497afefdc5ac170a664a231f6efb25526ef813f
    		0101010101010101010101010101010101010101010101010101010101010101
    		0202020202020202020202020202020202020202020202020202020202020202
    		0000000000000000000000000000000000000000000000000000000000000080
    		000000000000000000000000000000000000000000000000000000000000000a
    		3078303030303030314600000000000000000000000000000000000000000000
    	    "
        );

        assert_eq!(
            MyTy::abi_decode_params(&encoded).unwrap(),
            (
                address!("0x8497afefdc5ac170a664a231f6efb25526ef813f"),
                B256::repeat_byte(0x01),
                [0x02; 4].into(),
                "0x0000001F".into(),
            )
        );
    }

    #[test]
    fn decode_broken_utf8() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000004
    		e4b88de500000000000000000000000000000000000000000000000000000000
            "
        );

        assert_eq!(sol_data::String::abi_decode(&encoded).unwrap(), "不�".to_string());
    }

    #[test]
    #[cfg_attr(miri, ignore = "OOM https://github.com/rust-lang/miri/issues/3637")]
    fn decode_corrupted_dynamic_array() {
        type MyTy = sol_data::Array<sol_data::Uint<32>>;
        // line 1 at 0x00 =   0: tail offset of array
        // line 2 at 0x20 =  32: length of array
        // line 3 at 0x40 =  64: first word
        // line 4 at 0x60 =  96: second word
        let encoded = hex!(
            "
    	0000000000000000000000000000000000000000000000000000000000000020
    	00000000000000000000000000000000000000000000000000000000ffffffff
    	0000000000000000000000000000000000000000000000000000000000000001
    	0000000000000000000000000000000000000000000000000000000000000002
        "
        );
        assert!(MyTy::abi_decode_sequence(&encoded).is_err());
    }

    #[test]
    fn decode_verify_addresses() {
        let input = hex!(
            "
    	0000000000000000000000000000000000000000000000000000000000012345
    	0000000000000000000000000000000000000000000000000000000000054321
    	"
        );

        assert_eq!(
            sol_data::Address::abi_decode(&input).unwrap(),
            address!("0000000000000000000000000000000000012345")
        );
        assert!(<(sol_data::Address, sol_data::Address)>::abi_decode(&input).is_ok());
    }

    #[test]
    fn decode_verify_bytes() {
        type MyTy2 = (sol_data::Address, sol_data::Address);

        let input = hex!(
            "
    	0000000000000000000000001234500000000000000000000000000000012345
    	0000000000000000000000005432100000000000000000000000000000054321
    	"
        );
        assert!(MyTy2::abi_decode_params(&input).is_ok());
    }

    #[test]
    fn signed_int_dirty_high_bytes() {
        type MyTy = sol_data::Int<8>;

        let dirty_negative =
            hex!("f0ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

        assert_eq!(MyTy::abi_decode(&dirty_negative).unwrap(), -1);

        let dirty_positive =
            hex!("700000000000000000000000000000000000000000000000000000000000007f");

        assert_eq!(MyTy::abi_decode(&dirty_positive).unwrap(), 127);
    }

    // https://github.com/alloy-rs/core/issues/433
    #[test]
    fn fixed_before_dynamic() {
        sol! {
            #[derive(Debug, PartialEq, Eq)]
            struct Ty {
                bytes32[3] arr;
                bytes dyn;
            }
        }

        let ty = Ty {
            arr: [[0x11u8; 32].into(), [0x22u8; 32].into(), [0x33u8; 32].into()],
            r#dyn: bytes![0x44u8; 4],
        };
        let encoded = hex!(
            "0000000000000000000000000000000000000000000000000000000000000020"
            "1111111111111111111111111111111111111111111111111111111111111111"
            "2222222222222222222222222222222222222222222222222222222222222222"
            "3333333333333333333333333333333333333333333333333333333333333333"
            "0000000000000000000000000000000000000000000000000000000000000080"
            "0000000000000000000000000000000000000000000000000000000000000004"
            "4444444400000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(hex::encode(ty.abi_encode()), hex::encode(encoded));
        assert_eq!(ty.abi_encoded_size(), encoded.len());

        assert_eq!(<Ty as SolType>::abi_decode(&encoded).unwrap(), ty);
    }

    #[test]
    fn dynarray_before_dynamic() {
        sol! {
            #[derive(Debug, PartialEq, Eq)]
            struct Ty {
                bytes[3] arr;
                bytes dyn;
            }
        }

        let ty = Ty {
            arr: [bytes![0x11u8; 32], bytes![0x22u8; 32], bytes![0x33u8; 32]],
            r#dyn: bytes![0x44u8; 4],
        };
        let encoded = hex!(
            "0000000000000000000000000000000000000000000000000000000000000020" // struct offset
            "0000000000000000000000000000000000000000000000000000000000000040" // arr offset
            "0000000000000000000000000000000000000000000000000000000000000160" // dyn offset
            "0000000000000000000000000000000000000000000000000000000000000060" // arr[0] offset
            "00000000000000000000000000000000000000000000000000000000000000a0" // arr[1] offset
            "00000000000000000000000000000000000000000000000000000000000000e0" // arr[2] offset
            "0000000000000000000000000000000000000000000000000000000000000020" // arr[0]
            "1111111111111111111111111111111111111111111111111111111111111111"
            "0000000000000000000000000000000000000000000000000000000000000020" // arr[1]
            "2222222222222222222222222222222222222222222222222222222222222222"
            "0000000000000000000000000000000000000000000000000000000000000020" // arr[2]
            "3333333333333333333333333333333333333333333333333333333333333333"
            "0000000000000000000000000000000000000000000000000000000000000004" // dyn
            "4444444400000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(hex::encode(ty.abi_encode()), hex::encode(encoded));
        assert_eq!(ty.abi_encoded_size(), encoded.len());

        assert_eq!(<Ty as SolType>::abi_decode(&encoded).unwrap(), ty);
    }

    #[test]
    fn offset_overflow() {
        let encoded = hex!(
            "0000000000000000000000000000000000000000000000000000000000000020"
            "000000000000000000000000000000000000000000000000ffffffffffffffff"
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        let err = <sol_data::String as SolType>::abi_decode(&encoded).unwrap_err();
        assert_eq!(err, Error::Overrun);
    }

    #[test]
    fn config_defaults_and_setters() {
        let mut config = AbiDecoderConfig::new();
        assert_eq!(config.get_recursion_limit(), 16);
        assert_eq!(config.get_memory_limit(), DEFAULT_MEMORY_LIMIT);
        assert!(!config.get_strict());

        config.set_recursion_limit(300);
        config.set_memory_limit(42);
        config.set_strict(true);
        assert_eq!(config.get_recursion_limit(), 300);
        assert_eq!(config.get_memory_limit(), 42);
        assert!(config.get_strict());

        let config = AbiDecoderConfig::new().recursion_limit(400).memory_limit(24).strict(true);
        assert_eq!(config.get_recursion_limit(), 400);
        assert_eq!(config.get_memory_limit(), 24);
        assert!(config.get_strict());
    }

    #[test]
    fn configured_decoder_strict_validation() {
        let encoded = B256::repeat_byte(0x11);
        assert!(
            sol_data::Bool::abi_decode_with_config(encoded.as_slice(), AbiDecoderConfig::new())
                .is_ok()
        );
        assert!(
            sol_data::Bool::abi_decode_with_config(
                encoded.as_slice(),
                AbiDecoderConfig::new().strict(true),
            )
            .is_err()
        );
    }

    #[test]
    fn configured_decoder_enforces_memory_limit() {
        type Ty = sol_data::Array<sol_data::Uint<256>>;

        let encoded = Ty::abi_encode(&vec![U256::ZERO]);
        let err = decode_sequence_with_config::<<Ty as SolType>::Token<'_>>(
            &encoded,
            AbiDecoderConfig::new().memory_limit(31),
        )
        .unwrap_err();
        assert_eq!(err, Error::MemoryLimitExceeded(31));
    }

    #[test]
    fn direct_decoder_enforces_memory_limit() {
        type Ty = sol_data::Array<sol_data::Uint<256>>;

        let encoded = Ty::abi_encode(&vec![U256::ZERO]);
        let mut decoder = Decoder::with_config(&encoded, AbiDecoderConfig::new().memory_limit(31));
        let err = decoder.decode::<<Ty as SolType>::Token<'_>>().unwrap_err();
        assert_eq!(err, Error::MemoryLimitExceeded(31));
    }

    #[test]
    fn configured_decoder_tracks_child_allocations() {
        type Ty = sol_data::Array<sol_data::Array<sol_data::Uint<256>>>;

        let encoded = Ty::abi_encode(&vec![vec![U256::ZERO], vec![U256::ZERO]]);
        let err = decode_sequence_with_config::<<Ty as SolType>::Token<'_>>(
            &encoded,
            AbiDecoderConfig::new().memory_limit(100),
        )
        .unwrap_err();
        assert_eq!(err, Error::MemoryLimitExceeded(100));
    }

    #[test]
    fn configured_decoder_tracks_recursive_allocations() {
        type Uint = sol_data::Uint<256>;
        type Inner = sol_data::Array<Uint>;
        type Middle = sol_data::Array<Inner>;
        type Ty = sol_data::Array<Middle>;

        let value = vec![vec![vec![U256::ZERO]], vec![vec![U256::ZERO]]];
        let encoded = Ty::abi_encode(&value);
        let memory_used = 2 * core::mem::size_of::<<Middle as SolType>::Token<'_>>()
            + 2 * core::mem::size_of::<<Inner as SolType>::Token<'_>>()
            + 2 * core::mem::size_of::<<Uint as SolType>::Token<'_>>();

        decode_sequence_with_config::<<Ty as SolType>::Token<'_>>(
            &encoded,
            AbiDecoderConfig::new().memory_limit(memory_used),
        )
        .unwrap();
        let err = decode_sequence_with_config::<<Ty as SolType>::Token<'_>>(
            &encoded,
            AbiDecoderConfig::new().memory_limit(memory_used - 1),
        )
        .unwrap_err();
        assert_eq!(err, Error::MemoryLimitExceeded(memory_used - 1));
    }

    #[test]
    fn configured_decoder_tracks_aliased_tuple_allocations() {
        type Address = sol_data::Address;

        let width = 2_usize;
        let data = aliased_call_data(width);
        let memory_used = width * core::mem::size_of::<<Scope as SolType>::Token<'_>>()
            + width.pow(2) * core::mem::size_of::<<Rule as SolType>::Token<'_>>()
            + width.pow(3) * core::mem::size_of::<<Address as SolType>::Token<'_>>();

        applyCall::abi_decode_with_config(&data, AbiDecoderConfig::new().memory_limit(memory_used))
            .unwrap();
        let err = match applyCall::abi_decode_with_config(
            &data,
            AbiDecoderConfig::new().memory_limit(memory_used - 1),
        ) {
            Ok(_) => panic!("decoding should exceed the memory limit"),
            Err(err) => err,
        };
        assert_eq!(err, Error::MemoryLimitExceeded(memory_used - 1));
    }

    #[test]
    fn configured_decoder_enforces_recursion_limit() {
        type Ty = sol_data::Array<sol_data::Uint<256>>;

        let encoded = Ty::abi_encode(&vec![U256::ZERO]);
        let err = decode_sequence_with_config::<<Ty as SolType>::Token<'_>>(
            &encoded,
            AbiDecoderConfig::new().recursion_limit(0),
        )
        .unwrap_err();
        assert_eq!(err, Error::RecursionLimitExceeded(0));
    }
}
