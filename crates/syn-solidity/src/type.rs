use super::kw;
use crate::{sol_path, utils::DebugPunctuated, SolPath};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::{IntErrorKind, NonZeroU16, NonZeroU64},
};
use syn::{
    bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Paren},
    Error, Ident, LitInt, Result, Token,
};

/// An array type.
#[derive(Clone)]
pub struct SolArray {
    pub ty: Box<Type>,
    pub bracket_token: Bracket,
    pub size: Option<LitInt>,
}

impl PartialEq for SolArray {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.size == other.size
    }
}

impl Eq for SolArray {}

impl Hash for SolArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.size.hash(state);
    }
}

impl fmt::Debug for SolArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SolArray")
            .field(&self.ty)
            .field(&self.size.as_ref().map(|s| s.base10_digits()))
            .finish()
    }
}

impl fmt::Display for SolArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)?;
        f.write_str("[")?;
        if let Some(s) = &self.size {
            f.write_str(s.base10_digits())?;
        }
        f.write_str("]")
    }
}

impl Parse for SolArray {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ty = input.parse()?;
        Self::wrap(input, ty)
    }
}

impl SolArray {
    pub fn span(&self) -> Span {
        let span = self.ty.span();
        span.join(self.bracket_token.span.join()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.ty.set_span(span);
        self.bracket_token = Bracket(span);
        if let Some(size) = &mut self.size {
            size.set_span(span);
        }
    }

    pub fn wrap(input: ParseStream<'_>, ty: Type) -> Result<Self> {
        let content;
        Ok(Self {
            ty: Box::new(ty),
            bracket_token: bracketed!(content in input),
            size: {
                let size = content.parse::<Option<syn::LitInt>>()?;
                // Validate the size
                if let Some(sz) = &size {
                    sz.base10_parse::<NonZeroU64>()?;
                }
                size
            },
        })
    }
}

/// A tuple type.
#[derive(Clone)]
pub struct SolTuple {
    pub tuple_token: Option<kw::tuple>,
    pub paren_token: Paren,
    pub types: Punctuated<Type, Token![,]>,
}

impl PartialEq for SolTuple {
    fn eq(&self, other: &Self) -> bool {
        self.types == other.types
    }
}

impl Eq for SolTuple {}

impl Hash for SolTuple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.types.hash(state);
    }
}

impl fmt::Debug for SolTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SolTuple")
            .field(DebugPunctuated::new(&self.types))
            .finish()
    }
}

impl fmt::Display for SolTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (i, ty) in self.types.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            ty.fmt(f)?;
        }
        f.write_str(")")
    }
}

impl Parse for SolTuple {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let this = Self {
            tuple_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            types: content.parse_terminated(Type::parse, Token![,])?,
        };
        match this.types.len() {
            0 => Err(Error::new(
                this.paren_token.span.join(),
                "empty tuples are not allowed",
            )),
            1 if !this.types.trailing_punct() => Err(Error::new(
                this.paren_token.span.close(),
                "single element tuples must have a trailing comma",
            )),
            _ => Ok(this),
        }
    }
}

impl FromIterator<Type> for SolTuple {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        SolTuple {
            tuple_token: None,
            paren_token: Paren::default(),
            types: {
                let mut types = iter.into_iter().collect::<Punctuated<_, _>>();
                // ensure trailing comma for single item tuple
                if !types.trailing_punct() && types.len() == 1 {
                    types.push_punct(Default::default())
                }
                types
            },
        }
    }
}

impl SolTuple {
    pub fn span(&self) -> Span {
        let span = self.paren_token.span.join();
        self.tuple_token
            .and_then(|tuple_token| tuple_token.span.join(span))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        if let Some(tuple_token) = &mut self.tuple_token {
            tuple_token.span = span;
        }
        self.paren_token = Paren(span);
    }
}

/// A type name.
///
/// Solidity reference: <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.typeName>
#[derive(Clone)]
pub enum Type {
    /// `address [payable]`
    Address(Span, Option<kw::payable>),
    /// `bool`
    Bool(Span),
    /// `string`
    String(Span),

