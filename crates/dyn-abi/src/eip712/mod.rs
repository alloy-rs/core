//! Implementation of dynamic EIP-712.
//!
//! This allows for the encoding of EIP-712 messages without having to know the
//! types at compile time. This is useful for things like off-chain signing.
//! It implements the encoding portion of the EIP-712 spec, and does not
//! contain any of the signing logic.
//!
//! <https://eips.ethereum.org/EIPS/eip-712#specification-of-the-eth_signtypeddata-json-rpc>

pub mod parser;

mod typed_data;
pub use typed_data::{Eip712Types, TypedData};

mod resolver;
pub use resolver::{PropertyDef, Resolver, TypeDef};

pub(crate) mod coerce;
