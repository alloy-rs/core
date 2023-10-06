use crate::{kw, Spanned};
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

macro_rules! yul_evm_builtin_enum_builder {
    ($( $variant:ident($($token:tt)* ) ),* $(,)?) => {
        /// Representation of an EVM builtin opcode.
        ///
        /// Solidity Reference:
        /// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.YulEVMBuiltin>
        #[derive(Clone, Debug)]
        pub enum YulEVMBuiltIn {
            $( $variant($($token)*), )*
        }

        // Generate the parser
        impl Parse for YulEVMBuiltIn {
            fn parse(input: ParseStream<'_>) -> Result<Self> {
                let lookahead = input.lookahead1();
                $(
                    if lookahead.peek($($token)*) {
                        return input.parse().map(Self::$variant);
                    }
                )*
                Err(lookahead.error())
            }
        }

        impl Spanned for YulEVMBuiltIn {
            fn span(&self) -> Span {
                match self {
                    $( Self::$variant(inner) => inner.span(), )*
                }
            }

            fn set_span(&mut self, span: proc_macro2::Span) {
                match self {
                    $( Self::$variant(inner) => inner.span = span, )*
                }
            }
        }
    };
}

yul_evm_builtin_enum_builder!(
    Stop(kw::stop),
    Add(kw::add),
    Sub(kw::sub),
    Mul(kw::mul),
    Div(kw::div),
    Sdiv(kw::sdiv),
    Mod(Token![mod]),
    Smod(kw::smod),
    Exp(kw::exp),
    Not(kw::not),
    Lt(kw::lt),
    Gt(kw::gt),
    Slt(kw::slt),
    Sgt(kw::sgt),
    Eq(kw::eq),
    Iszero(kw::iszero),
    And(kw::and),
    Or(kw::or),
    Xor(kw::xor),
    Byte(kw::byte),
    Shl(kw::shl),
    Shr(kw::shr),
    Sar(kw::sar),
    Addmod(kw::addmod),
    Mulmod(kw::mulmod),
    Signextend(kw::signextend),
    Keccak256(kw::keccak256),
    Pop(kw::pop),
    Mload(kw::mload),
    Mstore(kw::mstore),
    Mstore8(kw::mstore8),
    Sload(kw::sload),
    Sstore(kw::sstore),
    Msize(kw::msize),
    Gas(kw::gas),
    Address(kw::address),
    Balance(kw::balance),
    Selfbalance(kw::selfbalance),
    Caller(kw::caller),
    Callvalue(kw::callvalue),
    Calldataload(kw::calldataload),
    Calldatasize(kw::calldatasize),
    Calldatacopy(kw::calldatacopy),
    Extcodesize(kw::extcodesize),
    Extcodecopy(kw::extcodecopy),
    Returndatasize(kw::returndatasize),
    Returndatacopy(kw::returndatacopy),
    Extcodehash(kw::extcodehash),
    Create(kw::create),
    Create2(kw::create2),
    Call(kw::call),
    Callcode(kw::callcode),
    Delegatecall(kw::delegatecall),
    Staticcall(kw::staticcall),
    Return(Token![return]),
    Revert(kw::revert),
    Selfdestruct(kw::selfdestruct),
    Invalid(kw::invalid),
    Log0(kw::log0),
    Log1(kw::log1),
    Log2(kw::log2),
    Log3(kw::log3),
    Log4(kw::log4),
    Chainid(kw::chainid),
    Origin(kw::origin),
    Gasprice(kw::gasprice),
    Blockhash(kw::blockhash),
    Coinbase(kw::coinbase),
    Timestamp(kw::timestamp),
    Number(kw::number),
    Difficulty(kw::difficulty),
    Prevrandao(kw::prevrandao),
    Gaslimit(kw::gaslimit),
    Basefee(kw::basefee),
);
