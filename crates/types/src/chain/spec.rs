use crate::{
    constants::{self, EIP1559_INITIAL_BASE_FEE, EMPTY_WITHDRAWALS},
    genesis::geth::Genesis as GethGenesis,
    primitives::{ForkFilter, ForkFilterKey, ForkHash, ForkId, Hardfork, Head, Header},
    Chain, Genesis, GenesisAccount,
};
use ethers_primitives::{Address, BlockNumber, B256, U256, U64};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The Ethereum mainnet chain spec.
pub static MAINNET: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain: Chain::mainnet(),
    genesis: serde_json::from_str(include_str!("../../res/genesis/mainnet.json"))
        .expect("Can't deserialize Mainnet genesis json"),
    genesis_hash: Some(constants::MAINNET_GENESIS),
    hardforks: BTreeMap::from([
        (Hardfork::Frontier, ForkCondition::Block(0)),
        (Hardfork::Homestead, ForkCondition::Block(1150000)),
        (Hardfork::Dao, ForkCondition::Block(1920000)),
        (Hardfork::Tangerine, ForkCondition::Block(2463000)),
        (Hardfork::SpuriousDragon, ForkCondition::Block(2675000)),
        (Hardfork::Byzantium, ForkCondition::Block(4370000)),
        (Hardfork::Constantinople, ForkCondition::Block(7280000)),
        (Hardfork::Petersburg, ForkCondition::Block(7280000)),
        (Hardfork::Istanbul, ForkCondition::Block(9069000)),
        (Hardfork::MuirGlacier, ForkCondition::Block(9200000)),
        (Hardfork::Berlin, ForkCondition::Block(12244000)),
        (Hardfork::London, ForkCondition::Block(12965000)),
        (Hardfork::ArrowGlacier, ForkCondition::Block(13773000)),
        (Hardfork::GrayGlacier, ForkCondition::Block(15050000)),
        (
            Hardfork::Paris,
            ForkCondition::TTD {
                fork_block: None,
                total_difficulty: U256::from(58_750_000_000_000_000_000_000_u128),
            },
        ),
        (Hardfork::Shanghai, ForkCondition::Timestamp(1681338455)),
    ]),
});

/// The Goerli chain spec.
pub static GOERLI: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain: Chain::goerli(),
    genesis: serde_json::from_str(include_str!("../../res/genesis/goerli.json"))
        .expect("Can't deserialize Goerli genesis json"),
    genesis_hash: Some(constants::GOERLI_GENESIS),
    hardforks: BTreeMap::from([
        (Hardfork::Frontier, ForkCondition::Block(0)),
        (Hardfork::Homestead, ForkCondition::Block(0)),
        (Hardfork::Dao, ForkCondition::Block(0)),
        (Hardfork::Tangerine, ForkCondition::Block(0)),
        (Hardfork::SpuriousDragon, ForkCondition::Block(0)),
        (Hardfork::Byzantium, ForkCondition::Block(0)),
        (Hardfork::Constantinople, ForkCondition::Block(0)),
        (Hardfork::Petersburg, ForkCondition::Block(0)),
        (Hardfork::Istanbul, ForkCondition::Block(1561651)),
        (Hardfork::Berlin, ForkCondition::Block(4460644)),
        (Hardfork::London, ForkCondition::Block(5062605)),
        (
            Hardfork::Paris,
            ForkCondition::TTD {
                fork_block: None,
                total_difficulty: U256::from(10_790_000),
            },
        ),
        (Hardfork::Shanghai, ForkCondition::Timestamp(1678832736)),
    ]),
});

/// The Sepolia chain spec.
pub static SEPOLIA: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain: Chain::sepolia(),
    genesis: serde_json::from_str(include_str!("../../res/genesis/sepolia.json"))
        .expect("Can't deserialize Sepolia genesis json"),
    genesis_hash: Some(constants::SEPOLIA_GENESIS),
    hardforks: BTreeMap::from([
        (Hardfork::Frontier, ForkCondition::Block(0)),
        (Hardfork::Homestead, ForkCondition::Block(0)),
        (Hardfork::Dao, ForkCondition::Block(0)),
        (Hardfork::Tangerine, ForkCondition::Block(0)),
        (Hardfork::SpuriousDragon, ForkCondition::Block(0)),
        (Hardfork::Byzantium, ForkCondition::Block(0)),
        (Hardfork::Constantinople, ForkCondition::Block(0)),
        (Hardfork::Petersburg, ForkCondition::Block(0)),
        (Hardfork::Istanbul, ForkCondition::Block(0)),
        (Hardfork::MuirGlacier, ForkCondition::Block(0)),
        (Hardfork::Berlin, ForkCondition::Block(0)),
        (Hardfork::London, ForkCondition::Block(0)),
        (
            Hardfork::Paris,
            ForkCondition::TTD {
                fork_block: Some(1735371),
                total_difficulty: U256::from(17_000_000_000_000_000u64),
            },
        ),
        (Hardfork::Shanghai, ForkCondition::Timestamp(1677557088)),
    ]),
});

/// An Ethereum chain specification.
///
/// A chain specification describes:
///
/// - Meta-information about the chain (the chain ID)
/// - The genesis block of the chain ([`Genesis`])
/// - What hardforks are activated, and under which conditions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainSpec {
    /// The chain ID
    pub chain: Chain,

    /// The hash of the genesis block.
    ///
    /// This acts as a small cache for known chains. If the chain is known, then
    /// the genesis hash is also known ahead of time, and this will be
    /// `Some`.
    #[serde(skip, default)]
    pub genesis_hash: Option<B256>,

    /// The genesis block
    pub genesis: Genesis,

    /// The active hard forks and their activation conditions
    pub hardforks: BTreeMap<Hardfork, ForkCondition>,
}

