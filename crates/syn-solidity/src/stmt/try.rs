use crate::{kw, Block, Expr, ParameterList, Returns, SolIdent, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// A try statement: `try fooBar(42) catch { ... }`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.tryStatement>
#[derive(Clone)]
pub struct StmtTry {
    pub try_token: Token![try],
    pub expr: Box<Expr>,
    pub returns: Option<Returns>,
    /// The try block.
    pub block: Block,
    /// The list of catch clauses. Cannot be parsed empty.
    pub catch: Vec<CatchClause>,
}

impl fmt::Debug for StmtTry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtTry")
            .field("expr", &self.expr)
            .field("returns", &self.returns)
            .field("block", &self.block)
            .field("catch", &self.catch)
            .finish()
    }
}

impl Parse for StmtTry {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            try_token: input.parse()?,
            expr: input.parse()?,
            returns: input.call(Returns::parse_opt)?,
            block: input.parse()?,
            catch: {
                let mut catch = Vec::new();
                let mut first = true;
                while first || input.peek(kw::catch) {
                    first = false;
                    catch.push(input.parse()?);
                }
                catch
            },
        })
    }
}

impl Spanned for StmtTry {
    fn span(&self) -> Span {
        let span = self.try_token.span;
        span.join(self.block.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.try_token.span = span;
        self.block.set_span(span);
    }
}

/// A catch clause of a [`StmtTry`]: `catch  { ... }`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.tryStatement>
#[derive(Clone)]
pub struct CatchClause {
    pub catch_token: kw::catch,
    pub name: Option<SolIdent>,
    pub paren_token: Option<Paren>,
    pub list: ParameterList,
    pub block: Block,
}

impl fmt::Debug for CatchClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CatchClause")
            .field("name", &self.name)
            .field("list", &self.list)
            .field("block", &self.block)
            .finish()
    }
}

impl Parse for CatchClause {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let catch_token = input.parse()?;
        let name = input.call(SolIdent::parse_opt)?;
        let (paren_token, list) = if input.peek(Paren) {
            let content;
            (Some(parenthesized!(content in input)), content.parse()?)
        } else {
            (None, ParameterList::new())
        };
        let block = input.parse()?;
        Ok(Self {
            catch_token,
            name,
            paren_token,
            list,
            block,
        })
    }
}

impl Spanned for CatchClause {
    fn span(&self) -> Span {
        let span = self.catch_token.span;
        span.join(self.block.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.catch_token.span = span;
        self.block.set_span(span);
    }
}
