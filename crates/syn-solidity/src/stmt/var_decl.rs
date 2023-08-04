use crate::{Expr, VariableDeclaration};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Result, Token,
};

/// A variable declaration statement: `uint256 foo = 42;`
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.variableDeclarationStatement>
#[derive(Clone)]
pub struct StmtVarDecl {
    pub declaration: VarDeclDecl,
    pub assignment: Option<(Token![=], Expr)>,
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtVarDecl")
            .field("declaration", &self.declaration)
            .field("assignment", &self.assignment)
            .finish()
    }
}

impl Parse for StmtVarDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            declaration: input.parse()?,
            assignment: {
                if input.peek(Token![=]) {
                    Some((input.parse()?, input.parse()?))
                } else {
                    None
                }
            },
            semi_token: input.parse()?,
        })
    }
}

impl StmtVarDecl {
    pub fn span(&self) -> Span {
        let span = self.declaration.span();
        self.assignment
            .as_ref()
            .and_then(|(_, expr)| expr.span().join(span))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.declaration.set_span(span);
        if let Some((eq, expr)) = &mut self.assignment {
            eq.span = span;
            expr.set_span(span);
        }
        self.semi_token.span = span;
    }
}

/// The declaration of the variable(s) in a [`StmtVarDecl`].
#[derive(Clone, Debug)]
pub enum VarDeclDecl {
    VarDecl(VariableDeclaration),
    Expression(VarDeclTuple),
}

impl Parse for VarDeclDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Paren) {
            input.parse().map(Self::Expression)
        } else {
            input.parse().map(Self::VarDecl)
        }
    }
}

impl VarDeclDecl {
    pub fn span(&self) -> Span {
        match self {
            Self::VarDecl(decl) => decl.span(),
            Self::Expression(decl) => decl.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::VarDecl(decl) => decl.set_span(span),
            Self::Expression(decl) => decl.set_span(span),
        }
    }
}

/// A declaration of variables in a tuple: `(,,uint256 foo,string memory bar)`
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.variableDeclarationTuple>
#[derive(Clone)]
pub struct VarDeclTuple {
    pub paren_token: Paren,
    /// The list of variables being declared. The list can't be empty, but it
    /// can contain `None` elements, indicating the field is empty.
    pub vars: Punctuated<Option<VariableDeclaration>, Token![,]>,
}

impl fmt::Debug for VarDeclTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TupleVarDecl")
            .field("vars", &self.vars)
            .finish()
    }
}

impl Parse for VarDeclTuple {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            vars: content.parse_terminated(Self::parse_var_opt, Token![,])?,
        })
    }
}

impl VarDeclTuple {
    pub fn span(&self) -> Span {
        self.paren_token.span.join()
    }

    pub fn set_span(&mut self, span: Span) {
        self.paren_token = Paren(span);
    }

    fn parse_var_opt(input: ParseStream<'_>) -> Result<Option<VariableDeclaration>> {
        if input.peek(Token![,]) {
            Ok(None)
        } else {
            input.parse().map(Some)
        }
    }
}
