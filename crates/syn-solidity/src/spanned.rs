use proc_macro2::{Span, TokenStream, TokenTree};
use syn::{punctuated::Punctuated, Token};

/// A trait for getting and setting the span of a syntax tree node.
pub trait Spanned {
    /// Returns the span of this syntax tree node.
    fn span(&self) -> Span;

    /// Sets the span of this syntax tree node.
    fn set_span(&mut self, span: Span);
}

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
        crate::utils::set_spans_clone(self, span);
    }
}

impl Spanned for TokenTree {
    #[inline]
    fn span(&self) -> Span {
        self.span()
    }

    #[inline]
    fn set_span(&mut self, span: Span) {
        self.set_span(span);
    }
}

impl Spanned for syn::LitStr {
    fn span(&self) -> Span {
        self.span()
    }

    fn set_span(&mut self, span: Span) {
        self.set_span(span);
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

impl<T: ?Sized + Spanned + Clone> Spanned for &T {
    #[inline]
    fn span(&self) -> Span {
        (**self).span()
    }

    #[inline]
    fn set_span(&mut self, _span: Span) {
        unimplemented!(
            "cannot set span of borrowed Spanned: {:?}",
            std::any::type_name::<T>()
        )
    }
}

impl<T: Spanned + Clone, P> Spanned for Punctuated<T, P> {
    fn span(&self) -> Span {
        crate::utils::join_spans(self)
    }

    fn set_span(&mut self, span: Span) {
        crate::utils::set_spans(self, span);
    }
}

macro_rules! deref_impl {
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

deref_impl! {
    [T: ?Sized + Spanned] &mut T,
    [T: ?Sized + Spanned] Box<T>,
    [T: Spanned] Vec<T>,
}

impl<T: Spanned> Spanned for [T] {
    #[inline]
    fn span(&self) -> Span {
        join_spans(self.iter().map(Spanned::span))
    }

    #[inline]
    fn set_span(&mut self, span: Span) {
        for item in self {
            item.set_span(span);
        }
    }
}

// For `syn::Token!`s
macro_rules! kw_impl {
    ($([$($t:tt)+])+) => { $(kw_impl!($($t)+);)+ };

    (__more $t:tt) => {
        impl Spanned for Token![$t] {
            #[inline]
            fn span(&self) -> Span {
                join_spans(self.spans)
            }

            #[inline]
            fn set_span(&mut self, span: Span) {
                set_spans(&mut self.spans, span);
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

fn join_spans<I: IntoIterator<Item = Span>>(spans: I) -> Span {
    let mut iter = spans.into_iter();
    let Some(first) = iter.next() else {
        return Span::call_site()
    };
    iter.last()
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}

fn set_spans<'a, I: IntoIterator<Item = &'a mut Span>>(spans: I, set_to: Span) {
    for span in spans {
        *span = set_to;
    }
}
