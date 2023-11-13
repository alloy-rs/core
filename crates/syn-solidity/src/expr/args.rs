use crate::{
    kw,
    utils::{DebugPunctuated, ParseNested},
    Expr, SolIdent, Spanned,
};
use proc_macro2::Span;
use std::fmt;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Paren},
    Result, Token,
};

/// A function call expression: `foo(42)` or `foo({ bar: 42 })`.
#[derive(Clone, Debug)]
pub struct ExprCall {
    pub expr: Box<Expr>,
    pub args: ArgList,
}

impl ParseNested for ExprCall {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { expr, args: input.parse()? })
    }
}

derive_parse!(ExprCall);

impl Spanned for ExprCall {
    fn span(&self) -> Span {
        let span = self.expr.span();
        span.join(self.args.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
        self.args.set_span(span);
    }
}

/// A `payable` expression: `payable(address(0x...))`.
#[derive(Clone)]
pub struct ExprPayable {
    pub payable_token: kw::payable,
    pub args: ArgList,
}

impl fmt::Debug for ExprPayable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprPayable").field("args", &self.args).finish()
    }
}

impl From<ExprPayable> for ExprCall {
    fn from(value: ExprPayable) -> Self {
        Self {
            expr: Box::new(Expr::Ident(SolIdent::new_spanned("payable", value.payable_token.span))),
            args: value.args,
        }
    }
}

impl Parse for ExprPayable {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { payable_token: input.parse()?, args: input.parse()? })
    }
}

impl Spanned for ExprPayable {
    fn span(&self) -> Span {
        let span = self.payable_token.span;
        span.join(self.args.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.payable_token.span = span;
        self.args.set_span(span);
    }
}

/// A list of named or unnamed arguments: `{ foo: 42, bar: 64 }` or `(42, 64)`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.callArgumentList>
#[derive(Clone)]
pub struct ArgList {
    pub paren_token: Paren,
    /// The list of arguments. Can be named or unnamed.
    ///
    /// When empty, this is an empty unnamed list.
    pub list: ArgListImpl,
}

impl fmt::Debug for ArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArgList").field("list", &self.list).finish()
    }
}

impl Parse for ArgList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self { paren_token: parenthesized!(content in input), list: content.parse()? })
    }
}

impl Spanned for ArgList {
    fn span(&self) -> Span {
        self.paren_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.paren_token = Paren(span);
    }
}

/// A list of either unnamed or named arguments.
#[derive(Clone)]
pub enum ArgListImpl {
    Unnamed(Punctuated<Expr, Token![,]>),
    Named(NamedArgList),
}

impl fmt::Debug for ArgListImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unnamed(list) => {
                f.debug_tuple("Unnamed").field(DebugPunctuated::new(list)).finish()
            }
            Self::Named(list) => f.debug_tuple("Named").field(list).finish(),
        }
    }
}

impl Parse for ArgListImpl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Brace) {
            input.parse().map(Self::Named)
        } else {
            input.parse_terminated(Expr::parse, Token![,]).map(Self::Unnamed)
        }
    }
}

/// Function call options: `foo.bar{ value: 1, gas: 2 }`.
#[derive(Clone, Debug)]
pub struct ExprCallOptions {
    pub expr: Box<Expr>,
    pub args: NamedArgList,
}

impl ParseNested for ExprCallOptions {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { expr, args: input.parse()? })
    }
}

derive_parse!(ExprCallOptions);

impl Spanned for ExprCallOptions {
    fn span(&self) -> Span {
        let span = self.expr.span();
        span.join(self.args.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
        self.args.set_span(span);
    }
}

/// A named argument list: `{ foo: uint256(42), bar: true }`.
#[derive(Clone)]
pub struct NamedArgList {
    pub brace_token: Brace,
    pub list: Punctuated<NamedArg, Token![,]>,
}

impl fmt::Debug for NamedArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedArgList").field("list", DebugPunctuated::new(&self.list)).finish()
    }
}

impl Parse for NamedArgList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            list: content.parse_terminated(NamedArg::parse, Token![,])?,
        })
    }
}

impl Spanned for NamedArgList {
    fn span(&self) -> Span {
        self.brace_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.brace_token = Brace(span);
    }
}

/// A named argument in an argument list: `foo: uint256(42)`.
#[derive(Clone)]
pub struct NamedArg {
    pub name: SolIdent,
    pub colon_token: Token![:],
    pub arg: Expr,
}

impl fmt::Debug for NamedArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedArg").field("name", &self.name).field("arg", &self.arg).finish()
    }
}

impl Parse for NamedArg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { name: input.parse()?, colon_token: input.parse()?, arg: input.parse()? })
    }
}

impl Spanned for NamedArg {
    fn span(&self) -> Span {
        let span = self.name.span();
        span.join(self.arg.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
        self.arg.set_span(span);
    }
}
