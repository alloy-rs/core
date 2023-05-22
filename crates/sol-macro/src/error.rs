use crate::common::{from_into_tuples, kw, Parameters, SolIdent};
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Attribute, Result, Token,
};

pub struct Error {
    _error_token: kw::error,
    pub name: SolIdent,
    _paren_token: Paren,
    pub fields: Parameters<Token![,]>,
    _semi_token: Token![;],
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("name", &self.name)
            .field("fields", &self.fields)
            .finish()
    }
}

impl Parse for Error {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _error_token: input.parse()?,
            name: input.parse()?,
            _paren_token: parenthesized!(content in input),
            fields: content.parse()?,
            _semi_token: input.parse()?,
        })
    }
}

impl Error {
    fn expand(&self, attrs: &[Attribute]) -> TokenStream {
        self.fields.assert_resolved();

        let name = &self.name;
        let fields = self.fields.iter();

        let (signature, selector) = self.fields.sig_and_sel(name.as_string());

        let size = self.fields.data_size(None);

        let converts = from_into_tuples(&name.0, &self.fields);
        quote! {
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Clone)]
            pub struct #name {
                #(pub #fields,)*
            }

            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            const _: () = {
                #converts

                #[automatically_derived]
                impl ::ethers_sol_types::SolError for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

                    const SIGNATURE: &'static str = #signature;
                    const SELECTOR: [u8; 4] = [#(#selector),*];

                    fn to_rust(&self) -> <Self::Tuple as ::ethers_sol_types::SolType>::RustType {
                        self.clone().into()
                    }

                    fn from_rust(tuple: <Self::Tuple as ::ethers_sol_types::SolType>::RustType) -> Self {
                        tuple.into()
                    }

                    fn data_size(&self) -> usize {
                        #size
                    }
                }
            };
        }
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        let tts = self.expand(attrs);
        tokens.extend(tts);
    }
}
