use crate::{ChainSpec, ForkCondition, ForkFilter, ForkId};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// The name of an Ethereum hardfork.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum Hardfork {
    /// Frontier.
    Frontier,
    /// Homestead.
    Homestead,
    /// The DAO fork.
    Dao,
    /// Tangerine.
    Tangerine,
    /// Spurious Dragon.
    SpuriousDragon,
    /// Byzantium.
    Byzantium,
    /// Constantinople.
    Constantinople,
    /// Petersburg.
    Petersburg,
    /// Istanbul.
    Istanbul,
    /// Muir Glacier.
    MuirGlacier,
    /// Berlin.
    Berlin,
    /// London.
    London,
    /// Arrow Glacier.
    ArrowGlacier,
    /// Gray Glacier.
    GrayGlacier,
    /// Paris.
    Paris,
    /// Shanghai.
    #[default]
    Shanghai,
}

impl Hardfork {
    /// Get the [ForkId] for this hardfork in the given spec, if the fork is
    /// activated at any point.
    #[cfg(feature = "proof")]
    pub fn fork_id(&self, spec: &ChainSpec) -> Option<ForkId> {
        match spec.fork(*self) {
            ForkCondition::Never => None,
            _ => Some(spec.fork_id(&spec.fork(*self).satisfy())),
        }
    }

    /// Get the [ForkFilter] for this hardfork in the given spec, if the fork is
    /// activated at any point.
    #[cfg(feature = "proof")]
    pub fn fork_filter(&self, spec: &ChainSpec) -> Option<ForkFilter> {
        match spec.fork(*self) {
            ForkCondition::Never => None,
            _ => Some(spec.fork_filter(spec.fork(*self).satisfy())),
        }
    }
}

impl FromStr for Hardfork {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let hardfork = match s.as_str() {
            "frontier" => Self::Frontier,
            "homestead" => Self::Homestead,
            "dao" => Self::Dao,
            "tangerine" => Self::Tangerine,
            "spuriousdragon" => Self::SpuriousDragon,
            "byzantium" => Self::Byzantium,
            "constantinople" => Self::Constantinople,
            "petersburg" => Self::Petersburg,
            "istanbul" => Self::Istanbul,
            "muirglacier" => Self::MuirGlacier,
            "berlin" => Self::Berlin,
            "london" => Self::London,
            "arrowglacier" => Self::ArrowGlacier,
            "grayglacier" => Self::GrayGlacier,
            "paris" => Self::Paris,
            "shanghai" => Self::Shanghai,
            _ => return Err(format!("Unknown hardfork: {s}")),
        };
        Ok(hardfork)
    }
}

impl fmt::Display for Hardfork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chain, Genesis};
    use std::collections::BTreeMap;

    #[test]
    fn check_hardfork_from_str() {
        let hardfork_str = [
            "frOntier",
            "homEstead",
            "dao",
            "tAngerIne",
            "spurIousdrAgon",
            "byzAntium",
            "constantinople",
            "petersburg",
            "istanbul",
            "muirglacier",
            "bErlin",
            "lonDon",
            "arrowglacier",
            "grayglacier",
            "PARIS",
            "ShAnGhAI",
        ];
        let expected_hardforks = [
            Hardfork::Frontier,
            Hardfork::Homestead,
            Hardfork::Dao,
            Hardfork::Tangerine,
            Hardfork::SpuriousDragon,
            Hardfork::Byzantium,
            Hardfork::Constantinople,
            Hardfork::Petersburg,
            Hardfork::Istanbul,
            Hardfork::MuirGlacier,
            Hardfork::Berlin,
            Hardfork::London,
            Hardfork::ArrowGlacier,
            Hardfork::GrayGlacier,
            Hardfork::Paris,
            Hardfork::Shanghai,
        ];

        let hardforks: Vec<Hardfork> = hardfork_str
            .iter()
            .map(|h| Hardfork::from_str(h).unwrap())
            .collect();

        assert_eq!(hardforks, expected_hardforks);
    }

    #[test]
    fn check_nonexistent_hardfork_from_str() {
        assert!(Hardfork::from_str("not a hardfork").is_err());
    }

    #[test]
    #[cfg(feature = "proof")]
    fn check_fork_id_chainspec_with_fork_condition_never() {
        let spec = ChainSpec {
            chain: Chain::mainnet(),
            genesis: Genesis::default(),
            genesis_hash: None,
            hardforks: BTreeMap::from([(Hardfork::Frontier, ForkCondition::Never)]),
        };

        assert_eq!(Hardfork::Frontier.fork_id(&spec), None);
    }

    #[test]
    #[cfg(feature = "proof")]
    fn check_fork_filter_chainspec_with_fork_condition_never() {
        let spec = ChainSpec {
            chain: Chain::mainnet(),
            genesis: Genesis::default(),
            genesis_hash: None,
            hardforks: BTreeMap::from([(Hardfork::Shanghai, ForkCondition::Never)]),
        };

        assert_eq!(Hardfork::Shanghai.fork_filter(&spec), None);
    }
}
