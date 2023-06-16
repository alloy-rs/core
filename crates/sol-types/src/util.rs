// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utils used by different modules.

use crate::{Error, Result, Word};

/// Calculates the padded length of a slice by rounding its length to the next
/// word
#[inline]
pub(crate) const fn words_for(data: &[u8]) -> usize {
    (data.len() + 31) / 32
}

/// `padded_len` rounds a slice length up to the next multiple of 32
#[inline]
pub(crate) const fn padded_len(data: &[u8]) -> usize {
    next_multiple_of_32(data.len())
}

/// See [`usize::next_multiple_of`].
#[inline]
pub const fn next_multiple_of_32(n: usize) -> usize {
    match n % 32 {
        0 => n,
        r => n + (32 - r),
    }
}

/// Converts a u32 to a right aligned array of 32 bytes.
#[inline]
pub(crate) fn pad_u32(value: u32) -> Word {
    let mut padded = Word::ZERO;
    padded[28..32].copy_from_slice(&value.to_be_bytes());
    padded
}

/// Return Ok(()). Exists for the UDT macro's typecheck.
#[doc(hidden)]
#[inline]
pub const fn just_ok<T>(_: &T) -> crate::Result<()> {
    Ok(())
}

#[inline]
pub(crate) fn check_zeroes(data: &[u8]) -> bool {
    data.iter().all(|b| *b == 0)
}

#[inline]
pub(crate) fn as_u32(word: Word, type_check: bool) -> Result<u32> {
    if type_check && !check_zeroes(&word[..28]) {
        return Err(Error::type_check_fail(
            &word[..],
            "Solidity pointer (uint32)",
        ))
    }

    let result = ((word[28] as u32) << 24)
        | ((word[29] as u32) << 16)
        | ((word[30] as u32) << 8)
        | (word[31] as u32);

    Ok(result)
}

#[inline]
pub(crate) fn check_bool(slice: Word) -> bool {
    check_zeroes(&slice[..31])
}

#[cfg(test)]
mod tests {
    use super::pad_u32;
    use hex_literal::hex;

    #[test]
    fn test_words_for() {
        assert_eq!(super::words_for(&[]), 0);
        assert_eq!(super::words_for(&[0; 31]), 1);
        assert_eq!(super::words_for(&[0; 32]), 1);
        assert_eq!(super::words_for(&[0; 33]), 2);
    }

    #[test]
    fn test_pad_u32() {
        // this will fail if endianness is not supported
        assert_eq!(
            pad_u32(0).to_vec(),
            hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec()
        );
        assert_eq!(
            pad_u32(1).to_vec(),
            hex!("0000000000000000000000000000000000000000000000000000000000000001").to_vec()
        );
        assert_eq!(
            pad_u32(0x100).to_vec(),
            hex!("0000000000000000000000000000000000000000000000000000000000000100").to_vec()
        );
        assert_eq!(
            pad_u32(0xffffffff).to_vec(),
            hex!("00000000000000000000000000000000000000000000000000000000ffffffff").to_vec()
        );
    }
}
