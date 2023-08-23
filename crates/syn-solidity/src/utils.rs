use crate::{Expr, Spanned};
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use std::fmt;
use syn::{parse::ParseStream, punctuated::Punctuated, Result, Token};

/// Helper trait to parsing nested expressions.
pub(crate) trait ParseNested: Sized {
    /// Parse `Self` as an expression that starts with `expr`.
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self>;
}

#[repr(transparent)]
pub(crate) struct DebugPunctuated<T, P>(Punctuated<T, P>);

impl<T, P> DebugPunctuated<T, P> {
    #[inline(always)]
    pub(crate) fn new(punctuated: &Punctuated<T, P>) -> &Self {
        unsafe { &*(punctuated as *const Punctuated<T, P> as *const Self) }
    }
}

impl<T: fmt::Debug, P> fmt::Debug for DebugPunctuated<T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[allow(dead_code)]
pub(crate) fn peek_tt(input: ParseStream<'_>) -> Option<TokenTree> {
    let tt = input.cursor().token_tree();
    tt.and_then(|(tt, _)| match tt {
        TokenTree::Group(g) if matches!(g.delimiter(), Delimiter::None) => {
            g.stream().into_iter().next()
        }
        _ => Some(tt),
    })
}

pub(crate) fn tts_until_semi(input: ParseStream<'_>) -> TokenStream {
    let mut tts = TokenStream::new();
    while !input.is_empty() && !input.peek(Token![;]) {
        let tt = input.parse::<TokenTree>().unwrap();
        tts.extend(std::iter::once(tt));
    }
    tts
}

pub(crate) fn join_spans<T: Spanned, I: IntoIterator<Item = T>>(items: I) -> Span {
    let mut iter = items.into_iter();
    let Some(first) = iter.next() else {
        return Span::call_site()
    };
    let first = first.span();
    iter.last()
        .and_then(|last| first.join(last.span()))
        .unwrap_or(first)
}

pub(crate) fn set_spans<'a, T, I>(items: I, span: Span)
where
    T: Spanned + 'a,
    I: IntoIterator<Item = &'a mut T>,
{
    for item in items {
        item.set_span(span);
    }
}

pub(crate) fn set_spans_clone<'a, T, I>(items: &mut I, span: Span)
where
    T: Spanned + 'a,
    I: Clone + IntoIterator<Item = T> + FromIterator<T>,
{
    *items = items
        .clone()
        .into_iter()
        .map(|mut item| {
            item.set_span(span);
            item
        })
        .collect();
}
