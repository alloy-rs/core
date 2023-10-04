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

use alloc::{string::String, vec::Vec};
use core::{slice, str};
use ident::parse_identifier;
use winnow::{
    ascii::space0,
    combinator::{cut_err, delimited, opt, preceded, separated0, terminated},
    error::{AddContext, ParserError, StrContext, StrContextValue},
    stream::Accumulate,
    trace::trace,
    PResult, Parser,
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

/// Parameter specifier.
mod parameter;
pub use parameter::{ParameterSpecifier, Parameters, Storage};

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
pub(crate) fn str_parser<'a, E>(s: &'static str) -> impl Parser<&'a str, &'a str, E>
where
    E: ParserError<&'a str> + AddContext<&'a str, StrContext>,
{
    #[cfg(feature = "debug")]
    let name = format!("str={s:?}");
    #[cfg(not(feature = "debug"))]
    let name = "str";
    trace(
        name,
        s.context(StrContext::Expected(StrContextValue::StringLiteral(s))),
    )
}

/// `( $( ${f()} ),* $(,)? )`
pub(crate) fn tuple_parser<'a, O1, O2, E>(
    f: impl Parser<&'a str, O1, E>,
) -> impl Parser<&'a str, O2, E>
where
    O2: Accumulate<O1>,
    E: ParserError<&'a str> + AddContext<&'a str, StrContext>,
{
    trace(
        "tuple",
        delimited(
            (str_parser("("), space0),
            cut_err(separated0(f, (str_parser(","), space0))),
            (opt(","), space0, cut_err(str_parser(")"))),
        ),
    )
}

pub(crate) fn opt_ws_ident<'a>(input: &mut &'a str) -> PResult<Option<&'a str>> {
    preceded(space0, opt(parse_identifier)).parse_next(input)
}

// Not public API.
#[doc(hidden)]
#[inline]
pub fn __internal_parse_item<'a>(s: &mut &'a str) -> Result<&'a str> {
    trace("item", terminated(parse_identifier, space0))
        .parse_next(s)
        .map_err(Error::parser)
}

#[doc(hidden)]
pub fn __internal_parse_signature<'a, const OUT: bool, F: Fn(ParameterSpecifier<'a>) -> T, T>(
    s: &'a str,
    f: F,
) -> Result<(String, Vec<T>, Vec<T>, bool)> {
    trace(
        "signature",
        (
            RootType::parser.map(|x| x.span().into()),
            preceded(space0, tuple_parser(ParameterSpecifier::parser.map(&f))),
            |i: &mut _| {
                if OUT {
                    preceded(
                        (space0, opt(":"), opt("returns"), space0),
                        opt(tuple_parser(ParameterSpecifier::parser.map(&f))),
                    )
                    .parse_next(i)
                    .map(Option::unwrap_or_default)
                } else {
                    Ok(vec![])
                }
            },
            preceded(space0, opt("anonymous").map(|x| x.is_some())),
        ),
    )
    .parse(s)
    .map_err(Error::parser)
}
