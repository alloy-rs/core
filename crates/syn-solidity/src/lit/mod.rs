use crate::{Spanned, kw};
use proc_macro2::Span;
use std::fmt;
use syn::{
    LitBool, Result,
    parse::{Lookahead1, Parse, ParseStream},
};

mod number;
pub use number::{LitDenominated, LitNumber, SubDenomination};

mod str;
pub use self::str::{HexStr, LitHexStr, LitStr, LitUnicodeStr, UnicodeStr};

/// A Solidity literal such as a string or integer or boolean.
#[derive(Clone)]
pub enum Lit {
    /// A boolean literal: `true` or `false`.
    Bool(LitBool),

    /// A hex string literal: `hex"1234"`.
    Hex(LitHexStr),

    /// An integer or fixed-point number literal: `1` or `1.0`.
    Number(LitNumber),

    /// A string literal.
    Str(LitStr),

    /// A unicode string literal.
    Unicode(LitUnicodeStr),
}

impl fmt::Display for Lit {
    /// Formats the literal as valid Solidity source code.
    ///
    /// This implementation ensures that all literals are formatted with proper
    /// syntax including quotes for string literals and prefixes for special literals.
    /// The output can be directly used as valid Solidity code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use syn::parse_str;
    /// use syn_solidity::{Expr, Lit};
    ///
    /// // String literals include quotes
    /// let expr: Expr = parse_str("\"hello world\"").unwrap();
    /// assert_eq!(format!("{}", expr), "\"hello world\"");
    ///
    /// // Hex literals include prefix and quotes
    /// let expr: Expr = parse_str("hex\"1234ABCD\"").unwrap();
    /// assert_eq!(format!("{}", expr), "hex\"1234ABCD\"");
    ///
    /// // Unicode literals include prefix and quotes
    /// let expr: Expr = parse_str("unicode\"Hello 世界\"").unwrap();
    /// assert_eq!(format!("{}", expr), "unicode\"Hello 世界\"");
    ///
    /// // Number literals are formatted directly
    /// let expr: Expr = parse_str("42").unwrap();
    /// assert_eq!(format!("{}", expr), "42");
    ///
    /// // Denominated literals include units
    /// let expr: Expr = parse_str("1 ether").unwrap();
    /// assert_eq!(format!("{}", expr), "1 ether");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(lit) => write!(f, "{}", if lit.value { "true" } else { "false" }),
            Self::Hex(lit) => write!(f, "hex\"{}\"", lit.value()),
            Self::Number(lit) => lit.fmt(f),
            Self::Str(lit) => write!(f, "\"{}\"", lit.value()),
            Self::Unicode(lit) => write!(f, "unicode\"{}\"", lit.value()),
        }
    }
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Lit::")?;
        match self {
            Self::Bool(lit) => lit.fmt(f),
            Self::Hex(lit) => lit.fmt(f),
            Self::Number(lit) => lit.fmt(f),
            Self::Str(lit) => lit.fmt(f),
            Self::Unicode(lit) => lit.fmt(f),
        }
    }
}

impl Parse for Lit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            input.parse().map(Self::Str)
        } else if LitNumber::peek(&lookahead) {
            input.parse().map(Self::Number)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else if lookahead.peek(kw::unicode) {
            input.parse().map(Self::Unicode)
        } else if lookahead.peek(kw::hex) {
            input.parse().map(Self::Hex)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Spanned for Lit {
    fn span(&self) -> Span {
        match self {
            Self::Bool(lit) => lit.span(),
            Self::Hex(lit) => lit.span(),
            Self::Number(lit) => lit.span(),
            Self::Str(lit) => lit.span(),
            Self::Unicode(lit) => lit.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Bool(lit) => lit.set_span(span),
            Self::Hex(lit) => lit.set_span(span),
            Self::Number(lit) => lit.set_span(span),
            Self::Str(lit) => lit.set_span(span),
            Self::Unicode(lit) => lit.set_span(span),
        }
    }
}

impl Lit {
    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(syn::Lit) || lookahead.peek(kw::unicode) || lookahead.peek(kw::hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        kw,
        lit::{LitDenominated, LitNumber, SubDenomination},
    };
    use proc_macro2::Span;
    use syn::{LitBool, LitFloat, LitInt};

    #[test]
    fn test_display_lit_bool() {
        let lit_true = LitBool { value: true, span: Span::call_site() };
        let lit = Lit::Bool(lit_true);
        assert_eq!(format!("{}", lit), "true");

        let lit_false = LitBool { value: false, span: Span::call_site() };
        let lit = Lit::Bool(lit_false);
        assert_eq!(format!("{}", lit), "false");
    }

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
    fn test_lit_number_in_lit_enum() {
        let lit_int = LitInt::new("100", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let lit = Lit::Number(number);
        assert_eq!(format!("{}", lit), "100");

        let lit_float = LitFloat::new("2.5", Span::call_site());
        let number = LitNumber::Float(lit_float);
        let lit = Lit::Number(number);
        assert_eq!(format!("{}", lit), "2.5");
    }

    #[test]
    fn test_display_lit_denominated_ether() {
        let lit_int = LitInt::new("1", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Ether(kw::ether(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "1 ether");
    }

    #[test]
    fn test_display_lit_denominated_wei() {
        let lit_int = LitInt::new("1000000000000000000", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Wei(kw::wei(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "1000000000000000000 wei");
    }

    #[test]
    fn test_display_lit_denominated_gwei() {
        let lit_int = LitInt::new("20", Span::call_site());
        let number = LitNumber::Int(lit_int);
        let denom = SubDenomination::Gwei(kw::gwei(Span::call_site()));
        let denominated = LitDenominated { number, denom };

        assert_eq!(format!("{}", denominated), "20 gwei");
    }

    #[test]
    fn test_display_lit_hex_strings() {
        // Test hex string literal display
        let expr: crate::Expr = syn::parse_str("hex\"1234\"").unwrap();
        assert_eq!(format!("{}", expr), "hex\"1234\"");

        let expr: crate::Expr = syn::parse_str("hex\"ABCDEF\"").unwrap();
        assert_eq!(format!("{}", expr), "hex\"ABCDEF\"");

        let expr: crate::Expr = syn::parse_str("hex\"001122FF\"").unwrap();
        assert_eq!(format!("{}", expr), "hex\"001122FF\"");
    }

    #[test]
    fn test_display_lit_unicode_strings() {
        // Test unicode string literal display
        let expr: crate::Expr = syn::parse_str("unicode\"Hello 世界\"").unwrap();
        assert_eq!(format!("{}", expr), "unicode\"Hello 世界\"");

        let expr: crate::Expr = syn::parse_str("unicode\"Test\"").unwrap();
        assert_eq!(format!("{}", expr), "unicode\"Test\"");

        let expr: crate::Expr = syn::parse_str("unicode\"🚀\"").unwrap();
        assert_eq!(format!("{}", expr), "unicode\"🚀\"");
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
    fn test_large_numbers() {
        let lit_int = LitInt::new(
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            Span::call_site(),
        );
        let number = LitNumber::Int(lit_int);
        let expected =
            "115792089237316195423570985008687907853269984665640564039457584007913129639935";
        assert_eq!(format!("{}", number), expected);
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
    fn test_complex_number_literals() {
        let lit_float = LitFloat::new("1e18", Span::call_site());
        let number = LitNumber::Float(lit_float);
        assert_eq!(format!("{}", number), "1e18");
    }
}
