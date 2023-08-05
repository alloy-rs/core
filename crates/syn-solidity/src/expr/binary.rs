use crate::{utils::ParseNested, Expr, Spanned};
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Result,
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
        Ok(Self {
            left: expr,
            op: input.parse()?,
            right: input.parse()?,
        })
    }
}

derive_parse!(ExprBinary);

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
