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
