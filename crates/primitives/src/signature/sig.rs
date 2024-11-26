use crate::{
    signature::super_sig::{ArbitrarySuperSig, K256SuperSig, RlpSuperSig, SerdeSuperSig},
    Parity, SignatureError, U256,
};
use core::str::FromStr;

/// An Ethereum Generic signature.
pub trait Signature:
    for<'a> TryFrom<&'a [u8], Error = SignatureError>
    + FromStr<Err = SignatureError>
    + RlpSuperSig
    + K256SuperSig
    + SerdeSuperSig
    + ArbitrarySuperSig
{
    /// Instantiate a new signature from `r`, `s`, and `v` values.
    fn new(r: U256, s: U256, v: Parity) -> Self;

    /// Returns the `r` component of this signature.
    fn r(&self) -> U256;

    /// Returns the `s` component of this signature.
    fn s(&self) -> U256;

    /// Returns the recovery ID as a `u8`.
    fn v(&self) -> Parity;

    /// Returns the chain ID associated with the V value, if this signature is
    /// replay-protected by [EIP-155].
    ///
    /// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
    fn chain_id(&self) -> Option<u64>;

    /// Returns true if the signature is replay-protected by [EIP-155].
    ///
    /// This is true if the V value is 35 or greater. Values less than 35 are
    /// either not replay protected (27/28), or are invalid.
    ///
    /// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
    fn has_eip155_value(&self) -> bool;

    /// Returns the byte-array representation of this signature.
    fn as_bytes(&self) -> [u8; 65];

    /// Normalize the signature into "low S" form.
    fn normalize_s(&self) -> Option<Self>
    where
        Self: Sized;

    /// Modifies the recovery ID by applying EIP-155 to a `v` value.
    fn with_chain_id(self, chain_id: u64) -> Self
    where
        Self: Sized;

    /// Modifies the recovery ID by dropping any EIP-155 v value.
    fn with_parity_bool(self) -> Self
    where
        Self: Sized;

    /// Sets the recovery ID by normalizing a `v` value.
    fn with_parity<T: Into<Parity>>(self, parity: T) -> Self
    where
        Self: Sized;

    /// Creates a signature from the serialized `r`, `s` scalar values and a `parity` value.
    fn from_scalars_and_parity<T, E>(
        r: crate::B256,
        s: crate::B256,
        parity: T,
    ) -> Result<Self, SignatureError>
    where
        T: TryInto<Parity, Error = E>,
        E: Into<SignatureError>,
        Self: Sized;

    /// Parses a signature from a byte slice and a `parity` value.
    fn from_bytes_and_parity<T, E>(bytes: &[u8], parity: T) -> Result<Self, SignatureError>
    where
        T: TryInto<Parity, Error = E>,
        E: Into<SignatureError>,
        Self: Sized;

    /// Instantiate from `r`, `s`, and `parity`.
    fn from_rs_and_parity<T, E>(r: U256, s: U256, parity: T) -> Result<Self, SignatureError>
    where
        T: TryInto<Parity, Error = E>,
        E: Into<SignatureError>,
        Self: Sized;
}
