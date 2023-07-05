// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_docs,
    unreachable_pub,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate alloc;

mod error;
pub use error::{DynAbiError, DynAbiResult};

#[doc(no_inline)]
pub use alloy_sol_types::{Decoder, Eip712Domain, Encoder, Error, Result, SolType, Word};

mod r#type;
pub use r#type::DynSolType;

mod value;
pub use value::DynSolValue;

mod token;
pub use token::DynToken;

pub mod parser;

#[cfg(feature = "eip712")]
pub mod eip712;
#[cfg(feature = "eip712")]
pub use eip712::{parser as eip712_parser, Eip712Types, PropertyDef, Resolver, TypeDef, TypedData};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_e2e() {
        // parse a type from a string
        let my_type: DynSolType = "uint8[2][]".parse().unwrap();

        // set values
        let uints = DynSolValue::FixedArray(vec![64u8.into(), 128u8.into()]);
        let my_values = DynSolValue::Array(vec![uints]);

        // tokenize and detokenize
        let tokens = my_values.tokenize();
        let detokenized = my_type.detokenize(tokens.clone()).unwrap();
        assert_eq!(detokenized, my_values);

        // encode
        let encoded = my_values.clone().encode_single();

        // decode
        let mut decoder = Decoder::new(&encoded, true);
        let mut decoded = my_type.empty_dyn_token();
        decoded.decode_single_populate(&mut decoder).unwrap();

        assert_eq!(decoded, tokens);
    }
}
