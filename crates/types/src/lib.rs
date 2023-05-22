#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    missing_debug_implementations,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
// TODO: no_std ?

/* --- Primitives --- */
pub mod constants;

pub mod chain;
pub use chain::{
    AllGenesisFormats, Chain, ChainInfo, ChainSpec, ChainSpecBuilder, ForkCondition, NamedChain,
    GOERLI, MAINNET, SEPOLIA,
};

pub mod genesis;
pub use genesis::{Genesis, GenesisAccount};

mod primitives;
pub use primitives::*;

mod utils;
pub use utils::*;

/* --- RPC Types --- */
mod admin;
pub use admin::*;

mod eth;
pub use eth::*;
