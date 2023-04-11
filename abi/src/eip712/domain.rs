use ethers_primitives::{B160, B256, U256};

use crate::{sol_type, util::keccak256, SolType};

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Eip712 Domain attributes used in determining the domain separator;
/// Unused fields are left out of the struct type.
///
/// Protocol designers only need to include the fields that make sense for
/// their signing domain. Unused fields are left out of the struct type.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "eip712-serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "eip712-serde", serde(rename_all = "camelCase"))]
pub struct Eip712Domain {
    ///  The user readable name of signing domain, i.e. the name of the DApp or the protocol.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<String>,

    /// The current major version of the signing domain. Signatures from different versions are not
    /// compatible.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub version: Option<String>,

    /// The EIP-155 chain id. The user-agent should refuse signing if it does not match the
    /// currently active chain.
    // TODO: re-enable serde_helpers
    #[cfg_attr(
        feature = "eip712-serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            // deserialize_with = "crate::types::serde_helpers::deserialize_stringified_numeric_opt"
        )
    )]
    pub chain_id: Option<U256>,

    /// The address of the contract that will verify the signature.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub verifying_contract: Option<B160>,

    /// A disambiguating salt for the protocol. This can be used as a domain separator of last
    /// resort.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub salt: Option<B256>,
}

impl Eip712Domain {
    const NAME: &'static str = "EIP712Domain";

    /// Calculate the domain separator for the domain object.
    pub fn separator(&self) -> B256 {
        self.hash_struct()
    }

