use crate::{event_param::EventParam, param::Param, StateMutability};
use alloc::{borrow::Cow, string::String, vec::Vec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Serde order:
// Public items -> public enum -> private enum -> private items
//
// Items are duplicated to be able to make use of the derived `serde` impl,
// while enforcing that the public items emit their tag, as per the spec.
//
// They are all declared with `repr(C)` because the default repr (`Rust`) does
// not have any layout guarantees, which we need to be able to transmute between
// the private and public types.
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
            #[repr(C)]
            $vis struct $name {$(
                $(#[$fattr])*
                $fvis $field: $type,
            )*}

            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    AbiItem::deserialize(deserializer).and_then(|item| match item {
                        AbiItem::$name(item) => Ok(item.into_owned()),
                        item => Err(serde::de::Error::invalid_type(
                            serde::de::Unexpected::Other(item.debug_name()),
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

        #[doc(hidden)]
        mod private {
            use super::*;

            $(
                #[derive(Clone, Serialize, Deserialize)]
                #[serde(rename_all = "camelCase")]
                #[repr(C)]
                pub(super) struct $name {$(
                    $field: $type,
                )*}
            )*

            #[derive(Serialize, Deserialize)]
            #[serde(tag = "type", rename_all = "lowercase")]
            #[repr(C)]
            pub(super) enum AbiItem<'a> {$(
                $name(Cow<'a, self::$name>),
            )*}
        }

        /// A JSON ABI item.
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        #[repr(C)]
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
        /// Signature included in the topic 0. Instead, the indexed arguments
        /// are 0-indexed.
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

impl AbiItem<'_> {
    /// Returns the name of the item.
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
}

// SAFETY: `AbiItem` and `private::AbiItem` have the exact same variants, and
// all the items use a non-Rust repr.
// This is enforced in the macro.
#[doc(hidden)]
impl<'a> From<private::AbiItem<'a>> for AbiItem<'a> {
    #[inline(always)]
    fn from(item: private::AbiItem<'a>) -> AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

#[doc(hidden)]
impl<'a> From<AbiItem<'a>> for private::AbiItem<'a> {
    #[inline(always)]
    fn from(item: AbiItem<'a>) -> private::AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

#[doc(hidden)]
impl<'a, 'r> From<&'r private::AbiItem<'a>> for &'r AbiItem<'a> {
    #[inline(always)]
    fn from(item: &'r private::AbiItem<'a>) -> &'r AbiItem<'a> {
        unsafe { core::mem::transmute(item) }
    }
}

#[doc(hidden)]
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

/// `format!("{name}({inputs.join(",")})")`
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

/// `keccak256({preimage})[..4]`
fn selector(preimage: &str) -> [u8; 4] {
    // SAFETY: splitting an array
    unsafe {
        alloy_primitives::keccak256(preimage.as_bytes())
            .0
            .get_unchecked(..4)
            .try_into()
            .unwrap_unchecked()
    }
}
