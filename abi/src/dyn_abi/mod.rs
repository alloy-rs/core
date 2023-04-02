//! Dynamic Solidity Type Encoder
//!
//! This module provides a runtime encoder/decoder for solidity types. It is
//! intended to be used when the solidity type is not known at compile time.
//! This is particularly useful for EIP-712 encoding and signing.
//!
mod sol_type;
pub use sol_type::DynSolType;

mod sol_value;
pub use sol_value::DynSolValue;

mod token;
pub use token::DynToken;

mod parser;
pub use parser::parse;
