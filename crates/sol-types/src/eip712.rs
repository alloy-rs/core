use crate::{
    no_std_prelude::{Cow, String, Vec},
    sol_data, SolType,
};
use alloy_primitives::{keccak256, Address, B256, U256};

/// Eip712 Domain attributes used in determining the domain separator;
/// Unused fields are left out of the struct type.
///
/// Protocol designers only need to include the fields that make sense for
/// their signing domain. Unused fields are left out of the struct type.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "eip712-serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "eip712-serde", serde(rename_all = "camelCase"))]
pub struct Eip712Domain {
    ///  The user readable name of signing domain, i.e. the name of the DApp or
    /// the protocol.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Cow<'static, str>>,

    /// The current major version of the signing domain. Signatures from
    /// different versions are not compatible.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub version: Option<Cow<'static, str>>,

    /// The EIP-155 chain id. The user-agent should refuse signing if it does
    /// not match the currently active chain.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub chain_id: Option<U256>,

    /// The address of the contract that will verify the signature.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub verifying_contract: Option<Address>,

    /// A disambiguating salt for the protocol. This can be used as a domain
    /// separator of last resort.
    #[cfg_attr(
        feature = "eip712-serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub salt: Option<B256>,
}

impl Eip712Domain {
    /// The name of the struct.
    pub const NAME: &'static str = "EIP712Domain";

    /// Instantiate a new domain.
    #[inline]
    pub const fn new(
        name: Option<Cow<'static, str>>,
        version: Option<Cow<'static, str>>,
        chain_id: Option<U256>,
        verifying_contract: Option<Address>,
        salt: Option<B256>,
    ) -> Self {
        Self {
            name,
            version,
            chain_id,
            verifying_contract,
            salt,
        }
    }

    /// Calculate the domain separator for the domain object.
    #[inline]
    pub fn separator(&self) -> B256 {
        self.hash_struct()
    }

    /// EIP-712 `encodeType`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    pub fn encode_type(&self) -> String {
        // commas not included
        macro_rules! encode_type {
            ($($field:ident => $repr:literal),+ $(,)?) => {
                let mut ty = String::with_capacity(Self::NAME.len() + 2 $(+ $repr.len() * self.$field.is_some() as usize)+);
                ty.push_str(Self::NAME);
                ty.push('(');

                $(
                    if self.$field.is_some() {
                        ty.push_str($repr);
                    }
                )+
                if ty.ends_with(',') {
                    ty.pop();
                }

                ty.push(')');
                ty
            };
        }

