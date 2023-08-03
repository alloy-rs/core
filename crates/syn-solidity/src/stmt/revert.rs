use crate::{call_args::CallArgs, expr::Stmt, kw};
use syn::{parse::Parse, Token};

#[derive(Clone, Debug)]
pub struct Revert {
    kw: kw::revert,
    expr: Box<Stmt>,
    args: CallArgs,
    semi: Token![;],
}

impl Parse for Revert {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            expr: Box::new(input.parse()?),
            args: input.parse()?,
            semi: input.parse()?,
        })
    }
}
