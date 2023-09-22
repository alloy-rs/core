use winnow::{
    error::{ErrMode, ErrorKind, ParserError},
    PResult,
};

/// The regular expression for a Solidity identfier.
///
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.Identifier>
pub const IDENT_REGEX: &str = "[a-zA-Z$_][a-zA-Z0-9$_]*";

/// Returns `true` if the given character is valid at the start of a Solidity
/// identfier.
#[inline]
pub const fn is_id_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$')
}

/// Returns `true` if the given character is valid in a Solidity identfier.
#[inline]
pub const fn is_id_continue(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$')
}

/// An identifier in Solidity has to start with a letter, a dollar-sign or
/// an underscore and may additionally contain numbers after the first
/// symbol.
///
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.Identifier>
pub fn is_valid_identifier<S: AsRef<str>>(s: S) -> bool {
    fn is_valid_identifier(s: &str) -> bool {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            is_id_start(first) && chars.all(is_id_continue)
        } else {
            false
        }
    }

    is_valid_identifier(s.as_ref())
}

#[inline]
pub(crate) fn parse_identifier<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let mut chars = input.chars();
    if let Some(first) = chars.next() {
        if is_id_start(first) {
            // 1 for the first character, we know it's ASCII
            let len = 1 + chars.take_while(|c| is_id_continue(*c)).count();
            // SAFETY: Only ASCII characters are valid in identifiers
            unsafe {
                let ident = input.get_unchecked(..len);
                *input = input.get_unchecked(len..);
                return Ok(ident)
            }
        }
    }
    Err(ErrMode::from_error_kind(input, ErrorKind::Fail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifier() {
        ident_test("foo", Ok("foo"), "");
        ident_test("foo ", Ok("foo"), " ");
        ident_test("$foo", Ok("$foo"), "");
        ident_test("foo$", Ok("foo$"), "");
        ident_test("foo2$", Ok("foo2$"), "");
        ident_test("foo 2$", Ok("foo"), " 2$");
        ident_test("_foo 2$", Ok("_foo"), " 2$");

        ident_test("èfoo", Err(()), "èfoo");
        ident_test("fèoo", Ok("f"), "èoo");
        ident_test("foèo", Ok("fo"), "èo");
        ident_test("fooè", Ok("foo"), "è");

        ident_test("3foo", Err(()), "3foo");
        ident_test("f3oo", Ok("f3oo"), "");
        ident_test("fo3o", Ok("fo3o"), "");
        ident_test("foo3", Ok("foo3"), "");
    }

    #[track_caller]
    fn ident_test(mut input: &str, expected: Result<&str, ()>, output: &str) {
        assert_eq!(
            parse_identifier(&mut input).map_err(drop),
            expected,
            "result mismatch"
        );
        if let Ok(expected) = expected {
            assert!(
                is_valid_identifier(expected),
                "expected is not a valid ident"
            );
        }
        assert_eq!(input, output, "output mismatch");
    }
}
