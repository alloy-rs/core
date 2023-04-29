use core::fmt;

static CHARS: &[u8] = b"0123456789abcdef";

/// Decoding bytes from hex string error.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FromHexError {
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
        match *self {
            Self::InvalidHex { character, index } => {
                write!(fmt, "invalid hex character: {character}, at {index}")
            }
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
    assert!(v.len() > 1 + bytes.len() * 2);

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

/// Decode given 0x-prefix-stripped hex string into provided slice.
/// Used for address checksumming and the `serde` feature to implement
/// `deserialize_check_len`.
///
/// The method will panic if `bytes` have incorrect length (make sure to
/// allocate enough beforehand).
pub(crate) fn from_hex_raw(
    v: &str,
    bytes: &mut [u8],
    stripped: bool,
) -> Result<usize, FromHexError> {
    let bytes_len = v.len();
    let mut modulus = bytes_len % 2;
    let mut buf = 0;
    let mut pos = 0;
    for (index, byte) in v.bytes().enumerate() {
        buf <<= 4;

        match byte {
            b'A'..=b'F' => buf |= byte - b'A' + 10,
            b'a'..=b'f' => buf |= byte - b'a' + 10,
            b'0'..=b'9' => buf |= byte - b'0',
            b' ' | b'\r' | b'\n' | b'\t' => {
                buf >>= 4;
                continue;
            }
            b => {
                let character = char::from(b);
                return Err(FromHexError::InvalidHex {
                    character,
                    index: index + if stripped { 2 } else { 0 },
                });
            }
        }

        modulus += 1;
        if modulus == 2 {
            modulus = 0;
            bytes[pos] = buf;
            pos += 1;
        }
    }

    Ok(pos)
}
