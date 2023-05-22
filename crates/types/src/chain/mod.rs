//! Ethereum [EIP-155] chains and chain IDs.
//!
//! [EIP-155]: https://eips.ethereum.org/EIPS/eip-155

use crate::{goerli_nodes, mainnet_nodes, sepolia_nodes, NodeRecord};
use ethers_primitives::Uint;
use ethers_rlp::{Decodable, Encodable};
use serde::{Deserialize, Serialize};
use std::fmt;

mod named;
pub use named::{
    NamedChain, NamedChainIter, ParseChainError, TryFromPrimitive, TryFromPrimitiveError,
};

mod info;
pub use info::ChainInfo;

mod spec;
pub use spec::*;

// TODO: `Chain` could be 8 bytes, not 16

/// An Ethereum [EIP-155] chain or chain ID.
///
/// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    /// A named chain.
    Named(NamedChain),
    /// A chain ID.
    Id(u64),
}

impl Default for Chain {
    #[inline]
    fn default() -> Self {
        Self::Named(NamedChain::Mainnet)
    }
}

impl fmt::Display for Chain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(chain) => chain.fmt(f),
            Self::Id(id) => id.fmt(f),
        }
    }
}

impl From<NamedChain> for Chain {
    #[inline]
    fn from(value: NamedChain) -> Self {
        Self::Named(value)
    }
}

