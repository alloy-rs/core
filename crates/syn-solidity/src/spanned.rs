//! Helper [trait](Spanned) and methods to manipulate syntax tree nodes' spans.

#![deny(unconditional_recursion)]

use proc_macro2::{Span, TokenStream};
use syn::{
    punctuated::{Pair, Punctuated},
    Token,
};

/// A trait that can provide the `Span` of the complete contents of a syntax
/// tree node.
///
/// The main difference between this trait and [`syn::spanned::Spanned`] is that
/// this trait does not depend on a [`quote::ToTokens`] implementation to
/// retrieve a span, as it is usually stored inside of the syntax tree node
/// itself.
pub trait Spanned {
    /// Returns a `Span` covering the complete contents of this syntax tree
    /// node, or [`Span::call_site()`] if this node is empty.
    ///
    /// [`Span::call_site()`]: proc_macro2::Span::call_site
    fn span(&self) -> Span;

    /// Sets the span of this syntax tree node if it is not empty.
    fn set_span(&mut self, span: Span);

    /// Sets the span of this owned syntax tree node if it is not empty.
    #[inline]
    fn with_span(mut self, span: Span) -> Self
    where
        Self: Sized,
    {
        self.set_span(span);
        self
    }
}

fn _object_safe(_: &dyn Spanned) {}

impl Spanned for Span {
    #[inline]
    fn span(&self) -> Span {
        *self
    }

    #[inline]
    fn set_span(&mut self, span: Span) {
        *self = span;
    }
}

impl Spanned for TokenStream {
    #[inline]
    fn span(&self) -> Span {
        syn::spanned::Spanned::span(self)
    }

    #[inline]
    fn set_span(&mut self, span: Span) {
        *self = self.clone().with_span(span);
    }

    fn with_span(self, span: Span) -> Self {
        self.into_iter()
            .map(|mut tt| {
                tt.set_span(span);
                tt
            })
            .collect()
    }
}

impl<T: Spanned> Spanned for Option<T> {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Some(t) => t.span(),
            None => Span::call_site(),
        }
    }

    #[inline]
    fn set_span(&mut self, span: Span) {
        if let Some(t) = self {
            t.set_span(span);
        }
    }
}

impl<T: ?Sized + Spanned> Spanned for &T {
    #[inline]
    fn span(&self) -> Span {
        (**self).span()
    }

    #[inline]
    #[track_caller]
    fn set_span(&mut self, _span: Span) {
        unimplemented!(
            "cannot set span of borrowed Spanned: {:?}",
            std::any::type_name::<&T>()
        )
    }
}

impl<T: Spanned> Spanned for [T] {
    #[inline]
    fn span(&self) -> Span {
        join_spans(self)
    }

    #[inline]
    fn set_span(&mut self, span: Span) {
        set_spans(self, span);
    }
}

impl<T: Spanned, P: Spanned> Spanned for Punctuated<T, P> {
    fn span(&self) -> Span {
        join_spans(self.pairs())
    }

    fn set_span(&mut self, span: Span) {
        set_spans(self.pairs_mut(), span);
    }
}

impl<T: Spanned, P: Spanned> Spanned for Pair<T, P> {
    fn span(&self) -> Span {
        let span = self.value().span();
        self.punct()
            .and_then(|punct| span.join(punct.span()))
            .unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.value_mut().set_span(span);
        self.punct_mut().set_span(span);
    }
}

