use crate::{
    Result, Word,
    abi::{self, AbiDecoderConfig, Token, TokenSeq},
    private::SolTypeValue,
};
use alloc::vec::Vec;

/// A Solidity type.
///
/// This trait is implemented by types that contain ABI encoding and decoding
/// info for Solidity types. Types may be combined to express arbitrarily
/// complex Solidity types.
///
/// These types are zero cost representations of Solidity types. They do not
/// exist at runtime. They **only** contain information about the type, they do
/// not carry any data.
///
/// # Implementer's Guide
///
/// It should not be necessary to implement this trait manually. Instead, use
/// the [`sol!`] procedural macro to parse Solidity syntax into types that
/// implement this trait.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use alloy_sol_types::{SolType, sol_data::*};
///
/// type Uint256DynamicArray = Array<Uint<256>>;
/// assert_eq!(Uint256DynamicArray::SOL_NAME, "uint256[]");
///
/// type Erc20FunctionArgs = (Address, Uint<256>);
/// assert_eq!(Erc20FunctionArgs::SOL_NAME, "(address,uint256)");
///
/// type LargeComplexType = (FixedArray<Array<Bool>, 2>, (FixedBytes<13>, String));
/// assert_eq!(LargeComplexType::SOL_NAME, "(bool[][2],(bytes13,string))");
/// ```
///
/// The previous example can be entirely replicated with the [`sol!`] macro:
///
/// ```
/// use alloy_sol_types::{SolType, sol};
///
/// type Uint256DynamicArray = sol!(uint256[]);
/// assert_eq!(Uint256DynamicArray::SOL_NAME, "uint256[]");
///
/// type Erc20FunctionArgs = sol!((address, uint256));
/// assert_eq!(Erc20FunctionArgs::SOL_NAME, "(address,uint256)");
///
/// type LargeComplexType = sol!((bool[][2],(bytes13,string)));
/// assert_eq!(LargeComplexType::SOL_NAME, "(bool[][2],(bytes13,string))");
/// ```
///
/// For more complex usage, it's recommended to use the
/// [`SolValue`](crate::SolValue) trait for primitive types, and the `Sol*`
/// traits for other types created with [`sol!`]:
///
/// ```
/// use alloy_primitives::Address;
/// use alloy_sol_types::{SolCall, SolStruct, SolValue, sol};
///
/// sol! {
///     struct MyStruct {
///         bool a;
///         uint64 b;
///         address c;
///     }
///
///     enum MyEnum {
///         A,
///         B,
///         C,
///     }
///
///     function myFunction(MyStruct my_struct, MyEnum my_enum);
/// }
///
/// // `SolValue`
/// let my_bool = true;
/// let _ = my_bool.abi_encode();
///
/// let my_struct = MyStruct { a: true, b: 1, c: Address::ZERO };
/// let _ = my_struct.abi_encode();
///
/// let my_enum = MyEnum::A;
/// let _ = my_enum.abi_encode();
///
/// // `SolCall`
/// let my_function_call = myFunctionCall { my_struct, my_enum };
/// let _ = my_function_call.abi_encode();
/// ```
///
/// [`sol!`]: crate::sol
pub trait SolType: Sized {
    /// The corresponding Rust type.
    type RustType: SolTypeValue<Self> + 'static;

    /// The corresponding [ABI token type](Token).
    ///
    /// This is the intermediate representation of the type that is used for
    /// ABI encoding and decoding.
    type Token<'a>: Token<'a>;

    /// The name of this type in Solidity.
    const SOL_NAME: &'static str;

    /// The statically-known ABI-encoded size of the type.
    ///
    /// If this is not known at compile time, this should be `None`, which indicates that the
    /// encoded size is dynamic.
    const ENCODED_SIZE: Option<usize>;

    /// The statically-known Non-standard Packed Mode ABI-encoded size of the type.
    ///
    /// If this is not known at compile time, this should be `None`, which indicates that the
    /// encoded size is dynamic.
    const PACKED_ENCODED_SIZE: Option<usize>;

    /// Whether the ABI-encoded size is dynamic.
    ///
    /// There should be no need to override the default implementation.
    const DYNAMIC: bool = Self::ENCODED_SIZE.is_none();

    /// Calculate the ABI-encoded size of the data, counting both head and tail
    /// words. For a single-word type this will always be 32.
    #[inline]
    fn abi_encoded_size<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> usize {
        rust.stv_abi_encoded_size()
    }

