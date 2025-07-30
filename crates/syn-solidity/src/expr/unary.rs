use crate::{Expr, Spanned, kw, utils::ParseNested};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// A unary operation: `!x`, `-x`.
#[derive(Clone, Debug)]
pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl Parse for ExprUnary {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { op: input.parse()?, expr: input.parse()? })
    }
}

impl fmt::Display for ExprUnary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.op, self.expr)
    }
}

impl Spanned for ExprUnary {
    fn span(&self) -> Span {
        let span = self.op.span();
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.op.set_span(span);
        self.expr.set_span(span);
    }
}

/// A unary `delete` expression: `delete vector`.
#[derive(Clone)]
pub struct ExprDelete {
    pub delete_token: kw::delete,
    pub expr: Box<Expr>,
}

impl fmt::Debug for ExprDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprDelete").field("expr", &self.expr).finish()
    }
}

impl Parse for ExprDelete {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { delete_token: input.parse()?, expr: input.parse()? })
    }
}

impl fmt::Display for ExprDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "delete {}", self.expr)
    }
}

impl Spanned for ExprDelete {
    fn span(&self) -> Span {
        let span = self.delete_token.span;
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.delete_token.span = span;
        self.expr.set_span(span);
    }
}

/// A postfix unary expression: `foo++`.
#[derive(Clone, Debug)]
pub struct ExprPostfix {
    pub expr: Box<Expr>,
    pub op: PostUnOp,
}

impl ParseNested for ExprPostfix {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { expr, op: input.parse()? })
    }
}

derive_parse!(ExprPostfix);

impl fmt::Display for ExprPostfix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.expr, self.op)
    }
}

impl Spanned for ExprPostfix {
    fn span(&self) -> Span {
        let span = self.op.span();
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.op.set_span(span);
        self.expr.set_span(span);
    }
}

op_enum! {
    /// Unary operators.
    pub enum UnOp {
        Increment(++) peek2,
        Decrement(--) peek2,
        Not(!),
        BitNot(~),
        Neg(-),
    }
}

op_enum! {
    /// Postfix unary operators.
    pub enum PostUnOp {
        Increment(++) peek2,
        Decrement(--) peek2,
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
    fn test_display_unary_prefix_expressions() {
        // expr "!flag" should display "!flag"
        let expr = parse_expr("!flag");
        assert_eq!(format!("{}", expr), "!flag");

        // expr "-value" should display "-value"
        let expr = parse_expr("-value");
        assert_eq!(format!("{}", expr), "-value");

        // expr "~bits" should display "~bits"
        let expr = parse_expr("~bits");
        assert_eq!(format!("{}", expr), "~bits");

        // expr "++counter" should display "++counter"
        let expr = parse_expr("++counter");
        assert_eq!(format!("{}", expr), "++counter");

        // expr "--index" should display "--index"
        let expr = parse_expr("--index");
        assert_eq!(format!("{}", expr), "--index");
    }

    #[test]
    fn test_display_postfix_expressions() {
        // expr "counter++" should display "counter++"
        let expr = parse_expr("counter++");
        assert_eq!(format!("{}", expr), "counter++");

        // expr "index--" should display "index--"
        let expr = parse_expr("index--");
        assert_eq!(format!("{}", expr), "index--");

        // expr "variable++" should display "variable++"
        let expr = parse_expr("variable++");
        assert_eq!(format!("{}", expr), "variable++");

        // expr "array[i]++" should display "array[i]++"
        let expr = parse_expr("array[i]++");
        assert_eq!(format!("{}", expr), "array[i]++");
    }

    #[test]
    fn test_display_delete_expressions() {
        // expr "delete myArray" should display "delete myArray"
        let expr = parse_expr("delete myArray");
        assert_eq!(format!("{}", expr), "delete myArray");

        // expr "delete storage.field" should display "delete storage.field"
        let expr = parse_expr("delete storage.field");
        assert_eq!(format!("{}", expr), "delete storage.field");

        // expr "delete mapping[key]" should display "delete mapping[key]"
        let expr = parse_expr("delete mapping[key]");
        assert_eq!(format!("{}", expr), "delete mapping[key]");

        // expr "delete data" should display "delete data"
        let expr = parse_expr("delete data");
        assert_eq!(format!("{}", expr), "delete data");
    }

    #[test]
    fn test_complex_unary_expressions() {
        let expr = parse_expr("!condition && flag");
        assert_eq!(format!("{}", expr), "!condition && flag");

        let expr = parse_expr("-(a + b)");
        assert_eq!(format!("{}", expr), "-(a + b)");

        let expr = parse_expr("~value & mask");
        assert_eq!(format!("{}", expr), "~value & mask");

        let expr = parse_expr("++array[index]");
        assert_eq!(format!("{}", expr), "++array[index]");
    }

    #[test]
    fn test_nested_unary_expressions() {
        let expr = parse_expr("!!flag");
        assert_eq!(format!("{}", expr), "!!flag");

        let expr = parse_expr("--counter");
        assert_eq!(format!("{}", expr), "--counter");

        let expr = parse_expr("~-value");
        assert_eq!(format!("{}", expr), "~-value");

        let expr = parse_expr("!~bits");
        assert_eq!(format!("{}", expr), "!~bits");
    }

    #[test]
    fn test_unary_with_member_access() {
        let expr = parse_expr("!obj.flag");
        assert_eq!(format!("{}", expr), "!obj.flag");

        let expr = parse_expr("-contract.balance");
        assert_eq!(format!("{}", expr), "-contract.balance");

        let expr = parse_expr("msg.sender++");
        assert_eq!(format!("{}", expr), "msg.sender++");

        let expr = parse_expr("delete storage.data");
        assert_eq!(format!("{}", expr), "delete storage.data");
    }

    #[test]
    fn test_unary_with_function_calls() {
        let expr = parse_expr("!isValid()");
        assert_eq!(format!("{}", expr), "!isValid()");

        let expr = parse_expr("-getValue()");
        assert_eq!(format!("{}", expr), "-getValue()");

        let expr = parse_expr("getCounter()++");
        assert_eq!(format!("{}", expr), "getCounter()++");

        let expr = parse_expr("delete getMapping()");
        assert_eq!(format!("{}", expr), "delete getMapping()");
    }
}
