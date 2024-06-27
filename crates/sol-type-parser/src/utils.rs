#![allow(missing_docs)]

use crate::{Error, ParameterSpecifier, Result, RootType, StateMutability};
use alloc::{string::String, vec::Vec};
use core::{slice, str};
use winnow::{
    ascii::space0,
    combinator::{alt, cut_err, opt, preceded, separated, terminated, trace},
    error::{AddContext, ParserError, StrContext, StrContextValue},
    stream::Accumulate,
    PResult, Parser,
};

pub use crate::ident::identifier;

#[inline]
pub fn spanned<'a, O, E>(
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
pub fn char_parser<'a, E>(c: char) -> impl Parser<&'a str, char, E>
where
    E: ParserError<&'a str> + AddContext<&'a str, StrContext>,
{
    #[cfg(feature = "debug")]
    let name = format!("char={c:?}");
    #[cfg(not(feature = "debug"))]
    let name = "char";
    trace(name, c.context(StrContext::Expected(StrContextValue::CharLiteral(c))))
}

#[inline]
pub fn str_parser<'a, E>(s: &'static str) -> impl Parser<&'a str, &'a str, E>
where
    E: ParserError<&'a str> + AddContext<&'a str, StrContext>,
{
    #[cfg(feature = "debug")]
    let name = format!("str={s:?}");
    #[cfg(not(feature = "debug"))]
    let name = "str";
    trace(name, s.context(StrContext::Expected(StrContextValue::StringLiteral(s))))
}

pub fn tuple_parser<'a, O1, O2, E>(f: impl Parser<&'a str, O1, E>) -> impl Parser<&'a str, O2, E>
where
    O2: Accumulate<O1>,
    E: ParserError<&'a str> + AddContext<&'a str, StrContext>,
{
    list_parser('(', ',', ')', f)
}

pub fn array_parser<'a, O1, O2, E>(f: impl Parser<&'a str, O1, E>) -> impl Parser<&'a str, O2, E>
where
    O2: Accumulate<O1>,
    E: ParserError<&'a str> + AddContext<&'a str, StrContext>,
{
    list_parser('[', ',', ']', f)
}

#[inline]
fn list_parser<'i, O1, O2, E>(
    open: char,
    delim: char,
    close: char,
    f: impl Parser<&'i str, O1, E>,
) -> impl Parser<&'i str, O2, E>
where
    O2: Accumulate<O1>,
    E: ParserError<&'i str> + AddContext<&'i str, StrContext>,
{
    #[cfg(feature = "debug")]
    let name = format!("list({open:?}, {delim:?}, {close:?})");
    #[cfg(not(feature = "debug"))]
    let name = "list";

    // These have to be outside of the closure for some reason.
    let elems_1 = separated(1.., f, (char_parser(delim), space0));
    let mut elems_and_end = terminated(elems_1, (opt(delim), space0, cut_err(char_parser(close))));
    trace(name, move |input: &mut &'i str| {
        let _ = char_parser(open).parse_next(input)?;
        let _ = space0(input)?;
        if let Some(stripped) = input.strip_prefix(close) {
            *input = stripped;
            return Ok(O2::initial(Some(0)));
        }
        elems_and_end.parse_next(input)
    })
}

pub fn opt_ws_ident<'a>(input: &mut &'a str) -> PResult<Option<&'a str>> {
    preceded(space0, opt(identifier)).parse_next(input)
}

// Not public API.
#[doc(hidden)]
#[inline]
pub fn parse_item<'a>(s: &mut &'a str) -> Result<&'a str> {
    trace("item", terminated(identifier, space0)).parse_next(s).map_err(Error::parser)
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedSignature<Param> {
    pub name: String,
    pub inputs: Vec<Param>,
    pub outputs: Vec<Param>,
    pub anonymous: bool,
    pub state_mutability: Option<StateMutability>,
}

#[doc(hidden)]
pub fn parse_signature<'a, const OUT: bool, F: Fn(ParameterSpecifier<'a>) -> T, T>(
    s: &'a str,
    f: F,
) -> Result<ParsedSignature<T>> {
    let params = || tuple_parser(ParameterSpecifier::parser.map(&f));
    trace(
        "signature",
        (
            // name
            RootType::parser,
            // inputs
            preceded(space0, params()),
            // visibility
            opt(preceded(space0, alt(("internal", "external", "private", "public")))),
            // state mutability
            opt(preceded(space0, alt(("pure", "view", "payable")))),
            // outputs
            move |i: &mut _| {
                if OUT {
                    preceded((space0, opt(":"), opt("returns"), space0), opt(params()))
                        .parse_next(i)
                        .map(Option::unwrap_or_default)
                } else {
                    Ok(vec![])
                }
            },
            // anonymous
            preceded(space0, opt("anonymous").map(|x| x.is_some())),
        ),
    )
    .map(|(name, inputs, _visibility, mutability, outputs, anonymous)| ParsedSignature {
        name: name.span().into(),
        inputs,
        outputs,
        anonymous,
        state_mutability: mutability.map(|s| s.parse().unwrap()),
    })
    .parse(s)
    .map_err(Error::parser)
}
