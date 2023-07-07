use crate::{kw, LitStr, SolIdent};
use proc_macro2::Span;
use std::fmt;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Result, Token,
};

/// An import directive: `import "foo.sol";`
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.importDirective>
#[derive(Clone, Debug)]
pub struct ImportDirective {
    pub import_token: kw::import,
    pub path: ImportPath,
    pub semi_token: Token![;],
}

impl Parse for ImportDirective {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            import_token: input.parse()?,
            path: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ImportDirective {
    pub fn span(&self) -> Span {
        let span = self.import_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.import_token.span = span;
        self.path.set_span(span);
        self.semi_token.span = span;
    }
}

/// The path of an import directive.
#[derive(Clone, Debug)]
pub enum ImportPath {
    /// A plain import directive: `import "foo.sol" as Foo;`
    Plain(ImportPlain),
    /// A list of import aliases: `import { Foo as Bar, Baz } from "foo.sol";`
    Aliases(ImportAliases),
    /// A glob import directive: `import * as Foo from "foo.sol";`
    Glob(ImportGlob),
}

impl Parse for ImportPath {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![*]) {
            input.parse().map(Self::Glob)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Aliases)
        } else {
            input.parse().map(Self::Plain)
        }
    }
}

impl ImportPath {
    pub fn span(&self) -> Span {
        match self {
            Self::Plain(p) => p.span(),
            Self::Aliases(p) => p.span(),
            Self::Glob(p) => p.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Plain(p) => p.set_span(span),
            Self::Aliases(p) => p.set_span(span),
            Self::Glob(p) => p.set_span(span),
        }
    }

    pub fn path(&self) -> &LitStr {
        match self {
            Self::Plain(ImportPlain { path, .. })
            | Self::Aliases(ImportAliases { path, .. })
            | Self::Glob(ImportGlob { path, .. }) => path,
        }
    }

    pub fn path_mut(&mut self) -> &mut LitStr {
        match self {
            Self::Plain(ImportPlain { path, .. })
            | Self::Aliases(ImportAliases { path, .. })
            | Self::Glob(ImportGlob { path, .. }) => path,
        }
    }
}

/// An import alias.
#[derive(Clone)]
pub struct ImportAlias {
    pub as_token: Token![as],
    pub alias: SolIdent,
}

impl fmt::Debug for ImportAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Alias").field(&self.alias).finish()
    }
}

impl Parse for ImportAlias {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            as_token: input.parse()?,
            alias: input.parse()?,
        })
    }
}

impl ImportAlias {
    pub fn span(&self) -> Span {
        let span = self.as_token.span;
        span.join(self.alias.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.as_token.span = span;
        self.alias.set_span(span);
    }

    pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
        if input.peek(Token![as]) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}

/// A plain import directive: `import "foo.sol" as Foo;`
#[derive(Clone)]
pub struct ImportPlain {
    pub path: LitStr,
    pub alias: Option<ImportAlias>,
}

impl fmt::Debug for ImportPlain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Plain")
            .field("path", &self.path)
            .field("alias", &self.alias)
            .finish()
    }
}

impl Parse for ImportPlain {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            path: input.parse()?,
            alias: input.call(ImportAlias::parse_opt)?,
        })
    }
}

impl ImportPlain {
    pub fn span(&self) -> Span {
        let span = self.path.span();
        if let Some(alias) = &self.alias {
            span.join(alias.span()).unwrap_or(span)
        } else {
            span
        }
    }

    pub fn set_span(&mut self, span: Span) {
        self.path.set_span(span);
        if let Some(alias) = &mut self.alias {
            alias.set_span(span);
        }
    }
}

/// A list of import aliases: `import { Foo as Bar, Baz } from "foo.sol";`
#[derive(Clone)]
pub struct ImportAliases {
    pub brace_token: Brace,
    pub imports: Punctuated<(SolIdent, ImportAlias), Token![,]>,
    pub from_token: kw::from,
    pub path: LitStr,
}

impl fmt::Debug for ImportAliases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Aliases")
            .field("imports", &self.imports)
            .field("path", &self.path)
            .finish()
    }
}

impl Parse for ImportAliases {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            imports: content.parse_terminated(|c| Ok((c.parse()?, c.parse()?)), Token![,])?,
            from_token: input.parse()?,
            path: input.parse()?,
        })
    }
}

impl ImportAliases {
    pub fn span(&self) -> Span {
        let span = self.brace_token.span.join();
        span.join(self.path.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.brace_token = Brace(span);
        self.from_token.span = span;
        self.path.set_span(span);
    }
}

/// A glob import directive: `import * as Foo from "foo.sol";`
#[derive(Clone)]
pub struct ImportGlob {
    pub star_token: Token![*],
    pub alias: ImportAlias,
    pub from_token: kw::from,
    pub path: LitStr,
}

impl fmt::Debug for ImportGlob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Glob")
            .field("alias", &self.alias)
            .field("path", &self.path)
            .finish()
    }
}

impl Parse for ImportGlob {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            star_token: input.parse()?,
            alias: input.parse()?,
            from_token: input.parse()?,
            path: input.parse()?,
        })
    }
}

impl ImportGlob {
    pub fn span(&self) -> Span {
        let span = self.star_token.span;
        span.join(self.path.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.star_token.span = span;
        self.alias.set_span(span);
        self.from_token.span = span;
        self.path.set_span(span);
    }
}
