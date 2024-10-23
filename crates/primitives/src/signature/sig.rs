use crate::{Parity, SignatureError, U256};
use core::str::FromStr;

/// An Ethereum Generic signature.
pub trait Signature<'a>:
    TryFrom<&'a [u8], Error = SignatureError> + FromStr<Err = SignatureError>
{
    /// Decode an RLP-encoded VRS signature.
    #[cfg(feature = "rlp")]
    fn decode_rlp_vrs(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error>
    where
        Self: Sized;

    #[doc(hidden)]
    fn test_signature() -> Self
    where
        Self: Sized;

    /// Returns the byte-array representation of this signature.
    fn as_bytes(&self) -> [u8; 65];

    /// Normalize the signature into "low S" form.
    fn normalize_s(&self) -> Option<Self>
    where
        Self: Sized;

    /// Recovers an [`Address`] from this signature and the given message by first prefixing and
    /// hashing the message according to [EIP-191](crate::eip191_hash_message).
    ///
    /// [`Address`]: crate::Address
    #[cfg(feature = "k256")]
    fn recover_address_from_msg<T: AsRef<[u8]>>(
        &self,
        msg: T,
    ) -> Result<crate::Address, SignatureError>;

    /// Recovers an [`Address`] from this signature and the given prehashed message.
    ///
    /// [`Address`]: crate::Address
    #[cfg(feature = "k256")]
    fn recover_address_from_prehash(
        &self,
        prehash: &crate::B256,
    ) -> Result<crate::Address, SignatureError>;

    /// Recovers a [`VerifyingKey`] from this signature and the given message by first prefixing and
    /// hashing the message according to [EIP-191](crate::eip191_hash_message).
    ///
    /// [`VerifyingKey`]: k256::ecdsa::VerifyingKey
    #[cfg(feature = "k256")]
    fn recover_from_msg<T: AsRef<[u8]>>(
        &self,
        msg: T,
    ) -> Result<k256::ecdsa::VerifyingKey, SignatureError>;

    /// Recovers a [`VerifyingKey`] from this signature and the given prehashed message.
    ///
    /// [`VerifyingKey`]: k256::ecdsa::VerifyingKey
    #[cfg(feature = "k256")]
    fn recover_from_prehash(
        &self,
        prehash: &crate::B256,
    ) -> Result<k256::ecdsa::VerifyingKey, SignatureError>;

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

    /// Length of RLP RS field encoding
    #[cfg(feature = "rlp")]
    fn rlp_rs_len(&self) -> usize;

    /// Length of RLP V field encoding
    #[cfg(feature = "rlp")]
    fn rlp_vrs_len(&self) -> usize;

    /// Write R and S to an RLP buffer in progress.
    #[cfg(feature = "rlp")]
    fn write_rlp_rs(&self, out: &mut dyn alloy_rlp::BufMut);

    /// Write the V to an RLP buffer without using EIP-155.
    #[cfg(feature = "rlp")]
    fn write_rlp_v(&self, out: &mut dyn alloy_rlp::BufMut);

    /// Write the VRS to the output. The V will always be 27 or 28.
    #[cfg(feature = "rlp")]
    fn write_rlp_vrs(&self, out: &mut dyn alloy_rlp::BufMut);

    /// Returns the inner ECDSA signature.
    #[cfg(feature = "k256")]
    #[deprecated(note = "use `Signature::to_k256` instead")]
    fn into_inner(self) -> k256::ecdsa::Signature;

    /// Returns the inner ECDSA signature.
    #[cfg(feature = "k256")]
    fn to_k256(&self) -> Result<k256::ecdsa::Signature, k256::ecdsa::Error>;

    /// Instantiate from a signature and recovery id.
    #[cfg(feature = "k256")]
    fn from_signature_and_parity<T, E>(
        sig: k256::ecdsa::Signature,
        parity: T,
    ) -> Result<Self, SignatureError>
    where
        T: TryInto<Parity, Error = E>,
        E: Into<SignatureError>,
        Self: Sized;
}
