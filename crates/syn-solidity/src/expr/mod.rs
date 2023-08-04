use proc_macro2::{Ident, Span};
use syn::{
    ext::IdentExt,
    parse::{discouraged::Speculative, Parse, ParseStream},
    token::{Brace, Bracket, Paren},
    Result, Token,
};

mod array;
pub use array::{ExprArray, ExprIndex};

mod args;
pub use args::{ArgList, ArgListImpl, ExprCall, ExprPayable, ExprStruct, NamedArg, NamedArgList};

mod binary;
pub use binary::{BinOp, ExprBinary};

mod member;
pub use member::ExprMember;

mod ternary;
pub use ternary::ExprTernary;

mod tuple;
pub use tuple::ExprTuple;

mod r#type;
pub use r#type::{ExprNew, ExprTypeCall};

mod unary;
pub use unary::{ExprDelete, ExprPostfix, ExprUnary, PostUnOp, UnOp};

use crate::{kw, Lit, SolIdent, Type};

/// An expression.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.expression>
#[derive(Clone, Debug)]
pub enum Expr {
    /// An array literal expression: `[a, b, c, d]`.
    Array(ExprArray),

    /// A binary operation: `a + b`, `a += b`.
    Binary(ExprBinary),

    /// A function call expression: `foo(42)` or `foo({ bar: 42 })`.
    Call(ExprCall),

    /// A unary `delete` expression: `delete vector`.
    Delete(ExprDelete),

    /// An identifier: `foo`.
    Ident(SolIdent),

    /// A square bracketed indexing expression: `vector[2]`.
    Index(ExprIndex),

    /// A literal: `hex"1234"`.
    Lit(Lit),

    /// Access of a named member: `obj.k`.
    Member(ExprMember),

    /// A `new` expression: `new Contract`.
    New(ExprNew),

    /// A `payable` expression: `payable(address(0x...))`.
    Payable(ExprPayable),

    /// A postfix unary expression: `foo++`.
    Postfix(ExprPostfix),

    /// A struct expression: `Foo { bar: 1, baz: 2 }`.
    Struct(ExprStruct),

    /// A ternary (AKA conditional) expression: `foo ? bar : baz`.
    Ternary(ExprTernary),

    /// A tuple expression: `(a, b, c, d)`.
    Tuple(ExprTuple),

    /// A type name.
    Type(Type),

    /// A `type()` expression: `type(uint256)`
    TypeCall(ExprTypeCall),

    /// A unary operation: `!x`, `*x`.
    Unary(ExprUnary),
}

impl Parse for Expr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Paren) {
            // TODO: tuple type?
            input.parse().map(Self::Tuple)
        } else if lookahead.peek(Bracket) {
            input.parse().map(Self::Array)
        } else if Lit::peek(&lookahead) {
            input.parse().map(Self::Lit)
        } else if lookahead.peek(kw::payable) {
            input.parse().map(Self::Payable)
        } else if lookahead.peek(Token![type]) {
            input.parse().map(Self::TypeCall)
        } else if lookahead.peek(kw::new) {
            input.parse().map(Self::New)
        } else if lookahead.peek(kw::delete) {
            input.parse().map(Self::Delete)
        } else if lookahead.peek(Ident::peek_any) {
            let fork = input.fork();
            match fork.parse() {
                Ok(ty) => {
                    input.advance_to(&fork);
                    Ok(Self::Type(ty))
                }
                Err(_) => input.parse().map(Self::Ident),
            }
        } else if UnOp::peek(input, &lookahead) {
            input.parse().map(Self::Unary)
        } else {
            let fork = input.fork();
            match input.parse::<Self>() {
                Ok(_) => Self::parse2(input, &fork),
                Err(_) => Err(lookahead.error()),
            }
        }
    }
}

impl Expr {
    /// Parse an expression that starts with an expression.
    fn parse2(input: ParseStream<'_>, start: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Bracket) {
            start.parse().map(Self::Index)
        } else if lookahead.peek(Brace) {
            start.parse().map(Self::Struct)
        } else if lookahead.peek(Paren) {
            start.parse().map(Self::Call)
        } else if lookahead.peek(Token![.]) {
            start.parse().map(Self::Member)
        } else if lookahead.peek(Token![?]) {
            start.parse().map(Self::Ternary)
        } else if PostUnOp::peek(input, &lookahead) {
            start.parse().map(Self::Postfix)
        } else if BinOp::peek(input, &lookahead) {
            start.parse().map(Self::Binary)
        } else {
            Err(lookahead.error())
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Index(expr) => expr.span(),
            Self::Member(expr) => expr.span(),
            Self::Struct(expr) => expr.span(),
            Self::Call(expr) => expr.span(),
            Self::Payable(expr) => expr.span(),
            Self::TypeCall(expr) => expr.span(),
            Self::Unary(expr) => expr.span(),
            Self::Binary(expr) => expr.span(),
            Self::Ternary(expr) => expr.span(),
            Self::Postfix(expr) => expr.span(),
            Self::New(expr) => expr.span(),
            Self::Delete(expr) => expr.span(),
            Self::Tuple(expr) => expr.span(),
            Self::Array(expr) => expr.span(),
            Self::Ident(expr) => expr.span(),
            Self::Lit(expr) => expr.span(),
            Self::Type(expr) => expr.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Index(expr) => expr.set_span(span),
            Self::Member(expr) => expr.set_span(span),
            Self::Struct(expr) => expr.set_span(span),
            Self::Call(expr) => expr.set_span(span),
            Self::Payable(expr) => expr.set_span(span),
            Self::TypeCall(expr) => expr.set_span(span),
            Self::Unary(expr) => expr.set_span(span),
            Self::Binary(expr) => expr.set_span(span),
            Self::Ternary(expr) => expr.set_span(span),
            Self::Postfix(expr) => expr.set_span(span),
            Self::New(expr) => expr.set_span(span),
            Self::Delete(expr) => expr.set_span(span),
            Self::Tuple(expr) => expr.set_span(span),
            Self::Array(expr) => expr.set_span(span),
            Self::Ident(expr) => expr.set_span(span),
            Self::Lit(expr) => expr.set_span(span),
            Self::Type(expr) => expr.set_span(span),
        }
    }
}
