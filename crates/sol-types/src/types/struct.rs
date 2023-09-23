//! This module contains the [`SolStruct`] trait, which is used to implement
//! Solidity structs logic, particularly for EIP-712 encoding/decoding.

use super::SolType;
use crate::Eip712Domain;
use alloc::{borrow::Cow, vec::Vec};
use alloy_primitives::{keccak256, B256};

/// A Solidity Struct.
///
/// This trait is used to implement ABI and EIP-712 encoding and decoding.
///
/// # Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity struct
/// definition.
///
/// # Note
///
/// Special attention should be paid to [`eip712_encode_type`] for complex
/// Solidity types. Nested Solidity structs **must** properly encode their type.
///
/// To be clear, a struct with a nested struct must encode the nested struct's
/// type as well.
///
/// See [EIP-712#definition-of-encodetype][ref] for more details.
///
/// [`eip712_encode_type`]: SolStruct::eip712_encode_type
/// [ref]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
pub trait SolStruct: SolType<RustType = Self> {
    /// The struct name.
    ///
    /// Used in [`eip712_encode_type`][SolType::sol_type_name].
    const NAME: &'static str;

    /// Returns component EIP-712 types. These types are used to construct
    /// the `encodeType` string. These are the types of the struct's fields,
    /// and should not include the root type.
    fn eip712_components() -> Vec<Cow<'static, str>>;

    /// Return the root EIP-712 type. This type is used to construct the
    /// `encodeType` string.
    fn eip712_root_type() -> Cow<'static, str>;

    /// EIP-712 `encodeType`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    fn eip712_encode_type() -> Cow<'static, str> {
        let root_type = Self::eip712_root_type();
        let mut components = Self::eip712_components();

        if components.is_empty() {
            return root_type
        }

        components.sort_unstable();
        components.dedup();
        Cow::Owned(core::iter::once(root_type).chain(components).collect())
    }

    /// EIP-712 `typeHash`
    /// <https://eips.ethereum.org/EIPS/eip-712#rationale-for-typehash>
    #[inline]
    fn eip712_type_hash(&self) -> B256 {
        keccak256(<Self as SolStruct>::eip712_encode_type().as_bytes())
    }

    /// EIP-712 `encodeData`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    fn eip712_encode_data(&self) -> Vec<u8>;

    /// EIP-712 `hashStruct`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct>
    #[inline]
    fn eip712_hash_struct(&self) -> B256 {
        let mut type_hash = self.eip712_type_hash().to_vec();
        type_hash.extend(self.eip712_encode_data());
        keccak256(type_hash)
    }

    /// EIP-712 `signTypedData`
    /// <https://eips.ethereum.org/EIPS/eip-712#specification-of-the-eth_signtypeddata-json-rpc>
    #[inline]
    fn eip712_signing_hash(&self, domain: &Eip712Domain) -> B256 {
        let domain_separator = domain.hash_struct();
        let struct_hash = self.eip712_hash_struct();

        let mut digest_input = [0u8; 2 + 32 + 32];
        digest_input[0] = 0x19;
        digest_input[1] = 0x01;
        digest_input[2..34].copy_from_slice(&domain_separator[..]);
        digest_input[34..66].copy_from_slice(&struct_hash[..]);
        keccak256(digest_input)
    }
}
