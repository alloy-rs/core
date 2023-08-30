use crate::{kw, Spanned};
use proc_macro2::Span;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};
use syn::{
    parse::{Lookahead1, Parse, ParseStream},
    Result,
};

macro_rules! str_lit {
    ($(#[$attr:meta])* $vis:vis struct $name:ident($t:ty) $(: $kw:ident)?) => {
        #[derive(Clone, Debug)]
        $vis struct $name {
            $vis values: Vec<$t>,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for (i, value) in self.values.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ")?;
                    }

                    $(
                        f.write_str(stringify!($kw))?;
                    )?
                    f.write_str(&value.value())?;
                }
                Ok(())
            }
        }

        impl Parse for $name {
            fn parse(input: ParseStream<'_>) -> Result<Self> {
                let mut values = Vec::new();
                let mut first = true;
                while first || Self::peek(&input.lookahead1()) {
                    first = false;
                    values.push(input.parse()?);
                }
                Ok(Self { values })
            }
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                crate::utils::join_spans(&self.values)
            }

            fn set_span(&mut self, span: Span) {
                crate::utils::set_spans(&mut self.values, span)
            }
        }

        impl $name {
            pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
                $(lookahead.peek(kw::$kw) || )? lookahead.peek(syn::LitStr)
            }

            pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
                if Self::peek(&input.lookahead1()) {
                    input.parse().map(Some)
                } else {
                    Ok(None)
                }
            }

            pub fn value(&self) -> String {
                self.values.iter().map(|v| v.value()).collect()
            }
        }
    };
}

macro_rules! wrap_str {
    ($(#[$attr:meta])* $vis:vis struct $name:ident { $token:ident : kw::$kw:ident $(,)? }) => {
        $(#[$attr])*
        #[derive(Clone)]
        $vis struct $name {
            /// The prefix of the string.
            $vis $token: kw::$kw,
            /// The string literal.
            $vis value: syn::LitStr,
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("value", &self.value)
                    .finish()
            }
        }

        impl Deref for $name {
            type Target = syn::LitStr;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }

        impl Parse for $name {
            fn parse(input: ParseStream<'_>) -> Result<Self> {
                Ok(Self {
                    $token: input.parse()?,
                    value: input.parse()?,
                })
            }
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                let span = self.$token.span;
                span.join(self.value.span()).unwrap_or(span)
            }

            fn set_span(&mut self, span: Span) {
                self.$token.span = span;
                self.value.set_span(span);
            }
        }
    };
}

str_lit! {
    /// A string literal.
    pub struct LitStr(syn::LitStr)
}

str_lit! {
    /// A unicode string literal.
    pub struct LitUnicodeStr(UnicodeStr): unicode
}

wrap_str! {
    /// A unicode string.
    pub struct UnicodeStr {
        unicode_token: kw::unicode,
    }
}

str_lit! {
    /// A hex string literal.
    pub struct LitHexStr(HexStr): hex
}

wrap_str! {
    /// A hex string.
    pub struct HexStr {
        hex_token: kw::hex,
    }
}