    /// EIP-712 `encodeType`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    pub fn encode_type(&self) -> String {
        let mut ty = format!("{}(", Self::NAME);
        if self.name.is_some() {
            ty.push_str("string name,");
        }
        if self.version.is_some() {
            ty.push_str("string version,");
        }
        if self.chain_id.is_some() {
            ty.push_str("uint256 chainId,");
        }
        if self.verifying_contract.is_some() {
            ty.push_str("address verifyingContract,");
        }
        if self.salt.is_some() {
            ty.push_str("bytes32 salt,");
        }
        if ty.ends_with(',') {
            ty.pop();
        }
        ty.push(')');
        ty
    }

    /// EIP-712 `typeHash`
    /// <https://eips.ethereum.org/EIPS/eip-712#rationale-for-typehash>
    pub fn type_hash(&self) -> B256 {
        keccak256(self.encode_type().as_bytes())
    }

    /// EIP-712 `encodeData`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    pub fn encode_data(&self) -> Vec<u8> {
        // This giant match block was produced with excel-based
        // meta-programming lmao
        match (
            self.name.as_ref(),
            self.version.as_ref(),
            self.chain_id,
            self.verifying_contract,
            self.salt,
        ) {
            (None, None, None, None, None) => vec![],
            (None, None, None, None, Some(salt)) => {
                <(sol_type::FixedBytes<32>,)>::encode((salt.0,))
            }
            (None, None, None, Some(verifying_contract), None) => {
                <(sol_type::Address,)>::encode((verifying_contract,))
            }
            (None, None, None, Some(verifying_contract), Some(salt)) => {
                <(sol_type::Address, sol_type::FixedBytes<32>)>::encode((
                    verifying_contract,
                    salt.0,
                ))
            }
            (None, None, Some(chain_id), None, None) => {
                <(sol_type::Uint<256>,)>::encode((chain_id,))
            }
            (None, None, Some(chain_id), None, Some(salt)) => {
                <(sol_type::Uint<256>, sol_type::FixedBytes<32>)>::encode((chain_id, salt.0))
            }
            (None, None, Some(chain_id), Some(verifying_contract), None) => {
                <(sol_type::Uint<256>, sol_type::Address)>::encode((chain_id, verifying_contract))
            }
            (None, None, Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::Uint<256>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((chain_id, verifying_contract, salt.0))
            }
            (None, Some(version), None, None, None) => {
                <(sol_type::FixedBytes<32>,)>::encode((keccak256(version).0,))
            }
            (None, Some(version), None, None, Some(salt)) => {
                <(sol_type::FixedBytes<32>, sol_type::FixedBytes<32>)>::encode((
                    keccak256(version).0,
                    salt.0,
                ))
            }
            (None, Some(version), None, Some(verifying_contract), None) => {
                <(sol_type::FixedBytes<32>, sol_type::Address)>::encode((
                    keccak256(version).0,
                    verifying_contract,
                ))
            }
            (None, Some(version), None, Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((keccak256(version).0, verifying_contract, salt.0))
            }
            (None, Some(version), Some(chain_id), None, None) => {
                <(sol_type::FixedBytes<32>, sol_type::Uint<256>)>::encode((
                    keccak256(version).0,
                    chain_id,
                ))
            }
            (None, Some(version), Some(chain_id), None, Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::FixedBytes<32>,
                )>::encode((keccak256(version).0, chain_id, salt.0))
            }
            (None, Some(version), Some(chain_id), Some(verifying_contract), None) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::Address,
                )>::encode((keccak256(version).0, chain_id, verifying_contract))
            }
            (None, Some(version), Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((
                    keccak256(version).0,
                    chain_id,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), None, None, None, None) => {
                <(sol_type::FixedBytes<32>,)>::encode((keccak256(name).0,))
            }
            (Some(name), None, None, None, Some(salt)) => {
                <(sol_type::FixedBytes<32>, sol_type::FixedBytes<32>)>::encode((
                    keccak256(name).0,
                    salt.0,
                ))
            }
            (Some(name), None, None, Some(verifying_contract), None) => {
                <(sol_type::FixedBytes<32>, sol_type::Address)>::encode((
                    keccak256(name).0,
                    verifying_contract,
                ))
            }
            (Some(name), None, None, Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((keccak256(name).0, verifying_contract, salt.0))
            }
            (Some(name), None, Some(chain_id), None, None) => {
                <(sol_type::FixedBytes<32>, sol_type::Uint<256>)>::encode((
                    keccak256(name).0,
                    chain_id,
                ))
            }
            (Some(name), None, Some(chain_id), None, Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::FixedBytes<32>,
                )>::encode((keccak256(name).0, chain_id, salt.0))
            }
            (Some(name), None, Some(chain_id), Some(verifying_contract), None) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::Address,
                )>::encode((keccak256(name).0, chain_id, verifying_contract))
            }
            (Some(name), None, Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((
                    keccak256(name).0,
                    chain_id,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), Some(version), None, None, None) => {
                <(sol_type::FixedBytes<32>, sol_type::FixedBytes<32>)>::encode((
                    keccak256(name).0,
                    keccak256(version).0,
                ))
            }
            (Some(name), Some(version), None, None, Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                )>::encode((keccak256(name).0, keccak256(version).0, salt.0))
            }
            (Some(name), Some(version), None, Some(verifying_contract), None) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::Address,
                )>::encode((
                    keccak256(name).0,
                    keccak256(version).0,
                    verifying_contract,
                ))
            }
            (Some(name), Some(version), None, Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((
                    keccak256(name).0,
                    keccak256(version).0,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), Some(version), Some(chain_id), None, None) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                )>::encode((keccak256(name).0, keccak256(version).0, chain_id))
            }
            (Some(name), Some(version), Some(chain_id), None, Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::FixedBytes<32>,
                )>::encode((
                    keccak256(name).0,
                    keccak256(version).0,
                    chain_id,
                    salt.0,
                ))
            }
            (Some(name), Some(version), Some(chain_id), Some(verifying_contract), None) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::Address,
                )>::encode((
                    keccak256(name).0,
                    keccak256(version).0,
                    chain_id,
                    verifying_contract,
                ))
            }
            (Some(name), Some(version), Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_type::FixedBytes<32>,
                    sol_type::FixedBytes<32>,
                    sol_type::Uint<256>,
                    sol_type::Address,
                    sol_type::FixedBytes<32>,
                )>::encode((
                    keccak256(name).0,
                    keccak256(version).0,
                    chain_id,
                    verifying_contract,
                    salt.0,
                ))
            }
        }
    }

    /// EIP-712 `hashStruct`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct>
    pub fn hash_struct(&self) -> B256 {
        let mut type_hash = self.type_hash().to_vec();
        type_hash.extend(self.encode_data());
        keccak256(type_hash)
    }
}
