use crate::{
    eip712::coerce::{self, coerce_custom_struct, coerce_custom_value},
    parser::TypeSpecifier,
    DynAbiError, DynAbiResult, DynSolValue, DynToken, Result, SolType, Word,
};
use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use alloy_sol_types::sol_data;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructProp {
    name: String,
    ty: DynSolType,
}

/// A Dynamic SolType. Equivalent to an enum wrapper around all implementers of
/// [`crate::SolType`]. This is used to represent Solidity types that are not
/// known at compile time. It is used in conjunction with [`DynToken`] and
/// [`DynSolValue`] to allow for dynamic ABI encoding and decoding.
///
/// Users will generally want to instantiate via the [`std::str::FromStr`] impl
/// on [`DynSolType`]. This will parse a string into a [`DynSolType`].
/// User-defined types can be instantiated directly.
///
/// # Example
/// ```
/// # use alloy_dyn_abi::{DynSolType, DynSolValue, Result};
/// # use alloy_primitives::U256;
/// # pub fn main() -> Result<()> {
/// let my_type = DynSolType::Uint(256);
/// let my_data: DynSolValue = U256::from(183).into();
///
/// let encoded = my_data.clone().encode_single();
/// let decoded = my_type.decode_single(&encoded)?;
///
/// assert_eq!(decoded, my_data);
///
/// let my_type = DynSolType::Array(Box::new(DynSolType::Uint(256)));
/// let my_data = DynSolValue::Array(vec![my_data.clone()]);
///
/// let encoded = my_data.clone().encode_single();
/// let decoded = my_type.decode_single(&encoded)?;
///
/// assert_eq!(decoded, my_data);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynSolType {
    /// Address.
    Address,
    /// Dynamic bytes.
    Bytes,
    /// Signed Integer.
    Int(usize),
    /// Unsigned Integer.
    Uint(usize),
    /// Boolean.
    Bool,
    /// Dynamically sized array.
    Array(Box<DynSolType>),
    /// String.
    String,
    /// Fixed-size bytes, up to 32.
    FixedBytes(usize),
    /// Fixed-sized array.
    FixedArray(Box<DynSolType>, usize),
    /// Tuple.
    Tuple(Vec<DynSolType>),
    /// User-defined struct.
    CustomStruct {
        /// Name of the struct.
        name: String,
        /// Prop names.
        prop_names: Vec<String>,
        /// Inner types.
        tuple: Vec<DynSolType>,
    },
    /// User-defined value.
    CustomValue {
        /// Name of the value type.
        name: String,
    },
}

impl core::str::FromStr for DynSolType {
    type Err = DynAbiError;

    #[inline]
    fn from_str(s: &str) -> DynAbiResult<Self, Self::Err> {
        TypeSpecifier::try_from(s).and_then(|t| t.resolve_basic_solidity())
    }
}

impl DynSolType {
    /// Check that a given [`DynSolValue`] matches this type.
    pub fn matches(&self, value: &DynSolValue) -> bool {
        match self {
            Self::Address => matches!(value, DynSolValue::Address(_)),
            Self::Bytes => matches!(value, DynSolValue::Bytes(_)),
            Self::Int(size) => matches!(value, DynSolValue::Int(_, s) if s == size),
            Self::Uint(size) => matches!(value, DynSolValue::Uint(_, s) if s == size),
            Self::Bool => matches!(value, DynSolValue::Bool(_)),
            Self::Array(t) => {
                matches!(value, DynSolValue::Array(v) if v.iter().all(|v| t.matches(v)))
            }
            Self::String => matches!(value, DynSolValue::String(_)),
            Self::FixedBytes(size) => matches!(value, DynSolValue::FixedBytes(_, s) if s == size),
            Self::FixedArray(t, size) => matches!(
                value,
                DynSolValue::FixedArray(v) if v.len() == *size && v.iter().all(|v| t.matches(v))
            ),
            Self::Tuple(types) => matches!(
                value,
                DynSolValue::CustomStruct { tuple, .. } | DynSolValue::Tuple(tuple)
                    if types.iter().zip(tuple).all(|(t, v)| t.matches(v))
            ),
            Self::CustomStruct {
                name,
                prop_names,
                tuple,
            } => {
                if let DynSolValue::CustomStruct {
                    name: n,
                    prop_names: p,
                    tuple: t,
                } = value
                {
                    name == n && prop_names == p && tuple.iter().zip(t).all(|(a, b)| a.matches(b))
                } else if let DynSolValue::Tuple(v) = value {
                    v.iter().zip(tuple).all(|(v, t)| t.matches(v))
                } else {
                    false
                }
            }
            Self::CustomValue { name } => {
                matches!(value, DynSolValue::CustomValue { name: n, .. } if name == n)
            }
        }
    }

