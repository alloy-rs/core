//! Encoding/Decoding

mod impl_core;

mod encoder;
pub use encoder::{encode, encode_params, encode_single, Encoder};

mod decoder;
pub use decoder::{decode, decode_params, decode_single, Decoder};

pub mod token;
