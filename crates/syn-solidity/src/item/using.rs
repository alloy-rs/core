use crate::{kw, SolPath, Type};
use proc_macro2::Span;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Result, Token,
};

/// A `using` directive: `using { A, B.mul as * } for uint256 global;`
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.usingDirective>
#[derive(Clone, Debug)]
pub struct UsingDirective {
    pub using_token: kw::using,
    pub list: UsingList,
    pub for_token: Token![for],
    pub ty: UsingType,
    pub global_token: Option<kw::global>,
    pub semi_token: Token![;],
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

impl UsingDirective {
    pub fn span(&self) -> Span {
        let span = self.using_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.using_token.span = span;
        self.semi_token.span = span;
    }
}

#[derive(Clone, Debug)]
pub enum UsingList {
    Single(SolPath),
    Multiple(Brace, Punctuated<UsingListItem, Token![,]>),
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

#[derive(Clone, Debug)]
pub struct UsingListItem {
    pub path: SolPath,
    pub op: Option<(Token![as], UserDefinableOperator)>,
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

#[derive(Clone, Debug)]
pub enum UsingType {
    Star(Token![*]),
    Type(Type),
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
