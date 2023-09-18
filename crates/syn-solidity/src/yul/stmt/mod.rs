use crate::{kw, Spanned, YulFnCall, YulFunctionDef};

use std::fmt;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Brace, Paren},
    Token,
};

mod r#if;
pub use r#if::YulIf;

mod block;
pub use block::YulBlock;

mod var_decl;
pub use var_decl::YulVarDecl;

mod r#for;
pub use r#for::YulFor;

mod switch;
pub use switch::{YulCaseBranch, YulSwitch, YulSwitchDefault};

mod assignment;
pub use assignment::YulVarAssign;

mod walrus_token;
pub use walrus_token::WalrusToken;

/// A Yul statement.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulStatement>
#[derive(Clone)]
pub enum YulStmt {
    /// A Yul blocked scope: `{ ... }`.
    Block(YulBlock),

    /// A variable declaration statement: `let x := 0`.
    Decl(YulVarDecl),

    /// A variable assignment statement: `x := 1`.
    Assign(YulVarAssign),

    /// A function call statement: `foo(a, b)`.
    Call(YulFnCall),

    /// A if statement: `if lt(a, b) { ... }`.
    If(YulIf),

    /// A for statement: `for {let i := 0} lt(i,10) {i := add(i,1)} { ... }`.
    For(YulFor),

    /// A switch statement: `switch expr case 0 { ... } default { ... }`.
    Switch(YulSwitch),

    /// A leave statement: `leave`.
    Leave(kw::leave),

    /// A break statement: `break`.
    Break(Token![break]),

    /// A continue statement: `continue`.
    Continue(Token![continue]),

    /// A function defenition statement: `function f() { ... }`.
    FunctionDef(YulFunctionDef),
}

impl Parse for YulStmt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let _ = input.call(syn::Attribute::parse_outer)?;

        if input.peek(Brace) {
            input.parse().map(Self::Block)
        } else if input.peek(Token![let]) {
            input.parse().map(Self::Decl)
        } else if input.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if input.peek(Token![for]) {
            input.parse().map(Self::For)
        } else if input.peek(kw::switch) {
            input.parse().map(Self::Switch)
        } else if input.peek(kw::leave) {
            input.parse().map(Self::Leave)
        } else if input.peek(Token![break]) {
            input.parse().map(Self::Break)
        } else if input.peek(Token![continue]) {
            input.parse().map(Self::Continue)
        } else if input.peek(kw::function) {
            input.parse().map(Self::FunctionDef)
        } else if input.peek2(Paren) {
            input.parse().map(Self::Call)
        } else {
            input.parse().map(Self::Assign)
        }
    }
}

impl Spanned for YulStmt {
    fn span(&self) -> Span {
        match self {
            YulStmt::Block(block) => block.span(),
            YulStmt::Decl(decl) => decl.span(),
            YulStmt::Assign(assign) => assign.span(),
            YulStmt::Call(call) => call.span(),
            YulStmt::If(r#if) => r#if.span(),
            YulStmt::For(r#for) => r#for.span(),
            YulStmt::Switch(switch) => switch.span(),
            YulStmt::Leave(leave) => leave.span(),
            YulStmt::Break(r#break) => r#break.span(),
            YulStmt::Continue(r#continue) => r#continue.span(),
            YulStmt::FunctionDef(fn_def) => fn_def.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            YulStmt::Block(block) => block.set_span(span),
            YulStmt::Decl(decl) => decl.set_span(span),
            YulStmt::Assign(assign) => assign.set_span(span),
            YulStmt::Call(call) => call.set_span(span),
            YulStmt::If(r#if) => r#if.set_span(span),
            YulStmt::For(r#for) => r#for.set_span(span),
            YulStmt::Switch(switch) => switch.set_span(span),
            YulStmt::Leave(leave) => leave.set_span(span),
            YulStmt::Break(r#break) => r#break.set_span(span),
            YulStmt::Continue(r#continue) => r#continue.set_span(span),
            YulStmt::FunctionDef(fn_def) => fn_def.set_span(span),
        }
    }
}

impl fmt::Debug for YulStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulStmt::")?;
        match self {
            YulStmt::Block(block) => block.fmt(f),
            YulStmt::Decl(decl) => decl.fmt(f),
            YulStmt::Assign(assign) => assign.fmt(f),
            YulStmt::Call(call) => call.fmt(f),
            YulStmt::If(r#if) => r#if.fmt(f),
            YulStmt::For(r#for) => r#for.fmt(f),
            YulStmt::Switch(switch) => switch.fmt(f),
            YulStmt::Leave(leave) => leave.fmt(f),
            YulStmt::Break(r#break) => r#break.fmt(f),
            YulStmt::Continue(r#continue) => r#continue.fmt(f),
            YulStmt::FunctionDef(fn_def) => fn_def.fmt(f),
        }
    }
}
