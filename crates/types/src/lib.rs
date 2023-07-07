#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![allow(unused_imports, ambiguous_glob_reexports, hidden_glob_reexports)] // TODO
#![warn(
    missing_docs,
    // unreachable_pub, // TODO
    // missing_copy_implementations, // TODO
    missing_debug_implementations,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
// TODO: no_std ?
// #![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

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

mod rpc;
pub use rpc::*;