        encode_type! {
            name               => "string name,",
            version            => "string version,",
            chain_id           => "uint256 chainId,",
            verifying_contract => "address verifyingContract,",
            salt               => "bytes32 salt",
        }
    }

    /// EIP-712 `typeHash`
    /// <https://eips.ethereum.org/EIPS/eip-712#rationale-for-typehash>
    #[inline]
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
                <(sol_data::FixedBytes<32>,)>::encode((salt.0,))
            }
            (None, None, None, Some(verifying_contract), None) => {
                <(sol_data::Address,)>::encode((verifying_contract,))
            }
            (None, None, None, Some(verifying_contract), Some(salt)) => {
                <(sol_data::Address, sol_data::FixedBytes<32>)>::encode((
                    verifying_contract,
                    salt.0,
                ))
            }
            (None, None, Some(chain_id), None, None) => {
                <(sol_data::Uint<256>,)>::encode((chain_id,))
            }
            (None, None, Some(chain_id), None, Some(salt)) => {
                <(sol_data::Uint<256>, sol_data::FixedBytes<32>)>::encode((chain_id, salt.0))
            }
            (None, None, Some(chain_id), Some(verifying_contract), None) => {
                <(sol_data::Uint<256>, sol_data::Address)>::encode((chain_id, verifying_contract))
            }
            (None, None, Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::Uint<256>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((chain_id, verifying_contract, salt.0))
            }
            (None, Some(version), None, None, None) => {
                <(sol_data::FixedBytes<32>,)>::encode((keccak256(version.as_bytes()).0,))
            }
            (None, Some(version), None, None, Some(salt)) => {
                <(sol_data::FixedBytes<32>, sol_data::FixedBytes<32>)>::encode((
                    keccak256(version.as_bytes()).0,
                    salt.0,
                ))
            }
            (None, Some(version), None, Some(verifying_contract), None) => {
                <(sol_data::FixedBytes<32>, sol_data::Address)>::encode((
                    keccak256(version.as_bytes()).0,
                    verifying_contract,
                ))
            }
            (None, Some(version), None, Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(version.as_bytes()).0,
                    verifying_contract,
                    salt.0,
                ))
            }
            (None, Some(version), Some(chain_id), None, None) => {
                <(sol_data::FixedBytes<32>, sol_data::Uint<256>)>::encode((
                    keccak256(version.as_bytes()).0,
                    chain_id,
                ))
            }
            (None, Some(version), Some(chain_id), None, Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::FixedBytes<32>,
                )>::encode((keccak256(version.as_bytes()).0, chain_id, salt.0))
            }
            (None, Some(version), Some(chain_id), Some(verifying_contract), None) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::Address,
                )>::encode((
                    keccak256(version.as_bytes()).0,
                    chain_id,
                    verifying_contract,
                ))
            }
            (None, Some(version), Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(version.as_bytes()).0,
                    chain_id,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), None, None, None, None) => {
                <(sol_data::FixedBytes<32>,)>::encode((keccak256(name.as_bytes()).0,))
            }
            (Some(name), None, None, None, Some(salt)) => {
                <(sol_data::FixedBytes<32>, sol_data::FixedBytes<32>)>::encode((
                    keccak256(name.as_bytes()).0,
                    salt.0,
                ))
            }
            (Some(name), None, None, Some(verifying_contract), None) => {
                <(sol_data::FixedBytes<32>, sol_data::Address)>::encode((
                    keccak256(name.as_bytes()).0,
                    verifying_contract,
                ))
            }
            (Some(name), None, None, Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), None, Some(chain_id), None, None) => {
                <(sol_data::FixedBytes<32>, sol_data::Uint<256>)>::encode((
                    keccak256(name.as_bytes()).0,
                    chain_id,
                ))
            }
            (Some(name), None, Some(chain_id), None, Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::FixedBytes<32>,
                )>::encode((keccak256(name.as_bytes()).0, chain_id, salt.0))
            }
            (Some(name), None, Some(chain_id), Some(verifying_contract), None) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::Address,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    chain_id,
                    verifying_contract,
                ))
            }
            (Some(name), None, Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    chain_id,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), Some(version), None, None, None) => {
                <(sol_data::FixedBytes<32>, sol_data::FixedBytes<32>)>::encode((
                    keccak256(name.as_bytes()).0,
                    keccak256(version.as_bytes()).0,
                ))
            }
            (Some(name), Some(version), None, None, Some(salt)) => <(
                sol_data::FixedBytes<32>,
                sol_data::FixedBytes<32>,
                sol_data::FixedBytes<32>,
            )>::encode((
                keccak256(name.as_bytes()).0,
                keccak256(version.as_bytes()).0,
                salt.0,
            )),
            (Some(name), Some(version), None, Some(verifying_contract), None) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::FixedBytes<32>,
                    sol_data::Address,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    keccak256(version.as_bytes()).0,
                    verifying_contract,
                ))
            }
            (Some(name), Some(version), None, Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::FixedBytes<32>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    keccak256(version.as_bytes()).0,
                    verifying_contract,
                    salt.0,
                ))
            }
            (Some(name), Some(version), Some(chain_id), None, None) => <(
                sol_data::FixedBytes<32>,
                sol_data::FixedBytes<32>,
                sol_data::Uint<256>,
            )>::encode((
                keccak256(name.as_bytes()).0,
                keccak256(version.as_bytes()).0,
                chain_id,
            )),
            (Some(name), Some(version), Some(chain_id), None, Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    keccak256(version.as_bytes()).0,
                    chain_id,
                    salt.0,
                ))
            }
            (Some(name), Some(version), Some(chain_id), Some(verifying_contract), None) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::Address,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    keccak256(version.as_bytes()).0,
                    chain_id,
                    verifying_contract,
                ))
            }
            (Some(name), Some(version), Some(chain_id), Some(verifying_contract), Some(salt)) => {
                <(
                    sol_data::FixedBytes<32>,
                    sol_data::FixedBytes<32>,
                    sol_data::Uint<256>,
                    sol_data::Address,
                    sol_data::FixedBytes<32>,
                )>::encode((
                    keccak256(name.as_bytes()).0,
                    keccak256(version.as_bytes()).0,
                    chain_id,
                    verifying_contract,
                    salt.0,
                ))
            }
        }
    }

    /// EIP-712 `hashStruct`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct>
    #[inline]
    pub fn hash_struct(&self) -> B256 {
        let mut type_hash = self.type_hash().to_vec();
        type_hash.extend(self.encode_data());
        keccak256(type_hash)
    }
}

