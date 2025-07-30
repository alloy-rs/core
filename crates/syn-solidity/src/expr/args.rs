use crate::{
    Expr, SolIdent, Spanned, kw,
    utils::{DebugPunctuated, ParseNested},
};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Paren},
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

impl fmt::Display for ExprCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.expr, self.args)
    }
}

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

impl fmt::Display for ExprPayable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "payable{}", self.args)
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

impl fmt::Display for ArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.list)
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

impl fmt::Display for ArgListImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unnamed(list) => {
                for (i, expr) in list.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    expr.fmt(f)?;
                }
                Ok(())
            }
            Self::Named(list) => list.fmt(f),
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

impl fmt::Display for ExprCallOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.expr, self.args)
    }
}

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

impl fmt::Display for NamedArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{ ")?;
        for (i, arg) in self.list.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            arg.fmt(f)?;
        }
        f.write_str(" }")
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

impl fmt::Display for NamedArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.arg)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, SolIdent};
    use proc_macro2::Span;
    use syn::{Token, parse_str, punctuated::Punctuated};

    fn parse_expr(input: &str) -> Expr {
        parse_str(input).expect(&format!("Failed to parse: {}", input))
    }

    #[test]
    fn test_display_empty_arg_list() {
        let arg_list = ArgList {
            paren_token: syn::token::Paren(Span::call_site()),
            list: ArgListImpl::Unnamed(Punctuated::new()),
        };

        assert_eq!(format!("{}", arg_list), "()");
    }

    #[test]
    fn test_display_single_unnamed_arg() {
        let mut args = Punctuated::new();
        args.push(parse_expr("value"));

        let arg_list = ArgList {
            paren_token: syn::token::Paren(Span::call_site()),
            list: ArgListImpl::Unnamed(args),
        };

        assert_eq!(format!("{}", arg_list), "(value)");
    }

    #[test]
    fn test_display_multiple_unnamed_args() {
        let mut args = Punctuated::new();
        args.push(parse_expr("arg1"));
        args.push_punct(Token![,](Span::call_site()));
        args.push(parse_expr("arg2"));
        args.push_punct(Token![,](Span::call_site()));
        args.push(parse_expr("arg3"));

        let arg_list = ArgList {
            paren_token: syn::token::Paren(Span::call_site()),
            list: ArgListImpl::Unnamed(args),
        };

        assert_eq!(format!("{}", arg_list), "(arg1, arg2, arg3)");
    }

    #[test]
    fn test_display_named_arg() {
        let named_arg = NamedArg {
            name: SolIdent::new("key"),
            colon_token: Token![:](Span::call_site()),
            arg: parse_expr("value"),
        };

        assert_eq!(format!("{}", named_arg), "key: value");
    }

    #[test]
    fn test_named_arg_with_complex_value() {
        let named_arg = NamedArg {
            name: SolIdent::new("balance"),
            colon_token: Token![:](Span::call_site()),
            arg: parse_expr("msg.value + existingBalance"),
        };

        assert_eq!(format!("{}", named_arg), "balance: msg.value + existingBalance");
    }

    #[test]
    fn test_display_empty_named_arg_list() {
        let named_arg_list = NamedArgList {
            brace_token: syn::token::Brace(Span::call_site()),
            list: Punctuated::new(),
        };

        assert_eq!(format!("{}", named_arg_list), "{  }");
    }

    #[test]
    fn test_display_single_named_arg_list() {
        let mut list = Punctuated::new();
        let named_arg = NamedArg {
            name: SolIdent::new("value"),
            colon_token: Token![:](Span::call_site()),
            arg: parse_expr("1 ether"),
        };
        list.push(named_arg);

        let named_arg_list =
            NamedArgList { brace_token: syn::token::Brace(Span::call_site()), list };

        assert_eq!(format!("{}", named_arg_list), "{ value: 1 ether }");
    }

    #[test]
    fn test_display_multiple_named_args_list() {
        let mut list = Punctuated::new();

        let arg1 = NamedArg {
            name: SolIdent::new("value"),
            colon_token: Token![:](Span::call_site()),
            arg: parse_expr("msg.value"),
        };
        list.push(arg1);
        list.push_punct(Token![,](Span::call_site()));

        let arg2 = NamedArg {
            name: SolIdent::new("gas"),
            colon_token: Token![:](Span::call_site()),
            arg: parse_expr("gasleft()"),
        };
        list.push(arg2);
        list.push_punct(Token![,](Span::call_site()));

        let arg3 = NamedArg {
            name: SolIdent::new("salt"),
            colon_token: Token![:](Span::call_site()),
            arg: parse_expr("keccak256(data)"),
        };
        list.push(arg3);

        let named_arg_list =
            NamedArgList { brace_token: syn::token::Brace(Span::call_site()), list };

        assert_eq!(
            format!("{}", named_arg_list),
            "{ value: msg.value, gas: gasleft(), salt: keccak256(data) }"
        );
    }

    #[test]
    fn test_display_payable_expression() {
        let expr = parse_expr("payable(recipient)");
        assert_eq!(format!("{}", expr), "payable(recipient)");
    }

    #[test]
    fn test_payable_with_complex_argument() {
        let expr = parse_expr("payable(addresses[index])");
        assert_eq!(format!("{}", expr), "payable(addresses[index])");
    }

    #[test]
    fn test_display_arg_list_impl_unnamed() {
        let mut args = Punctuated::new();
        args.push(parse_expr("first"));
        args.push_punct(Token![,](Span::call_site()));
        args.push(parse_expr("second"));

        let impl_list = ArgListImpl::Unnamed(args);
        assert_eq!(format!("{}", impl_list), "first, second");
    }

    #[test]
    fn test_display_arg_list_impl_empty_unnamed() {
        let args = Punctuated::new();
        let impl_list = ArgListImpl::Unnamed(args);
        assert_eq!(format!("{}", impl_list), "");
    }

    #[test]
    fn test_complex_function_call_arguments() {
        let expr = parse_expr("func(a.b.c, arr[index], condition ? true : false)");
        let expected = "func(a.b.c, arr[index], condition ? true : false)";
        assert_eq!(format!("{}", expr), expected);
    }

    #[test]
    fn test_nested_function_calls_in_arguments() {
        let expr = parse_expr("outer(inner(deep(value)), another())");
        assert_eq!(format!("{}", expr), "outer(inner(deep(value)), another())");
    }
}