macro_rules! deref_impls {
    ($($(#[$attr:meta])* [$($gen:tt)*] $t:ty),+ $(,)?) => {$(
        $(#[$attr])*
        impl<$($gen)*> Spanned for $t {
            #[inline]
            fn span(&self) -> Span {
                (**self).span()
            }

            #[inline]
            fn set_span(&mut self, span: Span) {
                (**self).set_span(span)
            }
        }
    )+};
}

deref_impls! {
    [T: ?Sized + Spanned] &mut T,
    [T: ?Sized + Spanned] Box<T>,
    [T: Spanned] Vec<T>,
}

macro_rules! inherent_impl {
    ($($t:ty),* $(,)?) => {$(
        impl Spanned for $t {
            #[inline]
            fn span(&self) -> Span {
                self.span()
            }

            #[inline]
            fn set_span(&mut self, span: Span) {
                self.set_span(span);
            }
        }
    )*};
}

inherent_impl!(
    proc_macro2::TokenTree,
    proc_macro2::Group,
    proc_macro2::Punct,
    proc_macro2::Ident,
    proc_macro2::Literal,
    syn::Lifetime,
    syn::Lit,
    syn::LitStr,
    syn::LitByteStr,
    syn::LitByte,
    syn::LitChar,
    syn::LitInt,
    syn::LitFloat,
    syn::LitBool,
);

/// Implements `Spanned` for `syn::Token!`s.
///
/// Prefix with `__more` when the underlying token has more than one span.
macro_rules! kw_impl {
    ($([$($t:tt)+])+) => { $(kw_impl!($($t)+);)+ };

    (__more $t:tt) => {
        impl Spanned for Token![$t] {
            #[inline]
            fn span(&self) -> Span {
                self.spans.span()
            }

            #[inline]
            fn set_span(&mut self, span: Span) {
                self.spans.set_span(span);
            }
        }
    };

    ($t:tt) => {
        impl Spanned for Token![$t] {
            #[inline]
            fn span(&self) -> Span {
                self.span
            }

            #[inline]
            fn set_span(&mut self, span: Span) {
                self.span = span;
            }
        }
    };
}

kw_impl! {
    [abstract]
    [as]
    [async]
    [auto]
    [await]
    [become]
    [box]
    [break]
    [const]
    [continue]
    [crate]
    [default]
    [do]
    [dyn]
    [else]
    [enum]
    [extern]
    [final]
    [fn]
    [for]
    [if]
    [impl]
    [in]
    [let]
    [loop]
    [macro]
    [match]
    [mod]
    [move]
    [mut]
    [override]
    [priv]
    [pub]
    [ref]
    [return]
    [Self]
    [self]
    [static]
    [struct]
    [super]
    [trait]
    [try]
    [type]
    [typeof]
    [union]
    [unsafe]
    [unsized]
    [use]
    [virtual]
    [where]
    [while]
    [yield]
    [&]
    [__more &&]
    [__more &=]
    [@]
    [^]
    [__more ^=]
    [:]
    [,]
    [$]
    [.]
    [__more ..]
    [__more ...]
    [__more ..=]
    [=]
    [__more ==]
    [__more =>]
    [__more >=]
    [>]
    [__more <-]
    [__more <=]
    [<]
    [-]
    [__more -=]
    [__more !=]
    [!]
    [|]
    [__more |=]
    [__more ||]
    [__more ::]
    [%]
    [__more %=]
    [+]
    [__more +=]
    [#]
    [?]
    [__more ->]
    [;]
    [__more <<]
    [__more <<=]
    [__more >>]
    [__more >>=]
    [/]
    [__more /=]
    [*]
    [__more *=]
    [~]
    [_]
}

macro_rules! delim_impl {
    ($($t:path),* $(,)?) => {$(
        impl Spanned for $t {
            #[inline]
            fn span(&self) -> Span {
                self.span.join()
            }

            #[inline]
            fn set_span(&mut self, span: Span) {
                *self = $t(span);
            }

            #[inline]
            fn with_span(self, span: Span) -> Self {
                $t(span)
            }
        }
    )*};
}

delim_impl!(syn::token::Brace, syn::token::Bracket, syn::token::Paren);

/// Joins the spans of each item in the given iterator.
pub fn join_spans<T: Spanned, I: IntoIterator<Item = T>>(items: I) -> Span {
    items
        .into_iter()
        .map(|t| t.span())
        .reduce(|span, other| span.join(other).unwrap_or(span))
        .unwrap_or_else(Span::call_site)
}

/// Sets the span of each item in the given iterator.
pub fn set_spans<T: Spanned, I: IntoIterator<Item = T>>(items: I, span: Span) {
    for mut item in items {
        item.set_span(span);
    }
}
