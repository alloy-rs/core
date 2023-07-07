//! Ethereum [EIP-155] chains and chain IDs.
//!
//! [EIP-155]: https://eips.ethereum.org/EIPS/eip-155

use crate::{goerli_nodes, mainnet_nodes, sepolia_nodes, NodeRecord};
use alloy_primitives::Uint;
use alloy_rlp::{Decodable, Encodable};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
    time::Duration,
};

mod named;
pub use named::{
    NamedChain, NamedChainIter, ParseChainError, TryFromPrimitive, TryFromPrimitiveError,
};

mod info;
pub use info::ChainInfo;

mod spec;
pub use spec::*;

/// An Ethereum [EIP-155] chain or chain ID.
///
/// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Chain(ChainRepr);

impl Default for Chain {
    #[inline]
    fn default() -> Self {
        Self::mainnet()
    }
}

impl fmt::Debug for Chain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Chain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ChainRepr::Named(chain) => chain.fmt(f),
            ChainRepr::Id(id) => id.fmt(f),
        }
    }
}

impl PartialEq for Chain {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_id() == other.as_id()
    }
}

impl Eq for Chain {}

impl PartialOrd for Chain {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_id().partial_cmp(other.as_id())
    }
}

impl Ord for Chain {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_id().cmp(other.as_id())
    }
}

impl Hash for Chain {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_id().hash(state)
    }
}

impl Serialize for Chain {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Chain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ChainRepr::deserialize(deserializer).map(Self)
    }
}

impl From<NamedChain> for Chain {
    #[inline]
    fn from(value: NamedChain) -> Self {
        Self::named(value)
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
    type Error = ParseChainError;

    #[inline]
    fn try_from(chain: Chain) -> Result<Self, Self::Error> {
        match chain.0 {
            ChainRepr::Named(chain) => Ok(chain),
            ChainRepr::Id(number) => Err(ParseChainError { number }),
        }
    }
}

impl std::str::FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(chain) = NamedChain::from_str(s) {
            Ok(Chain::named(chain))
        } else {
            s.parse()
                .map(Self::new)
                .map_err(|_| format!("Expected a known chain or chain ID, found: {s}"))
        }
    }
}

impl Encodable for Chain {
    #[inline]
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.id().encode(out)
    }

    #[inline]
    fn length(&self) -> usize {
        self.id().length()
    }
}

impl Decodable for Chain {
    #[inline]
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::DecodeError> {
        u64::decode(buf).map(Self::new)
    }
}

macro_rules! fwd {
    ($($vis:vis fn $name:ident($($arg:tt)*) -> $ret:ty;)+) => {$(
        #[doc = concat!("See [`NamedChain::", stringify!($name), "()`].\n")]
        #[doc = concat!("Will return `", stringify!($ret), "::default()` if the `self` not a [NamedChain].\n")]
        #[inline]
        $vis fn $name(self, $($arg)*) -> $ret {
            match self.0 {
                ChainRepr::Named(named) => NamedChain::$name(named, $($arg)*),
                ChainRepr::Id(_) => <$ret>::default(),
            }
        }
    )+};
}

impl Chain {
    /// Instantiates a new chain from a chain ID.
    #[inline]
    pub fn new(id: u64) -> Self {
        match NamedChain::try_from(id) {
            Ok(chain) => Self(ChainRepr::Named(chain)),
            Err(TryFromPrimitiveError { number }) => Self(ChainRepr::Id(number)),
        }
    }

    /// Instantiates a new chain from a [NamedChain].
    #[inline]
    pub const fn named(chain: NamedChain) -> Self {
        Self(ChainRepr::Named(chain))
    }

    #[doc(hidden)]
    #[deprecated(note = "Use `new` instead.")]
    #[allow(non_snake_case)]
    #[inline]
    pub fn Id(id: u64) -> Self {
        Self::new(id)
    }

    #[doc(hidden)]
    #[deprecated(note = "Use `named` instead.")]
    #[allow(non_snake_case)]
    #[inline]
    pub fn Named(named: NamedChain) -> Self {
        Self::named(named)
    }

    /// Returns the mainnet chain.
    #[inline]
    pub const fn mainnet() -> Self {
        Self::named(NamedChain::Mainnet)
    }

    /// Returns the goerli chain.
    #[inline]
    pub const fn goerli() -> Self {
        Self::named(NamedChain::Goerli)
    }

    /// Returns the sepolia chain.
    #[inline]
    pub const fn sepolia() -> Self {
        Self::named(NamedChain::Sepolia)
    }

    /// The ID of the chain.
    #[inline]
    pub const fn id(self) -> u64 {
        match self.0 {
            ChainRepr::Named(chain) => chain as u64,
            ChainRepr::Id(id) => id,
        }
    }

    /// The ID of the chain.
    #[inline]
    pub const fn as_id(&self) -> &u64 {
        match &self.0 {
            ChainRepr::Named(chain) => unsafe { &*(chain as *const NamedChain as *const u64) },
            ChainRepr::Id(id) => id,
        }
    }

    /// Returns the chain as a named chain, if possible.
    #[inline]
    pub fn as_named(self) -> Option<NamedChain> {
        match self.0 {
            ChainRepr::Named(named) => Some(named),
            ChainRepr::Id(_) => None,
        }
    }

