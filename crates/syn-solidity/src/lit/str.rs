use crate::{kw, utils::parse_vec};
use proc_macro2::Span;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};
use syn::{
    parse::{Parse, ParseStream},
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
                Ok(Self {
                    values: parse_vec(input, false)?,
                })
            }
        }

        impl $name {
            pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
                if $(input.peek(kw::$kw) || )? input.peek(syn::LitStr) {
                    input.parse().map(Some)
                } else {
                    Ok(None)
                }
            }

            pub fn span(&self) -> Span {
                let mut span = self.values.first().unwrap().span();
                for value in &self.values[1..] {
                    span = span.join(value.span()).unwrap_or(span);
                }
                span
            }

            pub fn set_span(&mut self, span: Span) {
                for value in &mut self.values {
                    value.set_span(span);
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

        impl $name {
            pub fn span(&self) -> Span {
                let span = self.$token.span;
                span.join(self.value.span()).unwrap_or(span)
            }

            pub fn set_span(&mut self, span: Span) {
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
    pub struct LitUnicode(UnicodeStr): unicode
}

wrap_str! {
    /// A unicode string.
    pub struct UnicodeStr {
        unicode_token: kw::unicode,
    }
}

str_lit! {
    /// A hex string literal.
    pub struct LitHex(HexStr): hex
}

wrap_str! {
    /// A hex string.
    pub struct HexStr {
        hex_token: kw::hex,
    }
}
