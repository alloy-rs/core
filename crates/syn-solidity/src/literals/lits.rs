use syn::{parse::Parse, Error, LitBool, LitInt, LitStr};

#[derive(Debug, Clone)]
pub enum Literals {
    String(LitStr),
    Number(LitInt),
    Bool(LitBool),
}

impl Parse for Literals {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::String(input.parse()?))
        } else if input.peek(LitInt) {
            Ok(Self::Number(input.parse()?))
        } else if input.peek(LitBool) {
            Ok(Self::Bool(input.parse()?))
        } else {
            Err(Error::new(input.span(), "failed to parse literal"))
        }
    }
}
