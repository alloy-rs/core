use crate::{no_std_prelude::*, token::TokenSeq, AbiResult, TokenType, Word};

/// A Solidity Type, for ABI enc/decoding
///
/// This trait is implemented by types that contain ABI enc/decoding info for
/// solidity types. Types may be combined to express arbitrarily complex
/// solidity types.
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
/// using the [`crate::sol`] proc macro to parse a solidity structdef into a
/// native Rust struct.
///
/// ```
/// # use ethers_abi_enc::sol;
/// sol! {
///     struct MyStruct {
///         bool a;
///         bytes2 b;
///     }
/// }
///
/// // This is the native rust representation of a Solidity type!
/// // How cool is that!
/// const MY_STRUCT: MyStruct = MyStruct { a: true, b: [0x01, 0x02] };
/// ```
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
        crate::Error::type_check_fail(data, Self::sol_type_name())
    }

    /// Detokenize
    fn detokenize(token: Self::TokenType) -> AbiResult<Self::RustType>;

    /// Tokenize
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType;

    /// The encoded struct type (as EIP-712), if any. None for non-structs
    fn eip712_encode_type() -> Option<Cow<'static, str>> {
        None
    }

    /// Encode this data according to EIP-712 `encodeData` rules, and hash it
    /// if necessary.
    ///
    /// Implementer's note: All single-word types are encoded as their word.
    /// All multi-word types are encoded as the hash the concatenated data
    /// words for each element
    ///
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word;

    /// Implemens Solidity's `encodePacked()` function, writing into the given buffer.
    fn encode_packed_to<B: Borrow<Self::RustType>>(target: &mut Vec<u8>, rust: B);

    /// Implements Solidity's `encodePacked()` function.
    fn encode_packed<B: Borrow<Self::RustType>>(rust: B) -> Vec<u8> {
        let mut res = Vec::new();
        Self::encode_packed_to(&mut res, rust);
        res
    }

    /* BOILERPLATE BELOW */

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
