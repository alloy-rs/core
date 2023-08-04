use crate::Expr;
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

impl Parse for ExprBinary {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            left: input.parse()?,
            op: input.parse()?,
            right: input.parse()?,
        })
    }
}

impl ExprBinary {
    pub fn span(&self) -> Span {
        let span = self.left.span();
        span.join(self.right.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.left.set_span(span);
        self.right.set_span(span);
    }
}

op_enum! {
    /// A binary operator: `+`, `+=`, `&`.
    pub enum BinOp {
        Add(+),
        Sub(-),
        Mul(*),
        Div(/),
        Rem(%),
        Pow(**),

        Sar(>>>),
        Shr(>>),
        Shl(<<),
        BitAnd(&),
        BitOr(|),
        BitXor(^),

        Lt(<),
        Gt(>),
        Le(<=),
        Ge(>=),
        Eq(==),
        Neq(!=),

        Assign(=),
        AddAssign(+=),
        SubAssign(-=),
        MulAssign(*=),
        DivAssign(/=),
        RemAssign(%=),
        BitAndAssign(&=),
        BitOrAssign(|=),
        BitXorAssign(^=),
        ShlAssign(<<=),
        ShrAssign(>>=),
        SarAssign(>>>=),
    }
}
