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
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate alloc;

use core::{slice, str};
use winnow::{
    error::{AddContext, ParserError, StrContext, StrContextValue},
    trace::trace,
    Parser,
};

/// Errors.
mod error;
pub use error::{Error, Result};

/// Solidity ident rules.
mod ident;
pub use ident::{is_id_continue, is_id_start, is_valid_identifier, IDENT_REGEX};

/// Root type specifier.
mod root;
pub use root::RootType;

/// Type stem.
mod stem;
pub use stem::TypeStem;

/// Tuple type specifier.
mod tuple;
pub use tuple::TupleSpecifier;

/// Type specifier.
mod type_spec;
pub use type_spec::TypeSpecifier;

#[inline]
pub(crate) fn spanned<'a, O, E>(
    mut f: impl Parser<&'a str, O, E>,
) -> impl Parser<&'a str, (&'a str, O), E> {
    trace("spanned", move |input: &mut &'a str| {
        let start = input.as_ptr();

        let mut len = input.len();
        let r = f.parse_next(input)?;
        len -= input.len();

        // SAFETY: str invariant
        unsafe {
            let span = slice::from_raw_parts(start, len);
            debug_assert!(str::from_utf8(span).is_ok());
            Ok((str::from_utf8_unchecked(span), r))
        }
    })
}

#[inline]
pub(crate) fn str_parser<'a, E: ParserError<&'a str> + AddContext<&'a str, StrContext>>(
    s: &'static str,
) -> impl Parser<&'a str, &'a str, E> {
    #[cfg(feature = "debug")]
    let name = format!("str={s:?}");
    #[cfg(not(feature = "debug"))]
    let name = "str";
    trace(
        name,
        s.context(StrContext::Expected(StrContextValue::StringLiteral(s))),
    )
}
