use bytes::{BufMut, BytesMut};

/// The header for an RLP item,
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Header {
    /// True if list, false otherwise
    pub list: bool,
    /// Length of the payload in bytes
    pub payload_length: usize,
}

impl Header {
    /// Encodes the header into the `out` buffer.
    pub fn encode(&self, out: &mut dyn BufMut) {
        if self.payload_length < 56 {
            let code = if self.list {
                EMPTY_LIST_CODE
            } else {
                EMPTY_STRING_CODE
            };
            out.put_u8(code + self.payload_length as u8);
        } else {
            let len_be = self.payload_length.to_be_bytes();
            let len_be = crate::encode::zeroless_view(&len_be);
            let code = if self.list { 0xF7 } else { 0xB7 };
            out.put_u8(code + len_be.len() as u8);
            out.put_slice(len_be);
        }
    }

    /// Returns the length of the encoded header
    pub fn length(&self) -> usize {
        let mut out = BytesMut::new();
        self.encode(&mut out);
        out.len()
    }
}

/// RLP prefix byte for 0-length string.
pub const EMPTY_STRING_CODE: u8 = 0x80;

/// RLP prefix byte for a 0-length array.
pub const EMPTY_LIST_CODE: u8 = 0xC0;
