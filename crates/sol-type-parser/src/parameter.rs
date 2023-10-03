use crate::{opt_ws_ident, spanned, tuple_parser, Error, Result, TypeSpecifier};
use alloc::vec::Vec;
use winnow::{trace::trace, PResult, Parser};

/// Represents a function parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSpecifier<'a> {
    /// The full span of the specifier.
    pub span: &'a str,
    /// The type of the parameter.
    pub ty: TypeSpecifier<'a>,
    /// Whether the parameter indexed.
    pub indexed: bool,
    /// The name of the parameter.
    pub name: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for ParameterSpecifier<'a> {
    type Error = Error;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self> {
        Self::parse(value)
    }
}

impl<'a> ParameterSpecifier<'a> {
    /// Parse a parameter from a string.
    #[inline]
    pub fn parse(input: &'a str) -> Result<Self> {
        Self::parser.parse(input).map_err(Error::parser)
    }

    pub(crate) fn parser(input: &mut &'a str) -> PResult<Self> {
        trace(
            "ParameterSpecifier",
            spanned(|input: &mut &'a str| {
                let ty = TypeSpecifier::parser(input)?;
                let mut indexed = false;
                let mut name = opt_ws_ident(input)?;
                // TODO: ?
                // if let Some("storage" | "memory" | "calldata") = name {
                //     name = opt_ws_ident(input)?;
                // }
                if let Some("indexed") = name {
                    indexed = true;
                    name = opt_ws_ident(input)?;
                }
                Ok((ty, indexed, name))
            }),
        )
        .parse_next(input)
        .map(|(span, (ty, indexed, name))| Self {
            span,
            ty,
            indexed,
            name,
        })
    }
}

/// Represents a list of function parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parameters<'a> {
    /// The full span of the specifier.
    pub span: &'a str,
    /// The parameters.
    pub params: Vec<ParameterSpecifier<'a>>,
}

impl<'a> TryFrom<&'a str> for Parameters<'a> {
    type Error = Error;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self> {
        Self::parse(value)
    }
}

impl<'a> Parameters<'a> {
    /// Parse a parameter list from a string.
    #[inline]
    pub fn parse(input: &'a str) -> Result<Self> {
        Self::parser.parse(input).map_err(Error::parser)
    }

    pub(crate) fn parser(input: &mut &'a str) -> PResult<Self> {
        trace(
            "Parameters",
            spanned(tuple_parser(ParameterSpecifier::parser)),
        )
        .parse_next(input)
        .map(|(span, params)| Self { span, params })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_param() {
        assert_eq!(
            ParameterSpecifier::parse("bool name"),
            Ok(ParameterSpecifier {
                span: "bool name",
                ty: TypeSpecifier::parse("bool").unwrap(),
                indexed: false,
                name: Some("name"),
            })
        );

        assert_eq!(
            ParameterSpecifier::parse("bool indexed name"),
            Ok(ParameterSpecifier {
                span: "bool indexed name",
                ty: TypeSpecifier::parse("bool").unwrap(),
                indexed: true,
                name: Some("name"),
            })
        );

        assert_eq!(
            ParameterSpecifier::parse("bool2    indexed \t name"),
            Ok(ParameterSpecifier {
                span: "bool2    indexed \t name",
                ty: TypeSpecifier::parse("bool2").unwrap(),
                indexed: true,
                name: Some("name"),
            })
        );

        ParameterSpecifier::parse("a b ").unwrap_err();
        ParameterSpecifier::parse(" a b ").unwrap_err();
        ParameterSpecifier::parse(" a b").unwrap_err();
    }

    #[test]
    fn parse_params() {
        assert_eq!(
            Parameters::parse("()"),
            Ok(Parameters {
                span: "()",
                params: vec![]
            })
        );
        assert_eq!(
            Parameters::parse("(,)"),
            Ok(Parameters {
                span: "(,)",
                params: vec![]
            })
        );
        assert_eq!(
            Parameters::parse("(, )"),
            Ok(Parameters {
                span: "(, )",
                params: vec![]
            })
        );
        assert_eq!(
            Parameters::parse("( , )"),
            Ok(Parameters {
                span: "( , )",
                params: vec![]
            })
        );

        assert_eq!(
            Parameters::parse("(\tuint256   , \t)"),
            Ok(Parameters {
                span: "(\tuint256   , \t)",
                params: vec![ParameterSpecifier {
                    span: "uint256   ",
                    ty: TypeSpecifier::parse("uint256").unwrap(),
                    indexed: false,
                    name: None,
                }]
            })
        );
        assert_eq!(
            Parameters::parse("( \t uint256 \ta,\t bool b, \t)"),
            Ok(Parameters {
                span: "( \t uint256 \ta,\t bool b, \t)",
                params: vec![
                    ParameterSpecifier {
                        span: "uint256 \ta",
                        ty: TypeSpecifier::parse("uint256").unwrap(),
                        indexed: false,
                        name: Some("a"),
                    },
                    ParameterSpecifier {
                        span: "bool b",
                        ty: TypeSpecifier::parse("bool").unwrap(),
                        indexed: false,
                        name: Some("b"),
                    }
                ]
            })
        );
    }
}
