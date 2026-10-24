#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_extern_crates)]
extern crate self as alloy_sol_types;

#[macro_use]
extern crate alloc;

#[macro_use]
mod macros;

pub mod abi;

mod errors;
pub use errors::{Error, Result};

#[cfg(feature = "json")]
mod ext;
#[cfg(feature = "json")]
pub use ext::JsonAbiExt;

mod impl_core;

mod types;
pub use types::{
    ContractError, EventTopic, GenericContractError, GenericRevertReason, Panic, PanicKind, Revert,
    RevertReason, Selectors, SolCall, SolConstructor, SolEnum, SolError, SolEvent,
    SolEventInterface, SolInterface, SolStruct, SolType, SolValue, TopicList,
    data_type as sol_data, decode_revert_reason,
};

pub mod utils;

mod eip712;
pub use eip712::Eip712Domain;

/// The ABI word type.
pub type Word = alloy_primitives::B256;

#[doc(no_inline)]
pub use alloy_sol_macro::sol;

// Not public API.
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub mod private {
    #[allow(deprecated)]
    pub use super::{
        abi::RECURSION_LIMIT,
        utils::{next_multiple_of_32, words_for, words_for_len},
    };
    pub use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        collections::BTreeMap,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
    pub use alloy_primitives::{
        self as primitives, Address, B256, Bytes, FixedBytes, Function, I256, IntoLogData, LogData,
        Signed, U256, Uint, bytes, keccak256,
    };
    pub use core::{
        borrow::{Borrow, BorrowMut},
        convert::From,
        default::Default,
        option::Option,
        result::Result,
    };

    pub use Option::{None, Some};
    pub use Result::{Err, Ok};

    #[cfg(feature = "json")]
    pub use alloy_json_abi;

    /// An ABI-encodable is any type that may be encoded via a given `SolType`.
    ///
    /// The `SolType` trait contains encoding logic for a single associated
    /// `RustType`. This trait allows us to plug in encoding logic for other
    /// `RustTypes`.
    ///
    /// **Note:** this trait is an implementation detail. As such, it should not
    /// be implemented directly unless implementing a custom
    /// [`SolType`](crate::SolType), which is also discouraged. Consider
    /// using [`SolValue`](crate::SolValue) instead.
    pub trait SolTypeValue<T: super::SolType> {
        // Note: methods are prefixed with `stv_` to avoid name collisions with
        // the `SolValue` trait.

        #[inline(always)]
        fn stv_abi_encoded_size(&self) -> usize {
            T::ENCODED_SIZE.unwrap()
        }
        fn stv_to_tokens(&self) -> T::Token<'_>;

        #[inline(always)]
        fn stv_abi_packed_encoded_size(&self) -> usize {
            T::PACKED_ENCODED_SIZE.unwrap()
        }
        fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>);

        fn stv_eip712_data_word(&self) -> super::Word;
    }

    #[inline(always)]
    pub const fn u256(n: u64) -> U256 {
        U256::from_limbs([n, 0, 0, 0])
    }

    pub struct AssertTypeEq<T>(pub T);

    #[inline(never)]
    pub fn str_to_owned(s: &str) -> String {
        s.to_owned()
    }

    /// Const-constructible mirrors of the [`alloy_json_abi`] items emitted by `#[sol(abi)]`.
    ///
    /// The macro emits each item as promoted constant data and converts it with a single
    /// non-generic call, instead of expanding a full owned literal for every item.
    #[cfg(feature = "json")]
    mod static_abi {
        use alloc::vec::Vec;
        use alloy_json_abi::{
            Constructor, Error, Event, EventParam, Function, Param, StateMutability,
        };

        #[derive(Clone, Copy)]
        pub struct StaticParam {
            pub ty: &'static str,
            pub name: &'static str,
            pub components: &'static [Self],
        }

        #[derive(Clone, Copy)]
        pub struct StaticEventParam {
            pub ty: &'static str,
            pub name: &'static str,
            pub indexed: bool,
            pub components: &'static [StaticParam],
        }

        #[derive(Clone, Copy)]
        pub struct StaticConstructor {
            pub inputs: &'static [StaticParam],
            pub state_mutability: StateMutability,
        }

        #[derive(Clone, Copy)]
        pub struct StaticFunction {
            pub name: &'static str,
            pub inputs: &'static [StaticParam],
            pub outputs: &'static [StaticParam],
            pub state_mutability: StateMutability,
        }

        #[derive(Clone, Copy)]
        pub struct StaticEvent {
            pub name: &'static str,
            pub inputs: &'static [StaticEventParam],
            pub anonymous: bool,
        }

        #[derive(Clone, Copy)]
        pub struct StaticError {
            pub name: &'static str,
            pub inputs: &'static [StaticParam],
        }

        impl StaticConstructor {
            #[inline(never)]
            pub fn to_abi(&self) -> Constructor {
                Constructor { inputs: params(self.inputs), state_mutability: self.state_mutability }
            }
        }

        impl StaticFunction {
            #[inline(never)]
            pub fn to_abi(&self) -> Function {
                Function {
                    name: self.name.into(),
                    inputs: params(self.inputs),
                    outputs: params(self.outputs),
                    state_mutability: self.state_mutability,
                }
            }
        }

        impl StaticEvent {
            #[inline(never)]
            pub fn to_abi(&self) -> Event {
                Event {
                    name: self.name.into(),
                    inputs: self
                        .inputs
                        .iter()
                        .map(|p| EventParam {
                            ty: p.ty.into(),
                            name: p.name.into(),
                            indexed: p.indexed,
                            components: params(p.components),
                            internal_type: None,
                        })
                        .collect(),
                    anonymous: self.anonymous,
                }
            }
        }

        impl StaticError {
            #[inline(never)]
            pub fn to_abi(&self) -> Error {
                Error { name: self.name.into(), inputs: params(self.inputs) }
            }
        }

        fn params(items: &[StaticParam]) -> Vec<Param> {
            items
                .iter()
                .map(|p| Param {
                    ty: p.ty.into(),
                    name: p.name.into(),
                    components: params(p.components),
                    internal_type: None,
                })
                .collect()
        }
    }

    #[cfg(feature = "json")]
    pub use static_abi::*;

    #[inline(never)]
    pub fn make_btree_map<K: Ord, V>(items: Vec<(K, V)>) -> BTreeMap<K, V> {
        BTreeMap::from_iter(items)
    }
}
