use crate::{
    assignment::AssignmentExpr,
    binop::BinopExpr,
    emit::Emit,
    index::Index,
    inline_array_expr::InlineArrayExpr,
    loop_ops::LoopOps,
    method_call::MethodCall,
    new::New,
    r#for::ForStmt,
    r#if::IfStmt,
    r#return::Return,
    r#while::{DoWhile, While},
    revert::Revert,
    tuple_expr::TupleExpr,
    unchecked::Unchecked,
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
    LoopOps(LoopOps),
    Assign(AssignmentExpr),
    Index(Index),
    Binop(BinopExpr),
    MethodCall(MethodCall),
    Return(Return),
    Emit(Emit),
    Unchecked(Unchecked),
    Revert(Revert),
    Tuple(TupleExpr),
    InlineArray(InlineArrayExpr),
    New(New),
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
            return Ok(Self::Binop(BinopExpr::parse(input)?))
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("{:?} Path not implimented", input),
            ))
        }
    }
}
