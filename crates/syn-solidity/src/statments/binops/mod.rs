pub mod pow;
pub mod ternary;

use crate::kw;

use self::pow::PowOps;

use super::binops::ternary::Ternary;
use syn::{parse::Parse, Token};

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

// impl Parse for Binop {
//     fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
//             input.peek(Token![~])
//                 || input.peek2(Token!(=))
//                 || input.peek2(Token!(+))
//                 || input.peek2(Token!(+=))
//                 || input.peek2(Token!(-))
//                 || input.peek2(Token!(-=))
//                 || input.peek2(Token!(*))
//                 || input.peek2(Token!(*=))
//                 || input.peek2(Token!(/))
//                 || input.peek2(Token!(/=))
//                 || input.peek2(Token!(%))
//                 || input.peek2(Token!(%=))
//                 || input.peek2(Token!(&))
//                 || input.peek2(Token!(&=))
//                 || input.peek2(Token!(^))
//                 || input.peek2(Token!(^=))
//                 || input.peek2(Token!(|))
//                 || input.peek2(Token!(<<))
//                 || input.peek2(Token!(<<=))
//                 || input.peek2(Token!(>>))
//                 || input.peek2(Token!(>>=))
//                 || input.peek2(Token!(==))
//                 || input.peek2(Token!(&&))
//                 || input.peek2(Token!(||))
//         }
//     }
// }
