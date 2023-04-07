//! Dynamic Solidity Type Encoder
//!
//! This module provides a runtime encoder/decoder for solidity types. It is
//! intended to be used when the solidity type is not known at compile time.
//! This is particularly useful for EIP-712 signing interfaces.
//!
//! We **strongly** recommend using the static encoder/decoder when possible.
//! The dyanmic encoder/decoder is significantly more expensive, especially for
//! complex types. It is also significantly more error prone, as the mapping
//! between solidity types and rust types is not enforced by the compiler.
//!
//! ## Example
//!
//! ```
//! # use ethers_abi_enc::{Decoder, Encoder, DynSolType, DynSolValue};
//! // parse a type from a string
//! let my_type: DynSolType = "uint8[2][]".parse().unwrap();
//!
//! // set values
//! let uints = DynSolValue::FixedArray(vec![0u8.into(), 1u8.into()]);
//! let my_values = DynSolValue::Array(vec![uints]);
//!
//! // encode
//! let encoded = my_type.encode_single(my_values.clone()).unwrap();
//!
//! // decode
//! let decoded = my_type.decode_single(&encoded).unwrap();
//!
//! assert_eq!(decoded, my_values);
//! ```
//!
//! ## How it works
//!
//! The dynamic encodr/decoder is implemented as a set of enums that represent
//! solidity types, solidity values (in rust representation form), and ABI
//! tokens. Unlike the static encoder, each of these must be instantiated at
//! runtime. The [`DynSolType`] enum represents a solidity type, and is
//! equivalent to an enum over types implementing the [`crate::SolType`] trait.
//! The [`DynSolValue`] enum represents a solidity value, and describes the
//! rust shapes of possible solidity values. It is similar to, but not
//! equivalent to an enum over types used as [`crate::SolType::RustType`]. The
//! [`DynToken`] enum represents an ABI token, and is equivalent to an enum over
//! the types implementing the [`crate::TokenType`] trait.
//!
//! Where the static encoding system encodes the expected type information into
//! the rust type system, the dynamic encoder/decoder encodes it as a concrete
//! instance of [`DynSolType`]. This type is used to tokenize and detokenize
//! [`DynSolValue`] instances. The [`parse`] function is used to parse a
//! solidity type string into a [`DynSolType`] object.
//!
//! Tokenizing - `DynSolType + `DynSolValue` = `DynToken`
//! Detokenizing - `DynSolType` + `DynToken` = `DynSolValue`
//!
//! Users must manually handle the conversions between [`DynSolValue`] and their
//! own rust types. We provide several `From` implementations, but they fall
//! short when dealing with arrays and tuples. We also provide fallible casts
//! into the contents of each variant.
//!
//! ## `DynToken::decode_populate`
//!
//! Because the shape of the data is known only at runtime, we cannot
//! compile-time allocate the memory needed to hold decoded data. Instead, we
//! pre-allocate a [`DynToken`] with the same shape as the expected type, and
//! empty values. We then populate the empty values with the decoded data.
//!
//! This is a significant behavior departure from the static decoder. We do not
//! recommend using the [`DynToken`] type directly. Instead, we recommend using
//! the encoding and decoding methods on [`DynSolType`].
mod sol_type;
pub use sol_type::DynSolType;

mod sol_value;
pub use sol_value::DynSolValue;

mod token;
pub use token::DynToken;

mod parser;
pub use parser::ParserError;

#[cfg(test)]
mod test {
    use crate::{decoder::Decoder, encoder::Encoder, DynSolType, DynSolValue};

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
