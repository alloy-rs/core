// Copyright 2015-2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Ethereum ABI params.

use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::Word;

/// Ethereum ABI params.
#[derive(PartialEq, Clone)]
pub enum Token {
    /// Single Word
    Word(Word),
    /// Tuple or `T[M]`
    FixedSeq(Vec<Token>),
    /// T[]
    DynSeq(Vec<Token>),
    /// String or Bytes
    PackedSeq(Vec<u8>),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word(arg0) => f.debug_tuple("Word").field(arg0).finish(),
            Self::FixedSeq(arg0) => f.debug_tuple("FixedSeq").field(arg0).finish(),
            Self::DynSeq(arg0) => f.debug_tuple("DynSeq").field(arg0).finish(),
            Self::PackedSeq(arg0) => f
                .debug_tuple("PackedSeq")
                .field(&hex::encode(arg0))
                .finish(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Word(contents) => write!(f, "Word {contents}"),
            Token::FixedSeq(contents) => write!(f, "FixedSeq {contents:?}"),
            Token::DynSeq(contents) => write!(f, "DynSeq {contents:?}"),
            Token::PackedSeq(contents) => write!(f, "PackedSeq {contents:?}"),
        }
    }
}

impl Token {
    /// Return a reference to the underlying word for a value type
    pub fn as_word(&self) -> Option<&Word> {
        match self {
            Token::Word(word) => Some(word),
            _ => None,
        }
    }

    /// Return a reference to the underlying word for a value type
    pub fn as_word_array(&self) -> Option<&[u8; 32]> {
        self.as_word().map(AsRef::as_ref)
    }

    /// Return a reference to the underlying buffer for a packed sequence
    /// (string or bytes)
    pub fn as_packed_data(&self) -> Option<&[u8]> {
        match self {
            Token::PackedSeq(buf) => Some(buf.as_ref()),
            _ => None,
        }
    }

    /// Return a reference to the underlying vector for a dynamic sequence
    pub fn as_dyn_seq(&self) -> Option<&[Token]> {
        match self {
            Token::DynSeq(buf) => Some(buf.as_ref()),
            _ => None,
        }
    }

    /// Return a reference to the underlying vector for a dynamic sequence
    pub fn as_fixed_seq(&self) -> Option<&[Token]> {
        match self {
            Token::FixedSeq(buf) => Some(buf.as_ref()),
            _ => None,
        }
    }

    /// Check if the token is a dynamic type resulting in prefixed encoding
    pub fn is_dynamic(&self) -> bool {
        match self {
            Token::DynSeq(_) | Token::PackedSeq(_) => true,
            Token::FixedSeq(tokens) => tokens.iter().any(Token::is_dynamic),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers_primitives::B256;

    #[cfg(not(feature = "std"))]
    use crate::no_std_prelude::*;
    use crate::{sol_type, SolType, Token};

    macro_rules! assert_type_check {
        ($sol:ty, $token:expr) => {
            assert!(<$sol>::type_check($token))
        };
        ($sol:ty, $token:expr,) => {
            assert_type_check!($sol, $token)
        };
    }

    macro_rules! assert_not_type_check {
        ($sol:ty, $token:expr) => {
            assert!(!<$sol>::type_check($token))
        };
        ($sol:ty, $token:expr,) => {
            assert_not_type_check!($sol, $token)
        };
    }

    #[test]
    fn test_type_check() {
        assert_type_check!(
            (sol_type::Uint<256>, sol_type::Bool),
            &Token::FixedSeq(vec!(
                Token::Word(B256::default()),
                Token::Word(B256::default())
            )),
        );
        assert_type_check!(
            (sol_type::Uint<32>, sol_type::Bool),
            &Token::FixedSeq(vec!(
                Token::Word(B256::default()),
                Token::Word(B256::default())
            )),
        );

        assert_not_type_check!(
            (sol_type::Uint<32>, sol_type::Bool),
            &Token::Word(B256::default()),
        );

        assert_not_type_check!(
            sol_type::Uint<32>,
            &Token::FixedSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );
        assert_type_check!(
            (sol_type::Uint<32>, sol_type::Bool),
            &Token::FixedSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default())
            ]),
        );

        assert_type_check!(
            sol_type::Array<sol_type::Bool>,
            &Token::DynSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );
        assert_type_check!(
            sol_type::Array<sol_type::Bool>,
            &Token::DynSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );
        assert_type_check!(
            sol_type::Array<sol_type::Address>,
            &Token::DynSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );

        assert_type_check!(
            sol_type::FixedArray<sol_type::Bool, 2>,
            &Token::FixedSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );
        assert_not_type_check!(
            sol_type::FixedArray<sol_type::Bool, 3>,
            &Token::FixedSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );

        assert_type_check!(
            sol_type::FixedArray<sol_type::Address, 2>,
            &Token::FixedSeq(vec![
                Token::Word(B256::default()),
                Token::Word(B256::default()),
            ]),
        );
    }

    #[test]
    fn test_is_dynamic() {
        assert!(!Token::Word(B256::default()).is_dynamic());
        assert!(Token::PackedSeq(vec![0, 0, 0, 0]).is_dynamic());
        assert!(!Token::Word(B256::default()).is_dynamic());
        assert!(!Token::Word(B256::default()).is_dynamic());
        assert!(!Token::Word(B256::default()).is_dynamic());
        assert!(Token::PackedSeq("".into()).is_dynamic());
        assert!(Token::DynSeq(vec![Token::Word(B256::default())]).is_dynamic());
        assert!(!Token::FixedSeq(vec![Token::Word(B256::default())]).is_dynamic());
        assert!(Token::FixedSeq(vec![Token::PackedSeq("".into())]).is_dynamic());
        assert!(
            Token::FixedSeq(vec![Token::DynSeq(vec![Token::Word(B256::default())])]).is_dynamic()
        );
    }
}
