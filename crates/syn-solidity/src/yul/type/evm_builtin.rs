// Respresentation of an EVM builtin opcode
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.YulEVMBuiltin>

enum YulEvmOpcode {
    Stop,
    Add,
    Sub,
    Mul,
    Div,
    Sdiv,
    Mod,
    Smod,
}
