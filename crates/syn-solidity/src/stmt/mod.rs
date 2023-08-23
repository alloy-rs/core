use crate::{kw, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
    Result, Token,
};

mod assembly;
pub use assembly::{AssemblyFlags, StmtAssembly};

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
pub use r#for::{ForInitStmt, StmtFor};

mod r#if;
pub use r#if::StmtIf;

mod r#return;
pub use r#return::StmtReturn;

mod revert;
pub use revert::StmtRevert;

mod r#try;
pub use r#try::{CatchClause, StmtTry};

mod var_decl;
pub use var_decl::{StmtVarDecl, VarDeclDecl, VarDeclTuple};

mod r#while;
pub use r#while::StmtWhile;

/// A statement, usually ending in a semicolon.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.statement>
#[derive(Clone)]
pub enum Stmt {
    /// An assembly block, with optional flags: `assembly "evmasm" { ... }`.
    Assembly(StmtAssembly),

    /// A blocked scope: `{ ... }`.
    Block(Block),

    /// A break statement: `break;`.
    Break(StmtBreak),

    /// A continue statement: `continue;`.
    Continue(StmtContinue),

    /// A do-while statement: `do { ... } while (condition);`.
    DoWhile(StmtDoWhile),

    /// An emit statement: `emit FooBar(42);`.
    Emit(StmtEmit),

    /// An expression with a trailing semicolon.
    Expr(StmtExpr),

    /// A for statement: `for (uint256 i; i < 42; ++i) { ... }`.
    For(StmtFor),

    /// An `if` statement with an optional `else` block: `if (expr) { ... } else
    /// { ... }`.
    If(StmtIf),

    /// A return statement: `return 42;`.
    Return(StmtReturn),

    /// A revert statement: `revert("error");`.
    Revert(StmtRevert),

    /// A try statement: `try fooBar(42) catch { ... }`.
    Try(StmtTry),

    /// An unchecked block: `unchecked { ... }`.
    UncheckedBlock(UncheckedBlock),

    /// A variable declaration statement: `uint256 foo = 42;`.
    VarDecl(StmtVarDecl),

    /// A while statement: `while (i < 42) { ... }`.
    While(StmtWhile),
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Stmt::")?;
        match self {
            Self::Assembly(stmt) => stmt.fmt(f),
            Self::Block(block) => block.fmt(f),
            Self::Break(stmt) => stmt.fmt(f),
            Self::Continue(stmt) => stmt.fmt(f),
            Self::DoWhile(stmt) => stmt.fmt(f),
            Self::Emit(stmt) => stmt.fmt(f),
            Self::Expr(stmt) => stmt.fmt(f),
            Self::For(stmt) => stmt.fmt(f),
            Self::If(stmt) => stmt.fmt(f),
            Self::Return(stmt) => stmt.fmt(f),
            Self::Revert(stmt) => stmt.fmt(f),
            Self::Try(stmt) => stmt.fmt(f),
            Self::UncheckedBlock(block) => block.fmt(f),
            Self::VarDecl(stmt) => stmt.fmt(f),
            Self::While(stmt) => stmt.fmt(f),
        }
    }
}

impl Parse for Stmt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // skip any attributes
        let _ = input.call(syn::Attribute::parse_outer)?;

        debug!("  > Stmt: {:?}", input.to_string());
        let stmt = Self::parse_simple(input)?;
        debug!("  < Stmt: {stmt:?}\n");
        Ok(stmt)
    }
}

impl Spanned for Stmt {
    fn span(&self) -> Span {
        match self {
            Self::Assembly(stmt) => stmt.span(),
            Self::Block(block) => block.span(),
            Self::Break(stmt) => stmt.span(),
            Self::Continue(stmt) => stmt.span(),
            Self::DoWhile(stmt) => stmt.span(),
            Self::Emit(stmt) => stmt.span(),
            Self::Expr(stmt) => stmt.span(),
            Self::For(stmt) => stmt.span(),
            Self::If(stmt) => stmt.span(),
            Self::Return(stmt) => stmt.span(),
            Self::Revert(stmt) => stmt.span(),
            Self::Try(stmt) => stmt.span(),
            Self::UncheckedBlock(block) => block.span(),
            Self::VarDecl(stmt) => stmt.span(),
            Self::While(stmt) => stmt.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Assembly(stmt) => stmt.set_span(span),
            Self::Block(block) => block.set_span(span),
            Self::Break(stmt) => stmt.set_span(span),
            Self::Continue(stmt) => stmt.set_span(span),
            Self::DoWhile(stmt) => stmt.set_span(span),
            Self::Emit(stmt) => stmt.set_span(span),
            Self::Expr(stmt) => stmt.set_span(span),
            Self::For(stmt) => stmt.set_span(span),
            Self::If(stmt) => stmt.set_span(span),
            Self::Return(stmt) => stmt.set_span(span),
            Self::Revert(stmt) => stmt.set_span(span),
            Self::Try(stmt) => stmt.set_span(span),
            Self::UncheckedBlock(block) => block.set_span(span),
            Self::VarDecl(stmt) => stmt.set_span(span),
            Self::While(stmt) => stmt.set_span(span),
        }
    }
}

impl Stmt {
    fn parse_simple(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Paren) {
            StmtVarDecl::parse_or_expr(input)
        } else if lookahead.peek(Bracket) {
            input.parse().map(Self::Expr)
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
        } else {
            StmtVarDecl::parse_or_expr(input)
        }
    }
}
