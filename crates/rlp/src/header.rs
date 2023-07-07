use crate::{EMPTY_LIST_CODE, EMPTY_STRING_CODE};
use bytes::BufMut;

/// The header of an RLP item.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Header {
    /// True if list, false otherwise.
    pub list: bool,
    /// Length of the payload in bytes.
    pub payload_length: usize,
}

impl Header {
    /// Encodes the header into the `out` buffer.
    #[inline]
    pub fn encode(&self, out: &mut dyn BufMut) {
        if self.payload_length < 56 {
            let code = if self.list {
                EMPTY_LIST_CODE
            } else {
                EMPTY_STRING_CODE
            };
            out.put_u8(code + self.payload_length as u8);
        } else {
            let len_be;
            let len_be = crate::encode::to_be_bytes_trimmed!(len_be, self.payload_length);
            let code = if self.list { 0xF7 } else { 0xB7 };
            out.put_u8(code + len_be.len() as u8);
            out.put_slice(len_be);
        }
    }

    /// Returns the length of the encoded header.
    #[inline]
    pub const fn length(&self) -> usize {
        crate::length_of_length(self.payload_length)
    }
}