    /// Dynamic detokenization.
    #[allow(clippy::unnecessary_to_owned)] // https://github.com/rust-lang/rust-clippy/issues/8148
    pub fn detokenize(&self, token: DynToken<'_>) -> Result<DynSolValue> {
        match (self, token) {
            (DynSolType::Address, DynToken::Word(word)) => Ok(DynSolValue::Address(
                sol_data::Address::detokenize(word.into()),
            )),
            (DynSolType::Bool, DynToken::Word(word)) => {
                Ok(DynSolValue::Bool(sol_data::Bool::detokenize(word.into())))
            }
            (DynSolType::Bytes, DynToken::PackedSeq(buf)) => Ok(DynSolValue::Bytes(buf.to_vec())),
            (DynSolType::FixedBytes(size), DynToken::Word(word)) => Ok(DynSolValue::FixedBytes(
                sol_data::FixedBytes::<32>::detokenize(word.into()).into(),
                *size,
            )),
            // cheating here, but it's ok
            (DynSolType::Int(size), DynToken::Word(word)) => Ok(DynSolValue::Int(
                sol_data::Int::<256>::detokenize(word.into()),
                *size,
            )),
            (DynSolType::Uint(size), DynToken::Word(word)) => Ok(DynSolValue::Uint(
                sol_data::Uint::<256>::detokenize(word.into()),
                *size,
            )),

            (DynSolType::String, DynToken::PackedSeq(buf)) => Ok(DynSolValue::String(
                sol_data::String::detokenize(buf.into()),
            )),
            (DynSolType::Tuple(types), DynToken::FixedSeq(tokens, _)) => {
                if types.len() != tokens.len() {
                    return Err(crate::Error::custom(
                        "tuple length mismatch on dynamic detokenization",
                    ))
                }
                types
                    .iter()
                    .zip(tokens.into_owned())
                    .map(|(t, w)| t.detokenize(w))
                    .collect::<Result<_>>()
                    .map(DynSolValue::Tuple)
            }
            (DynSolType::Array(t), DynToken::DynSeq { contents, .. }) => contents
                .into_owned()
                .into_iter()
                .map(|tok| t.detokenize(tok))
                .collect::<Result<_>>()
                .map(DynSolValue::Array),
            (DynSolType::FixedArray(t, size), DynToken::FixedSeq(tokens, _)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom(
                        "array length mismatch on dynamic detokenization",
                    ))
                }
                tokens
                    .into_owned()
                    .into_iter()
                    .map(|tok| t.detokenize(tok))
                    .collect::<Result<_>>()
                    .map(DynSolValue::FixedArray)
            }
            (
                DynSolType::CustomStruct {
                    name,
                    tuple,
                    prop_names,
                },
                DynToken::FixedSeq(tokens, len),
            ) => {
                if len != tokens.len() || len != tuple.len() {
                    return Err(crate::Error::custom(
                        "custom length mismatch on dynamic detokenization",
                    ))
                }
                let tuple = tuple
                    .iter()
                    .zip(tokens.into_owned())
                    .map(|(t, w)| t.detokenize(w))
                    .collect::<Result<_>>()?;

                Ok(DynSolValue::CustomStruct {
                    name: name.clone(),
                    prop_names: prop_names.clone(),
                    tuple,
                })
            }
            (DynSolType::CustomValue { .. }, token) => DynSolType::FixedBytes(32).detokenize(token),
            _ => Err(crate::Error::custom(
                "mismatched types on dynamic detokenization",
            )),
        }
    }

    #[inline]
    fn sol_type_name_simple(&self) -> Option<&str> {
        match self {
            Self::Address => Some("address"),
            Self::Bool => Some("bool"),
            Self::Bytes => Some("bytes"),
            Self::String => Some("string"),
            Self::CustomStruct { name, .. } | Self::CustomValue { name, .. } => Some(name),
            _ => None,
        }
    }

    #[inline]
    fn sol_type_name_raw(&self, out: &mut String) {
        match self {
            Self::Address
            | Self::Bool
            | Self::Bytes
            | Self::String
            | Self::CustomStruct { .. }
            | Self::CustomValue { .. } => {
                out.push_str(unsafe { self.sol_type_name_simple().unwrap_unchecked() });
            }

            Self::FixedBytes(size) | Self::Int(size) | Self::Uint(size) => {
                let prefix = match self {
                    Self::FixedBytes(..) => "bytes",
                    Self::Int(..) => "int",
                    Self::Uint(..) => "uint",
                    _ => unreachable!(),
                };
                out.push_str(prefix);
                out.push_str(itoa::Buffer::new().format(*size));
            }

            Self::Tuple(inner) => {
                out.push('(');
                for (i, val) in inner.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    val.sol_type_name_raw(out);
                }
                if inner.len() == 1 {
                    out.push(',');
                }
                out.push(')');
            }
            Self::Array(t) => {
                t.sol_type_name_raw(out);
                out.push_str("[]");
            }
            Self::FixedArray(t, len) => {
                t.sol_type_name_raw(out);
                out.push('[');
                out.push_str(itoa::Buffer::new().format(*len));
                out.push(']');
            }
        }
    }

    /// The Solidity type name. This returns the solidity type corresponding to
    /// this value, if it is known. A type will not be known if the value
    /// contains an empty sequence, e.g. `T[0]`.
    pub fn sol_type_name(&self) -> Cow<'_, str> {
        if let Some(s) = self.sol_type_name_simple() {
            Cow::Borrowed(s)
        } else {
            let mut s = String::with_capacity(64);
            self.sol_type_name_raw(&mut s);
            Cow::Owned(s)
        }
    }

    /// Coerce a json value to a sol value via this type.
    pub fn coerce(&self, value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
        match self {
            DynSolType::Address => coerce::address(value),
            DynSolType::Bytes => coerce::bytes(value),
            DynSolType::Int(n) => coerce::int(*n, value),
            DynSolType::Uint(n) => coerce::uint(*n, value),
            DynSolType::Bool => coerce::bool(value),
            DynSolType::Array(inner) => coerce::array(inner, value),
            DynSolType::String => coerce::string(value),
            DynSolType::FixedBytes(n) => coerce::fixed_bytes(*n, value),
            DynSolType::FixedArray(inner, n) => coerce::fixed_array(inner, *n, value),
            DynSolType::Tuple(inner) => coerce::tuple(inner, value),
            DynSolType::CustomStruct {
                name,
                prop_names,
                tuple,
            } => coerce_custom_struct(name, prop_names, tuple, value),
            DynSolType::CustomValue { name } => coerce_custom_value(name, value),
        }
    }

    /// Instantiate an empty dyn token, to be decoded into.
    pub(crate) fn empty_dyn_token(&self) -> DynToken<'_> {
        match self {
            DynSolType::Address => DynToken::Word(Word::ZERO),
            DynSolType::Bool => DynToken::Word(Word::ZERO),
            DynSolType::Bytes => DynToken::PackedSeq(&[]),
            DynSolType::FixedBytes(_) => DynToken::Word(Word::ZERO),
            DynSolType::Int(_) => DynToken::Word(Word::ZERO),
            DynSolType::Uint(_) => DynToken::Word(Word::ZERO),
            DynSolType::String => DynToken::PackedSeq(&[]),
            DynSolType::Tuple(types) => DynToken::FixedSeq(
                types.iter().map(|t| t.empty_dyn_token()).collect(),
                types.len(),
            ),
            DynSolType::Array(t) => DynToken::DynSeq {
                contents: Default::default(),
                template: Some(Box::new(t.empty_dyn_token())),
            },
            DynSolType::FixedArray(t, size) => {
                DynToken::FixedSeq(vec![t.empty_dyn_token(); *size].into(), *size)
            }
            DynSolType::CustomStruct { tuple, .. } => DynToken::FixedSeq(
                tuple.iter().map(|t| t.empty_dyn_token()).collect(),
                tuple.len(),
            ),
            DynSolType::CustomValue { .. } => DynToken::Word(Word::ZERO),
        }
    }

    /// Decode a [`DynSolValue`] from a byte slice. Fails if the value does not
    /// match this type.
    ///
    /// This method is used for decoding function arguments. It tries to
    /// determine whether the user intended to decode a sequence or an
    /// individual value. If the `self` type is a tuple, the `data` will be
    /// decoded as a sequence, otherwise it will be decoded as a single value.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// // This function takes a single simple param. The user should use
    /// // DynSolType::Uint(256).decode_params(data) to decode the param.
    /// function myFunc(uint256 a) public;
    ///
    /// // This function takes 2 params. The user should use
    /// // DynSolType::Tuple(
    /// //    vec![DynSolType::Uint(256), DynSolType::Bool])
    /// // .decode_params(data)
    /// function myFunc(uint256 b, bool c) public;
    /// ```
    #[inline]
    pub fn decode_params(&self, data: &[u8]) -> Result<DynSolValue> {
        match self {
            DynSolType::Tuple(_) => self.decode_sequence(data),
            _ => self.decode_single(data),
        }
    }

    /// Decode a [`DynSolValue`] from a byte slice. Fails if the value does not
    /// match this type.
    ///
    /// This method is used for decoding single values. It assumes the `data`
    /// argument is an encoded single-element sequence wrapping the `self` type.
    pub fn decode_single(&self, data: &[u8]) -> Result<DynSolValue> {
        let mut decoder = crate::Decoder::new(data, false);
        let mut token = self.empty_dyn_token();
        token.decode_single_populate(&mut decoder)?;
        self.detokenize(token)
    }

    /// Decode a [`DynSolValue`] from a byte slice. Fails if the value does not
    /// match this type.
    pub fn decode_sequence(&self, data: &[u8]) -> Result<DynSolValue> {
        let mut decoder = crate::Decoder::new(data, false);
        let mut token = self.empty_dyn_token();
        token.decode_sequence_populate(&mut decoder)?;
        self.detokenize(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{borrow::ToOwned, string::ToString};
    use alloy_primitives::{hex, Address};
    use serde_json::json;

    #[test]
    fn dynamically_encodes() {
        let word1 = "0000000000000000000000000101010101010101010101010101010101010101"
            .parse()
            .unwrap();
        let word2 = "0000000000000000000000000202020202020202020202020202020202020202"
            .parse()
            .unwrap();

        let val = DynSolValue::Address(Address::repeat_byte(0x01));
        let token = val.tokenize();
        assert_eq!(token, DynToken::from(word1));

        let val = DynSolValue::FixedArray(vec![
            Address::repeat_byte(0x01).into(),
            Address::repeat_byte(0x02).into(),
        ]);

        let token = val.tokenize();
        assert_eq!(
            token,
            DynToken::FixedSeq(vec![DynToken::Word(word1), DynToken::Word(word2)].into(), 2)
        );
        let mut enc = crate::Encoder::default();
        DynSolValue::encode_sequence(val.as_fixed_seq().unwrap(), &mut enc);
        assert_eq!(enc.finish(), vec![word1, word2]);
    }

    #[test]
    fn it_coerces() {
        let j = json!({
            "message": {
                "contents": "Hello, Bob!",
                "attachedMoneyInEth": 4.2,
                "from": {
                    "name": "Cow",
                    "wallets": [
                        "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                        "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF",
                    ]
                },
                "to": [{
                    "name": "Bob",
                    "wallets": [
                        "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                        "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57",
                        "0xB0B0b0b0b0b0B000000000000000000000000000",
                    ]
                }]
            }
        });

        let ty = DynSolType::CustomStruct {
            name: "Message".to_owned(),
            prop_names: vec!["contents".to_string(), "from".to_string(), "to".to_string()],
            tuple: vec![
                DynSolType::String,
                DynSolType::CustomStruct {
                    name: "Person".to_owned(),
                    prop_names: vec!["name".to_string(), "wallets".to_string()],
                    tuple: vec![
                        DynSolType::String,
                        DynSolType::Array(Box::new(DynSolType::Address)),
                    ],
                },
                DynSolType::Array(Box::new(DynSolType::CustomStruct {
                    name: "Person".to_owned(),
                    prop_names: vec!["name".to_string(), "wallets".to_string()],
                    tuple: vec![
                        DynSolType::String,
                        DynSolType::Array(Box::new(DynSolType::Address)),
                    ],
                })),
            ],
        };
        let top = j.as_object().unwrap().get("message").unwrap();

        assert_eq!(
            ty.coerce(top).unwrap(),
            DynSolValue::CustomStruct {
                name: "Message".to_owned(),
                prop_names: vec!["contents".to_string(), "from".to_string(), "to".to_string()],
                tuple: vec![
                    DynSolValue::String("Hello, Bob!".to_string()),
                    DynSolValue::CustomStruct {
                        name: "Person".to_owned(),
                        prop_names: vec!["name".to_string(), "wallets".to_string()],
                        tuple: vec![
                            DynSolValue::String("Cow".to_string()),
                            vec![
                                DynSolValue::Address(
                                    "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"
                                        .parse()
                                        .unwrap()
                                ),
                            ]
                            .into()
                        ]
                    },
                    vec![DynSolValue::CustomStruct {
                        name: "Person".to_owned(),
                        prop_names: vec!["name".to_string(), "wallets".to_string()],
                        tuple: vec![
                            DynSolValue::String("Bob".to_string()),
                            vec![
                                DynSolValue::Address(
                                    "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xB0B0b0b0b0b0B000000000000000000000000000"
                                        .parse()
                                        .unwrap()
                                ),
                            ]
                            .into()
                        ]
                    }]
                    .into()
                ]
            }
        )
    }

    // also tests the type name parser
    macro_rules! encoder_tests {
        ($($name:ident($ty:literal, $encoded:literal)),* $(,)?) => {$(
            #[test]
            fn $name() {
                encoder_test($ty, &hex!($encoded));
            }
        )*};
    }

    fn encoder_test(s: &str, encoded: &[u8]) {
        let t: DynSolType = s.parse().expect("parsing failed");
        assert_eq!(t.sol_type_name(), s, "type names are not the same");

        let dec = t.decode_params(encoded).expect("decoding failed");
        if let Some(value_name) = dec.sol_type_name() {
            assert_eq!(value_name, s, "value names are not the same");
        }

        // Tuples are treated as top-level lists. So if we encounter a
        // dynamic tuple, the total length of the encoded data will include
        // the offset, but the encoding/decoding process will not. To
        // account for this, we add 32 bytes to the expected length when
        // the type is a dynamic tuple.
        let mut len = encoded.len();
        if dec.as_tuple().is_some() && dec.is_dynamic() {
            len += 32;
        }
        assert_eq!(
            dec.total_words() * 32,
            len,
            "dyn_tuple={}",
            len != encoded.len()
        );

        let re_encoded = dec.encode_params();
        assert_eq!(re_encoded, encoded);
    }

    encoder_tests! {
        address("address", "0000000000000000000000001111111111111111111111111111111111111111"),

        dynamic_array_of_addresses("address[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        fixed_array_of_addresses("address[2]", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        two_addresses("(address,address)", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        fixed_array_of_dyanmic_arrays_of_addresses("address[][2]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        dynamic_array_of_fixed_arrays_of_addresses("address[2][]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        dynamic_array_of_dynamic_arrays("address[][]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        dynamic_array_of_dynamic_arrays2("address[][]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        fixed_array_of_fixed_arrays("address[2][2]", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        fixed_array_of_static_tuples_followed_by_dynamic_type("((uint256,uint256,address)[2],string)", "
                0000000000000000000000000000000000000000000000000000000005930cc5
                0000000000000000000000000000000000000000000000000000000015002967
                0000000000000000000000004444444444444444444444444444444444444444
                000000000000000000000000000000000000000000000000000000000000307b
                00000000000000000000000000000000000000000000000000000000000001c3
                0000000000000000000000002222222222222222222222222222222222222222
                00000000000000000000000000000000000000000000000000000000000000e0
                0000000000000000000000000000000000000000000000000000000000000009
                6761766f66796f726b0000000000000000000000000000000000000000000000
            "),

        empty_array("address[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
        "),

        empty_array_2("(address[],address[])", "
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000060
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
        "),

        // Nested empty arrays
        empty_array_3("(address[][],address[][])", "
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
        "),

        fixed_bytes("bytes2", "1234000000000000000000000000000000000000000000000000000000000000"),

        string("string", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
        "),

        bytes("bytes", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            1234000000000000000000000000000000000000000000000000000000000000
        "),

        bytes_2("bytes", "
            0000000000000000000000000000000000000000000000000000000000000020
            000000000000000000000000000000000000000000000000000000000000001f
            1000000000000000000000000000000000000000000000000000000000000200
        "),

        bytes_3("bytes", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            1000000000000000000000000000000000000000000000000000000000000000
            1000000000000000000000000000000000000000000000000000000000000000
        "),

        two_bytes("(bytes,bytes)", "
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            000000000000000000000000000000000000000000000000000000000000001f
            1000000000000000000000000000000000000000000000000000000000000200
            0000000000000000000000000000000000000000000000000000000000000020
            0010000000000000000000000000000000000000000000000000000000000002
        "),

        uint("uint256", "0000000000000000000000000000000000000000000000000000000000000004"),

        int("int256", "0000000000000000000000000000000000000000000000000000000000000004"),

        bool("bool", "0000000000000000000000000000000000000000000000000000000000000001"),

        bool2("bool", "0000000000000000000000000000000000000000000000000000000000000000"),

        comprehensive_test("(uint8,bytes,uint8,bytes)", "
            0000000000000000000000000000000000000000000000000000000000000005
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000003
            00000000000000000000000000000000000000000000000000000000000000e0
            0000000000000000000000000000000000000000000000000000000000000040
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
            0000000000000000000000000000000000000000000000000000000000000040
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
        "),

        comprehensive_test2("(bool,string,uint8,uint8,uint8,uint8[])", "
            0000000000000000000000000000000000000000000000000000000000000001
            00000000000000000000000000000000000000000000000000000000000000c0
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000003
            0000000000000000000000000000000000000000000000000000000000000004
            0000000000000000000000000000000000000000000000000000000000000100
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000003
            0000000000000000000000000000000000000000000000000000000000000005
            0000000000000000000000000000000000000000000000000000000000000006
            0000000000000000000000000000000000000000000000000000000000000007
        "),

        dynamic_array_of_bytes("bytes[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000026
            019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a421
            8a274f00d6070000000000000000000000000000000000000000000000000000
        "),

        dynamic_array_of_bytes2("bytes[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000026
            4444444444444444444444444444444444444444444444444444444444444444
            4444444444440000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000026
            6666666666666666666666666666666666666666666666666666666666666666
            6666666666660000000000000000000000000000000000000000000000000000
        "),

        static_tuple_of_addresses("(address,address)", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        dynamic_tuple("((string,string),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
        "),

        dynamic_tuple_of_bytes("((bytes,bytes),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000026
            4444444444444444444444444444444444444444444444444444444444444444
            4444444444440000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000026
            6666666666666666666666666666666666666666666666666666666666666666
            6666666666660000000000000000000000000000000000000000000000000000
        "),

        complex_tuple("((uint256,string,address,address),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            1111111111111111111111111111111111111111111111111111111111111111
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
        "),

        nested_tuple("((string,bool,string,(string,string,(string,string))),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000001
            00000000000000000000000000000000000000000000000000000000000000c0
            0000000000000000000000000000000000000000000000000000000000000100
            0000000000000000000000000000000000000000000000000000000000000004
            7465737400000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000006
            6379626f72670000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000060
            00000000000000000000000000000000000000000000000000000000000000a0
            00000000000000000000000000000000000000000000000000000000000000e0
            0000000000000000000000000000000000000000000000000000000000000005
            6e69676874000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000003
            6461790000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000004
            7765656500000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000008
            66756e7465737473000000000000000000000000000000000000000000000000
        "),

        params_containing_dynamic_tuple("(address,(bool,string,string),address,address,bool)", "
            0000000000000000000000002222222222222222222222222222222222222222
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000060
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000009
            7370616365736869700000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000006
            6379626f72670000000000000000000000000000000000000000000000000000
        "),

        params_containing_static_tuple("(address,(address,bool,bool),address,address)", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        dynamic_tuple_with_nested_static_tuples("((((bool,uint16),),uint16[]),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000777
            0000000000000000000000000000000000000000000000000000000000000060
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000042
            0000000000000000000000000000000000000000000000000000000000001337
        "),
    }
}
