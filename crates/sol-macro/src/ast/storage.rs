use super::kw;
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

/// A storage location.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Storage {
    Memory(kw::memory),
    Storage(kw::storage),
    Calldata(kw::calldata),
}

impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_debug_str())
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Parse for Storage {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::memory) {
            Ok(Self::Memory(input.parse()?))
        } else if lookahead.peek(kw::storage) {
            Ok(Self::Storage(input.parse()?))
        } else if lookahead.peek(kw::calldata) {
            Ok(Self::Calldata(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Storage {
    pub fn span(&self) -> Span {
        match self {
            Self::Memory(kw) => kw.span,
            Self::Storage(kw) => kw.span,
            Self::Calldata(kw) => kw.span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Memory(kw) => kw.span = span,
            Self::Storage(kw) => kw.span = span,
            Self::Calldata(kw) => kw.span = span,
        }
    }

    pub const fn as_debug_str(&self) -> &'static str {
        match self {
            Self::Memory(_) => "Memory",
            Self::Storage(_) => "Storage",
            Self::Calldata(_) => "Calldata",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Memory(_) => "memory",
            Self::Storage(_) => "storage",
            Self::Calldata(_) => "calldata",
        }
    }
}
