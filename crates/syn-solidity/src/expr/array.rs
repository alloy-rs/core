use crate::{
    Expr, Spanned,
    utils::{DebugPunctuated, ParseNested},
};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Bracket,
};

/// An array literal expression: `[a, b, c, d]`.
#[derive(Clone)]
pub struct ExprArray {
    pub bracket_token: Bracket,
    pub elems: Punctuated<Expr, Token![,]>,
}

impl fmt::Debug for ExprArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprArray").field("elems", DebugPunctuated::new(&self.elems)).finish()
    }
}

impl Parse for ExprArray {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            elems: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl fmt::Display for ExprArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            elem.fmt(f)?;
        }
        f.write_str("]")
    }
}

impl Spanned for ExprArray {
    fn span(&self) -> Span {
        self.bracket_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.bracket_token = Bracket(span);
    }
}

/// A square bracketed indexing expression: `vector[2]`.
#[derive(Clone)]
pub struct ExprIndex {
    pub expr: Box<Expr>,
    pub bracket_token: Bracket,
    pub start: Option<Box<Expr>>,
    pub colon_token: Option<Token![:]>,
    pub end: Option<Box<Expr>>,
}

impl fmt::Debug for ExprIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprIndex")
            .field("expr", &self.expr)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl ParseNested for ExprIndex {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let start = if content.is_empty() || content.peek(Token![:]) {
            None
        } else {
            Some(content.parse()?)
        };
        let colon_token = if content.is_empty() { None } else { Some(content.parse()?) };
        let end =
            if content.is_empty() || colon_token.is_none() { None } else { Some(content.parse()?) };
        Ok(Self { expr, bracket_token, start, colon_token, end })
    }
}

derive_parse!(ExprIndex);

impl fmt::Display for ExprIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[", self.expr)?;

        if let Some(start) = &self.start {
            start.fmt(f)?;
        }

        if self.colon_token.is_some() {
            f.write_str(":")?;
            if let Some(end) = &self.end {
                end.fmt(f)?;
            }
        }

        f.write_str("]")
    }
}