    /// `Some(size) => bytes<size>`, `None => bytes`
    Bytes {
        span: Span,
        size: Option<NonZeroU16>,
    },
    /// `Some(size) => int<size>`, `None => int`
    Int {
        span: Span,
        size: Option<NonZeroU16>,
    },
    /// `Some(size) => uint<size>`, `None => uint`
    Uint {
        span: Span,
        size: Option<NonZeroU16>,
    },

    /// `Some(size) => <type>[<size>]`, `None => <type>[]`
    Array(SolArray),
    /// `(tuple)? ( $($type),* )`
    Tuple(SolTuple),

    // TODO: function type
    // Function(...),
    /// A custom type.
    Custom(SolPath),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Address(..), Self::Address(..)) => true,
            (Self::Bool(_), Self::Bool(_)) => true,
            (Self::String(_), Self::String(_)) => true,
            (Self::Bytes { size: a, .. }, Self::Bytes { size: b, .. }) => a == b,
            (Self::Int { size: a, .. }, Self::Int { size: b, .. }) => a == b,
            (Self::Uint { size: a, .. }, Self::Uint { size: b, .. }) => a == b,
            (Self::Tuple(a), Self::Tuple(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Custom(a), Self::Custom(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Address(..) | Self::Bool(_) | Self::String(_) => {}
            Self::Bytes { size, .. } => size.hash(state),
            Self::Int { size, .. } => size.hash(state),
            Self::Uint { size, .. } => size.hash(state),
            Self::Tuple(tuple) => tuple.hash(state),
            Self::Array(array) => array.hash(state),
            Self::Custom(custom) => custom.hash(state),
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(_, None) => f.write_str("Address"),
            Self::Address(_, Some(_)) => f.write_str("AddressPayable"),
            Self::Bool(_) => f.write_str("Bool"),
            Self::String(_) => f.write_str("String"),
            Self::Bytes { size, .. } => f.debug_tuple("Bytes").field(size).finish(),
            Self::Int { size, .. } => f.debug_tuple("Int").field(size).finish(),
            Self::Uint { size, .. } => f.debug_tuple("Uint").field(size).finish(),
            Self::Tuple(tuple) => tuple.fmt(f),
            Self::Array(array) => array.fmt(f),
            Self::Custom(custom) => f.debug_tuple("Custom").field(custom).finish(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(_, None) => f.write_str("address"),
            Self::Address(_, Some(_)) => f.write_str("address payable"),
            Self::Bool(_) => f.write_str("bool"),
            Self::String(_) => f.write_str("string"),
            Self::Bytes { size, .. } => write_opt(f, "bytes", *size),
            Self::Int { size, .. } => write_opt(f, "int", *size),
            Self::Uint { size, .. } => write_opt(f, "uint", *size),
            Self::Tuple(tuple) => tuple.fmt(f),
            Self::Array(array) => array.fmt(f),
            Self::Custom(custom) => custom.fmt(f),
        }
    }
}

impl Parse for Type {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut candidate = if input.peek(Paren) || input.peek(kw::tuple) {
            Self::Tuple(input.parse()?)
        } else if input.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;
            let span = ident.span();
            let s = ident.to_string();
            match s.as_str() {
                "address" => Self::Address(span, input.parse()?),
                "bool" => Self::Bool(span),
                "string" => Self::String(span),
                s => {
                    if let Some(s) = s.strip_prefix("bytes") {
                        match parse_size(s, span)? {
                            None => Self::custom(ident),
                            Some(Some(size)) if size.get() > 32 => {
                                return Err(Error::new(span, "fixed bytes range is 1-32"))
                            }
                            Some(size) => Self::Bytes { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("int") {
                        match parse_size(s, span)? {
                            None => Self::custom(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(Error::new(
                                    span,
                                    "intX must be a multiple of 8 up to 256",
                                ))
                            }
                            Some(size) => Self::Int { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("uint") {
                        match parse_size(s, span)? {
                            None => Self::custom(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(Error::new(
                                    span,
                                    "uintX must be a multiple of 8 up to 256",
                                ))
                            }
                            Some(size) => Self::Uint { span, size },
                        }
                    } else {
                        Self::custom(ident)
                    }
                }
            }
        } else {
            return Err(input.error(
                "expected a Solidity type: \
                `address`, `bool`, `string`, `bytesN`, `intN`, `uintN`, a tuple, or a custom type name",
            ));
        };

        // while the next token is a bracket, parse an array size and nest the
        // candidate into an array
        while input.peek(Bracket) {
            candidate = Self::Array(SolArray::wrap(input, candidate)?);
        }

        Ok(candidate)
    }
}

impl Type {
    pub fn custom(ident: Ident) -> Self {
        Self::Custom(sol_path![ident])
    }

    pub fn span(&self) -> Span {
        match self {
            &Self::Address(span, payable) => {
                payable.and_then(|kw| span.join(kw.span)).unwrap_or(span)
            }
            Self::Bool(span)
            | Self::String(span)
            | Self::Bytes { span, .. }
            | Self::Int { span, .. }
            | Self::Uint { span, .. } => *span,
            Self::Tuple(tuple) => tuple.span(),
            Self::Array(array) => array.span(),
            Self::Custom(custom) => custom.span(),
        }
    }

    pub fn set_span(&mut self, new_span: Span) {
        match self {
            Self::Address(span, payable) => {
                *span = new_span;
                if let Some(kw) = payable {
                    kw.span = new_span;
                }
            }
            Self::Bool(span)
            | Self::String(span)
            | Self::Bytes { span, .. }
            | Self::Int { span, .. }
            | Self::Uint { span, .. } => *span = new_span,
            Self::Tuple(tuple) => tuple.set_span(new_span),
            Self::Array(array) => array.set_span(new_span),
            Self::Custom(custom) => custom.set_span(new_span),
        }
    }

    /// Returns whether a [Storage][crate::Storage] location can be
    /// specified for this type.
    pub const fn can_have_storage(&self) -> bool {
        self.is_dynamic() || self.is_custom()
    }

    pub const fn is_dynamic(&self) -> bool {
        matches!(
            self,
            Self::String(_) | Self::Bytes { size: None, .. } | Self::Array(_)
        )
    }

    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(..))
    }

    /// Traverses this type while calling `f`.
    #[cfg(feature = "visit")]
    pub fn visit(&self, f: impl FnMut(&Self)) {
        use crate::Visit;
        struct VisitType<F>(F);
        impl<F: FnMut(&Type)> Visit<'_> for VisitType<F> {
            fn visit_type(&mut self, ty: &Type) {
                (self.0)(ty);
                crate::visit::visit_type(self, ty);
            }
        }
        VisitType(f).visit_type(self)
    }

    /// Traverses this type while calling `f`.
    #[cfg(feature = "visit-mut")]
    pub fn visit_mut(&mut self, f: impl FnMut(&mut Self)) {
        use crate::VisitMut;
        struct VisitTypeMut<F>(F);
        impl<F: FnMut(&mut Type)> VisitMut<'_> for VisitTypeMut<F> {
            fn visit_type(&mut self, ty: &mut Type) {
                (self.0)(ty);
                crate::visit_mut::visit_type(self, ty);
            }
        }
        VisitTypeMut(f).visit_type(self);
    }
}

fn write_opt(f: &mut fmt::Formatter<'_>, name: &str, size: Option<NonZeroU16>) -> fmt::Result {
    f.write_str(name)?;
    if let Some(size) = size {
        write!(f, "{size}")?;
    }
    Ok(())
}

// None => Custom
// Some(size) => size
fn parse_size(s: &str, span: Span) -> Result<Option<Option<NonZeroU16>>> {
    let opt = match s.parse::<NonZeroU16>() {
        Ok(size) => Some(Some(size)),
        Err(e) => match e.kind() {
            // bytes
            IntErrorKind::Empty => Some(None),
            // bytes_
            IntErrorKind::InvalidDigit => None,
            // bytesN where N == 0 || N > MAX
            _ => return Err(Error::new(span, format_args!("invalid size: {e}"))),
        },
    };
    Ok(opt)
}
