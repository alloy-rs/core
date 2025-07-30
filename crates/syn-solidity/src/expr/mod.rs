use crate::{
    Lit, LitDenominated, SolIdent, Spanned, SubDenomination, Type, kw, utils::ParseNested,
};
use proc_macro2::{Ident, Span};
use std::fmt;
use syn::{
    Result, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn peel_parens(&self) -> &Self {
        let mut expr = self;
        while let Some(inner) = expr.peel_paren() {
            expr = inner;
        }
        expr
    }

    fn peel_paren(&self) -> Option<&Self> {
        if let Self::Tuple(t) = self {
            if t.elems.len() == 1 {
                return Some(&t.elems[0]);
            }
        }
        None
    }

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
            let ty = Type::parse_ident(ident.clone()).parse_payable(input);
            if ty.is_custom() { Ok(Self::Ident(ident.into())) } else { Ok(Self::Type(ty)) }
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
            if input.peek2(kw::catch) { parse!(break) } else { parse!(Self::CallOptions) }
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

#[cfg(test)]
mod tests {
    use crate::Expr;
    use syn::parse_str;

    fn parse_expr(input: &str) -> Expr {
        parse_str(input).expect(&format!("Failed to parse: {}", input))
    }

    #[test]
    fn test_display_binary_expression() {
        let expr = parse_expr("a + b");
        assert_eq!(format!("{}", expr), "a + b");

        let expr = parse_expr("x * y");
        assert_eq!(format!("{}", expr), "x * y");

        let expr = parse_expr("left == right");
        assert_eq!(format!("{}", expr), "left == right");

        let expr = parse_expr("value += increment");
        assert_eq!(format!("{}", expr), "value += increment");

        let expr = parse_expr("flag && condition");
        assert_eq!(format!("{}", expr), "flag && condition");

        let expr = parse_expr("a << 2");
        assert_eq!(format!("{}", expr), "a << 2");
    }

    #[test]
    fn test_display_unary_expression() {
        let expr = parse_expr("!flag");
        assert_eq!(format!("{}", expr), "!flag");

        let expr = parse_expr("-value");
        assert_eq!(format!("{}", expr), "-value");

        let expr = parse_expr("~bits");
        assert_eq!(format!("{}", expr), "~bits");

        let expr = parse_expr("++counter");
        assert_eq!(format!("{}", expr), "++counter");

        let expr = parse_expr("--index");
        assert_eq!(format!("{}", expr), "--index");
    }

    #[test]
    fn test_display_delete_expression() {
        let expr = parse_expr("delete myArray");
        assert_eq!(format!("{}", expr), "delete myArray");

        let expr = parse_expr("delete storage.field");
        assert_eq!(format!("{}", expr), "delete storage.field");
    }

    #[test]
    fn test_display_postfix_expression() {
        let expr = parse_expr("counter++");
        assert_eq!(format!("{}", expr), "counter++");

        let expr = parse_expr("index--");
        assert_eq!(format!("{}", expr), "index--");
    }

    #[test]
    fn test_display_member_access() {
        let expr = parse_expr("obj.field");
        assert_eq!(format!("{}", expr), "obj.field");

        let expr = parse_expr("contract.balance");
        assert_eq!(format!("{}", expr), "contract.balance");

        let expr = parse_expr("msg.sender");
        assert_eq!(format!("{}", expr), "msg.sender");
    }

    #[test]
    fn test_display_ternary_expression() {
        let expr = parse_expr("condition ? trueValue : falseValue");
        assert_eq!(format!("{}", expr), "condition ? trueValue : falseValue");

        let expr = parse_expr("x > 0 ? positive : negative");
        assert_eq!(format!("{}", expr), "x > 0 ? positive : negative");
    }

    #[test]
    fn test_display_array_literal() {
        let expr = parse_expr("[1, 2, 3]");
        assert_eq!(format!("{}", expr), "[1, 2, 3]");

        let expr = parse_expr("[]");
        assert_eq!(format!("{}", expr), "[]");

        let expr = parse_expr("[a, b, c, d]");
        assert_eq!(format!("{}", expr), "[a, b, c, d]");
    }

    #[test]
    fn test_display_index_expression() {
        let expr = parse_expr("array[0]");
        assert_eq!(format!("{}", expr), "array[0]");

        let expr = parse_expr("matrix[i][j]");
        assert_eq!(format!("{}", expr), "matrix[i][j]");

        let expr = parse_expr("data[1:]");
        assert_eq!(format!("{}", expr), "data[1:]");

        let expr = parse_expr("data[:5]");
        assert_eq!(format!("{}", expr), "data[:5]");
    }

    #[test]
    fn test_display_tuple_expression() {
        let expr = parse_expr("(a, b, c)");
        assert_eq!(format!("{}", expr), "(a, b, c)");

        let expr = parse_expr("()");
        assert_eq!(format!("{}", expr), "()");

        let expr = parse_expr("(single)");
        assert_eq!(format!("{}", expr), "(single)");
    }

    #[test]
    fn test_display_function_call() {
        let expr = parse_expr("func()");
        assert_eq!(format!("{}", expr), "func()");

        let expr = parse_expr("func(arg1, arg2)");
        assert_eq!(format!("{}", expr), "func(arg1, arg2)");

        let expr = parse_expr("contract.method(param1, param2, param3)");
        assert_eq!(format!("{}", expr), "contract.method(param1, param2, param3)");
    }

    #[test]
    fn test_display_type_call() {
        let expr = parse_expr("type(uint256)");
        assert_eq!(format!("{}", expr), "type(uint256)");

        let expr = parse_expr("type(MyContract)");
        assert_eq!(format!("{}", expr), "type(MyContract)");
    }

    #[test]
    fn test_display_new_expression() {
        let expr = parse_expr("new MyContract");
        assert_eq!(format!("{}", expr), "new MyContract");

        let expr = parse_expr("new uint256[]");
        assert_eq!(format!("{}", expr), "new uint256[]");
    }

    #[test]
    fn test_display_literal() {
        let expr = parse_expr("true");
        assert_eq!(format!("{}", expr), "true");

        let expr = parse_expr("false");
        assert_eq!(format!("{}", expr), "false");

        let expr = parse_expr("42");
        assert_eq!(format!("{}", expr), "42");

        let expr = parse_expr("123456789");
        assert_eq!(format!("{}", expr), "123456789");

        let expr = parse_expr("\"hello world\"");
        assert_eq!(format!("{}", expr), "\"hello world\"");
    }

    #[test]
    fn test_complex_nested_expressions() {
        let expr = parse_expr("a + b * c - d / e");
        assert_eq!(format!("{}", expr), "a + b * c - d / e");

        let expr = parse_expr("outer(inner(value))");
        assert_eq!(format!("{}", expr), "outer(inner(value))");

        let expr = parse_expr("arr[index].method(param)");
        assert_eq!(format!("{}", expr), "arr[index].method(param)");

        let expr = parse_expr("condition ? func(a, b) : other.field");
        assert_eq!(format!("{}", expr), "condition ? func(a, b) : other.field");
    }

    #[test]
    fn test_display_payable_expression() {
        let expr = parse_expr("payable(recipient)");
        assert_eq!(format!("{}", expr), "payable(recipient)");

        let expr = parse_expr("payable(addresses[index])");
        assert_eq!(format!("{}", expr), "payable(addresses[index])");
    }
}
