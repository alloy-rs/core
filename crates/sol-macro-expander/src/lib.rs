#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![allow(clippy::missing_const_for_fn, rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod codegen;
pub mod expand;
pub mod utils;
#[cfg(feature = "json")]
mod verbatim;

// Re-export commonly used utilities
pub use codegen::{
    CallCodegen, CallLayout, ConstructorInfo, ContractCodegen, ContractEventInfo,
    ContractFunctionInfo, Eip712Options, EnumCodegen, ErrorCodegen, EventCodegen, EventFieldInfo,
    InterfaceCodegen, ReturnInfo, SolInterfaceKind, StructCodegen, StructLayout,
    gen_from_into_tuple, gen_tokenize, is_reserved_method_name,
};
pub use utils::calc_selector as selector;

extern crate syn_solidity as ast;
