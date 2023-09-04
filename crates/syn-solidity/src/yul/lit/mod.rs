use syn::{LitBool, LitInt};

/// Yul literals e.g. 0x123, 42 or "abc"
///
/// Reference:
/// https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulIfStatement
pub enum YulLit {
    Decimal(LitInt),

    String(),

    Hex(),

    Boolean(LitBool),

    HexString(),
}
