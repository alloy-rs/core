use syn::{parse::Parse, Token};

use crate::Block;

#[derive(Debug, Clone)]
pub enum IfStmtType {
    ElseIf,
    Else,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub init: Token![if],
    pub optional_stmts: Vec<IfStmtType>,
    pub expr: Block,
}

impl Parse for IfStmtType {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek2(Token!(if)) {
            Ok(Self::ElseIf)
        } else if input.peek(Token![else]) {
            Ok(Self::Else)
        } else {
            Err(syn::Error::new(
                input.span(),
                "secondary control flow parsing failed",
            ))
        }
    }
}
impl Parse for IfStmt {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let init = input.parse()?;
        let mut optional = Vec::new();

        let fork = input.fork();
        while let Ok(stmt) = fork.parse::<IfStmtType>() {
            optional.push(stmt);
        }

        Ok(Self {
            init,
            optional_stmts: optional,
            expr: input.parse(),
        })
    }
}
