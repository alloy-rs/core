use crate::{Expr, Spanned, utils::ParseNested};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// A binary operation: `a + b`, `a += b`.
#[derive(Clone, Debug)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

impl ParseNested for ExprBinary {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { left: expr, op: input.parse()?, right: input.parse()? })
    }
}

derive_parse!(ExprBinary);

impl fmt::Display for ExprBinary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

impl Spanned for ExprBinary {
    fn span(&self) -> Span {
        let span = self.left.span();
        span.join(self.right.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.left.set_span(span);
        self.right.set_span(span);
    }
}

op_enum! {
    /// A binary operator: `+`, `+=`, `&`.
    pub enum BinOp {
        Le(<=),
        Ge(>=),
        Eq(==),
        Neq(!=),
        Or(||),
        And(&&),

        Assign(=),
        AddAssign(+=),
        SubAssign(-=),
        MulAssign(*=),
        DivAssign(/=),
        RemAssign(%=),
        BitAndAssign(&=),
        BitOrAssign(|=),
        BitXorAssign(^=),
        SarAssign(>>>=) peek3,
        ShlAssign(<<=),
        ShrAssign(>>=),

        Sar(>>>) peek3,
        Shr(>>),
        Shl(<<),
        BitAnd(&),
        BitOr(|),
        BitXor(^),

        Lt(<),
        Gt(>),

        Add(+),
        Sub(-),
        Pow(**) peek2,
        Mul(*),
        Div(/),
        Rem(%),
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
    fn test_display_arithmetic_binary_expressions() {
        // expr "a + b" should display "a + b"
        let expr = parse_expr("a + b");
        assert_eq!(format!("{}", expr), "a + b");

        // expr "x - y" should display "x - y"
        let expr = parse_expr("x - y");
        assert_eq!(format!("{}", expr), "x - y");

        // expr "a * b" should display "a * b"
        let expr = parse_expr("a * b");
        assert_eq!(format!("{}", expr), "a * b");

        // expr "x / y" should display "x / y"
        let expr = parse_expr("x / y");
        assert_eq!(format!("{}", expr), "x / y");

        // expr "a % b" should display "a % b"
        let expr = parse_expr("a % b");
        assert_eq!(format!("{}", expr), "a % b");

        // expr "base ** exponent" should display "base ** exponent"
        let expr = parse_expr("base ** exponent");
        assert_eq!(format!("{}", expr), "base ** exponent");
    }

    #[test]
    fn test_display_comparison_binary_expressions() {
        // expr "a < b" should display "a < b"
        let expr = parse_expr("a < b");
        assert_eq!(format!("{}", expr), "a < b");

        // expr "x > y" should display "x > y"
        let expr = parse_expr("x > y");
        assert_eq!(format!("{}", expr), "x > y");

        // expr "a <= b" should display "a <= b"
        let expr = parse_expr("a <= b");
        assert_eq!(format!("{}", expr), "a <= b");

        // expr "x >= y" should display "x >= y"
        let expr = parse_expr("x >= y");
        assert_eq!(format!("{}", expr), "x >= y");

        // expr "a == b" should display "a == b"
        let expr = parse_expr("a == b");
        assert_eq!(format!("{}", expr), "a == b");

        // expr "x != y" should display "x != y"
        let expr = parse_expr("x != y");
        assert_eq!(format!("{}", expr), "x != y");
    }

    #[test]
    fn test_display_logical_binary_expressions() {
        // expr "a && b" should display "a && b"
        let expr = parse_expr("a && b");
        assert_eq!(format!("{}", expr), "a && b");

        // expr "x || y" should display "x || y"
        let expr = parse_expr("x || y");
        assert_eq!(format!("{}", expr), "x || y");
    }

    #[test]
    fn test_display_bitwise_binary_expressions() {
        // expr "a & b" should display "a & b"
        let expr = parse_expr("a & b");
        assert_eq!(format!("{}", expr), "a & b");

        // expr "x | y" should display "x | y"
        let expr = parse_expr("x | y");
        assert_eq!(format!("{}", expr), "x | y");

        // expr "a ^ b" should display "a ^ b"
        let expr = parse_expr("a ^ b");
        assert_eq!(format!("{}", expr), "a ^ b");

        // expr "x << 2" should display "x << 2"
        let expr = parse_expr("x << 2");
        assert_eq!(format!("{}", expr), "x << 2");

        // expr "y >> 3" should display "y >> 3"
        let expr = parse_expr("y >> 3");
        assert_eq!(format!("{}", expr), "y >> 3");

        // expr "z >>> 1" should display "z >>> 1"
        let expr = parse_expr("z >>> 1");
        assert_eq!(format!("{}", expr), "z >>> 1");
    }

    #[test]
    fn test_display_assignment_binary_expressions() {
        // expr "a = b" should display "a = b"
        let expr = parse_expr("a = b");
        assert_eq!(format!("{}", expr), "a = b");

        // expr "x += y" should display "x += y"
        let expr = parse_expr("x += y");
        assert_eq!(format!("{}", expr), "x += y");

        // expr "a -= b" should display "a -= b"
        let expr = parse_expr("a -= b");
        assert_eq!(format!("{}", expr), "a -= b");

        // expr "x *= y" should display "x *= y"
        let expr = parse_expr("x *= y");
        assert_eq!(format!("{}", expr), "x *= y");

        // expr "a /= b" should display "a /= b"
        let expr = parse_expr("a /= b");
        assert_eq!(format!("{}", expr), "a /= b");

        // expr "x %= y" should display "x %= y"
        let expr = parse_expr("x %= y");
        assert_eq!(format!("{}", expr), "x %= y");

        // expr "a &= b" should display "a &= b"
        let expr = parse_expr("a &= b");
        assert_eq!(format!("{}", expr), "a &= b");

        // expr "x |= y" should display "x |= y"
        let expr = parse_expr("x |= y");
        assert_eq!(format!("{}", expr), "x |= y");

        // expr "a ^= b" should display "a ^= b"
        let expr = parse_expr("a ^= b");
        assert_eq!(format!("{}", expr), "a ^= b");

        // expr "x <<= 2" should display "x <<= 2"
        let expr = parse_expr("x <<= 2");
        assert_eq!(format!("{}", expr), "x <<= 2");

        // expr "y >>= 3" should display "y >>= 3"
        let expr = parse_expr("y >>= 3");
        assert_eq!(format!("{}", expr), "y >>= 3");

        // expr "z >>>= 1" should display "z >>>= 1"
        let expr = parse_expr("z >>>= 1");
        assert_eq!(format!("{}", expr), "z >>>= 1");
    }

    #[test]
    fn test_display_complex_binary_expressions_with_precedence() {
        // expr "a + b * c" should display "a + b * c" (preserving precedence)
        let expr = parse_expr("a + b * c");
        assert_eq!(format!("{}", expr), "a + b * c");

        // expr "(x + y) * z" should display "(x + y) * z" (preserving parentheses)
        let expr = parse_expr("(x + y) * z");
        assert_eq!(format!("{}", expr), "(x + y) * z");

        // expr "balance >= amount && msg.sender == owner" should display correctly
        let expr = parse_expr("balance >= amount && msg.sender == owner");
        assert_eq!(format!("{}", expr), "balance >= amount && msg.sender == owner");

        // expr "value << 8 | mask" should display "value << 8 | mask"
        let expr = parse_expr("value << 8 | mask");
        assert_eq!(format!("{}", expr), "value << 8 | mask");
    }

    #[test]
    fn test_display_nested_binary_expressions_with_associativity() {
        // expr "a + b + c" should display "a + b + c" (left-associative)
        let expr = parse_expr("a + b + c");
        assert_eq!(format!("{}", expr), "a + b + c");

        // expr "x && y && z" should display "x && y && z" (left-associative)
        let expr = parse_expr("x && y && z");
        assert_eq!(format!("{}", expr), "x && y && z");

        // expr "a == b && c != d" should display "a == b && c != d"
        let expr = parse_expr("a == b && c != d");
        assert_eq!(format!("{}", expr), "a == b && c != d");
    }
}
