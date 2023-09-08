use std::fmt;

use syn::parse::{Parse, ParseStream, Result};

mod r#if;
pub use r#if::YulIf;

mod block;
pub use block::YulBlock;

mod var_decl;
pub use var_decl::YulVarDecl;

mod r#for;

mod switch;

#[derive(Clone)]
pub enum YulStmt {
    If(YulIf),
    Block(YulBlock),
    VarDecl(YulVarDecl),
}

impl fmt::Debug for YulStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulStmt").finish()
    }
}

impl Parse for YulStmt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self::If(input.parse()?))
    }
}
