pub mod pow;
pub mod ternary;

use crate::kw;

use self::pow::PowOps;

use super::binops::ternary::Ternary;
use syn::{parse::Parse, Error, Token};

#[derive(Debug, Clone)]
pub enum Binop {
    Assign(Token![=]),
    Add(Token![+]),
    AddAssign(Token![+=]),
    Minus(Token![-]),
    MinusAssign(Token![-=]),
    Mul(Token![*]),
    MulAssign(Token![*=]),
    Not(Token![!]),
    BitNot(Token![~]),
    Div(Token![/]),
    DivAssign(Token![/=]),
    Mod(Token![%]),
    ModAssign(Token![%=]),
    BitAnd(Token![&]),
    AndAssign(Token![&=]),
    BitXor(Token![^]),
    XorAssign(Token![^=]),
    BitOr(Token![|]),
    BitOrAssign(Token![|=]),
    Shl(Token![<<]),
    ShlAssign(Token![<<=]),
    Shr(Token![>>]),
    ShrAssign(Token![>>=]),
    Equality(Token![==]),
    And(Token![&&]),
    Or(Token![||]),
    Ternary(Ternary),
    // don't have in rust but swag
    Exponent(PowOps),
    Delete(kw::delete),
}

impl Parse for Binop {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![=]) {
            Ok(Self::Assign(input.parse()?))
        } else if input.peek(Token![~]) {
            Ok(Self::BitNot(input.parse()?))
        } else if input.peek(Token![==]) {
            Ok(Self::Equality(input.parse()?))
        } else if input.peek(Token![+]) {
            Ok(Self::Add(input.parse()?))
        } else if input.peek(Token![+=]) {
            Ok(Self::AddAssign(input.parse()?))
        } else if input.peek(Token![-]) {
            Ok(Self::Minus(input.parse()?))
        } else if input.peek(Token![-=]) {
            Ok(Self::MinusAssign(input.parse()?))
        } else if input.peek(Token![*]) {
            Ok(Self::Mul(input.parse()?))
        } else if input.peek(Token![*=]) {
            Ok(Self::MulAssign(input.parse()?))
        } else if input.peek(Token![/]) {
            Ok(Self::Div(input.parse()?))
        } else if input.peek(Token![/=]) {
            Ok(Self::DivAssign(input.parse()?))
        } else if input.peek(Token![%]) {
            Ok(Self::Mod(input.parse()?))
        } else if input.peek(Token![%=]) {
            Ok(Self::ModAssign(input.parse()?))
        } else if input.peek(Token![&]) {
            Ok(Self::BitAnd(input.parse()?))
        } else if input.peek(Token![&=]) {
            Ok(Self::AndAssign(input.parse()?))
        } else if input.peek(Token![^]) {
            Ok(Self::BitXor(input.parse()?))
        } else if input.peek(Token![^=]) {
            Ok(Self::XorAssign(input.parse()?))
        } else if input.peek(Token![|]) {
            Ok(Self::BitOr(input.parse()?))
        } else if input.peek(Token![|=]) {
            Ok(Self::BitOrAssign(input.parse()?))
        } else if input.peek(Token![<<]) {
            Ok(Self::Shl(input.parse()?))
        } else if input.peek(Token![<<=]) {
            Ok(Self::ShlAssign(input.parse()?))
        } else if input.peek(Token![>>]) {
            Ok(Self::Shr(input.parse()?))
        } else if input.peek(Token![>>=]) {
            Ok(Self::ShrAssign(input.parse()?))
        } else if input.peek(Token![&&]) {
            Ok(Self::And(input.parse()?))
        } else if input.peek(Token![||]) {
            Ok(Self::Or(input.parse()?))
        } else {
            Err(Error::new(input.span(), "failed to parse binop"))
        }
    }
}