impl From<u64> for Chain {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Chain> for u64 {
    #[inline]
    fn from(value: Chain) -> Self {
        value.id()
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for Chain {
    fn from(value: Uint<BITS, LIMBS>) -> Self {
        value.wrapping_to::<u64>().into()
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Chain> for Uint<BITS, LIMBS> {
    #[inline]
    fn from(value: Chain) -> Self {
        Self::from(value.id())
    }
}

impl TryFrom<Chain> for NamedChain {
    type Error = <NamedChain as TryFrom<u64>>::Error;

    fn try_from(chain: Chain) -> Result<Self, Self::Error> {
        match chain {
            Chain::Named(chain) => Ok(chain),
            Chain::Id(id) => id.try_into(),
        }
    }
}

impl std::str::FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(chain) = NamedChain::from_str(s) {
            Ok(Chain::Named(chain))
        } else {
            s.parse()
                .map(Self::Id)
                .map_err(|_| format!("Expected a known chain or chain ID, found: {s}"))
        }
    }
}

impl Encodable for Chain {
    #[inline]
    fn encode(&self, out: &mut dyn ethers_rlp::BufMut) {
        self.id().encode(out)
    }

    #[inline]
    fn length(&self) -> usize {
        self.id().length()
    }
}

impl Decodable for Chain {
    #[inline]
    fn decode(buf: &mut &[u8]) -> Result<Self, ethers_rlp::DecodeError> {
        u64::decode(buf).map(Into::into)
    }
}

impl Chain {
    /// Instantiates a new chain from a chain ID.
    #[inline]
    pub fn new(id: u64) -> Self {
        match NamedChain::try_from(id) {
            Ok(chain) => Self::Named(chain),
            Err(_) => Self::Id(id),
        }
    }

    /// Returns the mainnet chain.
    #[inline]
    pub const fn mainnet() -> Self {
        Self::Named(NamedChain::Mainnet)
    }

    /// Returns the goerli chain.
    #[inline]
    pub const fn goerli() -> Self {
        Self::Named(NamedChain::Goerli)
    }

    /// Returns the sepolia chain.
    #[inline]
    pub const fn sepolia() -> Self {
        Self::Named(NamedChain::Sepolia)
    }

    /// The id of the chain
    #[inline]
    pub fn id(self) -> u64 {
        match self {
            Self::Named(chain) => chain as u64,
            Self::Id(id) => id,
        }
    }

    /// Helper function for checking if a chainid corresponds to a legacy
    /// chainid without eip1559
    #[inline]
    pub fn is_legacy(&self) -> bool {
        match self {
            Self::Named(c) => c.is_legacy(),
            Self::Id(_) => false,
        }
    }

    /// Returns the address of the public DNS node list for the given chain.
    ///
    /// See also <https://github.com/ethereum/discv4-dns-lists>
    #[inline]
    pub fn public_dns_network_protocol(self) -> Option<String> {
        use NamedChain::*;
        const DNS_PREFIX: &str = "enrtree://AKA3AM6LPBYEUDMVNU3BSVQJ5AD45Y7YPOHJLEF6W26QOE4VTUDPE@";

        let named: NamedChain = self.try_into().ok()?;

        if matches!(named, Mainnet | Goerli | Sepolia | Ropsten | Rinkeby) {
            Some(format!(
                "{DNS_PREFIX}all.{}.ethdisco.net",
                named.as_ref().to_lowercase()
            ))
        } else {
            None
        }
    }

    /// Returns bootnodes for the given chain.
    #[inline]
    pub fn bootnodes(self) -> Option<Vec<NodeRecord>> {
        use NamedChain::*;
        match self.try_into().ok()? {
            Mainnet => Some(mainnet_nodes()),
            Goerli => Some(goerli_nodes()),
            Sepolia => Some(sepolia_nodes()),
            _ => None,
        }
    }
}

#[cfg(any(test, feature = "arbitrary"))]
mod arbitrary {
    use super::*;
    use proptest::{
        arbitrary::ParamsFor,
        prelude::{any, Strategy},
        sample::Selector,
        strategy::BoxedStrategy,
    };
    use strum::{EnumCount, IntoEnumIterator};

    impl proptest::arbitrary::Arbitrary for Chain {
        type Parameters = ParamsFor<u32>;
        type Strategy = BoxedStrategy<Chain>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            let named =
                any::<Selector>().prop_map(move |sel| Chain::Named(sel.select(NamedChain::iter())));
            let id = any::<u64>().prop_map(Chain::from);
            proptest::strategy::Union::new_weighted(vec![(50, named.boxed()), (50, id.boxed())])
                .boxed()
        }
    }

    impl<'a> ::arbitrary::Arbitrary<'a> for Chain {
        fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
            if u.ratio(1, 2)? {
                let chain = u.int_in_range(0..=(NamedChain::COUNT - 1))?;

                return Ok(Chain::Named(
                    NamedChain::iter().nth(chain).expect("in range"),
                ))
            }

            Ok(Self::Id(u64::arbitrary(u)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers_primitives::{U256, U64};

    use super::*;

    #[test]
    fn test_id() {
        let chain = Chain::Id(1234);
        assert_eq!(chain.id(), 1234);
    }

    #[test]
    fn test_named_id() {
        let chain = Chain::Named(NamedChain::Goerli);
        assert_eq!(chain.id(), 5);
    }

    #[test]
    fn test_legacy_named_chain() {
        let chain = Chain::Named(NamedChain::Optimism);
        assert!(chain.is_legacy());
    }

    #[test]
    fn test_not_legacy_named_chain() {
        let chain = Chain::Named(NamedChain::Mainnet);
        assert!(!chain.is_legacy());
    }

    #[test]
    fn test_not_legacy_id_chain() {
        let chain = Chain::Id(1234);
        assert!(!chain.is_legacy());
    }

    #[test]
    fn test_display_named_chain() {
        let chain = Chain::Named(NamedChain::Mainnet);
        assert_eq!(format!("{chain}"), "mainnet");
    }

    #[test]
    fn test_display_id_chain() {
        let chain = Chain::Id(1234);
        assert_eq!(format!("{chain}"), "1234");
    }

    #[test]
    fn test_from_u256() {
        let n = U256::from(1234);
        let chain = Chain::from(n);
        let expected = Chain::Id(1234);

        assert_eq!(chain, expected);
    }

    #[test]
    fn test_into_u256() {
        let chain = Chain::Named(NamedChain::Goerli);
        let n: U256 = chain.into();
        let expected = U256::from(5);

        assert_eq!(n, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_into_U64() {
        let chain = Chain::Named(NamedChain::Goerli);
        let n: U64 = chain.into();
        let expected = U64::from(5);

        assert_eq!(n, expected);
    }

    #[test]
    fn test_from_str_named_chain() {
        assert_eq!(
            "mainnet".parse::<Chain>().unwrap(),
            Chain::Named(NamedChain::Mainnet)
        );
    }

    #[test]
    fn test_from_str_id_chain() {
        assert_eq!("1234".parse::<Chain>().unwrap(), Chain::Id(1234));
    }

    #[test]
    fn test_from_str_named_chain_error() {
        "chain".parse::<Chain>().unwrap_err();
    }

    #[test]
    fn test_default() {
        let default = Chain::default();
        let expected = Chain::Named(NamedChain::Mainnet);

        assert_eq!(default, expected);
    }

    #[test]
    fn test_id_chain_encodable_length() {
        let chain = Chain::Id(1234);

        assert_eq!(chain.length(), 3);
    }

    #[test]
    fn test_dns_network() {
        let s = "enrtree://AKA3AM6LPBYEUDMVNU3BSVQJ5AD45Y7YPOHJLEF6W26QOE4VTUDPE@all.mainnet.ethdisco.net";
        let chain: Chain = NamedChain::Mainnet.into();
        assert_eq!(s, chain.public_dns_network_protocol().unwrap().as_str());
    }
}
