use crate::{
    eip712::coerce::{self, coerce_custom_struct, coerce_custom_value},
    no_std_prelude::*,
    DynAbiError, DynSolValue, DynToken, Result, SolType, Word,
};
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
    use crate::*;
    use alloy_primitives::Address;
    use serde_json::json;

    use hex_literal::hex;

    macro_rules! encoder_test {
        ($ty:literal, $encoded:ident) => {
            let t: DynSolType = $ty.parse().expect("parsing failed");
            let dec = t.decode_params(&$encoded).expect("decoding failed");

            // Tuples are treated as top-level lists. So if we encounter a
            // dynamic tuple, the total length of the encoded data will include
            // the offset, but the encoding/decoding process will not. To
            // account for this, we add 32 bytes to the expected length when
            // the type is a dynamic tuple.
            if dec.as_tuple().is_some() && dec.is_dynamic() {
                assert_eq!(dec.total_words() * 32, $encoded.len() + 32);
            } else {
                assert_eq!(dec.total_words() * 32, $encoded.len());
            }

            let re_encoded = dec.encode_params();
            assert_eq!(re_encoded, $encoded);
        };
    }

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

    #[test]
    fn address() {
        let enc = hex! {"0000000000000000000000001111111111111111111111111111111111111111"};
        encoder_test!("address", enc);
    }

    #[test]
    fn dynamic_array_of_addresses() {
        let encoded = hex!(
            "
			0000000000000000000000000000000000000000000000000000000000000020
			0000000000000000000000000000000000000000000000000000000000000002
			0000000000000000000000001111111111111111111111111111111111111111
			0000000000000000000000002222222222222222222222222222222222222222
		"
        );
        encoder_test!("address[]", encoded);
    }

    #[test]
    fn fixed_array_of_addresses() {
        let encoded = hex!(
            "
                0000000000000000000000001111111111111111111111111111111111111111
                0000000000000000000000002222222222222222222222222222222222222222
            "
        );
        encoder_test!("address[2]", encoded);
    }

    #[test]
    fn two_addresses() {
        let encoded = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        );
        encoder_test!("(address,address)", encoded);
    }

    #[test]
    fn fixed_array_of_dyanmic_arrays_of_addresses() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        );
        encoder_test!("address[][2]", encoded);
    }

    #[test]
    fn dynamic_array_of_fixed_arrays_of_addresses() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        );
        encoder_test!("address[2][]", encoded);
    }

    #[test]
    fn dynamic_array_of_dynamic_arrays() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        );
        encoder_test!("address[][]", encoded);
    }

    #[test]
    fn dynamic_array_of_dynamic_arrays2() {
        let encoded = hex!(
            "
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
    	"
        );
        encoder_test!("address[][]", encoded);
    }

    #[test]
    fn fixed_array_of_fixed_arrays() {
        let encoded = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        );
        encoder_test!("address[2][2]", encoded);
    }

    #[test]
    fn fixed_array_of_static_tuples_followed_by_dynamic_type() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000005930cc5
    		0000000000000000000000000000000000000000000000000000000015002967
    		0000000000000000000000004444444444444444444444444444444444444444
    		000000000000000000000000000000000000000000000000000000000000307b
    		00000000000000000000000000000000000000000000000000000000000001c3
    		0000000000000000000000002222222222222222222222222222222222222222
    		00000000000000000000000000000000000000000000000000000000000000e0
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        );

        encoder_test!("((uint256,uint256,address)[2],string)", encoded);
    }

    #[test]
    fn empty_array() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000000
    	    "
        );
        encoder_test!("address[]", encoded);
    }

    #[test]
    fn empty_array_2() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000060
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000000
    	    "
        );
        encoder_test!("(address[],address[])", encoded);
    }

    #[test]
    fn empty_array_3() {
        // Nested empty arrays
        let encoded = hex!(
            "
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
            "
        );
        encoder_test!("(address[][], address[][])", encoded);
    }

    #[test]
    fn fixed_bytes() {
        let encoded = hex!("1234000000000000000000000000000000000000000000000000000000000000");
        encoder_test!("bytes2", encoded);
    }

    #[test]
    fn string() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("string", encoded);
    }

    #[test]
    fn bytes() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000002
    		1234000000000000000000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("bytes", encoded);
    }

    #[test]
    fn bytes_2() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		000000000000000000000000000000000000000000000000000000000000001f
    		1000000000000000000000000000000000000000000000000000000000000200
    	"
        );
        encoder_test!("bytes", encoded);
    }

    #[test]
    fn bytes_3() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		1000000000000000000000000000000000000000000000000000000000000000
    		1000000000000000000000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("bytes", encoded);
    }

    #[test]
    fn two_bytes() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		000000000000000000000000000000000000000000000000000000000000001f
    		1000000000000000000000000000000000000000000000000000000000000200
    		0000000000000000000000000000000000000000000000000000000000000020
    		0010000000000000000000000000000000000000000000000000000000000002
    	"
        );
        encoder_test!("(bytes,bytes)", encoded);
    }

    #[test]
    fn uint() {
        let encoded = hex!("0000000000000000000000000000000000000000000000000000000000000004");
        encoder_test!("uint", encoded);
    }

    #[test]
    fn int() {
        let encoded = hex!("0000000000000000000000000000000000000000000000000000000000000004");
        encoder_test!("int", encoded);
    }

    #[test]
    fn bool() {
        let encoded = hex!("0000000000000000000000000000000000000000000000000000000000000001");
        encoder_test!("bool", encoded);
    }

    #[test]
    fn bool2() {
        let encoded = hex!("0000000000000000000000000000000000000000000000000000000000000000");
        encoder_test!("bool", encoded);
    }

    #[test]
    fn comprehensive_test() {
        let encoded = hex!(
            "
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
    	"
        );
        encoder_test!("(uint8,bytes,uint8,bytes)", encoded);
    }

    #[test]
    fn comprehensive_test2() {
        let encoded = hex!(
            "
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
    	"
        );
        encoder_test!("(bool,string,uint8,uint8,uint8,uint8[])", encoded);
    }

    #[test]
    fn dynamic_array_of_bytes() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000026
    		019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a421
    		8a274f00d6070000000000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("bytes[]", encoded);
    }

    #[test]
    fn dynamic_array_of_bytes2() {
        let encoded = hex!(
            "
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
    	"
        );
        encoder_test!("bytes[]", encoded);
    }

    #[test]
    fn static_tuple_of_addresses() {
        let encoded = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    	"
        );
        encoder_test!("(address,address)", encoded);
    }

    #[test]
    fn dynamic_tuple() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		0000000000000000000000000000000000000000000000000000000000000080
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("((string,string),)", encoded);
    }

    #[test]
    fn dynamic_tuple_of_bytes() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000040
    		00000000000000000000000000000000000000000000000000000000000000a0
    		0000000000000000000000000000000000000000000000000000000000000026
    		4444444444444444444444444444444444444444444444444444444444444444
    		4444444444440000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000026
    		6666666666666666666666666666666666666666666666666666666666666666
    		6666666666660000000000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("((bytes,bytes),)", encoded);
    }

    #[test]
    fn complex_tuple() {
        let encoded = hex!(
            "
            0000000000000000000000000000000000000000000000000000000000000020
            1111111111111111111111111111111111111111111111111111111111111111
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000009
    		6761766f66796f726b0000000000000000000000000000000000000000000000
    	"
        );
        encoder_test!("((uint256,string,address,address),)", encoded);
    }

    #[test]
    fn nested_tuple() {
        let encoded = hex!(
            "
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
    	"
        );
        encoder_test!(
            "((string,bool,string,(string,string,(string,string))),)",
            encoded
        );
    }

    #[test]
    fn params_containing_dynamic_tuple() {
        let encoded = hex!(
            "
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
    	"
        );
        encoder_test!(
            "(address,(bool,string,string),address,address,bool)",
            encoded
        );
    }

    #[test]
    fn params_containing_static_tuple() {
        let encoded = hex!(
            "
    		0000000000000000000000001111111111111111111111111111111111111111
    		0000000000000000000000002222222222222222222222222222222222222222
    		0000000000000000000000000000000000000000000000000000000000000001
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000003333333333333333333333333333333333333333
    		0000000000000000000000004444444444444444444444444444444444444444
    	"
        );
        encoder_test!("(address,(address,bool,bool),address,address)", encoded);
    }

    #[test]
    fn dynamic_tuple_with_nested_static_tuples() {
        let encoded = hex!(
            "
    		0000000000000000000000000000000000000000000000000000000000000020
    		0000000000000000000000000000000000000000000000000000000000000000
    		0000000000000000000000000000000000000000000000000000000000000777
    		0000000000000000000000000000000000000000000000000000000000000060
    		0000000000000000000000000000000000000000000000000000000000000002
    		0000000000000000000000000000000000000000000000000000000000000042
    		0000000000000000000000000000000000000000000000000000000000001337
    	"
        );
        encoder_test!("((((bool,uint16),), uint16[]),)", encoded);
    }
}
