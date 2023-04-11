use ethers_primitives::B256;

use crate::{util::keccak256, SolType};

use crate::no_std_prelude::*;

type TupleFor<T> = <T as SolStruct>::Tuple;
type TupleTokenTypeFor<T> = <TupleFor<T> as SolType>::TokenType;

/// A Solidity Struct.
///
/// This trait is used to implement EIP-712 encoding/decoding. We generally
/// recommend implementing this via the [`crate::sol`] proc macro. Or by
/// deriving.
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
    fn to_tuple(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding/decoding
    fn from_tuple(tuple: <Self::Tuple as SolType>::RustType) -> Self
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
}

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

    fn struct_type() -> Option<String> {
        Some(Self::encode_type())
    }

    fn type_check(token: &Self::TokenType) -> crate::AbiResult<()> {
        TupleFor::<T>::type_check(token)
    }

    fn sol_type_name() -> String {
        Self::NAME.to_owned()
    }

    fn tokenize<Borrower>(rust: Borrower) -> Self::TokenType
    where
        Borrower: std::borrow::Borrow<Self::RustType>,
    {
        let tuple = rust.borrow().to_tuple();
        TupleFor::<T>::tokenize(tuple)
    }

    fn detokenize(token: Self::TokenType) -> crate::AbiResult<Self::RustType> {
        let tuple = TupleFor::<T>::detokenize(token)?;
        Ok(T::from_tuple(tuple))
    }

    fn encode_packed_to<Borrower>(target: &mut Vec<u8>, rust: Borrower)
    where
        Borrower: std::borrow::Borrow<Self::RustType>,
    {
        let tuple = rust.borrow().to_tuple();
        TupleFor::<T>::encode_packed_to(target, tuple)
    }
}
