use crate::{Expr, SolPath, Spanned, kw, utils::DebugPunctuated};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

mod function;
pub use function::{FunctionAttribute, FunctionAttributes};

mod variable;
pub use variable::{VariableAttribute, VariableAttributes};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, SolIdent};
    use proc_macro2::Span;
    use syn::{Token, parse_str, punctuated::Punctuated};

    #[test]
    fn test_display_override_without_paths() {
        let override_item = Override {
            override_token: Token![override](Span::call_site()),
            paren_token: None,
            paths: Punctuated::new(),
        };

        assert_eq!(format!("{}", override_item), "override");
    }

    #[test]
    fn test_display_modifier_regression() {
        // This is the key test: ensure expressions in modifiers display correctly
        // and don't show "<expr>" placeholders anymore

        let mut arguments = Punctuated::new();
        let expr: Expr = parse_str("msg.sender").unwrap();
        arguments.push(expr);

        let mut path = SolPath::new();
        path.push(SolIdent::new("onlyRole"));

        let modifier = Modifier {
            name: path,
            paren_token: Some(syn::token::Paren(Span::call_site())),
            arguments,
        };

        let result = format!("{}", modifier);
        assert_eq!(result, "onlyRole(msg.sender)");
        assert!(!result.contains("<expr>"), "Still contains <expr> placeholder");
    }

    #[test]
    fn test_display_modifier_complex_expression() {
        let mut arguments = Punctuated::new();
        let expr: Expr = parse_str("balances[user] >= amount").unwrap();
        arguments.push(expr);

        let mut path = SolPath::new();
        path.push(SolIdent::new("require"));

        let modifier = Modifier {
            name: path,
            paren_token: Some(syn::token::Paren(Span::call_site())),
            arguments,
        };

        let result = format!("{}", modifier);
        assert_eq!(result, "require(balances[user] >= amount)");
    }

    #[test]
    fn test_display_modifier_multiple_expressions() {
        let mut arguments = Punctuated::new();

        let arg1: Expr = parse_str("value > 0").unwrap();
        arguments.push(arg1);
        arguments.push_punct(Token![,](Span::call_site()));

        let arg2: Expr = parse_str("msg.sender != address(0)").unwrap();
        arguments.push(arg2);

        let mut path = SolPath::new();
        path.push(SolIdent::new("validateInputs"));

        let modifier = Modifier {
            name: path,
            paren_token: Some(syn::token::Paren(Span::call_site())),
            arguments,
        };

        let result = format!("{}", modifier);
        assert_eq!(result, "validateInputs(value > 0, msg.sender != address(0))");
    }
}

kw_enum! {
    /// A storage location.
    pub enum Storage {
        Memory(kw::memory),
        Storage(kw::storage),
        Calldata(kw::calldata),
    }
}

kw_enum! {
    /// A visibility attribute.
    pub enum Visibility {
        External(kw::external),
        Public(kw::public),
        Internal(kw::internal),
        Private(kw::private),
    }
}

kw_enum! {
    /// A mutability attribute.
    pub enum Mutability {
        Pure(kw::pure),
        View(kw::view),
        Constant(kw::constant),
        Payable(kw::payable),
    }
}

/// The `override` attribute.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Override {
    pub override_token: Token![override],
    pub paren_token: Option<Paren>,
    pub paths: Punctuated<SolPath, Token![,]>,
}

impl fmt::Display for Override {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("override")?;
        if self.paren_token.is_some() {
            f.write_str("(")?;
            for (i, path) in self.paths.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                path.fmt(f)?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Override {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Override").field(DebugPunctuated::new(&self.paths)).finish()
    }
}

impl Parse for Override {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let override_token = input.parse()?;
        let this = if input.peek(Paren) {
            let content;
            Self {
                override_token,
                paren_token: Some(parenthesized!(content in input)),
                paths: content.parse_terminated(SolPath::parse, Token![,])?,
            }
        } else {
            Self { override_token, paren_token: None, paths: Default::default() }
        };
        Ok(this)
    }
}

impl Spanned for Override {
    fn span(&self) -> Span {
        let span = self.override_token.span;
        self.paren_token.and_then(|paren_token| span.join(paren_token.span.join())).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.override_token.span = span;
        if let Some(paren_token) = &mut self.paren_token {
            *paren_token = Paren(span);
        }
    }
}

/// A modifier invocation, or an inheritance specifier.
#[derive(Clone)]
pub struct Modifier {
    pub name: SolPath,
    pub paren_token: Option<Paren>,
    pub arguments: Punctuated<Expr, Token![,]>,
}

impl PartialEq for Modifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Modifier {}

impl Hash for Modifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;
        if self.paren_token.is_some() {
            f.write_str("(")?;
            for (i, arg) in self.arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                arg.fmt(f)?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Modifier")
            .field("name", &self.name)
            .field("arguments", DebugPunctuated::new(&self.arguments))
            .finish()
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse()?;
        let this = if input.peek(Paren) {
            let content;
            let paren_token = parenthesized!(content in input);
            let arguments = content.parse_terminated(Expr::parse, Token![,])?;
            Self { name, paren_token: Some(paren_token), arguments }
        } else {
            Self { name, paren_token: None, arguments: Punctuated::new() }
        };
        Ok(this)
    }
}

impl Spanned for Modifier {
    fn span(&self) -> Span {
        let span = self.name.span();
        self.paren_token.and_then(|paren_token| span.join(paren_token.span.join())).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
        if let Some(paren_token) = &mut self.paren_token {
            *paren_token = Paren(span);
        }
    }
}
