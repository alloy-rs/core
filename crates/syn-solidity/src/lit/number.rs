use crate::{Spanned, kw};
use proc_macro2::{Literal, Span};
use std::{fmt, str::FromStr};
use syn::{
    LitFloat, LitInt, Result,
    parse::{Lookahead1, Parse, ParseStream},
};

// TODO: Fixed point numbers

/// An integer or fixed-point number literal: `1` or `1.0`.
#[derive(Clone)]
pub enum LitNumber {
    Int(LitInt),
    Float(LitFloat),
}

impl fmt::Display for LitNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(lit) => write!(f, "{}", lit.base10_digits()),
            Self::Float(lit) => write!(f, "{}", lit.base10_digits()),
        }
    }
}

impl fmt::Debug for LitNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(lit) => lit.fmt(f),
            Self::Float(lit) => lit.fmt(f),
        }
    }
}

impl Parse for LitNumber {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else if lookahead.peek(LitFloat) {
            input.parse().map(Self::Float)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Spanned for LitNumber {
    fn span(&self) -> Span {
        match self {
            Self::Int(lit) => lit.span(),
            Self::Float(lit) => lit.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Int(lit) => lit.set_span(span),
            Self::Float(lit) => lit.set_span(span),
        }
    }
}

impl LitNumber {
    pub fn new_int(repr: &str, span: Span) -> Self {
        Self::Int(LitInt::new(repr, span))
    }

    pub fn new_fixed(repr: &str, span: Span) -> Self {
        Self::Float(LitFloat::new(repr, span))
    }

    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(LitInt) || lookahead.peek(LitFloat)
    }

    /// Returns the base-10 digits of the literal.
    pub fn base10_digits(&self) -> &str {
        match self {
            Self::Int(lit) => lit.base10_digits(),
            Self::Float(lit) => lit.base10_digits(),
        }
    }

    /// Parses the literal into a selected number type.
    ///
    /// This is equivalent to `lit.base10_digits().parse()` except that the
    /// resulting errors will be correctly spanned to point to the literal token
    /// in the macro input.
    pub fn base10_parse<N>(&self) -> Result<N>
    where
        N: FromStr,
        N::Err: fmt::Display,
    {
        match self {
            Self::Int(lit) => lit.base10_parse(),
            Self::Float(lit) => lit.base10_parse(),
        }
    }

    pub fn suffix(&self) -> &str {
        match self {
            Self::Int(lit) => lit.suffix(),
            Self::Float(lit) => lit.suffix(),
        }
    }

    pub fn token(&self) -> Literal {
        match self {
            Self::Int(lit) => lit.token(),
            Self::Float(lit) => lit.token(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LitDenominated {
    pub number: LitNumber,
    pub denom: SubDenomination,
}

impl Parse for LitDenominated {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { number: input.parse()?, denom: input.parse()? })
    }
}

impl fmt::Display for LitDenominated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.denom)
    }
}

