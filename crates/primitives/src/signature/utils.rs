use crate::ChainId;

/// Applies [EIP-155](https://eips.ethereum.org/EIPS/eip-155).
#[inline]
pub const fn to_eip155_v(v: u8, chain_id: ChainId) -> ChainId {
    (v as u64) + 35 + chain_id * 2
}

/// Attempts to normalize the v value to a boolean parity value. Returns None if the value is
/// invalid for any of the known Ethereum parity encodings.
pub const fn normalize_v(v: u64) -> Option<bool> {
    match v {
        // Case 1: raw/bare
        0 => Some(false),
        1 => Some(true),
        // Case 2: non-EIP-155 v value
        27 => Some(false),
        28 => Some(true),
        // Case 3: EIP-155 V value
        35.. => Some(((v - 35) % 2) != 0),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "k256")]
    fn normalizes_v() {
        use super::*;
        assert_eq!(normalize_v(27), Some(false));
        assert_eq!(normalize_v(28), Some(true));
    }
}
