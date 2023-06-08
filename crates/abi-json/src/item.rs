use crate::{event_param::EventParam, param::Param, StateMutability};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

macro_rules! abi_items {
    ($(
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {$(
            $(#[$fattr:meta])*
            $fvis:vis $field:ident : $type:ty,
        )*}
    )*) => {
        $(
            $(#[$attr])*
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            $vis struct $name {$(
                $(#[$fattr])*
                $fvis $field: $type,
            )*}

            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    AbiItem::deserialize(deserializer).and_then(|item| match item {
                        AbiItem::$name(item) => Ok(item.into_owned()),
                        item => Err(serde::de::Error::invalid_type(
                            serde::de::Unexpected::Other(&format!("{item:?}")),
                            &stringify!($name),
                        )),
                    })
                }
            }

            impl Serialize for $name {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    AbiItem::$name(Cow::Borrowed(self)).serialize(serializer)
                }
            }
        )*

        mod private {
            use super::*;

            $(
                #[derive(Clone, Serialize, Deserialize)]
                #[serde(rename_all = "camelCase")]
                pub(super) struct $name {$(
                    $field: $type,
                )*}
            )*

            #[derive(Serialize, Deserialize)]
            #[serde(tag = "type", rename_all = "lowercase")]
            pub(super) enum AbiItem<'a> {$(
                $name(Cow<'a, self::$name>),
            )*}
        }

        /// A JSON ABI item.
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum AbiItem<'a> {$(
            #[doc = concat!("A JSON ABI [`", stringify!($name), "`].")]
            $name(Cow<'a, $name>),
        )*}

    };
}

abi_items! {
    /// A JSON ABI constructor function.
    pub struct Constructor {
        /// The input types of the constructor. May be empty.
        pub inputs: Vec<Param>,
        /// The state mutability of the constructor.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI fallback function.
    #[derive(Copy)]
    pub struct Fallback {
        /// The state mutability of the fallback function.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI receive function.
    #[derive(Copy)]
    pub struct Receive {
        /// The state mutability of the receive function.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI function.
    pub struct Function {
        /// The name of the function.
        pub name: String,
        /// The input types of the function. May be empty.
        pub inputs: Vec<Param>,
        /// The output types of the function. May be empty.
        pub outputs: Vec<Param>,
        /// The state mutability of the function.
        pub state_mutability: StateMutability,
    }

    /// A JSON ABI event.
    pub struct Event {
        /// The name of the event.
        pub name: String,
        /// A list of the event's inputs, in order.
        pub inputs: Vec<EventParam>,
        /// Whether the event is anonymous. Anonymous events do not have their
        pub anonymous: bool,
    }

    /// A JSON ABI error.
    pub struct Error {
        /// The name of the error.
        pub name: String,
        /// A list of the error's components, in order.
        pub inputs: Vec<Param>,
    }
}

impl Serialize for AbiItem<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <&private::AbiItem<'_>>::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AbiItem<'_> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        private::AbiItem::deserialize(deserializer).map(Into::into)
    }
}

impl<'a> From<private::AbiItem<'a>> for AbiItem<'a> {
    #[inline(always)]
    fn from(item: private::AbiItem<'a>) -> AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

impl<'a> From<AbiItem<'a>> for private::AbiItem<'a> {
    #[inline(always)]
    fn from(item: AbiItem<'a>) -> private::AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

impl<'a, 'r> From<&'r private::AbiItem<'a>> for &'r AbiItem<'a> {
    #[inline(always)]
    fn from(item: &'r private::AbiItem<'a>) -> &'r AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

impl<'a, 'r> From<&'r AbiItem<'a>> for &'r private::AbiItem<'a> {
    #[inline(always)]
    fn from(item: &'r AbiItem<'a>) -> &'r private::AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

impl Error {
    /// Generate the selector preimage for this error.
    pub fn selector_preimage(&self) -> String {
        preimage(&self.name, &self.inputs)
    }

    /// Generate the selector for this error.
    pub fn selector(&self) -> alloy_primitives::Selector {
        selector(&self.selector_preimage())
    }
}

impl Function {
    /// Generate the selector preimage for this function.
    pub fn selector_preimage(&self) -> String {
        preimage(&self.name, &self.inputs)
    }

    /// Generate the selector for this function.
    pub fn selector(&self) -> alloy_primitives::Selector {
        selector(&self.selector_preimage())
    }
}

fn preimage(name: &str, inputs: &[Param]) -> String {
    let mut preimage = String::with_capacity(name.len() + 2 + inputs.len() * 32);
    preimage.push_str(name);

    preimage.push('(');
    let mut first = true;
    for input in inputs {
        if !first {
            preimage.push(',');
        }
        preimage.push_str(&input.selector_type());
        first = false;
    }
    preimage.push(')');

    preimage
}

fn selector(preimage: &str) -> [u8; 4] {
    unsafe {
        alloy_primitives::keccak256(preimage.as_bytes())
            .0
            .get_unchecked(..4)
            .try_into()
            .unwrap_unchecked()
    }
}
