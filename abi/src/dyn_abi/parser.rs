use core::{num::ParseIntError, str::FromStr};

use super::DynSolType;

/// Error parsing a dynamic solidity type
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    /// Tried to parse a list, but the value was not a list
    NotAList,
    /// Unmatched parenthesis
    UnmatchedParen,
    /// Unmatched bracket
    UnmatchedBracket,
    /// Error parsing int
    ParseInt(ParseIntError),
    /// Unknown type in parser input
    Unknown(String),
}

impl From<ParseIntError> for ParserError {
    fn from(value: ParseIntError) -> Self {
        ParserError::ParseInt(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CommaSeparatedList<'a> {
    elements: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for CommaSeparatedList<'a> {
    type Error = ParserError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let s = s.strip_prefix("tuple").unwrap_or(s);
        let s = s.strip_prefix('(').ok_or(ParserError::NotAList)?;
        let s = s.strip_suffix(')').ok_or(ParserError::UnmatchedParen)?;

        let mut elements = vec![];
        let mut depth = 0;
        let mut start = 0;

        for (i, c) in s.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => {
                    elements.push(s[start..i].trim());
                    start = i + 1;
                }
                _ => {}
            }
        }
        // Everything left is also a list item. Check prevents empty list items
        if !s[start..].is_empty() {
            elements.push(s[start..].trim());
        }
        Ok(CommaSeparatedList { elements })
    }
}

/// Parse a solidity type from a string
pub(crate) fn parse(s: &str) -> Result<DynSolType, ParserError> {
    let s = s.trim();

    if s.ends_with(')') {
        let csl: CommaSeparatedList<'_> = s.try_into()?;
        let v: Vec<_> = csl
            .elements
            .into_iter()
            .map(parse)
            .collect::<Result<_, _>>()?;
        return Ok(DynSolType::Tuple(v));
    }
    if s.ends_with(']') {
        let (prefix, suffix) = s.rsplit_once('[').ok_or(ParserError::UnmatchedBracket)?;
        let inner = Box::new(parse(prefix)?);

        let suffix = suffix.strip_suffix(']').unwrap();
        if !suffix.is_empty() {
            return Ok(DynSolType::FixedArray(inner, suffix.parse()?));
        } else {
            return Ok(DynSolType::Array(inner));
        }
    }

    if let Some(s) = s.strip_prefix("int") {
        return Ok(DynSolType::Int(s.parse()?));
    }

    if let Some(s) = s.strip_prefix("uint") {
        return Ok(DynSolType::Uint(s.parse()?));
    }

    match s {
        "address" => return Ok(DynSolType::Address),
        "bytes" => return Ok(DynSolType::Bytes),
        "bool" => return Ok(DynSolType::Bool),
        "string" => return Ok(DynSolType::String),
        _ => {}
    }

    if let Some(s) = s.strip_prefix("bytes") {
        return Ok(DynSolType::FixedBytes(s.parse()?));
    }

    Err(ParserError::Unknown(s.to_string()))
}

impl FromStr for DynSolType {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::AbiResult<Self> {
        Ok(crate::dyn_abi::parser::parse(s)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DynSolType;

    #[test]
    fn csl_parse() {
        assert_eq!(
            CommaSeparatedList::try_from("(a,b,c)"),
            Ok(CommaSeparatedList {
                elements: vec!["a", "b", "c"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("(a,(b,c),d)"),
            Ok(CommaSeparatedList {
                elements: vec!["a", "(b,c)", "d"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("(a, (b,c), (d, (e, f)))"),
            Ok(CommaSeparatedList {
                elements: vec!["a", "(b,c)", "(d, (e, f))"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("(a, (b,c), (d, (e, f))[3])"),
            Ok(CommaSeparatedList {
                elements: vec!["a", "(b,c)", "(d, (e, f))[3]"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("tuple(a, b)"),
            Ok(CommaSeparatedList {
                elements: vec!["a", "b"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("(  a  )"),
            Ok(CommaSeparatedList {
                elements: vec!["a"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("()"),
            Ok(CommaSeparatedList { elements: vec![] })
        );
        assert_eq!(
            CommaSeparatedList::try_from("(a,)"),
            Ok(CommaSeparatedList {
                elements: vec!["a"]
            })
        );
        assert_eq!(
            CommaSeparatedList::try_from("(a,,b)"),
            Ok(CommaSeparatedList {
                elements: vec!["a", "", "b"]
            })
        );

        assert_eq!(
            CommaSeparatedList::try_from("("),
            Err(ParserError::UnmatchedParen)
        );
        assert_eq!(
            CommaSeparatedList::try_from("a"),
            Err(ParserError::NotAList)
        );
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("bytes[][3]"),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::Array(Box::new(DynSolType::Bytes))),
                3
            ))
        );

        assert_eq!(
            parse(r#"tuple(address,bytes, (bool, (string, uint256)[][3]))[2]"#),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Address,
                    DynSolType::Bytes,
                    DynSolType::Tuple(vec![
                        DynSolType::Bool,
                        DynSolType::FixedArray(
                            Box::new(DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                                DynSolType::String,
                                DynSolType::Uint(256)
                            ])))),
                            3
                        ),
                    ]),
                ])),
                2
            ))
        );
    }
}
