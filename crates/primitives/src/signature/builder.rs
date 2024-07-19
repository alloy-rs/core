use crate::{Parity, SignatureError, U256};

/// Trait used to uniformize signature creation.
pub trait SignatureBuilder<S>: Sized {
    /// Instantiate from v, r, s.
    fn from_rs_and_parity<P: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        r: U256,
        s: U256,
        parity: P,
    ) -> Result<Self, SignatureError>;

    /// Parses a signature from a byte slice, with a v value
    fn from_bytes_and_parity<P: TryInto<Parity, Error = E>, E: Into<SignatureError>>(
        bytes: &[u8],
        parity: P,
    ) -> Result<Self, SignatureError>;
}
