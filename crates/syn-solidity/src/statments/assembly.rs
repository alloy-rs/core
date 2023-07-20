use proc_macro2::TokenStream;
use syn::token::Brace;

use crate::kw;

#[derive(Debug, Clone)]
pub struct Assembly {
    kw: kw::assembly,
    // stuff here
    brace: Brace,
    input: TokenStream,
}