    /// Returns the string representation of the chain.
    #[inline]
    pub fn as_str(self) -> Option<&'static str> {
        match self.0 {
            ChainRepr::Named(named) => Some(named.as_str()),
            ChainRepr::Id(_) => None,
        }
    }

    /// Returns `true` if the chain is a named chain.
    #[inline]
    pub const fn is_named(self) -> bool {
        matches!(self.0, ChainRepr::Named(_))
    }

    fwd! {
        pub fn average_blocktime_hint() -> Option<Duration>;
        pub fn is_legacy() -> bool;
        pub fn supports_push0() -> bool;
        pub fn etherscan_urls() -> Option<(&'static str, &'static str)>;
        pub fn etherscan_api_key_name() -> Option<&'static str>;
        pub fn etherscan_api_key() -> Option<String>;
    }

    /// Returns the address of the public DNS node list for the given chain.
    ///
    /// See also <https://github.com/ethereum/discv4-dns-lists>
    #[inline]
    pub fn public_dns_network_protocol(self) -> Option<String> {
        use NamedChain::*;

        let named = self.as_named()?;
        matches!(named, Mainnet | Goerli | Sepolia | Ropsten | Rinkeby).then(|| {
            format!(
                "enrtree://AKA3AM6LPBYEUDMVNU3BSVQJ5AD45Y7YPOHJLEF6W26QOE4VTUDPE@all.{named}.ethdisco.net",
            )
        })
    }

    /// Returns bootnodes for the given chain.
    #[inline]
    pub fn bootnodes(self) -> Option<Vec<NodeRecord>> {
        use NamedChain::*;
        match self.as_named()? {
            Mainnet => Some(mainnet_nodes()),
            Goerli => Some(goerli_nodes()),
            Sepolia => Some(sepolia_nodes()),
            _ => None,
        }
    }
}

/// Representation of a chain.
///
/// This is private to enforce the invariant that the `Id` variant is never one
/// of the named chains, e.g. this is just a wrapper around the
/// `Result<NamedChain, u64>` of `NamedChain::try_from`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
enum ChainRepr {
    /// A named chain.
    Named(NamedChain),
    /// A chain ID.
    Id(u64),
}

#[cfg(feature = "arbitrary")]
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
                any::<Selector>().prop_map(move |sel| Chain::named(sel.select(NamedChain::iter())));
            let id = any::<u64>().prop_map(Chain::from);
            proptest::strategy::Union::new_weighted(vec![(50, named.boxed()), (50, id.boxed())])
                .boxed()
        }
    }

    impl<'a> ::arbitrary::Arbitrary<'a> for Chain {
        fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
            if u.ratio(1, 2)? {
                let chain = u.int_in_range(0..=(NamedChain::COUNT - 1))?;

                return Ok(Chain::named(
                    NamedChain::iter().nth(chain).expect("in range"),
                ))
            }

            Ok(Self::new(u64::arbitrary(u)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{U256, U64};

    #[test]
    fn test_id() {
        let chain = Chain::new(1234);
        assert_eq!(chain.id(), 1234);
    }

    #[test]
    fn test_named_id() {
        let chain = Chain::named(NamedChain::Goerli);
        assert_eq!(chain.id(), 5);
    }

    #[test]
    fn test_legacy_named_chain() {
        let chain = Chain::named(NamedChain::Optimism);
        assert!(chain.is_legacy());
    }

    #[test]
    fn test_not_legacy_named_chain() {
        let chain = Chain::named(NamedChain::Mainnet);
        assert!(!chain.is_legacy());
    }

    #[test]
    fn test_not_legacy_id_chain() {
        let chain = Chain::new(1234);
        assert!(!chain.is_legacy());
    }

    #[test]
    fn test_display_named_chain() {
        let chain = Chain::named(NamedChain::Mainnet);
        assert_eq!(format!("{chain}"), "mainnet");
    }

    #[test]
    fn test_display_id_chain() {
        let chain = Chain::new(1234);
        assert_eq!(format!("{chain}"), "1234");
    }

    #[test]
    fn test_from_u256() {
        let n = U256::from(1234);
        let chain = Chain::from(n);
        let expected = Chain::new(1234);

        assert_eq!(chain, expected);
    }

    #[test]
    fn test_into_u256() {
        let chain = Chain::named(NamedChain::Goerli);
        let n: U256 = chain.into();
        let expected = U256::from(5);

        assert_eq!(n, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_into_U64() {
        let chain = Chain::named(NamedChain::Goerli);
        let n: U64 = chain.into();
        let expected = U64::from(5);

        assert_eq!(n, expected);
    }

    #[test]
    fn test_from_str_named_chain() {
        assert_eq!(
            "mainnet".parse::<Chain>().unwrap(),
            Chain::named(NamedChain::Mainnet)
        );
    }

    #[test]
    fn test_from_str_id_chain() {
        assert_eq!("1234".parse::<Chain>().unwrap(), Chain::new(1234));
    }

    #[test]
    fn test_from_str_named_chain_error() {
        "chain".parse::<Chain>().unwrap_err();
    }

    #[test]
    fn test_default() {
        let default = Chain::default();
        let expected = Chain::named(NamedChain::Mainnet);

        assert_eq!(default, expected);
    }

    #[test]
    fn test_id_chain_encodable_length() {
        let chain = Chain::new(1234);

        assert_eq!(chain.length(), 3);
    }

    #[test]
    fn test_dns_network() {
        let s = "enrtree://AKA3AM6LPBYEUDMVNU3BSVQJ5AD45Y7YPOHJLEF6W26QOE4VTUDPE@all.mainnet.ethdisco.net";
        let chain: Chain = NamedChain::Mainnet.into();
        assert_eq!(s, chain.public_dns_network_protocol().unwrap().as_str());
    }
}
