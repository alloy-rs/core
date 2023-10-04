use crate::{param::Param, utils::*, EventParam, StateMutability};
use alloc::{borrow::Cow, string::String, vec::Vec};
use alloy_primitives::{keccak256, Selector, B256};
use alloy_sol_type_parser::{Error as ParserError, Result as ParserResult};
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Declares all JSON ABI items.
macro_rules! abi_items {
    ($(
        $(#[$attr:meta])*
        $vis:vis struct $name:ident : $name_lower:literal {$(
            $(#[$fattr:meta])*
            $fvis:vis $field:ident : $type:ty,
        )*}
    )*) => {
        $(
            $(#[$attr])*
            #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
            #[serde(rename = $name_lower, rename_all = "camelCase", tag = "type")]
            $vis struct $name {$(
                $(#[$fattr])*
                $fvis $field: $type,
            )*}

            impl From<$name> for AbiItem<'_> {
                #[inline]
                fn from(item: $name) -> Self {
                    AbiItem::$name(Cow::Owned(item))
                }
            }

            impl<'a> From<&'a $name> for AbiItem<'a> {
                #[inline]
                fn from(item: &'a $name) -> Self {
                    AbiItem::$name(Cow::Borrowed(item))
                }
            }
        )*

        // Note: `AbiItem` **must not** derive `Serialize`, since we use `tag`
        // only for deserialization, while we treat it as `untagged` for serialization.
        // This is because the individual item structs are already tagged, and
        // deriving `Serialize` would emit the tag field twice.

        /// A JSON ABI item.
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
        #[serde(tag = "type", rename_all = "camelCase")]
        pub enum AbiItem<'a> {$(
            #[doc = concat!("A JSON ABI [`", stringify!($name), "`].")]
            $name(Cow<'a, $name>),
        )*}

        impl Serialize for AbiItem<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self {$(
                    Self::$name(item) => item.serialize(serializer),
                )*}
            }
        }
    };
}

