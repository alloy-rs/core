use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod common;
mod error;
mod function;
mod input;
mod r#struct;
mod r#type;
mod udt;

#[proc_macro]
pub fn sol(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as input::Input);
    s.to_token_stream().into()
}
