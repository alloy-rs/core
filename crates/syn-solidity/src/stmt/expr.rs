use crate::{
    assembly::Assembly,
    binop::BinopExpr,
    emit::Emit,
    index::Index,
    inline_array_expr::InlineArrayExpr,
    kw,
    literals::lits::Literals,
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
    Block,
};

use syn::{
    parse::Parse,
    token::{Brace, Bracket, Paren},
    Ident, LitBool, LitInt, LitStr, Token,
};

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Block),
    While(While),
    DoWhile(DoWhile),
    If(IfStmt),
    ForLoop(ForStmt),
    LoopOps(LoopOps),
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
    Lit(Literals),
    Assembly(Assembly),
}

impl Parse for Stmt {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![while]) {
            return Ok(Self::While(While::parse(input)?))
        // this prob wrong lol
        } else if input.peek(Brace) {
            return Ok(Self::Block(Block::parse(input)?))
        } else if input.peek(Token![do]) {
            return Ok(Self::DoWhile(DoWhile::parse(input)?))
        } else if input.peek(Token![if]) {
            return Ok(Self::If(IfStmt::parse(input)?))
        } else if input.peek(Token![for]) {
            return Ok(Self::ForLoop(ForStmt::parse(input)?))
        } else if input.peek(Ident) && input.peek2(Bracket) {
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
        } else if input.peek(kw::emit) {
            Ok(Self::Emit(Emit::parse(input)?))
        } else if input.peek(kw::returns) {
            Ok(Self::Return(Return::parse(input)?))
        } else if input.peek(kw::unchecked) {
            Ok(Self::Unchecked(Unchecked::parse(input)?))
        } else if input.peek(kw::revert) {
            Ok(Self::Revert(Revert::parse(input)?))
        } else if input.peek(Paren) && input.peek3(Token!(,)) {
            Ok(Self::Tuple(TupleExpr::parse(input)?))
        } else if input.peek(Ident) && input.peek2(Paren) {
            Ok(Self::MethodCall(MethodCall::parse(input)?))
        } else if input.peek(Bracket) {
            Ok(Self::InlineArray(InlineArrayExpr::parse(input)?))
        } else if input.peek(kw::new) {
            Ok(Self::New(New::parse(input)?))
        } else if input.peek(kw::assembly) {
            Ok(Self::Assembly(Assembly::parse(input)?))
        } else if input.peek(LitBool) || input.peek(LitStr) || input.peek(LitInt) {
            Ok(Self::Lit(Literals::parse(input)?))
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("{:?} Path not implimented", input),
            ))
        }
    }
}
