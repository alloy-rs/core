/// This trait is enabled when the "rlp" feature is active.
#[cfg(not(feature = "rlp"))]
pub trait RlpSuperSig {}

/// RLP encoding/decoding functionality for signatures.
/// This trait is enabled when the "rlp" feature is active.
#[cfg(feature = "rlp")]
pub trait RlpSuperSig: alloy_rlp::Encodable + alloy_rlp::Decodable {
    /// Decode an RLP-encoded VRS signature.
    #[cfg(feature = "rlp")]
    fn decode_rlp_vrs(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error>
    where
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
}

/// This trait is enabled when the "k256" feature is active.
#[cfg(not(feature = "k256"))]
pub trait K256SuperSig {}
#[cfg(feature = "k256")]
use crate::{Parity, SignatureError};
/// K256 (secp256k1) cryptographic functionality for signatures.
/// This trait provides methods for signature recovery and verification
/// when the "k256" feature is active.
#[cfg(feature = "k256")]
pub trait K256SuperSig: From<(k256::ecdsa::Signature, k256::ecdsa::RecoveryId)> {
    /// Recovers an [`Address`] from this signature and the given message by first prefixing and
    /// hashing the message according to [EIP-191](crate::eip191_hash_message).
    ///
    /// [`Address`]: crate::Address
    fn recover_address_from_msg<T: AsRef<[u8]>>(
        &self,
        msg: T,
    ) -> Result<crate::Address, SignatureError>;

    /// Recovers an [`Address`] from this signature and the given prehashed message.
    ///
    /// [`Address`]: crate::Address
    fn recover_address_from_prehash(
        &self,
        prehash: &crate::B256,
    ) -> Result<crate::Address, SignatureError>;

    /// Recovers a [`VerifyingKey`] from this signature and the given message by first prefixing and
    /// hashing the message according to [EIP-191](crate::eip191_hash_message).
    ///
    /// [`VerifyingKey`]: k256::ecdsa::VerifyingKey
    fn recover_from_msg<T: AsRef<[u8]>>(
        &self,
        msg: T,
    ) -> Result<k256::ecdsa::VerifyingKey, SignatureError>;

    /// Recovers a [`VerifyingKey`] from this signature and the given prehashed message.
    ///
    /// [`VerifyingKey`]: k256::ecdsa::VerifyingKey
    fn recover_from_prehash(
        &self,
        prehash: &crate::B256,
    ) -> Result<k256::ecdsa::VerifyingKey, SignatureError>;
    /// Returns the inner ECDSA signature.
    #[deprecated(note = "use `Signature::to_k256` instead")]
    fn into_inner(self) -> k256::ecdsa::Signature;

    /// Returns the inner ECDSA signature.
    fn to_k256(&self) -> Result<k256::ecdsa::Signature, k256::ecdsa::Error>;

    /// Instantiate from a signature and recovery id.
    fn from_signature_and_parity<T, E>(
        sig: k256::ecdsa::Signature,
        parity: T,
    ) -> Result<Self, SignatureError>
    where
        T: TryInto<Parity, Error = E>,
        E: Into<SignatureError>,
        Self: Sized;
    /// Returns the recovery ID.
    fn recid(&self) -> k256::ecdsa::RecoveryId;

    /// Deprecated - use `Signature::recid` instead
    fn recovery_id(&self) -> k256::ecdsa::RecoveryId;
}

/// This trait is enabled when the "serde" feature is active.
#[cfg(not(feature = "serde"))]
pub trait SerdeSuperSig {}

/// Serialization/deserialization support for signatures via serde.
/// This trait is enabled when the "serde" feature is active.
#[cfg(feature = "serde")]
pub trait SerdeSuperSig: serde::Serialize + for<'a> serde::Deserialize<'a> {}

/// This trait is enabled when the "arbitrary" feature is active.
#[cfg(not(feature = "arbitrary"))]
pub trait ArbitrarySuperSig {}

/// Property-based testing support for signatures.
/// This trait provides arbitrary instance generation capabilities
/// when the "arbitrary" feature is active.
#[cfg(feature = "arbitrary")]
pub trait ArbitrarySuperSig: for<'a> arbitrary::Arbitrary<'a> {}