impl Spanned for ExprIndex {
    fn span(&self) -> Span {
        let span = self.expr.span();
        span.join(self.bracket_token.span.join()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
        self.bracket_token = Bracket(span);
    }
}

impl ExprIndex {
    pub fn is_range(&self) -> bool {
        self.colon_token.is_some()
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
    fn test_empty_array_literal() {
        let expr = parse_expr("[]");
        assert_eq!(format!("{}", expr), "[]");
    }

    #[test]
    fn test_single_element_array_literal() {
        let expr = parse_expr("[1]");
        assert_eq!(format!("{}", expr), "[1]");

        let expr = parse_expr("[variable]");
        assert_eq!(format!("{}", expr), "[variable]");

        let expr = parse_expr("[\"string\"]");
        assert_eq!(format!("{}", expr), "[\"string\"]");

        let expr = parse_expr("[true]");
        assert_eq!(format!("{}", expr), "[true]");
    }

    #[test]
    fn test_multiple_element_array_literal() {
        let expr = parse_expr("[1, 2, 3]");
        assert_eq!(format!("{}", expr), "[1, 2, 3]");

        let expr = parse_expr("[a, b, c, d]");
        assert_eq!(format!("{}", expr), "[a, b, c, d]");

        let expr = parse_expr("[\"first\", \"second\", \"third\"]");
        assert_eq!(format!("{}", expr), "[\"first\", \"second\", \"third\"]");

        let expr = parse_expr("[true, false, true]");
        assert_eq!(format!("{}", expr), "[true, false, true]");
    }

    #[test]
    fn test_array_literal_with_complex_expressions() {
        let expr = parse_expr("[a + b, c * d]");
        assert_eq!(format!("{}", expr), "[a + b, c * d]");

        let expr = parse_expr("[func(), variable]");
        assert_eq!(format!("{}", expr), "[func(), variable]");

        let expr = parse_expr("[obj.field, array[index]]");
        assert_eq!(format!("{}", expr), "[obj.field, array[index]]");

        let expr = parse_expr("[condition ? true : false, other.value]");
        assert_eq!(format!("{}", expr), "[condition ? true : false, other.value]");
    }

    #[test]
    fn test_nested_array_literals() {
        let expr = parse_expr("[[1, 2], [3, 4]]");
        assert_eq!(format!("{}", expr), "[[1, 2], [3, 4]]");

        let expr = parse_expr("[[], [single], [a, b]]");
        assert_eq!(format!("{}", expr), "[[], [single], [a, b]]");

        let expr = parse_expr("[[[inner]], outer]");
        assert_eq!(format!("{}", expr), "[[[inner]], outer]");

        let expr = parse_expr("[[a, b, c], [d, e, f], [g, h, i]]");
        assert_eq!(format!("{}", expr), "[[a, b, c], [d, e, f], [g, h, i]]");
    }

    #[test]
    fn test_simple_array_indexing() {
        let expr = parse_expr("array[0]");
        assert_eq!(format!("{}", expr), "array[0]");

        let expr = parse_expr("data[index]");
        assert_eq!(format!("{}", expr), "data[index]");

        let expr = parse_expr("values[key]");
        assert_eq!(format!("{}", expr), "values[key]");

        let expr = parse_expr("storage[position]");
        assert_eq!(format!("{}", expr), "storage[position]");
    }

    #[test]
    fn test_multidimensional_array_indexing() {
        let expr = parse_expr("matrix[i][j]");
        assert_eq!(format!("{}", expr), "matrix[i][j]");

        let expr = parse_expr("cube[x][y][z]");
        assert_eq!(format!("{}", expr), "cube[x][y][z]");

        let expr = parse_expr("data[row][col][depth]");
        assert_eq!(format!("{}", expr), "data[row][col][depth]");

        let expr = parse_expr("tensor[a][b][c][d]");
        assert_eq!(format!("{}", expr), "tensor[a][b][c][d]");
    }

    #[test]
    fn test_array_slicing_start_only() {
        let expr = parse_expr("data[1:]");
        assert_eq!(format!("{}", expr), "data[1:]");

        let expr = parse_expr("array[start:]");
        assert_eq!(format!("{}", expr), "array[start:]");

        let expr = parse_expr("values[index:]");
        assert_eq!(format!("{}", expr), "values[index:]");

        let expr = parse_expr("bytes[offset:]");
        assert_eq!(format!("{}", expr), "bytes[offset:]");
    }

    #[test]
    fn test_array_slicing_end_only() {
        let expr = parse_expr("data[:5]");
        assert_eq!(format!("{}", expr), "data[:5]");

        let expr = parse_expr("array[:end]");
        assert_eq!(format!("{}", expr), "array[:end]");

        let expr = parse_expr("values[:limit]");
        assert_eq!(format!("{}", expr), "values[:limit]");

        let expr = parse_expr("bytes[:length]");
        assert_eq!(format!("{}", expr), "bytes[:length]");
    }

    #[test]
    fn test_array_slicing_both_bounds() {
        let expr = parse_expr("data[1:5]");
        assert_eq!(format!("{}", expr), "data[1:5]");

        let expr = parse_expr("array[start:end]");
        assert_eq!(format!("{}", expr), "array[start:end]");

        let expr = parse_expr("values[from:to]");
        assert_eq!(format!("{}", expr), "values[from:to]");

        let expr = parse_expr("bytes[offset:length]");
        assert_eq!(format!("{}", expr), "bytes[offset:length]");
    }

    #[test]
    fn test_array_slicing_full_range() {
        let expr = parse_expr("data[:]");
        assert_eq!(format!("{}", expr), "data[:]");

        let expr = parse_expr("array[:]");
        assert_eq!(format!("{}", expr), "array[:]");

        let expr = parse_expr("values[:]");
        assert_eq!(format!("{}", expr), "values[:]");

        let expr = parse_expr("bytes[:]");
        assert_eq!(format!("{}", expr), "bytes[:]");
    }

    #[test]
    fn test_array_indexing_with_complex_expressions() {
        let expr = parse_expr("array[getIndex()]");
        assert_eq!(format!("{}", expr), "array[getIndex()]");

        let expr = parse_expr("data[i + offset]");
        assert_eq!(format!("{}", expr), "data[i + offset]");

        let expr = parse_expr("matrix[row * cols + col]");
        assert_eq!(format!("{}", expr), "matrix[row * cols + col]");

        let expr = parse_expr("values[condition ? index1 : index2]");
        assert_eq!(format!("{}", expr), "values[condition ? index1 : index2]");
    }

    #[test]
    fn test_mixed_array_operations() {
        let expr = parse_expr("array[getIndex()].field");
        assert_eq!(format!("{}", expr), "array[getIndex()].field");

        let expr = parse_expr("data[i][j].method()");
        assert_eq!(format!("{}", expr), "data[i][j].method()");

        let expr = parse_expr("storage[key].values[index]");
        assert_eq!(format!("{}", expr), "storage[key].values[index]");

        let expr = parse_expr("matrix[row][col] + offset");
        assert_eq!(format!("{}", expr), "matrix[row][col] + offset");
    }

    #[test]
    fn test_array_literal_with_function_calls() {
        let expr = parse_expr("[getValue(), getOther()]");
        assert_eq!(format!("{}", expr), "[getValue(), getOther()]");

        let expr = parse_expr("[func(arg1), func(arg2), func(arg3)]");
        assert_eq!(format!("{}", expr), "[func(arg1), func(arg2), func(arg3)]");

        let expr = parse_expr("[contract.method(), simple.field]");
        assert_eq!(format!("{}", expr), "[contract.method(), simple.field]");

        let expr = parse_expr("[a.b.c(), d.e.f(), g.h.i()]");
        assert_eq!(format!("{}", expr), "[a.b.c(), d.e.f(), g.h.i()]");
    }
}
