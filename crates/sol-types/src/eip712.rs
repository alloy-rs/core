use crate::{abi::token::WordToken, sol_data, Encodable, SolType};
use alloc::{borrow::Cow, string::String, vec::Vec};
use alloy_primitives::{keccak256, Address, FixedBytes, B256, U256};

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

    /// EIP-712 `encodeType`:
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

    /// EIP-712 `typeHash`:
    /// <https://eips.ethereum.org/EIPS/eip-712#rationale-for-typehash>
    #[inline]
    pub fn type_hash(&self) -> B256 {
        keccak256(self.encode_type().as_bytes())
    }

    /// Returns the number of words that will be used to encode the domain.
    #[inline]
    pub const fn num_words(&self) -> usize {
        self.name.is_some() as usize
            + self.version.is_some() as usize
            + self.chain_id.is_some() as usize
            + self.verifying_contract.is_some() as usize
            + self.salt.is_some() as usize
    }

    /// Returns the number of bytes that will be used to encode the domain.
    #[inline]
    pub const fn abi_encoded_size(&self) -> usize {
        self.num_words() * 32
    }

    /// EIP-712 `encodeData` into the given buffer:
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    pub fn encode_data_to(&self, out: &mut Vec<u8>) {
        // This only works because all of the fields are encoded as words.
        #[inline]
        fn encode_opt<S, T>(opt: Option<&T>, out: &mut Vec<u8>)
        where
            S: for<'a> SolType<TokenType<'a> = WordToken>,
            T: Encodable<S>,
        {
            if let Some(t) = opt {
                out.extend_from_slice(t.to_tokens().as_slice());
            }
        }

        #[inline]
        #[allow(clippy::ptr_arg)]
        fn cow_keccak256(s: &Cow<'_, str>) -> FixedBytes<32> {
            keccak256(s.as_bytes())
        }

        out.reserve(self.abi_encoded_size());
        let name = self.name.as_ref().map(cow_keccak256);
        encode_opt::<sol_data::FixedBytes<32>, _>(name.as_ref(), out);
        let version = self.version.as_ref().map(cow_keccak256);
        encode_opt::<sol_data::FixedBytes<32>, _>(version.as_ref(), out);
        encode_opt::<sol_data::Uint<256>, _>(self.chain_id.as_ref(), out);
        encode_opt::<sol_data::Address, _>(self.verifying_contract.as_ref(), out);
        encode_opt::<sol_data::FixedBytes<32>, _>(self.salt.as_ref(), out);
    }

    /// EIP-712 `encodeData`:
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    pub fn encode_data(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode_data_to(&mut out);
        out
    }

    /// EIP-712 `hashStruct`:
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct>
    #[inline]
    pub fn hash_struct(&self) -> B256 {
        let mut type_hash = self.type_hash().to_vec();
        type_hash.extend(self.encode_data());
        keccak256(type_hash)
    }
}

/// Convenience macro to instantiate an EIP-712 domain.
///
/// This macro allows you to instantiate an Eip712Domain struct without
/// manually writing `None` for unused fields. It may be used to declare
/// a domain with any combination of fields. Each field must be labeled
/// with the name of the field, and the fields must be in order. The
/// fields for the domain are:
/// - `name`
/// - `version`
/// - `chain_id`
/// - `verifying_contract`
/// - `salt`
///
/// ```
/// # use alloy_sol_types::{Eip712Domain, eip712_domain};
/// # use alloy_primitives::keccak256;
///
/// const MY_DOMAIN: Eip712Domain = eip712_domain! {
///     name: "MyCoolProtocol",
/// };
///
/// # fn main() {
/// let my_other_domain: Eip712Domain = eip712_domain! {
///     name: String::from("MyCoolProtocol"),
///     version: "1.0.0",
///     salt: keccak256("my domain salt"),
/// };
/// # }
/// ```
#[macro_export]
macro_rules! eip712_domain {
    (@opt) => { $crate::private::None };
    (@opt $e:expr) => { $crate::private::Some($e) };

    // special case literals to allow calling this in const contexts
    (@cow) => { $crate::private::None };
    (@cow $l:literal) => { $crate::private::Some($crate::private::Cow::Borrowed($l)) };
    (@cow $e:expr) => { $crate::private::Some(<$crate::private::Cow<'static, str> as $crate::private::From<_>>::from($e)) };

    (
        $(name: $name:expr,)?
        $(version: $version:expr,)?
        $(chain_id: $chain_id:expr,)?
        $(verifying_contract: $verifying_contract:expr,)?
        $(salt: $salt:expr)?
        $(,)?
    ) => {
        $crate::Eip712Domain::new(
            $crate::eip712_domain!(@cow $($name)?),
            $crate::eip712_domain!(@cow $($version)?),
            $crate::eip712_domain!(@opt $($crate::private::u256($chain_id))?),
            $crate::eip712_domain!(@opt $($verifying_contract)?),
            $crate::eip712_domain!(@opt $($salt)?),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        version: "1",
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        version: "1",
        chain_id: 1,
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        version: "1",
        chain_id: 1,
        verifying_contract: Address::ZERO,
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        version: "1",
        chain_id: 1,
        verifying_contract: Address::ZERO,
        salt: B256::ZERO // no trailing comma
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        version: "1",
        chain_id: 1,
        verifying_contract: Address::ZERO,
        salt: B256::ZERO, // trailing comma
    };

    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        version: "1",
        // chain_id: 1,
        verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        // version: "1",
        chain_id: 1,
        verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        // version: "1",
        // chain_id: 1,
        verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };
    const _: Eip712Domain = eip712_domain! {
        name: "abcd",
        // version: "1",
        // chain_id: 1,
        // verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };
    const _: Eip712Domain = eip712_domain! {
        // name: "abcd",
        version: "1",
        // chain_id: 1,
        // verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };
    const _: Eip712Domain = eip712_domain! {
        // name: "abcd",
        version: "1",
        // chain_id: 1,
        verifying_contract: Address::ZERO,
        salt: B256::ZERO,
    };

    #[test]
    fn runtime_domains() {
        let _: Eip712Domain = eip712_domain! {
            name: String::new(),
            version: String::new(),
        };

        let my_string = String::from("!@#$%^&*()_+");
        let _: Eip712Domain = eip712_domain! {
            name: my_string.clone(),
            version: my_string,
        };

        let my_cow = Cow::from("my_cow");
        let _: Eip712Domain = eip712_domain! {
            name: my_cow.clone(),
            version: my_cow.into_owned(),
        };
    }
}
