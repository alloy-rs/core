use crate::{
    assignment::AssignmentExpr,
    binop::BinopExpr,
    binops::{pow::PowOps, Binop},
    index::Index,
    method_call::MethodCall,
    r#for::ForStmt,
    r#if::IfStmt,
    r#while::{DoWhile, While},
};

use syn::{
    parse::Parse,
    token::{Bracket, Paren},
    Ident, Token,
};

#[derive(Clone, Debug)]
pub enum Expr {
    While(While),
    DoWhile(DoWhile),
    If(IfStmt),
    ForLoop(ForStmt),
    Assign(AssignmentExpr),
    Index(Index),
    Binop(BinopExpr),
    MethodCall(MethodCall),
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        todo!()
    }
}

impl Parse for Expr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![while]) {
            return Ok(Self::While(While::parse(input)?))
        } else if input.peek(Token![do]) {
            return Ok(Self::DoWhile(DoWhile::parse(input)?))
        } else if input.peek(Token![if]) {
            return Ok(Self::If(IfStmt::parse(input)?))
        } else if input.peek(Token![for]) {
            return Ok(Self::ForLoop(ForStmt::parse(input)?))
        } else if input.peek2(Bracket) {
            return Ok(Self::Index(Index::parse(input)?))
        } else if input.peek(Ident) && input.peek2(Paren) {
            return Ok(Self::MethodCall(MethodCall::parse(input)?))
            // so jank feel like should be better way but just send it ig
        } else if input.peek(Token![!])
            || input.peek(Token![~])
            || input.peek2(Token!(=))
            || input.peek2(Token!(+))
            || input.peek2(Token!(+=))
            || input.peek2(Token!(-))
            || input.peek2(Token!(-=))
            || input.peek2(Token!(*))
            || input.peek2(Token!(*=))
            || input.peek2(Token!(/))
            || input.peek2(Token!(/=))
            || input.peek2(Token!(%))
            || input.peek2(Token!(%=))
            || input.peek2(Token!(&))
            || input.peek2(Token!(&=))
            || input.peek2(Token!(^))
            || input.peek2(Token!(^=))
            || input.peek2(Token!(|))
            || input.peek2(Token!(<<))
            || input.peek2(Token!(<<=))
            || input.peek2(Token!(>>))
            || input.peek2(Token!(>>=))
            || input.peek2(Token!(==))
            || input.peek2(Token!(&&))
            || input.peek2(Token!(||))
        {
            return Ok(Self::Binop(Binop::parse(input)?))
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("{:?} Path not implimented", input),
            ))
        }
    }
}