impl ChainSpec {
    /// Get information about the chain itself
    pub fn chain(&self) -> Chain {
        self.chain
    }

    /// Get the genesis block specification.
    ///
    /// To get the header for the genesis block, use [`Self::genesis_header`]
    /// instead.
    pub fn genesis(&self) -> &Genesis {
        &self.genesis
    }

    /// Get the header for the genesis block.
    pub fn genesis_header(&self) -> Header {
        // If London is activated at genesis, we set the initial base fee as per
        // EIP-1559.
        let base_fee_per_gas = self.initial_base_fee();

        // If shanghai is activated, initialize the header with an empty withdrawals
        // hash, and empty withdrawals list.
        let withdrawals_root = (self
            .fork(Hardfork::Shanghai)
            .active_at_timestamp(self.genesis.timestamp.to()))
        .then_some(EMPTY_WITHDRAWALS);

        Header {
            gas_limit: self.genesis.gas_limit.to(),
            difficulty: self.genesis.difficulty,
            nonce: self.genesis.nonce.to(),
            extra_data: self.genesis.extra_data.clone(),
            #[cfg(TODO_TRIE)]
            state_root: genesis_state_root(&self.genesis.alloc),
            timestamp: self.genesis.timestamp.to(),
            mix_hash: self.genesis.mix_hash,
            beneficiary: self.genesis.coinbase,
            base_fee_per_gas,
            withdrawals_root,
            ..Default::default()
        }
    }

    /// Get the initial base fee of the genesis block.
    pub fn initial_base_fee(&self) -> Option<u64> {
        // If London is activated at genesis, we set the initial base fee as per
        // EIP-1559.
        (self.fork(Hardfork::London).active_at_block(0)).then_some(EIP1559_INITIAL_BASE_FEE)
    }

    /// Get the hash of the genesis block.
    pub fn genesis_hash(&self) -> B256 {
        if let Some(hash) = self.genesis_hash {
            hash
        } else {
            self.genesis_header().hash_slow()
        }
    }

    /// Returns the forks in this specification and their activation conditions.
    pub fn hardforks(&self) -> &BTreeMap<Hardfork, ForkCondition> {
        &self.hardforks
    }

    /// Get the fork condition for the given fork.
    pub fn fork(&self, fork: Hardfork) -> ForkCondition {
        self.hardforks
            .get(&fork)
            .copied()
            .unwrap_or(ForkCondition::Never)
    }

    /// Get an iterator of all hardforks with their respective activation
    /// conditions.
    pub fn forks_iter(&self) -> impl Iterator<Item = (Hardfork, ForkCondition)> + '_ {
        self.hardforks.iter().map(|(f, b)| (*f, *b))
    }

    /// Convenience method to check if a fork is active at a given timestamp.
    #[inline]
    pub fn is_fork_active_at_timestamp(&self, fork: Hardfork, timestamp: u64) -> bool {
        self.fork(fork).active_at_timestamp(timestamp)
    }

    /// Convenience method to check if [Hardfork::Shanghai] is active at a given
    /// timestamp.
    #[inline]
    pub fn is_shanghai_activated_at_timestamp(&self, timestamp: u64) -> bool {
        self.is_fork_active_at_timestamp(Hardfork::Shanghai, timestamp)
    }

    /// Creates a [`ForkFilter`](crate::ForkFilter) for the block described by
    /// [Head].
    pub fn fork_filter(&self, head: Head) -> ForkFilter {
        let forks = self.forks_iter().filter_map(|(_, condition)| {
            // We filter out TTD-based forks w/o a pre-known block since those do not show
            // up in the fork filter.
            Some(match condition {
                ForkCondition::Block(block) => ForkFilterKey::Block(block),
                ForkCondition::Timestamp(time) => ForkFilterKey::Time(time),
                ForkCondition::TTD {
                    fork_block: Some(block),
                    ..
                } => ForkFilterKey::Block(block),
                _ => return None,
            })
        });

        ForkFilter::new(head, self.genesis_hash(), forks)
    }

    /// Compute the [`ForkId`] for the given [`Head`]
    pub fn fork_id(&self, head: &Head) -> ForkId {
        let mut curr_forkhash = ForkHash::from(self.genesis_hash());
        let mut current_applied_value = 0;

        for (_, cond) in self.forks_iter() {
            let value = match cond {
                ForkCondition::Block(block) => block,
                ForkCondition::Timestamp(time) => time,
                ForkCondition::TTD {
                    fork_block: Some(block),
                    ..
                } => block,
                _ => continue,
            };

            if cond.active_at_head(head) {
                if value != current_applied_value {
                    curr_forkhash += value;
                    current_applied_value = value;
                }
            } else {
                return ForkId {
                    hash: curr_forkhash,
                    next: value,
                }
            }
        }
        ForkId {
            hash: curr_forkhash,
            next: 0,
        }
    }

    /// Build a chainspec using [`ChainSpecBuilder`]
    pub fn builder() -> ChainSpecBuilder {
        ChainSpecBuilder::default()
    }
}