abi_items! {
    /// A JSON ABI constructor function.
    pub struct Constructor: "constructor" {
        /// The input types of the constructor. May be empty.
        pub inputs: Vec<Param>,
        /// The state mutability of the constructor.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI fallback function.
    #[derive(Copy)]
    pub struct Fallback: "fallback" {
        /// The state mutability of the fallback function.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI receive function.
    #[derive(Copy)]
    pub struct Receive: "receive" {
        /// The state mutability of the receive function.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI function.
    pub struct Function: "function" {
        /// The name of the function.
        #[serde(deserialize_with = "validate_identifier")]
        pub name: String,
        /// The input types of the function. May be empty.
        pub inputs: Vec<Param>,
        /// The output types of the function. May be empty.
        pub outputs: Vec<Param>,
        /// The state mutability of the function.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI event.
    pub struct Event: "event" {
        /// The name of the event.
        #[serde(deserialize_with = "validate_identifier")]
        pub name: String,
        /// A list of the event's inputs, in order.
        pub inputs: Vec<EventParam>,
        /// Whether the event is anonymous. Anonymous events do not have their
        /// signature included in the topic 0. Instead, the indexed arguments
        /// are 0-indexed.
        pub anonymous: bool,
    }

    /// A JSON ABI error.
    pub struct Error: "error" {
        /// The name of the error.
        #[serde(deserialize_with = "validate_identifier")]
        pub name: String,
        /// A list of the error's components, in order.
        pub inputs: Vec<Param>,
    }
}

#[inline(always)]
fn validate_identifier<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let s = String::deserialize(deserializer)?;
    validate_identifier!(&s);
    Ok(s)
}

impl FromStr for AbiItem<'_> {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl AbiItem<'_> {
    /// Parses a single [Human-Readable ABI] string into an ABI item.
    ///
    /// [Human-Readable ABI]: https://docs.ethers.org/v5/api/utils/abi/formats/#abi-formats--human-readable-abi
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_json_abi::{AbiItem, Function, Param};
    /// assert_eq!(
    ///     AbiItem::parse("function foo(bool bar)"),
    ///     Ok(AbiItem::from(Function::parse("foo(bool bar)").unwrap()).into()),
    /// );
    /// ```
    pub fn parse(mut input: &str) -> ParserResult<Self> {
        // need this for Constructor, since the keyword is also the name of the function
        let copy = input;
        match alloy_sol_type_parser::__internal_parse_item(&mut input)? {
            "constructor" => Constructor::parse(copy).map(Into::into),
            "function" => Function::parse(input).map(Into::into),
            "error" => Error::parse(input).map(Into::into),
            "event" => Event::parse(input).map(Into::into),
            keyword => Err(ParserError::invalid_type_string(format_args!(
                "invalid AbiItem keyword: {keyword:?}, \
                 expected one of \"constructor\", \"function\", \"error\", or \"event\""
            ))),
        }
    }

    /// Returns the debug name of the item.
    #[inline]
    pub const fn debug_name(&self) -> &'static str {
        match self {
            AbiItem::Constructor(_) => "Constructor",
            AbiItem::Fallback(_) => "Fallback",
            AbiItem::Receive(_) => "Receive",
            AbiItem::Function(_) => "Function",
            AbiItem::Event(_) => "Event",
            AbiItem::Error(_) => "Error",
        }
    }

    /// Returns an immutable reference to the name of the item.
    #[inline]
    pub fn name(&self) -> Option<&String> {
        match self {
            Self::Event(item) => Some(&item.name),
            Self::Error(item) => Some(&item.name),
            Self::Function(item) => Some(&item.name),
            Self::Constructor(_) | Self::Fallback(_) | Self::Receive(_) => None,
        }
    }

    /// Returns a mutable reference to the name of the item.
    ///
    /// Clones the item if it is not already owned.
    #[inline]
    pub fn name_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Event(item) => Some(&mut item.to_mut().name),
            Self::Error(item) => Some(&mut item.to_mut().name),
            Self::Function(item) => Some(&mut item.to_mut().name),
            Self::Constructor(_) | Self::Fallback(_) | Self::Receive(_) => None,
        }
    }

    /// Returns the state mutability of the item.
    #[inline]
    pub fn state_mutability(&self) -> Option<StateMutability> {
        match self {
            Self::Constructor(item) => Some(item.state_mutability),
            Self::Fallback(item) => Some(item.state_mutability),
            Self::Receive(item) => Some(item.state_mutability),
            Self::Function(item) => Some(item.state_mutability),
            Self::Event(_) | Self::Error(_) => None,
        }
    }

    /// Returns a mutable reference to the state mutability of the item.
    ///
    /// Clones the item if it is not already owned.
    #[inline]
    pub fn state_mutability_mut(&mut self) -> Option<&mut StateMutability> {
        match self {
            Self::Constructor(item) => Some(&mut item.to_mut().state_mutability),
            Self::Fallback(item) => Some(&mut item.to_mut().state_mutability),
            Self::Receive(item) => Some(&mut item.to_mut().state_mutability),
            Self::Function(item) => Some(&mut item.to_mut().state_mutability),
            Self::Event(_) | Self::Error(_) => None,
        }
    }

    /// Returns an immutable reference to the inputs of the item.
    ///
    /// Use [`event_inputs`](Self::event_inputs) for events instead.
    #[inline]
    pub fn inputs(&self) -> Option<&Vec<Param>> {
        match self {
            Self::Error(item) => Some(&item.inputs),
            Self::Constructor(item) => Some(&item.inputs),
            Self::Function(item) => Some(&item.inputs),
            Self::Event(_) | Self::Fallback(_) | Self::Receive(_) => None,
        }
    }

    /// Returns a mutable reference to the inputs of the item.
    ///
    /// Clones the item if it is not already owned.
    ///
    /// Use [`event_inputs`](Self::event_inputs) for events instead.
    #[inline]
    pub fn inputs_mut(&mut self) -> Option<&mut Vec<Param>> {
        match self {
            Self::Error(item) => Some(&mut item.to_mut().inputs),
            Self::Constructor(item) => Some(&mut item.to_mut().inputs),
            Self::Function(item) => Some(&mut item.to_mut().inputs),
            Self::Event(_) | Self::Fallback(_) | Self::Receive(_) => None,
        }
    }

    /// Returns an immutable reference to the event inputs of the item.
    ///
    /// Use [`inputs`](Self::inputs) for other items instead.
    #[inline]
    pub fn event_inputs(&self) -> Option<&Vec<EventParam>> {
        match self {
            Self::Event(item) => Some(&item.inputs),
            Self::Constructor(_)
            | Self::Fallback(_)
            | Self::Receive(_)
            | Self::Error(_)
            | Self::Function(_) => None,
        }
    }

    /// Returns a mutable reference to the event inputs of the item.
    ///
    /// Clones the item if it is not already owned.
    ///
    /// Use [`inputs`](Self::inputs) for other items instead.
    #[inline]
    pub fn event_inputs_mut(&mut self) -> Option<&mut Vec<EventParam>> {
        match self {
            Self::Event(item) => Some(&mut item.to_mut().inputs),
            Self::Constructor(_)
            | Self::Fallback(_)
            | Self::Receive(_)
            | Self::Error(_)
            | Self::Function(_) => None,
        }
    }

    /// Returns an immutable reference to the outputs of the item.
    #[inline]
    pub fn outputs(&self) -> Option<&Vec<Param>> {
        match self {
            Self::Function(item) => Some(&item.outputs),
            Self::Constructor(_)
            | Self::Fallback(_)
            | Self::Receive(_)
            | Self::Error(_)
            | Self::Event(_) => None,
        }
    }

    /// Returns an immutable reference to the outputs of the item.
    #[inline]
    pub fn outputs_mut(&mut self) -> Option<&mut Vec<Param>> {
        match self {
            Self::Function(item) => Some(&mut item.to_mut().outputs),
            Self::Constructor(_)
            | Self::Fallback(_)
            | Self::Receive(_)
            | Self::Error(_)
            | Self::Event(_) => None,
        }
    }
}