impl Spanned for LitDenominated {
    fn span(&self) -> Span {
        let span = self.number.span();
        span.join(self.denom.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.number.set_span(span);
        self.denom.set_span(span);
    }
}

kw_enum! {
    /// A sub-denomination suffix for a number literal.
    pub enum SubDenomination {
        Wei(kw::wei),
        Gwei(kw::gwei),
        Ether(kw::ether),

        Seconds(kw::seconds),
        Minutes(kw::minutes),
        Hours(kw::hours),
        Days(kw::days),
        Weeks(kw::weeks),
        Years(kw::years),
    }
}

impl SubDenomination {
    /// Returns the value of this sub-denomination.
    pub const fn value(self) -> u64 {
        // https://github.com/ethereum/solidity/blob/2a2a9d37ee69ca77ef530fe18524a3dc8b053104/libsolidity/ast/Types.cpp#L973
        match self {
            Self::Wei(..) => 1,
            Self::Gwei(..) => 1_000_000_000,
            Self::Ether(..) => 1_000_000_000_000_000_000,

            Self::Seconds(..) => 1,
            Self::Minutes(..) => 60,
            Self::Hours(..) => 3_600,
            Self::Days(..) => 86_400,
            Self::Weeks(..) => 604_800,
            Self::Years(..) => 31_536_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::{LitFloat, LitInt};

    #[test]
    fn test_display_lit_number_int() {
        let lit_int = LitInt::new("42", Span::call_site());
        let number = LitNumber::Int(lit_int);
        assert_eq!(format!("{}", number), "42");

        let lit_int = LitInt::new("0", Span::call_site());
        let number = LitNumber::Int(lit_int);
        assert_eq!(format!("{}", number), "0");

        let lit_int = LitInt::new("123456789", Span::call_site());
        let number = LitNumber::Int(lit_int);
        assert_eq!(format!("{}", number), "123456789");
    }

    #[test]
    fn test_display_lit_number_float() {
        let lit_float = LitFloat::new("3.14", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "3.14");

        let lit_float = LitFloat::new("0.0", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "0.0");

        let lit_float = LitFloat::new("123.456", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "123.456");
    }

    #[test]
    fn test_lit_number_scientific_notation() {
        let lit_float = LitFloat::new("1e18", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "1e18");

        let lit_float = LitFloat::new("2.5e-3", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "2.5e-3");

        let lit_float = LitFloat::new("6.022e23", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "6.022e23");
    }

    #[test]
    fn test_lit_number_hex_and_other_formats() {
        let lit_int = LitInt::new("0x42", Span::call_site());
        let number = LitNumber::Int(lit_int);
        assert_eq!(format!("{}", number), "66"); // base10_digits() converts to decimal

        let lit_int = LitInt::new("0b1010", Span::call_site());
        let number = LitNumber::Int(lit_int);
        assert_eq!(format!("{}", number), "10"); // base10_digits() converts to decimal

        let lit_int = LitInt::new("0o755", Span::call_site());
        let number = LitNumber::Int(lit_int);
        assert_eq!(format!("{}", number), "493"); // base10_digits() converts to decimal
    }

    #[test]
    fn test_lit_denominated_ether_units() {
        let lit_int = LitInt::new("1", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Ether(kw::ether(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "1 ether");
    }

    #[test]
    fn test_lit_denominated_wei_units() {
        let lit_int = LitInt::new("1000000000000000000", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Wei(kw::wei(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "1000000000000000000 wei");
    }

    #[test]
    fn test_lit_denominated_gwei_units() {
        let lit_int = LitInt::new("20", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Gwei(kw::gwei(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "20 gwei");
    }

    #[test]
    fn test_lit_denominated_time_units() {
        let lit_int = LitInt::new("60", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Seconds(kw::seconds(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "60 seconds");

        let lit_int = LitInt::new("5", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Minutes(kw::minutes(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "5 minutes");

        let lit_int = LitInt::new("24", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Hours(kw::hours(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "24 hours");

        let lit_int = LitInt::new("7", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Days(kw::days(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "7 days");

        let lit_int = LitInt::new("2", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Weeks(kw::weeks(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "2 weeks");

        let lit_int = LitInt::new("1", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Years(kw::years(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "1 years");
    }

    #[test]
    fn test_lit_denominated_with_float() {
        let lit_float = LitFloat::new("0.5", Span::call_site());
        let number = LitNumber::Float(lit_float);
        let denom = SubDenomination::Ether(kw::ether(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "0.5 ether");

        let lit_float = LitFloat::new("1.5", Span::call_site());
        let number = LitNumber::Float(lit_float);
        let denom = SubDenomination::Hours(kw::hours(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "1.5 hours");
    }

    #[test]
    fn test_sub_denomination_values() {
        assert_eq!(SubDenomination::Wei(kw::wei(Span::call_site())).value(), 1);
        assert_eq!(SubDenomination::Gwei(kw::gwei(Span::call_site())).value(), 1_000_000_000);
        assert_eq!(
            SubDenomination::Ether(kw::ether(Span::call_site())).value(),
            1_000_000_000_000_000_000
        );

        assert_eq!(SubDenomination::Seconds(kw::seconds(Span::call_site())).value(), 1);
        assert_eq!(SubDenomination::Minutes(kw::minutes(Span::call_site())).value(), 60);
        assert_eq!(SubDenomination::Hours(kw::hours(Span::call_site())).value(), 3_600);
        assert_eq!(SubDenomination::Days(kw::days(Span::call_site())).value(), 86_400);
        assert_eq!(SubDenomination::Weeks(kw::weeks(Span::call_site())).value(), 604_800);
        assert_eq!(SubDenomination::Years(kw::years(Span::call_site())).value(), 31_536_000);
    }

    #[test]
    fn test_large_numbers() {
        let lit_int = LitInt::new(
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            Span::call_site(),
        );
        let number = LitNumber::Int(lit_int);
        let expected =
            "115792089237316195423570985008687907853269984665640564039457584007913129639935";
        assert_eq!(format!("{}", number), expected);

        // Test with Ether denomination
        let lit_int = LitInt::new("999999999999999999", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Wei(kw::wei(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "999999999999999999 wei");
    }

    #[test]
    fn test_zero_values() {
        let lit_int = LitInt::new("0", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Ether(kw::ether(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "0 ether");

        let lit_float = LitFloat::new("0.0", Span::call_site());
        let number = LitNumber::Float(lit_float);
        let denom = SubDenomination::Wei(kw::wei(Span::call_site()));
        let denominated = LitDenominated { number, denom };
        assert_eq!(format!("{}", denominated), "0.0 wei");
    }

    #[test]
    fn test_lit_number_new_constructors() {
        let number_int = LitNumber::new_int("42", Span::call_site());
        assert_eq!(format!("{}", number_int), "42");

        let number_float = LitNumber::new_fixed("3.14", Span::call_site());
        assert_eq!(format!("{}", number_float), "3.14");
    }

    #[test]
    fn test_lit_number_methods() {
        let lit_int = LitInt::new("42", Span::call_site());
        let number = LitNumber::Int(lit_int);

        assert_eq!(number.base10_digits(), "42");
        assert_eq!(number.suffix(), "");

        let lit_float = LitFloat::new("3.14", Span::call_site());
        let number = LitNumber::Float(lit_float);

        assert_eq!(number.base10_digits(), "3.14");
        assert_eq!(number.suffix(), "");
    }
}
