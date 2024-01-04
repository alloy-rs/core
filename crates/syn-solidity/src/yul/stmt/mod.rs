use crate::{kw, Spanned, YulFnCall, YulFunctionDef};
use proc_macro2::Span;
use std::fmt;
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

    /// A function definition statement: `function f() { ... }`.
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
            Self::Block(block) => block.span(),
            Self::Decl(decl) => decl.span(),
            Self::Assign(assign) => assign.span(),
            Self::Call(call) => call.span(),
            Self::If(r#if) => r#if.span(),
            Self::For(r#for) => r#for.span(),
            Self::Switch(switch) => switch.span(),
            Self::Leave(leave) => leave.span(),
            Self::Break(r#break) => r#break.span(),
            Self::Continue(r#continue) => r#continue.span(),
            Self::FunctionDef(fn_def) => fn_def.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Block(block) => block.set_span(span),
            Self::Decl(decl) => decl.set_span(span),
            Self::Assign(assign) => assign.set_span(span),
            Self::Call(call) => call.set_span(span),
            Self::If(r#if) => r#if.set_span(span),
            Self::For(r#for) => r#for.set_span(span),
            Self::Switch(switch) => switch.set_span(span),
            Self::Leave(leave) => leave.set_span(span),
            Self::Break(r#break) => r#break.set_span(span),
            Self::Continue(r#continue) => r#continue.set_span(span),
            Self::FunctionDef(fn_def) => fn_def.set_span(span),
        }
    }
}

impl fmt::Debug for YulStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulStmt::")?;
        match self {
            Self::Block(block) => block.fmt(f),
            Self::Decl(decl) => decl.fmt(f),
            Self::Assign(assign) => assign.fmt(f),
            Self::Call(call) => call.fmt(f),
            Self::If(r#if) => r#if.fmt(f),
            Self::For(r#for) => r#for.fmt(f),
            Self::Switch(switch) => switch.fmt(f),
            Self::Leave(leave) => leave.fmt(f),
            Self::Break(r#break) => r#break.fmt(f),
            Self::Continue(r#continue) => r#continue.fmt(f),
            Self::FunctionDef(fn_def) => fn_def.fmt(f),
        }
    }
}
