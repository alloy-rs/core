use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

pub fn verbatim<T: Verbatim>(t: &T) -> TokenStream {
    let mut s = TokenStream::new();
    t.to_tokens(&mut s);
    s
}

pub trait Verbatim {
    fn to_tokens(&self, s: &mut TokenStream);

    #[inline]
    fn verbatim(&self) -> ToTokensCompat<'_, Self> {
        ToTokensCompat(self)
    }

    #[inline]
    fn into_verbatim(self) -> IntoTokensCompat<Self>
    where
        Self: Sized,
    {
        IntoTokensCompat(self)
    }
}

pub struct ToTokensCompat<'a, T: ?Sized + Verbatim>(pub &'a T);

impl<T: Verbatim> quote::ToTokens for ToTokensCompat<'_, T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

pub struct IntoTokensCompat<T: ?Sized + Verbatim>(pub T);

impl<T: Verbatim> quote::ToTokens for IntoTokensCompat<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl Verbatim for String {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(if self.is_empty() {
            quote!(::alloy_sol_types::private::String::new())
        } else {
            quote!(::alloy_sol_types::private::ToOwned::to_owned(#self))
        })
    }
}

impl Verbatim for bool {
    #[inline]
    fn to_tokens(&self, s: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, s)
    }
}

impl Verbatim for usize {
    #[inline]
    fn to_tokens(&self, s: &mut TokenStream) {
        quote::ToTokens::to_tokens(self, s)
    }
}

impl<T: Verbatim> Verbatim for Vec<T> {
    fn to_tokens(&self, s: &mut TokenStream) {
        let iter = self.iter().map(ToTokensCompat);
        s.extend(quote!(::alloy_sol_types::private::vec![#(#iter),*]));
    }
}

impl<K: Verbatim, V: Verbatim> Verbatim for BTreeMap<K, V> {
    fn to_tokens(&self, s: &mut TokenStream) {
        let k = self.keys().map(ToTokensCompat);
        let v = self.values().map(ToTokensCompat);
        s.extend(quote!(::alloy_sol_types::private::BTreeMap::from([#( (#k, #v) ),*])));
    }
}

impl<T: Verbatim> Verbatim for Option<T> {
    fn to_tokens(&self, s: &mut TokenStream) {
        let tts = match self {
            Some(t) => {
                let mut s = TokenStream::new();
                t.to_tokens(&mut s);
                quote!(::core::option::Option::Some(#s))
            }
            None => quote!(::core::option::Option::None),
        };
        s.extend(tts);
    }
}

macro_rules! derive_verbatim {
    () => {};

    (struct $name:ident { $($field:ident),* $(,)? } $($rest:tt)*) => {
        impl Verbatim for alloy_json_abi::$name {
            fn to_tokens(&self, s: &mut TokenStream) {
                let Self { $($field),* } = self;
                $(
                    let $field = ToTokensCompat($field);
                )*
                s.extend(quote! {
                    ::alloy_sol_types::private::alloy_json_abi::$name {
                        $($field: #$field,)*
                    }
                });
            }
        }
        derive_verbatim!($($rest)*);
    };

    (enum $name:ident { $($variant:ident $( { $($field_idx:tt : $field:ident),* $(,)? } )?),* $(,)? } $($rest:tt)*) => {
        impl Verbatim for alloy_json_abi::$name {
            fn to_tokens(&self, s: &mut TokenStream) {
                match self {$(
                    Self::$variant $( { $($field_idx: $field),* } )? => {
                        $($(
                            let $field = ToTokensCompat($field);
                        )*)?
                        s.extend(quote! {
                            ::alloy_sol_types::private::alloy_json_abi::$name::$variant $( { $($field_idx: #$field),* } )?
                        });
                    }
                )*}
            }
        }
        derive_verbatim!($($rest)*);
    };
}

derive_verbatim! {
    // struct JsonAbi { constructor, functions, events, errors, receive, fallback }
    struct Constructor { inputs, state_mutability }
    struct Fallback { state_mutability }
    struct Receive { state_mutability }
    struct Function { name, inputs, outputs, state_mutability }
    struct Error { name, inputs }
    struct Event { name, inputs, anonymous }
    struct Param { ty, name, components, internal_type }
    struct EventParam { ty, name, indexed, components, internal_type }

    enum InternalType {
        AddressPayable { 0: s },
        Contract { 0: s },
        Enum { contract: contract, ty: ty },
        Struct { contract: contract, ty: ty },
        Other { contract: contract, ty: ty },
    }

    enum StateMutability {
        Pure,
        View,
        NonPayable,
        Payable,
    }
}