impl FromStr for Constructor {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Constructor {
    /// Parses a Solidity constructor string:
    /// `constructor($($inputs),*) $(anonymous)?`
    ///
    /// Note:
    /// - the name must always be `constructor`.
    /// - that [`state_mutability`](Self::state_mutability) is not parsed from
    ///   the input and is always set to [`StateMutability::NonPayable`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_json_abi::{Constructor, Param, StateMutability};
    /// assert_eq!(
    ///     Constructor::parse("constructor(uint foo, address bar)"),
    ///     Ok(Constructor {
    ///         inputs: vec![
    ///             Param::parse("uint foo").unwrap(),
    ///             Param::parse("address bar").unwrap()
    ///         ],
    ///         state_mutability: StateMutability::NonPayable,
    ///     }),
    /// );
    /// ```
    #[inline]
    pub fn parse(s: &str) -> ParserResult<Self> {
        parse_sig::<false>(s).map(|(_, inputs, _, _)| Self {
            inputs,
            state_mutability: StateMutability::NonPayable,
        })
    }
}

impl FromStr for Error {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Error {
    /// Parses a Solidity error signature string:
    /// `$name($($inputs),*)`
    ///
    /// Note that the "error" keyword is not parsed as part of this function. If
    /// you want to parse a [Human-Readable ABI] string, use [`AbiItem::parse`].
    ///
    /// [Human-Readable ABI]: https://docs.ethers.org/v5/api/utils/abi/formats/#abi-formats--human-readable-abi
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use alloy_json_abi::{Error, Param, StateMutability};
    /// assert_eq!(
    ///     Error::parse("foo(bool bar)"),
    ///     Ok(Error {
    ///         name: "foo".to_string(),
    ///         inputs: vec![Param::parse("bool bar").unwrap()],
    ///     }),
    /// )
    /// ```
    #[inline]
    pub fn parse(s: &str) -> ParserResult<Self> {
        parse_sig::<false>(s).map(|(name, inputs, _, _)| Self { name, inputs })
    }

    /// Computes this error's signature: `$name($($inputs),*)`.
    ///
    /// This is the preimage input used to [compute the
    /// selector](Self::selector).
    #[inline]
    pub fn signature(&self) -> String {
        signature(&self.name, &self.inputs, None)
    }