// This was generated by excel meta-programming
/// Instantiate an EIP-712 domain.
#[macro_export]
macro_rules! domain {
    (salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            None,
            None,
            Some($salt),
        )
    };
    (verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            None,
            Some($verifying_contract),
            None,
        )
    };
    (verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            None,
            Some($verifying_contract),
            Some($salt),
        )
    };
    (chain_id: $chain_id:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            Some($chain_id)
            None,
            None,
        )
    };
    (chain_id: $chain_id:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            Some($chain_id)
            None,
            Some($salt),
        )
    };
    (chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            Some($chain_id),
            Some($verifying_contract),
            None,
        )
    };
    (chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            None,
            Some($chain_id),
            Some($verifying_contract),
            Some($salt),
        )
    };
    (version: $version:literal,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            None,
            None,
        )
    };
    (version: $version:literal,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            None,
            Some($salt),
        )
    };
    (version: $version:literal,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            Some($verifying_contract),
            None,
        )
};
    (version: $version:literal,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            Some($verifying_contract),
            Some($salt),
        )
    };
    (version: $version:literal,chain_id: $chain_id:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id)
            None,
            None,
        )
    };
    (version: $version:literal,chain_id: $chain_id:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id)
            None,
            Some($salt),
        )
    };
    (version: $version:literal,chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id),
            Some($verifying_contract),
            None,
        )
    };
    (version: $version:literal,chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            None,
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id),
            Some($verifying_contract),
            Some($salt),
        )
    };
    (name: $name:literal,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            None,
            None,
            None,
        )
    };
    (name: $name:literal,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            None,
            None,
            Some($salt),
        )
    };
    (name: $name:literal,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            None,
            Some($verifying_contract),
            None,
        )
    };
    (name: $name:literal,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            None,
            Some($verifying_contract),
            Some($salt),
        )
    };
    (name: $name:literal,chain_id: $chain_id:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            Some($chain_id)
            None,
            None,
        )
    };
    (name: $name:literal,chain_id: $chain_id:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            Some($chain_id)
            None,
            Some($salt),
        )
    };
    (name: $name:literal,chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            Some($chain_id),
            Some($verifying_contract),
            None,
        )
    };
    (name: $name:literal,chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            None,
            Some($chain_id),
            Some($verifying_contract),
            Some($salt),
        )
    };
    (name: $name:literal,version: $version:literal,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            None,
            None,
        )
    };
    (name: $name:literal,version: $version:literal,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            None,
            Some($salt),
        )
    };
    (name: $name:literal,version: $version:literal,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            Some($verifying_contract),
            None,
        )
    };
    (name: $name:literal,version: $version:literal,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            None,
            Some($verifying_contract),
            Some($salt),
        )
    };
    (name: $name:literal,version: $version:literal,chain_id: $chain_id:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id)
            None,
            None,
        )
    };
    (name: $name:literal,version: $version:literal,chain_id: $chain_id:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id)
            None,
            Some($salt),
        )
    };
    (name: $name:literal,version: $version:literal,chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id),
            Some($verifying_contract),
            None,
        )
    };
    (name: $name:literal,version: $version:literal,chain_id: $chain_id:expr,verifying_contract: $verifying_contract:expr,salt: $salt:expr,) => {
        $crate::Eip712Domain::new(
            Some($crate::no_std_prelude::Cow::Borrowed($name)),
            Some($crate::no_std_prelude::Cow::Borrowed($version)),
            Some($chain_id),
            Some($verifying_contract),
            Some($salt),
        )
    };
}

#[cfg(test)]
mod test {
    use super::*;
    const _DOMAIN: Eip712Domain = domain! {
        name: "abcd",
        version: "1",
        chain_id: U256::from_limbs([1, 0, 0, 0]),
        verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };
}
