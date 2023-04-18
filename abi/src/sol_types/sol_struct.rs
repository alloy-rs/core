//! This module contains the [`SolStruct`] trait, which is used to implement
//! Solidity structs logic, particularly for EIP-712 encoding/decoding.

use ethers_primitives::B256;

use crate::{util::keccak256, SolType, Word};

use crate::{no_std_prelude::*, Eip712Domain};

type TupleFor<T> = <T as SolStruct>::Tuple;
type TupleTokenTypeFor<T> = <TupleFor<T> as SolType>::TokenType;

/// A Solidity Struct.
///
/// This trait is used to implement EIP-712 encoding/decoding. We generally
/// recommend implementing this via the [`crate::sol`] proc macro. Or by
/// deriving.
///
/// # Note
///
/// Special attention should be payed to `encode_type` for complex solidity
/// types. Nested solidity structs MUST properly encode their type. To be clear,
/// a struct with a nested struct must encode the nested struct's type as well.
/// See: <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
pub trait SolStruct {
    /// The corresponding Tuple type, used for encoding/decoding
    type Tuple: SolType;

    /// The struct name
    const NAME: &'static str;

    /// The field types and names. Type is a solidity string, and must conform
    /// to the name of the sol type at the same index in the associated tuple
    const FIELDS: &'static [(&'static str, &'static str)];

    /// Convert to the tuple type used for ABI encoding/decoding
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding/decoding
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized;

    /// EIP-712 `encodeType`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    fn encode_type() -> String {
        let inner = Self::FIELDS
            .iter()
            .map(|(ty, name)| format!("{} {}", ty, name))
            .collect::<Vec<_>>()
            .join(",");

        format!("{}({})", Self::NAME, inner)
    }

    /// EIP-712 `typeHash`
    /// <https://eips.ethereum.org/EIPS/eip-712#rationale-for-typehash>
    fn type_hash(&self) -> B256 {
        keccak256(Self::encode_type().as_bytes())
    }

    /// EIP-712 `encodeData`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    fn encode_data(&self) -> Vec<u8>;

    /// EIP-712 `hashStruct`
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct>
    fn hash_struct(&self) -> B256 {
        let mut type_hash = self.type_hash().to_vec();
        type_hash.extend(self.encode_data());
        keccak256(type_hash)
    }

    /// EIP-712 `signTypedData`
    /// <https://eips.ethereum.org/EIPS/eip-712#specification-of-the-eth_signtypeddata-json-rpc>
    fn encode_eip712(&self, domain: &Eip712Domain) -> B256 {
        let domain_separator = domain.hash_struct();
        let struct_hash = self.hash_struct();

        let mut digest_input = [0u8; 2 + 32 + 32];
        digest_input[0] = 0x19;
        digest_input[1] = 0x01;
        digest_input[2..34].copy_from_slice(&domain_separator[..]);
        digest_input[34..66].copy_from_slice(&struct_hash[..]);
        keccak256(digest_input)
    }
}

// blanket impl
impl<T> SolType for T
where
    T: SolStruct,
{
    type RustType = T;
    type TokenType = TupleTokenTypeFor<T>;

    fn is_dynamic() -> bool {
        <<Self as SolStruct>::Tuple as SolType>::is_dynamic()
    }

    fn is_user_defined() -> bool {
        true
    }

    fn eip712_encode_type() -> Option<String> {
        Some(Self::encode_type())
    }

    fn type_check(token: &Self::TokenType) -> crate::AbiResult<()> {
        TupleFor::<T>::type_check(token)
    }

    fn sol_type_name() -> String {
        Self::NAME.to_owned()
    }

    fn eip712_data_word<B>(rust: B) -> Word
    where
        B: Borrow<Self::RustType>,
    {
        keccak256(SolStruct::encode_data(rust.borrow()))
    }

    fn tokenize<Borrower>(rust: Borrower) -> Self::TokenType
    where
        Borrower: Borrow<Self::RustType>,
    {
        let tuple = rust.borrow().to_rust();
        TupleFor::<T>::tokenize(tuple)
    }

    fn detokenize(token: Self::TokenType) -> crate::AbiResult<Self::RustType> {
        let tuple = TupleFor::<T>::detokenize(token)?;
        Ok(T::from_rust(tuple))
    }

    fn encode_packed_to<Borrower>(target: &mut Vec<u8>, rust: Borrower)
    where
        Borrower: Borrow<Self::RustType>,
    {
        let tuple = rust.borrow().to_rust();
        TupleFor::<T>::encode_packed_to(target, tuple)
    }
}
