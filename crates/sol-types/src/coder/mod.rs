//! ABI encoder.
//!
//! ### `{encode,decode}`
//!
//! [`crate::SolType::encode()`] and [`encode()`] operate on a
//! single token. They wrap this token in a tuple, and pass it to the encoder.
//! Use this interface when abi-encoding a single token. This is suitable for
//! encoding a type in isolation, or for encoding parameters for single-param
//! functions.
//!
//! ### `{encode,decode}_params`
//!
//! [`crate::SolType::encode_params()`] and [`encode_params()`] operate on a
//! sequence. If the sequence is a tuple, the tuple is inferred to be a set of
//! Solidity function parameters,
//!
//! The corresponding [`crate::SolType::decode_params()`] and
//! [`crate::decode_params()`] reverse this operation, decoding a tuple from a
//! blob.
//!
//! This is used to encode the parameters for a Solidity function.
//!
//! ### `{encode,decode}_sequence`
//!
//! [`crate::SolType::encode()`] and [`encode()`] operate on a sequence of
//! tokens. This sequence is inferred not to be function parameters.
//!
//! This is the least useful one. Most users will not need it.

mod encoder;
pub use encoder::{encode, encode_params, encode_sequence, Encoder};

mod decoder;
pub use decoder::{decode, decode_params, decode_sequence, Decoder};

pub mod token;