    /// Returns `true` if the given token can be detokenized with this type.
    fn valid_token(token: &Self::Token<'_>) -> bool;

    /// Returns an error if the given token cannot be detokenized with this
    /// type.
    #[inline]
    fn type_check(token: &Self::Token<'_>) -> Result<()> {
        if Self::valid_token(token) {
            Ok(())
        } else {
            Err(crate::Error::type_check_fail_token::<Self>(token))
        }
    }

    /// Detokenize this type's value from the given token.
    ///
    /// See the [`abi::token`] module for more information.
    fn detokenize(token: Self::Token<'_>) -> Self::RustType;

    /// Tokenizes the given value into this type's token.
    ///
    /// See the [`abi::token`] module for more information.
    #[inline]
    fn tokenize<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> Self::Token<'_> {
        rust.stv_to_tokens()
    }

    /// Encode this data according to EIP-712 `encodeData` rules, and hash it
    /// if necessary.
    ///
    /// Implementer's note: All single-word types are encoded as their word.
    /// All multi-word types are encoded as the hash the concatenated data
    /// words for each element
    ///
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    #[inline]
    fn eip712_data_word<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> Word {
        rust.stv_eip712_data_word()
    }

    /// Returns the length of this value when ABI-encoded in Non-standard Packed Mode.
    ///
    /// See [`abi_encode_packed`][SolType::abi_encode_packed] for more details.
    #[inline]
    fn abi_packed_encoded_size<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> usize {
        rust.stv_abi_packed_encoded_size()
    }

    /// Non-standard Packed Mode ABI encoding.
    ///
    /// See [`abi_encode_packed`][SolType::abi_encode_packed] for more details.
    #[inline]
    fn abi_encode_packed_to<E: ?Sized + SolTypeValue<Self>>(rust: &E, out: &mut Vec<u8>) {
        rust.stv_abi_encode_packed_to(out)
    }

    /// Non-standard Packed Mode ABI encoding.
    ///
    /// This is different from normal ABI encoding:
    /// - types shorter than 32 bytes are concatenated directly, without padding or sign extension;
    /// - dynamic types are encoded in-place and without the length;
    /// - array elements are padded, but still encoded in-place.
    ///
    /// More information can be found in the [Solidity docs](https://docs.soliditylang.org/en/latest/abi-spec.html#non-standard-packed-mode).
    #[inline]
    fn abi_encode_packed<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> Vec<u8> {
        let mut out = Vec::with_capacity(Self::abi_packed_encoded_size(rust));
        Self::abi_encode_packed_to(rust, &mut out);
        out
    }

    /// Tokenizes and ABI-encodes the given value by wrapping it in a
    /// single-element sequence.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    fn abi_encode<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> Vec<u8> {
        abi::encode(&rust.stv_to_tokens())
    }

    /// Tokenizes and ABI-encodes the given value as function parameters.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    fn abi_encode_params<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> Vec<u8>
    where
        for<'a> Self::Token<'a>: TokenSeq<'a>,
    {
        abi::encode_params(&rust.stv_to_tokens())
    }

    /// Tokenizes and ABI-encodes the given value as a sequence.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    fn abi_encode_sequence<E: ?Sized + SolTypeValue<Self>>(rust: &E) -> Vec<u8>
    where
        for<'a> Self::Token<'a>: TokenSeq<'a>,
    {
        abi::encode_sequence(&rust.stv_to_tokens())
    }

