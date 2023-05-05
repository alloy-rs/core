// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utils used by different modules.

use crate::{AbiResult, Error, Word};

/// Converts a u32 to a right aligned array of 32 bytes.
pub(crate) fn pad_u32(value: u32) -> Word {
    let mut padded = Word::default();
    padded[28..32].copy_from_slice(&value.to_be_bytes());
    padded
}

// clippy issue
#[doc(hidden)]
#[allow(clippy::missing_const_for_fn)]
/// Return Ok(()). Exists for the UDT macro's typecheck
pub fn just_ok<T>(_: T) -> crate::AbiResult<()> {
    Ok(())
}

pub(crate) fn check_zeroes(data: &[u8]) -> bool {
    data.iter().all(|b| *b == 0)
}

pub(crate) const fn round_up_nearest_multiple(value: usize, padding: usize) -> usize {
    (value + padding - 1) / padding * padding
}

pub(crate) fn check_fixed_bytes(word: Word, len: usize) -> bool {
    if word.is_empty() {
        return true;
    }
    match len {
        0 => panic!("cannot have bytes0"),
        1..=31 => check_zeroes(&word[len..]),
        32 => true, // always valid
        _ => panic!("cannot have bytes33 or higher"),
    }
}

pub(crate) fn as_u32(word: Word, type_check: bool) -> AbiResult<u32> {
    if type_check && !check_zeroes(&word.as_slice()[..28]) {
        return Err(Error::type_check_fail(
            hex::encode(word),
            "Solidity pointer (uint32)",
        ));
    }

    let result = ((word[28] as u32) << 24)
        + ((word[29] as u32) << 16)
        + ((word[30] as u32) << 8)
        + (word[31] as u32);

    Ok(result)
}

pub(crate) fn check_bool(slice: Word) -> bool {
    check_zeroes(&slice[..31])
}

#[cfg(feature = "eip712-serde")]
pub(crate) use serde_helper::*;

#[cfg(feature = "eip712-serde")]
mod serde_helper {
    use ethers_primitives::U256;
    use serde::{Deserialize, Deserializer};

    /// Helper type to parse numeric strings, `u64` and `U256`
    #[derive(Deserialize, Debug, Clone)]
    #[serde(untagged)]
    pub(crate) enum StringifiedNumeric {
        String(String),
        U256(U256),
        Num(u64),
    }

    impl TryFrom<StringifiedNumeric> for U256 {
        type Error = String;

        fn try_from(value: StringifiedNumeric) -> Result<Self, Self::Error> {
            match value {
                StringifiedNumeric::U256(n) => Ok(n),
                StringifiedNumeric::Num(n) => Ok(U256::from(n)),
                StringifiedNumeric::String(s) => {
                    if let Ok(val) = s.parse::<u128>() {
                        Ok(U256::from(val))
                    } else if s.starts_with("0x") {
                        U256::from_str_radix(s.strip_prefix("0x").unwrap(), 16)
                            .map_err(|err| err.to_string())
                    } else {
                        U256::from_str_radix(&s, 10).map_err(|err| err.to_string())
                    }
                }
            }
        }
    }

    /// Supports parsing numbers as strings
    ///
    /// See <https://github.com/gakonst/ethers-rs/issues/1507>
    pub(crate) fn deserialize_stringified_numeric_opt<'de, D>(
        deserializer: D,
    ) -> Result<Option<U256>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Some(num) = Option::<StringifiedNumeric>::deserialize(deserializer)? {
            num.try_into().map(Some).map_err(serde::de::Error::custom)
        } else {
            Ok(None)
        }
    }
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
