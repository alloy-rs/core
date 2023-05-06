use crate::{no_std_prelude::*, token::TokenSeq, AbiResult, TokenType};

/// A Solidity Type, for ABI enc/decoding
///
/// This trait is implemented by types that contain ABI enc/decoding info for
/// solidity types. Types may be combined to express arbitrarily complex
/// solidity types.
///
/// Future work will add derive for this trait :)
///
/// ```
/// use ethers_abi_enc::{SolType, sol_data::*};
///
/// // uint256[]
/// type DynUint256Array = Array<Uint<256>>;
/// assert_eq!(&DynUint256Array::sol_type_name(), "uint256[]");
///
/// type Erc20FunctionArgs = (Address, Uint<256>);
/// assert_eq!(&Erc20FunctionArgs::sol_type_name(), "tuple(address,uint256)");
///
/// type LargeComplexType = (FixedArray<Array<Bool>, 2>, (FixedBytes<13>, String));
/// assert_eq!(&LargeComplexType::sol_type_name(), "tuple(bool[][2],tuple(bytes13,string))");
/// ```
///
/// These types are zero cost representations of Solidity types. They do not
/// exist at runtime. They ONLY information about the type, they do not carry
/// data
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`crate::sol`] proc macro to parse a solidity structdef.
///
/// You may want to implement this on your own struct, for example, to encode a
/// named solidity struct. Overall, implementing this trait is straightforward,
/// because we can delegate to an underlying combination of primitive types.
///
/// ```
/// # use ethers_abi_enc::{AbiResult, SolType};
/// # use ethers_abi_enc::sol_data::*;
/// # use ethers_primitives::U256;
///
/// // This is the solidity type:
/// //
/// // struct MySolidityStruct {
/// //    uint256 a;
/// //    uint256 b;
/// // }
///
/// // This should be a ZST. See note.
/// pub struct MySolidityStruct;
///
/// // This will be the data type in rust.
/// pub struct MyRustStruct {
///    a: U256,
///    b: U256,
/// }
///
/// // We're going to get really cute here.
/// //
/// // Structs are encoded as Tuples. So we can entirely define this trait by
/// // delegating to a tuple type!
/// type UnderlyingTuple = (Uint<256>, Uint<256>);
///
/// impl SolType for MySolidityStruct {
///     type RustType = MyRustStruct;
///     type TokenType = <UnderlyingTuple as SolType>::TokenType;
///
///     // The name in solidity
///     fn sol_type_name() -> std::borrow::Cow<'static, str> {
///         "MySolidityStruct".into()
///     }
///
///     // True if your type has a dynamic encoding length. This is dynamic
///     // arrays, strings, bytes, dynamic tuple etc.
///     //
///     // Of course, we can cheat here by delegating to the tuple
///     fn is_dynamic() -> bool {
///         UnderlyingTuple::is_dynamic()
///     }
///
///     // This function should check the data in the token and enforce any
///     // type rules. For example, a bool should ONLY be 0 or 1. This function
///     // should check the data, and return false if the bool is 2 or 3 or
///     // whatever.
///     //
///     // It will be ignored if the decoder runs without validation
///     fn type_check(token: &Self::TokenType) -> AbiResult<()> {
///         UnderlyingTuple::type_check(token)
///     }
///
///     // Convert from the token to the rust type. We cheat here again by
///     // delegating.
///     fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType> {
///         let (a, b) = UnderlyingTuple::detokenize(token)?;
///         Ok(MyRustStruct{ a, b })
///     }
///
///     // Convert from the rust type to the token type. We cheat here AGAIN
///     // by delegating.
///     fn tokenize<B>(rust: B) -> Self::TokenType
///     where
///        B: std::borrow::Borrow<Self::RustType>,
///     {
///         let MyRustStruct { a, b } = *rust.borrow();
///         UnderlyingTuple::tokenize((a, b))
///     }
/// }
/// ```
///
/// As you can see, because any NEW SolPrimitive corresponds to some
/// combination of OLD sol types, it's really easy to implement
/// [`SolPrimitive`] for anything you want!
pub trait SolType {
    /// The corresponding Rust type. This type may be borrowed (e.g. `str`)
    type RustType;

