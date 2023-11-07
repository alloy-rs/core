use crate::{
    kw, utils::ParseNested, Lit, LitDenominated, SolIdent, Spanned, SubDenomination, Type,
};
use proc_macro2::{Ident, Span};
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
    Result, Token,
};

mod array;
pub use array::{ExprArray, ExprIndex};

mod args;
pub use args::{
    ArgList, ArgListImpl, ExprCall, ExprCallOptions, ExprPayable, NamedArg, NamedArgList,
};

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

/// An expression.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.expression>
#[derive(Clone)]
pub enum Expr {
    /// An array literal expression: `[a, b, c, d]`.
    Array(ExprArray),

    /// A binary operation: `a + b`, `a += b`.
    Binary(ExprBinary),

    /// A function call expression: `foo(42)` or `foo({ bar: 42 })`.
    Call(ExprCall),

    /// Function call options: `foo.bar{ value: 1, gas: 2 }`.
    CallOptions(ExprCallOptions),

    /// A unary `delete` expression: `delete vector`.
    Delete(ExprDelete),

    /// An identifier: `foo`.
    Ident(SolIdent),

    /// A square bracketed indexing expression: `vector[2]`.
    Index(ExprIndex),

    /// A literal: `hex"1234"`.
    Lit(Lit),

    /// A number literal with a sub-denomination: `1 ether`.
    LitDenominated(LitDenominated),

    /// Access of a named member: `obj.k`.
    Member(ExprMember),

    /// A `new` expression: `new Contract`.
    New(ExprNew),

    /// A `payable` expression: `payable(address(0x...))`.
    Payable(ExprPayable),

    /// A postfix unary expression: `foo++`.
    Postfix(ExprPostfix),

    /// A ternary (AKA conditional) expression: `foo ? bar : baz`.
    Ternary(ExprTernary),

    /// A tuple expression: `(a, b, c, d)`.
    Tuple(ExprTuple),

    /// A type name.
    ///
    /// Cannot be `Custom`, as custom identifiers are parsed as `Ident` instead.
    Type(Type),

    /// A `type()` expression: `type(uint256)`
    TypeCall(ExprTypeCall),

    /// A unary operation: `!x`, `-x`.
    Unary(ExprUnary),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Expr::")?;
        match self {
            Self::Array(expr) => expr.fmt(f),
            Self::Binary(expr) => expr.fmt(f),
            Self::Call(expr) => expr.fmt(f),
            Self::CallOptions(expr) => expr.fmt(f),
            Self::Delete(expr) => expr.fmt(f),
            Self::Ident(ident) => ident.fmt(f),
            Self::Index(expr) => expr.fmt(f),
            Self::Lit(lit) => lit.fmt(f),
            Self::LitDenominated(lit) => lit.fmt(f),
            Self::Member(expr) => expr.fmt(f),
            Self::New(expr) => expr.fmt(f),
            Self::Payable(expr) => expr.fmt(f),
            Self::Postfix(expr) => expr.fmt(f),
            Self::Ternary(expr) => expr.fmt(f),
            Self::Tuple(expr) => expr.fmt(f),
            Self::Type(ty) => ty.fmt(f),
            Self::TypeCall(expr) => expr.fmt(f),
            Self::Unary(expr) => expr.fmt(f),
        }
    }
}