    /// Decodes this type's value from an ABI blob by interpreting it as a
    /// single-element sequence.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    fn abi_decode(data: &[u8]) -> Result<Self::RustType> {
        abi::decode::<Self::Token<'_>>(data).map(Self::detokenize)
    }

    /// Decodes this type's value with a custom ABI decoder configuration.
    #[inline]
    fn abi_decode_with_config(data: &[u8], config: AbiDecoderConfig) -> Result<Self::RustType> {
        let token = abi::decode_with_config::<Self::Token<'_>>(data, config)?;
        if config.get_validate() {
            Self::type_check(&token)?;
        }
        Ok(Self::detokenize(token))
    }

    /// Decodes this type's value from an ABI blob by interpreting it as
    /// single-element sequence, rejecting words that are not valid for this type.
    ///
    /// This is the same as [`abi_decode`](Self::abi_decode), but it also calls
    /// [`type_check`](Self::type_check) on the decoded token and returns an
    /// error, usually [`Error::TypeCheckFail`](crate::Error::TypeCheckFail), if the check fails.
    /// It is equivalent to [`abi_decode_with_config`](Self::abi_decode_with_config) with
    /// `AbiDecoderConfig::new().validate(true)`. The other `*_validate` methods in this crate
    /// perform the same check.
    ///
    /// # What is checked
    ///
    /// Each decoded value must be valid for its type. This applies to array elements, tuple
    /// fields, and struct fields as well:
    ///
    /// | Type | Rejected |
    /// | --- | --- |
    /// | `bool` | any word other than `0` and `1`; `abi_decode` reads any non-zero word as `true` |
    /// | `address` | non-zero bytes above the low 20 |
    /// | `function` | non-zero bytes after the first 24 |
    /// | `uintN` | non-zero bytes above the low `N / 8` |
    /// | `intN` | upper bytes that are not the sign extension of the low `N / 8` bytes |
    /// | `bytesN` | non-zero bytes after the first `N` |
    /// | `string` | invalid UTF-8; `abi_decode` replaces invalid sequences with `U+FFFD` |
    /// | `enum` | non-zero bytes above the low byte, and values that are not a variant |
    /// | user-defined value type | whatever its underlying type rejects, plus any `#[sol(type_check = ...)]` function |
    ///
    /// `uint256` and `int256` accept every word, and `bytes` accepts any contents.
    ///
    /// # What is not checked
    ///
    /// Validation checks values, not layout. Offsets and lengths must still point inside the
    /// input, as in [`abi_decode`](Self::abi_decode). Beyond that, validation accepts:
    ///
    /// - bytes after the end of the encoding;
    /// - non-zero or missing padding after the contents of `bytes` and `string`;
    /// - gaps, overlaps, and reordering between dynamic data areas.
    ///
    /// [`AbiDecoderConfig::strict`] rejects all of these and implies validation. Setting
    /// [`AbiDecoderConfig::validate_allow_trailing_bytes`] as well accepts trailing bytes again
    /// and keeps every other strict check.
    ///
    /// ```
    /// use alloy_sol_types::{SolType, abi::AbiDecoderConfig, sol_data::Bool};
    ///
    /// // A `bool` word holding 2.
    /// let mut data = [0u8; 32];
    /// data[31] = 2;
    /// assert!(Bool::abi_decode(&data)?);
    /// assert!(Bool::abi_decode_validate(&data).is_err());
    ///
    /// // Trailing bytes pass validation but not strict decoding.
    /// let mut data = Bool::abi_encode(&true);
    /// data.push(0xff);
    /// assert!(Bool::abi_decode_validate(&data).is_ok());
    /// assert!(Bool::abi_decode_with_config(&data, AbiDecoderConfig::new().strict(true)).is_err());
    /// # Ok::<(), alloy_sol_types::Error>(())
    /// ```
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    // TODO: Deprecate in favor of a validating decoder configuration.
    // #[deprecated(note = "use a validating decoder configuration")]
    fn abi_decode_validate(data: &[u8]) -> Result<Self::RustType> {
        Self::abi_decode_with_config(data, AbiDecoderConfig::new().validate(true))
    }

    /// Decodes this type's value from an ABI blob by interpreting it as
    /// function parameters.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    fn abi_decode_params<'de>(data: &'de [u8]) -> Result<Self::RustType>
    where
        Self::Token<'de>: TokenSeq<'de>,
    {
        abi::decode_params::<Self::Token<'_>>(data).map(Self::detokenize)
    }

    /// Decodes this type's function parameters with a custom ABI decoder
    /// configuration.
    #[inline]
    fn abi_decode_params_with_config<'de>(
        data: &'de [u8],
        config: AbiDecoderConfig,
    ) -> Result<Self::RustType>
    where
        Self::Token<'de>: TokenSeq<'de>,
    {
        let token = abi::decode_params_with_config::<Self::Token<'_>>(data, config)?;
        if config.get_validate() {
            Self::type_check(&token)?;
        }
        Ok(Self::detokenize(token))
    }

    /// Decodes this type's value from an ABI blob by interpreting it as
    /// function parameters, with validation.
    ///
    /// This is the same as [`abi_decode_params`](Self::abi_decode_params), but also validates the
    /// decoded token. See [`abi_decode_validate`](Self::abi_decode_validate) for what is checked.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    // TODO: Deprecate in favor of a validating decoder configuration.
    // #[deprecated(note = "use a validating decoder configuration")]
    fn abi_decode_params_validate<'de>(data: &'de [u8]) -> Result<Self::RustType>
    where
        Self::Token<'de>: TokenSeq<'de>,
    {
        Self::abi_decode_params_with_config(data, AbiDecoderConfig::new().validate(true))
    }

    /// Decodes this type's value from an ABI blob by interpreting it as a
    /// sequence.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    fn abi_decode_sequence<'de>(data: &'de [u8]) -> Result<Self::RustType>
    where
        Self::Token<'de>: TokenSeq<'de>,
    {
        abi::decode_sequence::<Self::Token<'_>>(data).map(Self::detokenize)
    }

    /// Decodes this type's sequence with a custom ABI decoder configuration.
    #[inline]
    fn abi_decode_sequence_with_config<'de>(
        data: &'de [u8],
        config: AbiDecoderConfig,
    ) -> Result<Self::RustType>
    where
        Self::Token<'de>: TokenSeq<'de>,
    {
        let token = abi::decode_sequence_with_config::<Self::Token<'_>>(data, config)?;
        if config.get_validate() {
            Self::type_check(&token)?;
        }
        Ok(Self::detokenize(token))
    }

    /// Decodes this type's value from an ABI blob by interpreting it as
    /// sequence, with validation.
    ///
    /// This is the same as [`abi_decode_sequence`](Self::abi_decode_sequence), but also validates
    /// the decoded token. See [`abi_decode_validate`](Self::abi_decode_validate) for what is
    /// checked.
    ///
    /// See the [`abi`] module for more information.
    #[inline]
    // TODO: Deprecate in favor of a validating decoder configuration.
    // #[deprecated(note = "use a validating decoder configuration")]
    fn abi_decode_sequence_validate<'de>(data: &'de [u8]) -> Result<Self::RustType>
    where
        Self::Token<'de>: TokenSeq<'de>,
    {
        Self::abi_decode_sequence_with_config(data, AbiDecoderConfig::new().validate(true))
    }
}
