use crate::{call_args::CallArgs, expr::Stmt, kw};
use syn::{parse::Parse, Token};

#[derive(Debug, Clone)]
pub struct Emit {
    keyword: kw::emit,
    expr: Box<Stmt>,
    args: CallArgs,
    semi: Token!(;),
}

impl Parse for Emit {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            expr: Box::new(input.parse()?),
            args: input.parse()?,
            semi: input.parse()?,
        })
    }
}
