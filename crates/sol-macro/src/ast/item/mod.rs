use super::{kw, Type};
use proc_macro2::{Ident, Span};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

mod error;
pub use error::Error;

mod function;
pub use function::Function;

mod r#struct;
pub use r#struct::Struct;

mod udt;
pub use udt::Udt;

/// An AST item. A more expanded version of a [Solidity source unit][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.sourceUnit
#[derive(Debug)]
pub enum Item {
    Udt(Udt),
    Struct(Struct),
    Function(Function),
    Error(Error),
    Type(Type),
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();
        let mut item = if lookahead.peek(Token![type]) {
            input.parse().map(Self::Udt)
        } else if lookahead.peek(Token![struct]) {
            input.parse().map(Self::Struct)
        } else if lookahead.peek(kw::function) {
            input.parse().map(Self::Function)
        } else if lookahead.peek(kw::error) {
            input.parse().map(Self::Error)
        } else if lookahead.peek(kw::tuple)
            || lookahead.peek(syn::token::Paren)
            || lookahead.peek(Ident::peek_any)
        {
            input.parse().map(Self::Type)
        } else {
            Err(lookahead.error())
        }?;

        if let Some(old_attrs) = item.replace_attrs(Vec::new()) {
            attrs.extend(old_attrs);
            item.replace_attrs(attrs);
        } else if !attrs.is_empty() {
            // TODO: Should this be an error?
            return Err(syn::Error::new(item.span(), "item cannot have attributes"))
        }

        Ok(item)
    }
}

impl Item {
    fn span(&self) -> Span {
        match self {
            Self::Udt(udt) => udt.span(),
            Self::Struct(strukt) => strukt.span(),
            Self::Function(function) => function.span(),
            Self::Error(error) => error.span(),
            Self::Type(ty) => ty.span(),
        }
    }

    fn replace_attrs(&mut self, new: Vec<Attribute>) -> Option<Vec<Attribute>> {
        match self {
            Self::Struct(Struct { attrs, .. })
            | Self::Function(Function { attrs, .. })
            | Self::Error(Error { attrs, .. }) => Some(std::mem::replace(attrs, new)),
            Self::Udt(_) | Self::Type(_) => None,
        }
    }
}
