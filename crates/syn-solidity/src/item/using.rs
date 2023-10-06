use crate::{kw, SolPath, Spanned, Type};
use proc_macro2::Span;
use std::fmt;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Result, Token,
};

/// A `using` directive: `using { A, B.mul as * } for uint256 global;`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.usingDirective>
#[derive(Clone)]
pub struct UsingDirective {
    pub using_token: kw::using,
    pub list: UsingList,
    pub for_token: Token![for],
    pub ty: UsingType,
    pub global_token: Option<kw::global>,
    pub semi_token: Token![;],
}

impl fmt::Display for UsingDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "using {} for {}{};",
            self.list,
            self.ty,
            if self.global_token.is_some() {
                " global"
            } else {
                ""
            }
        )
    }
}

impl fmt::Debug for UsingDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UsingDirective")
            .field("list", &self.list)
            .field("ty", &self.ty)
            .field("global", &self.global_token.is_some())
            .finish()
    }
}

impl Parse for UsingDirective {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            using_token: input.parse()?,
            list: input.parse()?,
            for_token: input.parse()?,
            ty: input.parse()?,
            global_token: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for UsingDirective {
    fn span(&self) -> Span {
        let span = self.using_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.using_token.span = span;
        self.semi_token.span = span;
    }
}

#[derive(Clone, Debug)]
pub enum UsingList {
    Single(SolPath),
    Multiple(Brace, Punctuated<UsingListItem, Token![,]>),
}

impl fmt::Display for UsingList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(path) => path.fmt(f),
            Self::Multiple(_, list) => {
                f.write_str("{")?;
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    item.fmt(f)?;
                }
                f.write_str("}")
            }
        }
    }
}

impl Parse for UsingList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Brace) {
            let content;
            Ok(Self::Multiple(
                braced!(content in input),
                content.parse_terminated(UsingListItem::parse, Token![,])?,
            ))
        } else {
            input.parse().map(Self::Single)
        }
    }
}

impl Spanned for UsingList {
    fn span(&self) -> Span {
        match self {
            Self::Single(path) => path.span(),
            Self::Multiple(brace, list) => {
                let span = brace.span.join();
                span.join(list.span()).unwrap_or(span)
            }
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Single(path) => path.set_span(span),
            Self::Multiple(brace, list) => {
                *brace = Brace(span);
                list.set_span(span);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UsingListItem {
    pub path: SolPath,
    pub op: Option<(Token![as], UserDefinableOperator)>,
}

impl fmt::Display for UsingListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(f)?;
        if let Some((_, op)) = &self.op {
            write!(f, " as {op}")?;
        }
        Ok(())
    }
}

impl Parse for UsingListItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            path: input.parse()?,
            op: if input.peek(Token![as]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl Spanned for UsingListItem {
    fn span(&self) -> Span {
        self.path.span()
    }

    fn set_span(&mut self, span: Span) {
        self.path.set_span(span);
    }
}

#[derive(Clone, Debug)]
pub enum UsingType {
    Star(Token![*]),
    Type(Type),
}

impl fmt::Display for UsingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Star(_) => f.write_str("*"),
            Self::Type(ty) => ty.fmt(f),
        }
    }
}

impl Parse for UsingType {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Token![*]) {
            input.parse().map(Self::Star)
        } else {
            input.parse().map(Self::Type)
        }
    }
}

impl Spanned for UsingType {
    fn span(&self) -> Span {
        match self {
            Self::Star(star) => star.span,
            Self::Type(ty) => ty.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Star(star) => star.span = span,
            Self::Type(ty) => ty.set_span(span),
        }
    }
}

op_enum! {
    /// A user-definable operator: `+`, `*`, `|`, etc.
    ///
    /// Solidity reference:
    /// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.userDefinableOperator>
    pub enum UserDefinableOperator {
        BitAnd(&),
        BitNot(~),
        BitOr(|),
        BitXor(^),
        Add(+),
        Div(/),
        Rem(%),
        Mul(*),
        Sub(-),
        Eq(==),
        Ge(>=),
        Gt(>),
        Le(<=),
        Lt(<),
        Ne(!=),
    }
}
