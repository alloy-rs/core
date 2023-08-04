mod assembly;
pub use assembly::StmtAssembly;

mod blocks;
pub use blocks::{Block, UncheckedBlock};

mod r#break;
pub use r#break::StmtBreak;

mod r#continue;
pub use r#continue::StmtContinue;

mod do_while;
pub use do_while::StmtDoWhile;

mod emit;
pub use emit::StmtEmit;

mod expr;
pub use expr::StmtExpr;

mod r#for;
pub use r#for::StmtFor;

mod r#if;
pub use r#if::StmtIf;

mod r#return;
pub use r#return::StmtReturn;

mod revert;
pub use revert::StmtRevert;

mod r#try;
pub use r#try::StmtTry;

mod var_decl;
pub use var_decl::StmtVarDecl;

mod r#while;
pub use r#while::StmtWhile;

use crate::kw;
use proc_macro2::Span;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    token::{Brace, Paren},
    Result, Token,
};

/// A statement.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.statement>
#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    VarDecl(StmtVarDecl),
    Expr(StmtExpr),
    If(StmtIf),
    For(StmtFor),
    While(StmtWhile),
    DoWhile(StmtDoWhile),
    Continue(StmtContinue),
    Break(StmtBreak),
    Try(StmtTry),
    Return(StmtReturn),
    Emit(StmtEmit),
    Revert(StmtRevert),
    Assembly(StmtAssembly),
}

impl Parse for Stmt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Paren) {
            if input.peek2(Token![=]) {
                input.parse().map(Self::VarDecl)
            } else {
                input.parse().map(Self::Expr)
            }
        } else if lookahead.peek(kw::unchecked) {
            input.parse().map(Self::UncheckedBlock)
        } else if lookahead.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if lookahead.peek(Token![for]) {
            input.parse().map(Self::For)
        } else if lookahead.peek(Token![while]) {
            input.parse().map(Self::While)
        } else if lookahead.peek(Token![do]) {
            input.parse().map(Self::DoWhile)
        } else if lookahead.peek(Token![continue]) {
            input.parse().map(Self::Continue)
        } else if lookahead.peek(Token![break]) {
            input.parse().map(Self::Break)
        } else if lookahead.peek(Token![try]) {
            input.parse().map(Self::Try)
        } else if lookahead.peek(Token![return]) {
            input.parse().map(Self::Return)
        } else if lookahead.peek(kw::emit) {
            input.parse().map(Self::Emit)
        } else if lookahead.peek(kw::revert) {
            input.parse().map(Self::Revert)
        } else if lookahead.peek(kw::assembly) {
            input.parse().map(Self::Assembly)
        } else if lookahead.peek(kw::tuple)
            || lookahead.peek(kw::function)
            || lookahead.peek(kw::mapping)
        {
            input.parse().map(Self::VarDecl)
        } else {
            // TODO: Handle this better
            let start = input.fork();
            match input.parse() {
                Ok(var) => Ok(Self::VarDecl(var)),
                Err(_) => match start.parse() {
                    Ok(expr) => {
                        input.advance_to(&start);
                        Ok(Self::Expr(expr))
                    }
                    Err(_) => Err(lookahead.error()),
                },
            }
        }
    }
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Block(block) => block.span(),
            Stmt::UncheckedBlock(block) => block.span(),
            Stmt::VarDecl(stmt) => stmt.span(),
            Stmt::Expr(stmt) => stmt.span(),
            Stmt::If(stmt) => stmt.span(),
            Stmt::For(stmt) => stmt.span(),
            Stmt::While(stmt) => stmt.span(),
            Stmt::DoWhile(stmt) => stmt.span(),
            Stmt::Continue(stmt) => stmt.span(),
            Stmt::Break(stmt) => stmt.span(),
            Stmt::Try(stmt) => stmt.span(),
            Stmt::Return(stmt) => stmt.span(),
            Stmt::Emit(stmt) => stmt.span(),
            Stmt::Revert(stmt) => stmt.span(),
            Stmt::Assembly(stmt) => stmt.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Stmt::Block(block) => block.set_span(span),
            Stmt::UncheckedBlock(block) => block.set_span(span),
            Stmt::VarDecl(stmt) => stmt.set_span(span),
            Stmt::Expr(stmt) => stmt.set_span(span),
            Stmt::If(stmt) => stmt.set_span(span),
            Stmt::For(stmt) => stmt.set_span(span),
            Stmt::While(stmt) => stmt.set_span(span),
            Stmt::DoWhile(stmt) => stmt.set_span(span),
            Stmt::Continue(stmt) => stmt.set_span(span),
            Stmt::Break(stmt) => stmt.set_span(span),
            Stmt::Try(stmt) => stmt.set_span(span),
            Stmt::Return(stmt) => stmt.set_span(span),
            Stmt::Emit(stmt) => stmt.set_span(span),
            Stmt::Revert(stmt) => stmt.set_span(span),
            Stmt::Assembly(stmt) => stmt.set_span(span),
        }
    }
}