impl From<GethGenesis> for ChainSpec {
    fn from(genesis: GethGenesis) -> Self {
        let GethGenesis {
            config,
            nonce,
            timestamp,
            extra_data,
            gas_limit,
            difficulty,
            mix_hash,
            coinbase,
            alloc,
            number: _,
            gas_used: _,
            parent_hash: _,
            base_fee_per_gas: _,
        } = genesis;

        let genesis_block = Genesis {
            nonce,
            timestamp,
            gas_limit,
            difficulty,
            mix_hash,
            coinbase,
            extra_data,
            alloc,
        };

        // Block-based hardforks
        let hardfork_opts = vec![
            (Hardfork::Homestead, config.homestead_block),
            (Hardfork::Dao, config.dao_fork_block),
            (Hardfork::Tangerine, config.eip150_block),
            (Hardfork::SpuriousDragon, config.eip155_block),
            (Hardfork::Byzantium, config.byzantium_block),
            (Hardfork::Constantinople, config.constantinople_block),
            (Hardfork::Petersburg, config.petersburg_block),
            (Hardfork::Istanbul, config.istanbul_block),
            (Hardfork::MuirGlacier, config.muir_glacier_block),
            (Hardfork::Berlin, config.berlin_block),
            (Hardfork::London, config.london_block),
            (Hardfork::ArrowGlacier, config.arrow_glacier_block),
            (Hardfork::GrayGlacier, config.gray_glacier_block),
        ];
        let mut hardforks = hardfork_opts
            .iter()
            .filter_map(|(hardfork, opt)| {
                opt.map(|block| (*hardfork, ForkCondition::Block(block.to())))
            })
            .collect::<BTreeMap<_, _>>();

        // Paris
        if let Some(ttd) = config.terminal_total_difficulty {
            hardforks.insert(
                Hardfork::Paris,
                ForkCondition::TTD {
                    total_difficulty: ttd.into(),
                    fork_block: config.merge_netsplit_block.map(|x| x.to()),
                },
            );
        }

        // Time-based hardforks
        let time_hardforks = config
            .shanghai_time
            .map(|time| (Hardfork::Shanghai, ForkCondition::Timestamp(time.to())))
            .into_iter()
            .collect::<BTreeMap<_, _>>();

        hardforks.extend(time_hardforks);

        Self {
            chain: config.chain_id.into(),
            genesis: genesis_block,
            genesis_hash: None,
            hardforks,
        }
    }
}

/// A helper type for compatibility with geth's config
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AllGenesisFormats {
    /// The Geth genesis format
    Geth(GethGenesis),
    /// The Reth genesis format
    Reth(ChainSpec),
}

impl From<GethGenesis> for AllGenesisFormats {
    fn from(genesis: GethGenesis) -> Self {
        Self::Geth(genesis)
    }
}

impl From<ChainSpec> for AllGenesisFormats {
    fn from(genesis: ChainSpec) -> Self {
        Self::Reth(genesis)
    }
}

impl From<AllGenesisFormats> for ChainSpec {
    fn from(genesis: AllGenesisFormats) -> Self {
        match genesis {
            AllGenesisFormats::Geth(genesis) => genesis.into(),
            AllGenesisFormats::Reth(genesis) => genesis,
        }
    }
}

/// A helper to build custom chain specs
#[derive(Debug, Default)]
pub struct ChainSpecBuilder {
    chain: Option<Chain>,
    genesis: Option<Genesis>,
    hardforks: BTreeMap<Hardfork, ForkCondition>,
}

impl ChainSpecBuilder {
    /// Construct a new builder from the mainnet chain spec.
    pub fn mainnet() -> Self {
        Self {
            chain: Some(MAINNET.chain),
            genesis: Some(MAINNET.genesis.clone()),
            hardforks: MAINNET.hardforks.clone(),
        }
    }

    /// Set the chain ID
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = Some(chain);
        self
    }

    /// Set the genesis block.
    pub fn genesis(mut self, genesis: Genesis) -> Self {
        self.genesis = Some(genesis);
        self
    }

    /// Add the given fork with the given activation condition to the spec.
    pub fn with_fork(mut self, fork: Hardfork, condition: ForkCondition) -> Self {
        self.hardforks.insert(fork, condition);
        self
    }

    /// Enable Frontier at genesis.
    pub fn frontier_activated(mut self) -> Self {
        self.hardforks
            .insert(Hardfork::Frontier, ForkCondition::Block(0));
        self
    }

    /// Enable Homestead at genesis.
    pub fn homestead_activated(mut self) -> Self {
        self = self.frontier_activated();
        self.hardforks
            .insert(Hardfork::Homestead, ForkCondition::Block(0));
        self
    }

    /// Enable Tangerine at genesis.
    pub fn tangerine_whistle_activated(mut self) -> Self {
        self = self.homestead_activated();
        self.hardforks
            .insert(Hardfork::Tangerine, ForkCondition::Block(0));
        self
    }

    /// Enable Spurious Dragon at genesis.
    pub fn spurious_dragon_activated(mut self) -> Self {
        self = self.tangerine_whistle_activated();
        self.hardforks
            .insert(Hardfork::SpuriousDragon, ForkCondition::Block(0));
        self
    }

    /// Enable Byzantium at genesis.
    pub fn byzantium_activated(mut self) -> Self {
        self = self.spurious_dragon_activated();
        self.hardforks
            .insert(Hardfork::Byzantium, ForkCondition::Block(0));
        self
    }

    /// Enable Petersburg at genesis.
    pub fn petersburg_activated(mut self) -> Self {
        self = self.byzantium_activated();
        self.hardforks
            .insert(Hardfork::Petersburg, ForkCondition::Block(0));
        self
    }

    /// Enable Istanbul at genesis.
    pub fn istanbul_activated(mut self) -> Self {
        self = self.petersburg_activated();
        self.hardforks
            .insert(Hardfork::Istanbul, ForkCondition::Block(0));
        self
    }

    /// Enable Berlin at genesis.
    pub fn berlin_activated(mut self) -> Self {
        self = self.istanbul_activated();
        self.hardforks
            .insert(Hardfork::Berlin, ForkCondition::Block(0));
        self
    }

    /// Enable London at genesis.
    pub fn london_activated(mut self) -> Self {
        self = self.berlin_activated();
        self.hardforks
            .insert(Hardfork::London, ForkCondition::Block(0));
        self
    }

    /// Enable Paris at genesis.
    pub fn paris_activated(mut self) -> Self {
        self = self.london_activated();
        self.hardforks.insert(
            Hardfork::Paris,
            ForkCondition::TTD {
                fork_block: Some(0),
                total_difficulty: U256::ZERO,
            },
        );
        self
    }

    /// Enable Shanghai at genesis.
    pub fn shanghai_activated(mut self) -> Self {
        self = self.paris_activated();
        self.hardforks
            .insert(Hardfork::Shanghai, ForkCondition::Timestamp(0));
        self
    }

    /// Build the resulting [`ChainSpec`].
    ///
    /// # Panics
    ///
    /// This function panics if the chain ID and genesis is not set
    /// ([`Self::chain`] and [`Self::genesis`])
    pub fn build(self) -> ChainSpec {
        ChainSpec {
            chain: self.chain.expect("The chain is required"),
            genesis: self.genesis.expect("The genesis is required"),
            genesis_hash: None,
            hardforks: self.hardforks,
        }
    }
}

