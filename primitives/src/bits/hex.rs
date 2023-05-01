use core::fmt;

static CHARS: &[u8] = b"0123456789abcdef";

/// Decoding bytes from hex string error.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FromHexError {
    OddLength,
    /// Invalid (non-hex) character encountered.
    InvalidHex {
        /// The unexpected character.
        character: char,
        /// Index of that occurrence.
        index: usize,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {}

impl fmt::Display for FromHexError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHex { character, index } => {
                write!(fmt, "invalid hex character: {character}, at {index}")
            }
            Self::OddLength => write!(fmt, "Odd number of hex digits"),
        }
    }
}

/// Encode given bytes into hex in the `v` buffer
pub(crate) fn to_hex_raw<'a>(
    v: &'a mut [u8],
    bytes: &[u8],
    skip_leading_zero: bool,
    add_prefix: bool,
) -> &'a str {
    let necessary_len = 2 * (bytes.len() + add_prefix as usize);
    debug_assert!(
        v.len() >= necessary_len,
        "must supply a buffer of length {} or greater",
        necessary_len
    );

    let mut idx = 0;
    if add_prefix {
        v[0] = b'0';
        v[1] = b'x';
        idx = 2;
    }

    let first_nibble = bytes[0] >> 4;
    if first_nibble != 0 || !skip_leading_zero {
        v[idx] = CHARS[first_nibble as usize];
        idx += 1;
    }
    v[idx] = CHARS[(bytes[0] & 0xf) as usize];
    idx += 1;

    for &byte in bytes.iter().skip(1) {
        v[idx] = CHARS[(byte >> 4) as usize];
        v[idx + 1] = CHARS[(byte & 0xf) as usize];
        idx += 2;
    }

    // SAFETY: all characters come either from CHARS or "0x", therefore valid UTF8
    unsafe { core::str::from_utf8_unchecked(&v[0..idx]) }
}

/// Decode given hex string into provided slice.

/// The method will panic if the output buffer is short (so make sure to
/// allocate enough beforehand).
pub(crate) fn from_hex_raw(v: &str, bytes: &mut [u8]) -> Result<usize, FromHexError> {
    if v.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }

    let mut stripped = false;
    let v = v.strip_prefix("0x").unwrap_or_else(|| {
        stripped = true;
        v
    });

    v.as_bytes()
        .chunks_exact(2)
        .enumerate()
        .try_for_each(|(index, pair): (usize, &[u8])| {
            let mut buf = 0;

            match pair[0] {
                b'A'..=b'F' => buf |= pair[0] - b'A' + 10,
                b'a'..=b'f' => buf |= pair[0] - b'a' + 10,
                b'0'..=b'9' => buf |= pair[0] - b'0',
                b => {
                    let character = char::from(b);
                    return Err(FromHexError::InvalidHex {
                        character,
                        index: (index * 2) + if stripped { 2 } else { 0 },
                    });
                }
            }

            buf <<= 4;

            match pair[1] {
                b'A'..=b'F' => buf |= pair[1] - b'A' + 10,
                b'a'..=b'f' => buf |= pair[1] - b'a' + 10,
                b'0'..=b'9' => buf |= pair[1] - b'0',
                b => {
                    let character = char::from(b);
                    return Err(FromHexError::InvalidHex {
                        character,
                        index: (index * 2) + 1 + if stripped { 2 } else { 0 },
                    });
                }
            }

            bytes[index] = buf;

            Ok(())
        })?;

    Ok(v.len() / 2)
}