    /// Computes this error's selector: `keccak256(self.signature())[..4]`
    #[inline]
    pub fn selector(&self) -> Selector {
        selector(&self.signature())
    }
}

impl FromStr for Function {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Function {
    /// Parses a Solidity function signature string:
    /// `$name($($inputs),*)$(($($outputs),*))?`
    ///
    /// Note:
    /// - the "function" keyword is not parsed as part of this function. If you
    ///   want to parse a [Human-Readable ABI] string, use [`AbiItem::parse`].
    /// - [`state_mutability`](Self::state_mutability) is not parsed from the
    ///   input and is always set to [`StateMutability::NonPayable`].
    ///
    /// [Human-Readable ABI]: https://docs.ethers.org/v5/api/utils/abi/formats/#abi-formats--human-readable-abi
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use alloy_json_abi::{Function, Param, StateMutability};
    /// assert_eq!(
    ///     Function::parse("foo(bool bar)"),
    ///     Ok(Function {
    ///         name: "foo".to_string(),
    ///         inputs: vec![Param::parse("bool bar").unwrap()],
    ///         outputs: vec![],
    ///         state_mutability: StateMutability::NonPayable,
    ///     }),
    /// )
    /// ```
    ///
    /// [Function]s also support parsing output parameters:
    ///
    /// ```
    /// # use alloy_json_abi::{Function, Param, StateMutability};
    /// assert_eq!(
    ///     Function::parse("toString(uint number)(string s)"),
    ///     Ok(Function {
    ///         name: "toString".to_string(),
    ///         inputs: vec![Param::parse("uint number").unwrap()],
    ///         outputs: vec![Param::parse("string s").unwrap()],
    ///         state_mutability: StateMutability::NonPayable,
    ///     }),
    /// );
    /// ```
    #[inline]
    pub fn parse(s: &str) -> ParserResult<Self> {
        parse_sig::<true>(s).map(|(name, inputs, outputs, _)| Self {
            name,
            inputs,
            outputs,
            state_mutability: StateMutability::NonPayable,
        })
    }

    /// Returns this function's signature: `$name($($inputs),*)`.
    ///
    /// This is the preimage input used to [compute the
    /// selector](Self::selector).
    #[inline]
    pub fn signature(&self) -> String {
        signature(&self.name, &self.inputs, None)
    }

    /// Returns this function's full signature:
    /// `$name($($inputs),*)($(outputs),*)`.
    ///
    /// This is the same as [`signature`](Self::signature), but also includes
    /// the output types.
    #[inline]
    pub fn signature_full(&self) -> String {
        signature(&self.name, &self.inputs, Some(&self.outputs))
    }

    /// Computes this error's selector: `keccak256(self.signature())[..4]`
    #[inline]
    pub fn selector(&self) -> Selector {
        selector(&self.signature())
    }
}

impl FromStr for Event {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Event {
    /// Parses a Solidity event signature string:
    /// `$name($($inputs),*) $(anonymous)?`
    ///
    /// Note that the "event" keyword is not parsed as part of this function. If
    /// you want to parse a [Human-Readable ABI] string, use [`AbiItem::parse`].
    ///
    /// [Human-Readable ABI]: https://docs.ethers.org/v5/api/utils/abi/formats/#abi-formats--human-readable-abi
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_json_abi::{Event, EventParam};
    /// assert_eq!(
    ///     Event::parse("foo(bool bar, uint indexed baz)"),
    ///     Ok(Event {
    ///         name: "foo".to_string(),
    ///         inputs: vec![
    ///             EventParam::parse("bool bar").unwrap(),
    ///             EventParam::parse("uint indexed baz").unwrap()
    ///         ],
    ///         anonymous: false,
    ///     }),
    /// );
    /// ```
    #[inline]
    pub fn parse(s: &str) -> ParserResult<Self> {
        parse_event_sig(s).map(|(name, inputs, _, anonymous)| Self {
            name,
            inputs,
            anonymous,
        })
    }

    /// Returns this event's signature: `$name($($inputs),*)`.
    ///
    /// This is the preimage input used to [compute the
    /// selector](Self::selector).
    #[inline]
    pub fn signature(&self) -> String {
        event_signature(&self.name, &self.inputs)
    }

    /// Computes this event's selector: `keccak256(self.signature())`
    #[inline]
    pub fn selector(&self) -> B256 {
        keccak256(self.signature().as_bytes())
    }

    /// Computes the number of this event's indexed topics.
    #[inline]
    pub fn num_topics(&self) -> usize {
        !self.anonymous as usize + self.inputs.iter().filter(|input| input.indexed).count()
    }
}
