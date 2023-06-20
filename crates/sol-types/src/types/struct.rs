//! This module contains the [`SolStruct`] trait, which is used to implement
//! Solidity structs logic, particularly for EIP-712 encoding/decoding.

use super::{r#type::Encodable, SolType};
use crate::{no_std_prelude::*, token::TokenSeq, Eip712Domain, Word};
use alloy_primitives::{keccak256, B256};

type TupleFor<'a, T> = <T as SolStruct>::Tuple<'a>;
type TupleTokenTypeFor<'a, T> = <TupleFor<'a, T> as SolType>::TokenType<'a>;

/// A Solidity Struct.
///
/// This trait is used to implement ABI and EIP-712 encoding and decoding.
///
/// # Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity struct
/// definition.
///
/// # Note
///
/// Special attention should be paid to [`eip712_encode_type`] for complex
/// Solidity types. Nested Solidity structs **must** properly encode their type.
///
/// To be clear, a struct with a nested struct must encode the nested struct's
/// type as well.
///
/// See [EIP-712#definition-of-encodetype][ref] for more details.
///
/// [`eip712_encode_type`]: SolStruct::eip712_encode_type
/// [ref]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
pub trait SolStruct: 'static {
    /// The corresponding Tuple type, used for encoding/decoding.
    type Tuple<'a>: SolType<TokenType<'a> = Self::Token<'a>>;

    /// The corresponding Token type.
    type Token<'a>: TokenSeq<'a>;

    /// The struct name.
    ///
    /// Used in [`eip712_encode_type`][SolStruct::eip712_encode_type].
    const NAME: &'static str;

    /// The field types and names. Type is a Solidity string, and must conform
    /// to the name of the Solidty type at the same index in the associated
    /// tuple.
    ///
    /// Used in [`eip712_encode_type`][SolStruct::eip712_encode_type].
    const FIELDS: &'static [(&'static str, &'static str)];

    // TODO: avoid clones here
    /// Convert to the tuple type used for ABI encoding and decoding.
    fn to_rust<'a>(&self) -> <Self::Tuple<'a> as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn from_rust(tuple: <Self::Tuple<'_> as SolType>::RustType) -> Self;

    /// Convert to the token type used for EIP-712 encoding and decoding.
    fn tokenize(&self) -> Self::Token<'_>;

    /// The size of the struct when encoded, in bytes
    fn encoded_size(&self) -> usize {
        // This avoids unnecessary clones.
        if let Some(size) = <Self::Tuple<'_> as SolType>::ENCODED_SIZE {
            return size
        }

        <<Self as SolStruct>::Tuple<'_> as SolType>::encoded_size(&self.to_rust())
    }

    /// EIP-712 `encodeType`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    fn eip712_encode_type() -> Cow<'static, str> {
        let capacity = Self::FIELDS
            .iter()
            .map(|(ty, name)| ty.len() + name.len() + 1)
            .sum::<usize>()
            + Self::NAME.len()
            + 2;
        let mut out = String::with_capacity(capacity);
        out.push_str(Self::NAME);
        out.push('(');
        for (i, &(ty, name)) in Self::FIELDS.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(ty);
            out.push(' ');
            out.push_str(name);
        }
        out.push(')');
        out.into()
    }

    /// EIP-712 `typeHash`
    /// <https://eips.ethereum.org/EIPS/eip-712#rationale-for-typehash>
    #[inline]
    fn eip712_type_hash(&self) -> B256 {
        keccak256(Self::eip712_encode_type().as_bytes())
    }

    /// EIP-712 `encodeData`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    fn eip712_encode_data(&self) -> Vec<u8>;

    /// EIP-712 `hashStruct`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct>
    #[inline]
    fn eip712_hash_struct(&self) -> B256 {
        let mut type_hash = self.eip712_type_hash().to_vec();
        type_hash.extend(self.eip712_encode_data());
        keccak256(type_hash)
    }

    /// EIP-712 `signTypedData`
    /// <https://eips.ethereum.org/EIPS/eip-712#specification-of-the-eth_signtypeddata-json-rpc>
    #[inline]
    fn eip712_signing_hash(&self, domain: &Eip712Domain) -> B256 {
        let domain_separator = domain.hash_struct();
        let struct_hash = self.eip712_hash_struct();

        let mut digest_input = [0u8; 2 + 32 + 32];
        digest_input[0] = 0x19;
        digest_input[1] = 0x01;
        digest_input[2..34].copy_from_slice(&domain_separator[..]);
        digest_input[34..66].copy_from_slice(&struct_hash[..]);
        keccak256(digest_input)
    }
}

impl<T: SolStruct> Encodable<T> for T {
    fn to_tokens(&self) -> <Self as SolType>::TokenType<'_> {
        <Self as SolStruct>::tokenize(self)
    }
}

// blanket impl
// TODO: Maybe move this to `sol!`?
impl<T: SolStruct> SolType for T {
    type RustType = T;
    type TokenType<'a> = TupleTokenTypeFor<'a, T>;

    const DYNAMIC: bool = TupleFor::<T>::DYNAMIC;

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> crate::Result<()> {
        TupleFor::<T>::type_check(token)
    }

    #[inline]
    fn encoded_size<'a>(rust: &Self::RustType) -> usize {
        let tuple = rust.to_rust();
        TupleFor::<T>::encoded_size(&tuple)
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        Self::NAME.into()
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        let tuple = TupleFor::<T>::detokenize(token);
        T::from_rust(tuple)
    }

    #[inline]
    fn eip712_encode_type() -> Option<Cow<'static, str>> {
        Some(Self::eip712_encode_type())
    }

    #[inline]
    fn eip712_data_word<'a>(rust: &Self::RustType) -> Word {
        keccak256(SolStruct::eip712_hash_struct(rust.borrow()))
    }

    #[inline]
    fn encode_packed_to<'a>(rust: &Self::RustType, out: &mut Vec<u8>) {
        let tuple = rust.to_rust();
        TupleFor::<T>::encode_packed_to(&tuple, out)
    }
}