    /// The corresponding ABI token type.
    ///
    /// See implementers of [`TokenType`].
    type TokenType: TokenType;

    /// The name of the type in solidity
    fn sol_type_name() -> Cow<'static, str>;

    /// True if the type is dynamic according to ABI rules
    fn is_dynamic() -> bool;

    /// True if the type is a user defined type. These include structs, enums,
    /// and user defined value types
    fn is_user_defined() -> bool {
        false
    }

    /// Check a token to see if it can be detokenized with this type
    fn type_check(token: &Self::TokenType) -> AbiResult<()>;

    #[doc(hidden)]
    fn type_check_fail(data: &[u8]) -> crate::Error {
        crate::Error::type_check_fail(hex::encode(data), Self::sol_type_name())
    }

    /// Detokenize
    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType>;

    /// Tokenize
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType;

    /// Encode a single ABI token by wrapping it in a 1-length sequence
    fn encode_single<B: Borrow<Self::RustType>>(rust: B) -> Vec<u8> {
        let token = Self::tokenize(rust);
        crate::encode_single(token)
    }

    /// Encode an ABI sequence
    fn encode<B>(rust: B) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        let token = Self::tokenize(rust);
        crate::encode(token)
    }

    /// Encode an ABI sequence suitable for function params
    fn encode_params<B>(rust: B) -> Vec<u8>
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        let token = Self::tokenize(rust);
        crate::encode_params(token)
    }

    /// Hex output of encode
    fn hex_encode<B>(rust: B) -> String
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        format!("0x{}", hex::encode(Self::encode(rust)))
    }

    /// Hex output of encode_single
    fn hex_encode_single<B: Borrow<Self::RustType>>(rust: B) -> String {
        format!("0x{}", hex::encode(Self::encode_single(rust)))
    }

    /// Hex output of encode_params
    fn hex_encode_params<B>(rust: B) -> String
    where
        Self::TokenType: TokenSeq,
        B: Borrow<Self::RustType>,
    {
        format!("0x{}", hex::encode(Self::encode_params(rust)))
    }

    /// Decode a Rust type from an ABI blob
    fn decode(data: &[u8], validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let decoded = crate::decode::<Self::TokenType>(data, validate)?;
        if validate {
            Self::type_check(&decoded)?;
        }
        Self::detokenize(decoded)
    }

    /// Decode a Rust type from an ABI blob
    fn decode_params(data: &[u8], validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let decoded = crate::decode_params::<Self::TokenType>(data, validate)?;
        if validate {
            Self::type_check(&decoded)?;
        }
        Self::detokenize(decoded)
    }

    /// Decode a Rust type from an ABI blob
    fn decode_single(data: &[u8], validate: bool) -> AbiResult<Self::RustType> {
        let decoded = crate::decode_single::<Self::TokenType>(data, validate)?;
        if validate {
            Self::type_check(&decoded)?;
        }
        Self::detokenize(decoded)
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode(data: &str, validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(Into::into)
            .and_then(|buf| Self::decode(&buf, validate))
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode_single(data: &str, validate: bool) -> AbiResult<Self::RustType> {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(Into::into)
            .and_then(|buf| Self::decode_single(&buf, validate))
    }

    /// Decode a Rust type from a hex-encoded ABI blob
    fn hex_decode_params(data: &str, validate: bool) -> AbiResult<Self::RustType>
    where
        Self::TokenType: TokenSeq,
    {
        let payload = data.strip_prefix("0x").unwrap_or(data);
        hex::decode(payload)
            .map_err(Into::into)
            .and_then(|buf| Self::decode_params(&buf, validate))
    }
}