impl From<&ChainSpec> for ChainSpecBuilder {
    fn from(value: &ChainSpec) -> Self {
        Self {
            chain: Some(value.chain),
            genesis: Some(value.genesis.clone()),
            hardforks: value.hardforks.clone(),
        }
    }
}

/// The condition at which a fork is activated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ForkCondition {
    /// The fork is activated after a certain block.
    Block(BlockNumber),
    /// The fork is activated after a total difficulty has been reached.
    TTD {
        /// The block number at which TTD is reached, if it is known.
        ///
        /// This should **NOT** be set unless you want this block advertised as
        /// [EIP-2124][eip2124] `FORK_NEXT`. This is currently only the
        /// case for Sepolia.
        ///
        /// [eip2124]: https://eips.ethereum.org/EIPS/eip-2124
        fork_block: Option<BlockNumber>,
        /// The total difficulty after which the fork is activated.
        total_difficulty: U256,
    },
    /// The fork is activated after a specific timestamp.
    Timestamp(u64),
    /// The fork is never activated
    #[default]
    Never,
}

impl ForkCondition {
    /// Checks whether the fork condition is satisfied at the given block.
    ///
    /// For TTD conditions, this will only return true if the activation block
    /// is already known.
    ///
    /// For timestamp conditions, this will always return false.
    pub fn active_at_block(&self, current_block: BlockNumber) -> bool {
        match self {
            ForkCondition::Block(block) => current_block >= *block,
            ForkCondition::TTD {
                fork_block: Some(block),
                ..
            } => current_block >= *block,
            _ => false,
        }
    }

    /// Checks if the given block is the first block that satisfies the fork
    /// condition.
    ///
    /// This will return false for any condition that is not block based.
    pub fn transitions_at_block(&self, current_block: BlockNumber) -> bool {
        match self {
            ForkCondition::Block(block) => current_block == *block,
            _ => false,
        }
    }

    /// Checks whether the fork condition is satisfied at the given total
    /// difficulty and difficulty of a current block.
    ///
    /// The fork is considered active if the _previous_ total difficulty is
    /// above the threshold. To achieve that, we subtract the passed
    /// `difficulty` from the current block's total difficulty, and check if
    /// it's above the Fork Condition's total difficulty (here:
    /// 58_750_000_000_000_000_000_000)
    ///
    /// This will return false for any condition that is not TTD-based.
    pub fn active_at_ttd(&self, ttd: U256, difficulty: U256) -> bool {
        if let ForkCondition::TTD {
            total_difficulty, ..
        } = self
        {
            ttd.saturating_sub(difficulty) >= *total_difficulty
        } else {
            false
        }
    }

    /// Checks whether the fork condition is satisfied at the given timestamp.
    ///
    /// This will return false for any condition that is not timestamp-based.
    pub fn active_at_timestamp(&self, timestamp: u64) -> bool {
        if let ForkCondition::Timestamp(time) = self {
            timestamp >= *time
        } else {
            false
        }
    }

    /// Checks whether the fork condition is satisfied at the given head block.
    ///
    /// This will return true if:
    ///
    /// - The condition is satisfied by the block number;
    /// - The condition is satisfied by the timestamp;
    /// - or the condition is satisfied by the total difficulty
    pub fn active_at_head(&self, head: &Head) -> bool {
        self.active_at_block(head.number)
            || self.active_at_timestamp(head.timestamp)
            || self.active_at_ttd(head.total_difficulty, head.difficulty)
    }

    /// Get the total terminal difficulty for this fork condition.
    ///
    /// Returns `None` for fork conditions that are not TTD based.
    pub fn ttd(&self) -> Option<U256> {
        match self {
            ForkCondition::TTD {
                total_difficulty, ..
            } => Some(*total_difficulty),
            _ => None,
        }
    }