impl Parse for Expr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // skip any attributes
        let _ = input.call(syn::Attribute::parse_outer)?;

        debug!("  > Expr: {:?}", input.to_string());
        let mut expr = Self::parse_simple(input)?;
        debug!("  < Expr: {expr:?}");
        loop {
            let (new, cont) = Self::parse_nested(expr, input)?;
            if cont {
                debug!(" << Expr: {new:?}");
                expr = new;
            } else {
                return Ok(new);
            }
        }
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Array(expr) => expr.span(),
            Self::Binary(expr) => expr.span(),
            Self::Call(expr) => expr.span(),
            Self::CallOptions(expr) => expr.span(),
            Self::Delete(expr) => expr.span(),
            Self::Ident(ident) => ident.span(),
            Self::Index(expr) => expr.span(),
            Self::Lit(lit) => lit.span(),
            Self::LitDenominated(lit) => lit.span(),
            Self::Member(expr) => expr.span(),
            Self::New(expr) => expr.span(),
            Self::Payable(expr) => expr.span(),
            Self::Postfix(expr) => expr.span(),
            Self::Ternary(expr) => expr.span(),
            Self::Tuple(expr) => expr.span(),
            Self::Type(ty) => ty.span(),
            Self::TypeCall(expr) => expr.span(),
            Self::Unary(expr) => expr.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Array(expr) => expr.set_span(span),
            Self::Binary(expr) => expr.set_span(span),
            Self::Call(expr) => expr.set_span(span),
            Self::CallOptions(expr) => expr.set_span(span),
            Self::Delete(expr) => expr.set_span(span),
            Self::Ident(ident) => ident.set_span(span),
            Self::Index(expr) => expr.set_span(span),
            Self::Lit(lit) => lit.set_span(span),
            Self::LitDenominated(lit) => lit.set_span(span),
            Self::Member(expr) => expr.set_span(span),
            Self::New(expr) => expr.set_span(span),
            Self::Payable(expr) => expr.set_span(span),
            Self::Postfix(expr) => expr.set_span(span),
            Self::Ternary(expr) => expr.set_span(span),
            Self::Tuple(expr) => expr.set_span(span),
            Self::Type(ty) => ty.set_span(span),
            Self::TypeCall(expr) => expr.set_span(span),
            Self::Unary(expr) => expr.set_span(span),
        }
    }
}

impl Expr {
    fn parse_simple(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Paren) {
            input.parse().map(Self::Tuple)
        } else if lookahead.peek(Bracket) {
            input.parse().map(Self::Array)
        } else if UnOp::peek(input, &lookahead) {
            input.parse().map(Self::Unary)
        } else if Lit::peek(&lookahead) {
            match (input.parse()?, input.call(SubDenomination::parse_opt)?) {
                (Lit::Number(number), Some(denom)) => {
                    Ok(Self::LitDenominated(LitDenominated { number, denom }))
                }
                (lit, None) => Ok(Self::Lit(lit)),
                (_, Some(denom)) => {
                    Err(syn::Error::new(denom.span(), "unexpected subdenomination for literal"))
                }
            }
        } else if lookahead.peek(kw::payable) {
            input.parse().map(Self::Payable)
        } else if lookahead.peek(Token![type]) {
            input.parse().map(Self::TypeCall)
        } else if lookahead.peek(kw::new) {
            input.parse().map(Self::New)
        } else if lookahead.peek(kw::delete) {
            input.parse().map(Self::Delete)
        } else if lookahead.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;
            match Type::parse_ident(ident.clone()) {
                Ok(ty) if !ty.is_custom() => ty.parse_payable(input).map(Self::Type),
                _ => Ok(Self::Ident(ident.into())),
            }
        } else {
            Err(lookahead.error())
        }
    }

    /// Parse an expression that starts with an expression.
    ///
    /// Returns `(ParseResult, continue_parsing)`
    fn parse_nested(expr: Self, input: ParseStream<'_>) -> Result<(Self, bool)> {
        macro_rules! parse {
            (break) => {
                Ok((expr, false))
            };

            ($map:expr) => {
                ParseNested::parse_nested(expr.into(), input).map(|e| ($map(e), true))
            };
        }

        let lookahead = input.lookahead1();
        if lookahead.peek(Bracket) {
            parse!(Self::Index)
        } else if lookahead.peek(Brace) {
            // Special case: `try` stmt block
            if input.peek2(kw::catch) {
                parse!(break)
            } else {
                parse!(Self::CallOptions)
            }
        } else if lookahead.peek(Paren) {
            parse!(Self::Call)
        } else if lookahead.peek(Token![.]) {
            parse!(Self::Member)
        } else if lookahead.peek(Token![?]) {
            parse!(Self::Ternary)
        } else if PostUnOp::peek(input, &lookahead) {
            parse!(Self::Postfix)
        } else if BinOp::peek(input, &lookahead) {
            parse!(Self::Binary)
        } else {
            parse!(break)
        }
    }
}
