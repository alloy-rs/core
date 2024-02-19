#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(unused_imports))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[doc(inline)]
pub use alloy_primitives as primitives;
#[doc(no_inline)]
pub use primitives::hex;

#[cfg(feature = "dyn-abi")]
#[doc(inline)]
pub use alloy_dyn_abi as dyn_abi;

#[cfg(feature = "json-abi")]
#[doc(inline)]
pub use alloy_json_abi as json_abi;

#[cfg(feature = "sol-types")]
#[doc(inline)]
pub use alloy_sol_types as sol_types;
#[cfg(all(doc, feature = "sol-types"))]
#[doc(no_inline)]
pub use sol_types::sol;

#[cfg(feature = "rlp")]
#[doc(inline)]
pub use alloy_rlp as rlp;

/// [`sol!`](sol_types::sol!) macro wrapper to route imports to the correct crate.
///
/// See [`sol!`](sol_types::sol!) for the actual macro documentation.
#[cfg(all(not(doc), feature = "sol-types"))]
#[doc(hidden)]
#[macro_export]
macro_rules! sol {
    ($($t:tt)*) => {
        $crate::sol_types::sol! {
            #![sol(alloy_sol_types = $crate::sol_types)]
            $($t)*
        }
    };
}
