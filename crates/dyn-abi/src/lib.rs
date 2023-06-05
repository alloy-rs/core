// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate alloc;

mod no_std_prelude {
    pub(crate) use alloc::{
        borrow::{Borrow, ToOwned},
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };
}

mod error;
pub use error::DynAbiError;

pub use ethers_sol_types::{Decoder, Eip712Domain, Encoder, Error, Result, SolType, Word};

mod r#type;
pub use r#type::DynSolType;

mod value;
pub use value::DynSolValue;

mod token;
pub use token::DynToken;

pub mod parser;

pub mod eip712;
pub use eip712::{parser as eip712_parser, Resolver, TypedData};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_e2e() {
        // parse a type from a string
        let my_type: DynSolType = "uint8[2][]".parse().unwrap();

        // set values
        let uints = DynSolValue::FixedArray(vec![64u8.into(), 128u8.into()]);
        let my_values = DynSolValue::Array(vec![uints]);

        // tokenize and detokenize
        let tokens = my_type.tokenize(my_values.clone()).unwrap();
        let detokenized = my_type.detokenize(tokens.clone()).unwrap();
        assert_eq!(detokenized, my_values);

        // encode
        let mut encoder = Encoder::default();
        tokens.encode_single(&mut encoder).unwrap();
        let encoded = encoder.into_bytes();

        // decode
        let mut decoder = Decoder::new(&encoded, true);
        let mut decoded = my_type.empty_dyn_token();
        decoded.decode_single_populate(&mut decoder).unwrap();

        assert_eq!(decoded, tokens);
    }
}
