// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utils used by different modules.

use crate::{Error, Result, Word};

/// Converts a u32 to a right aligned array of 32 bytes.
pub(crate) fn pad_u32(value: u32) -> Word {
    let mut padded = Word::default();
    padded[28..32].copy_from_slice(&value.to_be_bytes());
    padded
}

/// Return Ok(()). Exists for the UDT macro's typecheck.
#[doc(hidden)]
#[allow(clippy::missing_const_for_fn)] // clippy issue
#[inline]
pub fn just_ok<T>(_: T) -> crate::Result<()> {
    Ok(())
}

#[inline]
pub(crate) fn check_zeroes(data: &[u8]) -> bool {
    data.iter().all(|b| *b == 0)
}

#[inline]
pub(crate) const fn round_up_nearest_multiple(value: usize, padding: usize) -> usize {
    (value + padding - 1) / padding * padding
}

#[inline]
pub(crate) fn check_fixed_bytes(word: Word, len: usize) -> bool {
    if word.is_empty() {
        return true
    }
    match len {
        0 => panic!("cannot have bytes0"),
        1..=31 => check_zeroes(&word[len..]),
        32 => true, // always valid
        _ => panic!("cannot have bytes33 or higher"),
    }
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
