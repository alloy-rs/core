//! This module contains the [`SolStruct`] trait, which is used to implement
//! Solidity structs logic, particularly for EIP-712 encoding/decoding.

use super::SolType;
use crate::{no_std_prelude::*, token::TokenSeq, Eip712Domain, Result, Word};
use alloy_primitives::{keccak256, B256};

type TupleFor<T> = <T as SolStruct>::Tuple;
type TupleTokenTypeFor<T> = <TupleFor<T> as SolType>::TokenType;

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
pub trait SolStruct {
    /// The corresponding Token type.
    type Token: TokenSeq;

    /// The corresponding Tuple type, used for encoding/decoding.
    type Tuple: SolType<TokenType = Self::Token>;

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

    /// Convert to the tuple type used for ABI encoding and decoding.
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self;

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

// blanket impl
// TODO: Maybe move this to `sol!`?
impl<T: SolStruct> SolType for T {
    type RustType = T;
    type TokenType = TupleTokenTypeFor<T>;

    #[inline]
    fn is_dynamic() -> bool {
        <<Self as SolStruct>::Tuple as SolType>::is_dynamic()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> crate::Result<()> {
        TupleFor::<T>::type_check(token)
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        Self::NAME.into()
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Result<Self::RustType> {
        let tuple = TupleFor::<T>::detokenize(token)?;
        Ok(T::from_rust(tuple))
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        let tuple = rust.borrow().to_rust();
        TupleFor::<T>::tokenize(tuple)
    }

    #[inline]
    fn eip712_encode_type() -> Option<Cow<'static, str>> {
        Some(Self::eip712_encode_type())
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        keccak256(SolStruct::eip712_hash_struct(rust.borrow()))
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B) {
        let tuple = rust.borrow().to_rust();
        TupleFor::<T>::encode_packed_to(target, tuple)
    }
}