    /// An internal helper function that gives a value that satisfies this
    /// condition.
    pub(crate) fn satisfy(&self) -> Head {
        match *self {
            ForkCondition::Block(number) => Head {
                number,
                ..Default::default()
            },
            ForkCondition::Timestamp(timestamp) => Head {
                timestamp,
                ..Default::default()
            },
            ForkCondition::TTD {
                total_difficulty, ..
            } => Head {
                total_difficulty,
                ..Default::default()
            },
            ForkCondition::Never => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AllGenesisFormats, Chain, ChainSpec, ChainSpecBuilder, ForkCondition, ForkHash, ForkId,
        Genesis, Hardfork, Head, NamedChain, GOERLI, MAINNET, SEPOLIA,
    };
    use bytes::BytesMut;
    use ethers_primitives::{B256, U256};
    use ethers_rlp::Encodable;
    use hex_literal::hex;

    fn test_fork_ids(spec: &ChainSpec, cases: &[(Head, ForkId)]) {
        for (block, expected_id) in cases {
            let computed_id = spec.fork_id(block);
            assert_eq!(
                expected_id, &computed_id,
                "Expected fork ID {:?}, computed fork ID {:?} at block {}",
                expected_id, computed_id, block.number
            );
        }
    }

    // Tests that we skip any fork blocks in block #0 (the genesis ruleset)
    #[test]
    fn ignores_genesis_fork_blocks() {
        let spec = ChainSpec::builder()
            .chain(Chain::mainnet())
            .genesis(Genesis::default())
            .with_fork(Hardfork::Frontier, ForkCondition::Block(0))
            .with_fork(Hardfork::Homestead, ForkCondition::Block(0))
            .with_fork(Hardfork::Tangerine, ForkCondition::Block(0))
            .with_fork(Hardfork::SpuriousDragon, ForkCondition::Block(0))
            .with_fork(Hardfork::Byzantium, ForkCondition::Block(0))
            .with_fork(Hardfork::Constantinople, ForkCondition::Block(0))
            .with_fork(Hardfork::Istanbul, ForkCondition::Block(0))
            .with_fork(Hardfork::MuirGlacier, ForkCondition::Block(0))
            .with_fork(Hardfork::Berlin, ForkCondition::Block(0))
            .with_fork(Hardfork::London, ForkCondition::Block(0))
            .with_fork(Hardfork::ArrowGlacier, ForkCondition::Block(0))
            .with_fork(Hardfork::GrayGlacier, ForkCondition::Block(0))
            .build();

        assert_eq!(spec.hardforks().len(), 12, "12 forks should be active.");
        assert_eq!(
            spec.fork_id(&Head {
                number: 1,
                ..Default::default()
            }),
            ForkId {
                hash: ForkHash::from(spec.genesis_hash()),
                next: 0
            },
            "the fork ID should be the genesis hash; forks at genesis are ignored for fork filters"
        );
    }

    #[test]
    fn ignores_duplicate_fork_blocks() {
        let empty_genesis = Genesis::default();
        let unique_spec = ChainSpec::builder()
            .chain(Chain::mainnet())
            .genesis(empty_genesis.clone())
            .with_fork(Hardfork::Frontier, ForkCondition::Block(0))
            .with_fork(Hardfork::Homestead, ForkCondition::Block(1))
            .build();

        let duplicate_spec = ChainSpec::builder()
            .chain(Chain::mainnet())
            .genesis(empty_genesis)
            .with_fork(Hardfork::Frontier, ForkCondition::Block(0))
            .with_fork(Hardfork::Homestead, ForkCondition::Block(1))
            .with_fork(Hardfork::Tangerine, ForkCondition::Block(1))
            .build();

        assert_eq!(
            unique_spec.fork_id(&Head {
                number: 2,
                ..Default::default()
            }),
            duplicate_spec.fork_id(&Head {
                number: 2,
                ..Default::default()
            }),
            "duplicate fork blocks should be deduplicated for fork filters"
        );
    }

    #[test]
    fn mainnet_forkids() {
        test_fork_ids(
            &MAINNET,
            &[
                (
                    Head {
                        number: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xfc, 0x64, 0xec, 0x04]),
                        next: 1150000,
                    },
                ),
                (
                    Head {
                        number: 1150000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x97, 0xc2, 0xc3, 0x4c]),
                        next: 1920000,
                    },
                ),
                (
                    Head {
                        number: 1920000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x91, 0xd1, 0xf9, 0x48]),
                        next: 2463000,
                    },
                ),
                (
                    Head {
                        number: 2463000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x7a, 0x64, 0xda, 0x13]),
                        next: 2675000,
                    },
                ),
                (
                    Head {
                        number: 2675000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x3e, 0xdd, 0x5b, 0x10]),
                        next: 4370000,
                    },
                ),
                (
                    Head {
                        number: 4370000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xa0, 0x0b, 0xc3, 0x24]),
                        next: 7280000,
                    },
                ),
                (
                    Head {
                        number: 7280000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x66, 0x8d, 0xb0, 0xaf]),
                        next: 9069000,
                    },
                ),
                (
                    Head {
                        number: 9069000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x87, 0x9d, 0x6e, 0x30]),
                        next: 9200000,
                    },
                ),
                (
                    Head {
                        number: 9200000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xe0, 0x29, 0xe9, 0x91]),
                        next: 12244000,
                    },
                ),
                (
                    Head {
                        number: 12244000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x0e, 0xb4, 0x40, 0xf6]),
                        next: 12965000,
                    },
                ),
                (
                    Head {
                        number: 12965000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb7, 0x15, 0x07, 0x7d]),
                        next: 13773000,
                    },
                ),
                (
                    Head {
                        number: 13773000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x20, 0xc3, 0x27, 0xfc]),
                        next: 15050000,
                    },
                ),
                (
                    Head {
                        number: 15050000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xf0, 0xaf, 0xd0, 0xe3]),
                        next: 1681338455,
                    },
                ),
                // First Shanghai block
                (
                    Head {
                        number: 20000000,
                        timestamp: 1681338455,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xdc, 0xe9, 0x6c, 0x2d]),
                        next: 0,
                    },
                ),
                // Future Shanghai block
                (
                    Head {
                        number: 20000000,
                        timestamp: 2000000000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xdc, 0xe9, 0x6c, 0x2d]),
                        next: 0,
                    },
                ),
            ],
        );
    }

    #[test]
    fn goerli_forkids() {
        test_fork_ids(
            &GOERLI,
            &[
                (
                    Head {
                        number: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xa3, 0xf5, 0xab, 0x08]),
                        next: 1561651,
                    },
                ),
                (
                    Head {
                        number: 1561650,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xa3, 0xf5, 0xab, 0x08]),
                        next: 1561651,
                    },
                ),
                (
                    Head {
                        number: 1561651,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xc2, 0x5e, 0xfa, 0x5c]),
                        next: 4460644,
                    },
                ),
                (
                    Head {
                        number: 4460643,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xc2, 0x5e, 0xfa, 0x5c]),
                        next: 4460644,
                    },
                ),
                (
                    Head {
                        number: 4460644,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x75, 0x7a, 0x1c, 0x47]),
                        next: 5062605,
                    },
                ),
                (
                    Head {
                        number: 5062605,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb8, 0xc6, 0x29, 0x9d]),
                        next: 1678832736,
                    },
                ),
                (
                    Head {
                        number: 6000000,
                        timestamp: 1678832735,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb8, 0xc6, 0x29, 0x9d]),
                        next: 1678832736,
                    },
                ),
                // First Shanghai block
                (
                    Head {
                        number: 6000001,
                        timestamp: 1678832736,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xf9, 0x84, 0x3a, 0xbf]),
                        next: 0,
                    },
                ),
                // Future Shanghai block
                (
                    Head {
                        number: 6500000,
                        timestamp: 1678832736,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xf9, 0x84, 0x3a, 0xbf]),
                        next: 0,
                    },
                ),
            ],
        );
    }

    #[test]
    fn sepolia_forkids() {
        test_fork_ids(
            &SEPOLIA,
            &[
                (
                    Head {
                        number: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xfe, 0x33, 0x66, 0xe7]),
                        next: 1735371,
                    },
                ),
                (
                    Head {
                        number: 1735370,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xfe, 0x33, 0x66, 0xe7]),
                        next: 1735371,
                    },
                ),
                (
                    Head {
                        number: 1735371,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb9, 0x6c, 0xbd, 0x13]),
                        next: 1677557088,
                    },
                ),
                (
                    Head {
                        number: 1735372,
                        timestamp: 1677557087,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb9, 0x6c, 0xbd, 0x13]),
                        next: 1677557088,
                    },
                ),
                (
                    Head {
                        number: 1735372,
                        timestamp: 1677557088,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xf7, 0xf9, 0xbc, 0x08]),
                        next: 0,
                    },
                ),
            ],
        );
    }

    /// Checks that time-based forks work
    ///
    /// This is based off of the test vectors here: https://github.com/ethereum/go-ethereum/blob/5c8cc10d1e05c23ff1108022f4150749e73c0ca1/core/forkid/forkid_test.go#L155-L188
    #[test]
    fn timestamped_forks() {
        let mainnet_with_shanghai = ChainSpecBuilder::mainnet()
            .with_fork(Hardfork::Shanghai, ForkCondition::Timestamp(1668000000))
            .build();
        test_fork_ids(
            &mainnet_with_shanghai,
            &[
                (
                    Head {
                        number: 0,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xfc, 0x64, 0xec, 0x04]),
                        next: 1150000,
                    },
                ), // Unsynced
                (
                    Head {
                        number: 1149999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xfc, 0x64, 0xec, 0x04]),
                        next: 1150000,
                    },
                ), // Last Frontier block
                (
                    Head {
                        number: 1150000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x97, 0xc2, 0xc3, 0x4c]),
                        next: 1920000,
                    },
                ), // First Homestead block
                (
                    Head {
                        number: 1919999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x97, 0xc2, 0xc3, 0x4c]),
                        next: 1920000,
                    },
                ), // Last Homestead block
                (
                    Head {
                        number: 1920000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x91, 0xd1, 0xf9, 0x48]),
                        next: 2463000,
                    },
                ), // First DAO block
                (
                    Head {
                        number: 2462999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x91, 0xd1, 0xf9, 0x48]),
                        next: 2463000,
                    },
                ), // Last DAO block
                (
                    Head {
                        number: 2463000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x7a, 0x64, 0xda, 0x13]),
                        next: 2675000,
                    },
                ), // First Tangerine block
                (
                    Head {
                        number: 2674999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x7a, 0x64, 0xda, 0x13]),
                        next: 2675000,
                    },
                ), // Last Tangerine block
                (
                    Head {
                        number: 2675000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x3e, 0xdd, 0x5b, 0x10]),
                        next: 4370000,
                    },
                ), // First Spurious block
                (
                    Head {
                        number: 4369999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x3e, 0xdd, 0x5b, 0x10]),
                        next: 4370000,
                    },
                ), // Last Spurious block
                (
                    Head {
                        number: 4370000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xa0, 0x0b, 0xc3, 0x24]),
                        next: 7280000,
                    },
                ), // First Byzantium block
                (
                    Head {
                        number: 7279999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xa0, 0x0b, 0xc3, 0x24]),
                        next: 7280000,
                    },
                ), // Last Byzantium block
                (
                    Head {
                        number: 7280000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x66, 0x8d, 0xb0, 0xaf]),
                        next: 9069000,
                    },
                ), // First and last Constantinople, first Petersburg block
                (
                    Head {
                        number: 9068999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x66, 0x8d, 0xb0, 0xaf]),
                        next: 9069000,
                    },
                ), // Last Petersburg block
                (
                    Head {
                        number: 9069000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x87, 0x9d, 0x6e, 0x30]),
                        next: 9200000,
                    },
                ), // First Istanbul and first Muir Glacier block
                (
                    Head {
                        number: 9199999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x87, 0x9d, 0x6e, 0x30]),
                        next: 9200000,
                    },
                ), // Last Istanbul and first Muir Glacier block
                (
                    Head {
                        number: 9200000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xe0, 0x29, 0xe9, 0x91]),
                        next: 12244000,
                    },
                ), // First Muir Glacier block
                (
                    Head {
                        number: 12243999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xe0, 0x29, 0xe9, 0x91]),
                        next: 12244000,
                    },
                ), // Last Muir Glacier block
                (
                    Head {
                        number: 12244000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x0e, 0xb4, 0x40, 0xf6]),
                        next: 12965000,
                    },
                ), // First Berlin block
                (
                    Head {
                        number: 12964999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x0e, 0xb4, 0x40, 0xf6]),
                        next: 12965000,
                    },
                ), // Last Berlin block
                (
                    Head {
                        number: 12965000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb7, 0x15, 0x07, 0x7d]),
                        next: 13773000,
                    },
                ), // First London block
                (
                    Head {
                        number: 13772999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xb7, 0x15, 0x07, 0x7d]),
                        next: 13773000,
                    },
                ), // Last London block
                (
                    Head {
                        number: 13773000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x20, 0xc3, 0x27, 0xfc]),
                        next: 15050000,
                    },
                ), // First Arrow Glacier block
                (
                    Head {
                        number: 15049999,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x20, 0xc3, 0x27, 0xfc]),
                        next: 15050000,
                    },
                ), // Last Arrow Glacier block
                (
                    Head {
                        number: 15050000,
                        timestamp: 0,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xf0, 0xaf, 0xd0, 0xe3]),
                        next: 1668000000,
                    },
                ), // First Gray Glacier block
                (
                    Head {
                        number: 19999999,
                        timestamp: 1667999999,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0xf0, 0xaf, 0xd0, 0xe3]),
                        next: 1668000000,
                    },
                ), // Last Gray Glacier block
                (
                    Head {
                        number: 20000000,
                        timestamp: 1668000000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x71, 0x14, 0x76, 0x44]),
                        next: 0,
                    },
                ), // First Shanghai block
                (
                    Head {
                        number: 20000000,
                        timestamp: 2668000000,
                        ..Default::default()
                    },
                    ForkId {
                        hash: ForkHash([0x71, 0x14, 0x76, 0x44]),
                        next: 0,
                    },
                ), // Future Shanghai block
            ],
        );
    }

    /// Checks that the fork is not active at a terminal ttd block.
    #[test]
    fn check_terminal_ttd() {
        let chainspec = ChainSpecBuilder::mainnet().build();

        // Check that Paris is not active on terminal PoW block #15537393.
        let terminal_block_ttd = U256::from(58750003716598352816469_u128);
        let terminal_block_difficulty = U256::from(11055787484078698_u128);
        assert!(!chainspec
            .fork(Hardfork::Paris)
            .active_at_ttd(terminal_block_ttd, terminal_block_difficulty));

        // Check that Paris is active on first PoS block #15537394.
        let first_pos_block_ttd = U256::from(58750003716598352816469_u128);
        let first_pos_difficulty = U256::ZERO;
        assert!(chainspec
            .fork(Hardfork::Paris)
            .active_at_ttd(first_pos_block_ttd, first_pos_difficulty));
    }

    #[test]
    fn geth_genesis_with_shanghai() {
        let geth_genesis = r#"
        {
            "config": {
                "chainId": 1337,
                "homesteadBlock": 0,
                "eip150Block": 0,
                "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "eip155Block": 0,
                "eip158Block": 0,
                "byzantiumBlock": 0,
                "constantinopleBlock": 0,
                "petersburgBlock": 0,
                "istanbulBlock": 0,
                "muirGlacierBlock": 0,
                "berlinBlock": 0,
                "londonBlock": 0,
                "arrowGlacierBlock": 0,
                "grayGlacierBlock": 0,
                "shanghaiTime": 0,
                "terminalTotalDifficulty": 0,
                "terminalTotalDifficultyPassed": true,
                "ethash": {}
            },
            "nonce": "0x0",
            "timestamp": "0x0",
            "extraData": "0x",
            "gasLimit": "0x4c4b40",
            "difficulty": "0x1",
            "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "coinbase": "0x0000000000000000000000000000000000000000",
            "alloc": {
                "658bdf435d810c91414ec09147daa6db62406379": {
                    "balance": "0x487a9a304539440000"
                },
                "aa00000000000000000000000000000000000000": {
                    "code": "0x6042",
                    "storage": {
                        "0x0000000000000000000000000000000000000000000000000000000000000000": "0x0000000000000000000000000000000000000000000000000000000000000000",
                        "0x0100000000000000000000000000000000000000000000000000000000000000": "0x0100000000000000000000000000000000000000000000000000000000000000",
                        "0x0200000000000000000000000000000000000000000000000000000000000000": "0x0200000000000000000000000000000000000000000000000000000000000000",
                        "0x0300000000000000000000000000000000000000000000000000000000000000": "0x0000000000000000000000000000000000000000000000000000000000000303"
                    },
                    "balance": "0x1",
                    "nonce": "0x1"
                },
                "bb00000000000000000000000000000000000000": {
                    "code": "0x600154600354",
                    "storage": {
                        "0x0000000000000000000000000000000000000000000000000000000000000000": "0x0000000000000000000000000000000000000000000000000000000000000000",
                        "0x0100000000000000000000000000000000000000000000000000000000000000": "0x0100000000000000000000000000000000000000000000000000000000000000",
                        "0x0200000000000000000000000000000000000000000000000000000000000000": "0x0200000000000000000000000000000000000000000000000000000000000000",
                        "0x0300000000000000000000000000000000000000000000000000000000000000": "0x0000000000000000000000000000000000000000000000000000000000000303"
                    },
                    "balance": "0x2",
                    "nonce": "0x1"
                }
            },
            "number": "0x0",
            "gasUsed": "0x0",
            "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "baseFeePerGas": "0x3b9aca00"
        }
        "#;

        let genesis: GethGenesis = serde_json::from_str(geth_genesis).unwrap();
        let chainspec = ChainSpec::from(genesis);

        // assert a bunch of hardforks that should be set
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Homestead).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Tangerine).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::SpuriousDragon).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Byzantium).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Constantinople).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Petersburg).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Istanbul).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::MuirGlacier).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Berlin).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::London).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::ArrowGlacier).unwrap(),
            &ForkCondition::Block(0)
        );
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::GrayGlacier).unwrap(),
            &ForkCondition::Block(0)
        );

        // including time based hardforks
        assert_eq!(
            chainspec.hardforks.get(&Hardfork::Shanghai).unwrap(),
            &ForkCondition::Timestamp(0)
        );

        // alloc key -> expected rlp mapping
        let key_rlp = vec![
            (hex!("658bdf435d810c91414ec09147daa6db62406379"), "f84d8089487a9a304539440000a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"),
            (hex!("aa00000000000000000000000000000000000000"), "f8440101a08afc95b7d18a226944b9c2070b6bda1c3a36afcc3730429d47579c94b9fe5850a0ce92c756baff35fa740c3557c1a971fd24d2d35b7c8e067880d50cd86bb0bc99"),
            (hex!("bb00000000000000000000000000000000000000"), "f8440102a08afc95b7d18a226944b9c2070b6bda1c3a36afcc3730429d47579c94b9fe5850a0e25a53cbb501cec2976b393719c63d832423dd70a458731a0b64e4847bbca7d2"),
        ];

        for (key, expected_rlp) in key_rlp {
            let account = chainspec
                .genesis
                .alloc
                .get(&Address::from(key))
                .expect("account should exist");
            let mut account_rlp = BytesMut::new();
            account.encode(&mut account_rlp);
            assert_eq!(hex::encode(account_rlp), expected_rlp)
        }

        assert_eq!(chainspec.genesis_hash, None);
        let expected_state_root: B256 =
            hex_literal::hex!("078dc6061b1d8eaa8493384b59c9c65ceb917201221d08b80c4de6770b6ec7e7")
                .into();
        assert_eq!(chainspec.genesis_header().state_root, expected_state_root);

        let expected_withdrawals_hash: B256 =
            hex_literal::hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421")
                .into();
        assert_eq!(
            chainspec.genesis_header().withdrawals_root,
            Some(expected_withdrawals_hash)
        );

        let expected_hash: B256 =
            hex_literal::hex!("1fc027d65f820d3eef441ebeec139ebe09e471cf98516dce7b5643ccb27f418c")
                .into();
        let hash = chainspec.genesis_hash();
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn hive_geth_json() {
        let hive_json = r#"
        {
            "nonce": "0x0000000000000042",
            "difficulty": "0x2123456",
            "mixHash": "0x123456789abcdef123456789abcdef123456789abcdef123456789abcdef1234",
            "coinbase": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "timestamp": "0x123456",
            "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "extraData": "0xfafbfcfd",
            "gasLimit": "0x2fefd8",
            "alloc": {
                "dbdbdb2cbd23b783741e8d7fcf51e459b497e4a6": {
                    "balance": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                },
                "e6716f9544a56c530d868e4bfbacb172315bdead": {
                    "balance": "0x11",
                    "code": "0x12"
                },
                "b9c015918bdaba24b4ff057a92a3873d6eb201be": {
                    "balance": "0x21",
                    "storage": {
                        "0x0000000000000000000000000000000000000000000000000000000000000001": "0x22"
                    }
                },
                "1a26338f0d905e295fccb71fa9ea849ffa12aaf4": {
                    "balance": "0x31",
                    "nonce": "0x32"
                },
                "0000000000000000000000000000000000000001": {
                    "balance": "0x41"
                },
                "0000000000000000000000000000000000000002": {
                    "balance": "0x51"
                },
                "0000000000000000000000000000000000000003": {
                    "balance": "0x61"
                },
                "0000000000000000000000000000000000000004": {
                    "balance": "0x71"
                }
            },
            "config": {
                "ethash": {},
                "chainId": 10,
                "homesteadBlock": 0,
                "eip150Block": 0,
                "eip155Block": 0,
                "eip158Block": 0,
                "byzantiumBlock": 0,
                "constantinopleBlock": 0,
                "petersburgBlock": 0,
                "istanbulBlock": 0
            }
        }
        "#;
        let genesis = serde_json::from_str::<AllGenesisFormats>(hive_json).unwrap();
        let chainspec: ChainSpec = genesis.into();
        assert_eq!(chainspec.genesis_hash, None);
        assert_eq!(Chain::Named(NamedChain::Optimism), chainspec.chain);
        let expected_state_root: B256 =
            hex_literal::hex!("9a6049ac535e3dc7436c189eaa81c73f35abd7f282ab67c32944ff0301d63360")
                .into();
        assert_eq!(chainspec.genesis_header().state_root, expected_state_root);
        let hard_forks = vec![
            Hardfork::Byzantium,
            Hardfork::Homestead,
            Hardfork::Istanbul,
            Hardfork::Petersburg,
            Hardfork::Constantinople,
        ];
        for ref fork in hard_forks {
            assert_eq!(
                chainspec.hardforks.get(fork).unwrap(),
                &ForkCondition::Block(0)
            );
        }

        let expected_hash: B256 =
            hex_literal::hex!("5ae31c6522bd5856129f66be3d582b842e4e9faaa87f21cce547128339a9db3c")
                .into();
        let hash = chainspec.genesis_header().hash_slow();
        assert_eq!(hash, expected_hash);
    }
}
