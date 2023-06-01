//! Type aliases for common primitive types.

use crate::{Signed, B256};

pub use ruint::aliases::{
    U0, U1, U1024, U128, U16, U160, U192, U2048, U256, U32, U320, U384, U4096, U448, U512, U64, U8,
};

/// The 0-bit signed integer type, capable of representing 0.
pub type I0 = Signed<0, 0>;

/// The 1-bit signed integer type, capable of representing 0 and -1.
pub type I1 = Signed<1, 1>;

/// 8-bit signed integer type.
pub type I8 = Signed<8, 1>;

/// 16-bit signed integer type.
pub type I16 = Signed<16, 1>;

/// 24-bit signed integer type.
pub type I24 = Signed<24, 1>;

/// 32-bit signed integer type.
pub type I32 = Signed<32, 1>;

/// 40-bit signed integer type.
pub type I40 = Signed<40, 1>;

/// 48-bit signed integer type.
pub type I48 = Signed<48, 1>;

/// 56-bit signed integer type.
pub type I56 = Signed<56, 1>;

/// 64-bit signed integer type.
pub type I64 = Signed<64, 1>;

/// 72-bit signed integer type.
pub type I72 = Signed<72, 2>;

/// 80-bit signed integer type.
pub type I80 = Signed<80, 2>;

/// 88-bit signed integer type.
pub type I88 = Signed<88, 2>;

/// 96-bit signed integer type.
pub type I96 = Signed<96, 2>;

/// 104-bit signed integer type.
pub type I104 = Signed<104, 2>;

/// 112-bit signed integer type.
pub type I112 = Signed<112, 2>;

/// 120-bit signed integer type.
pub type I120 = Signed<120, 2>;

/// 128-bit signed integer type.
pub type I128 = Signed<128, 2>;

/// 136-bit signed integer type.
pub type I136 = Signed<136, 3>;

/// 144-bit signed integer type.
pub type I144 = Signed<144, 3>;

/// 152-bit signed integer type.
pub type I152 = Signed<152, 3>;

/// 160-bit signed integer type.
pub type I160 = Signed<160, 3>;

/// 168-bit signed integer type.
pub type I168 = Signed<168, 3>;

/// 176-bit signed integer type.
pub type I176 = Signed<176, 3>;

/// 184-bit signed integer type.
pub type I184 = Signed<184, 3>;

/// 192-bit signed integer type.
pub type I192 = Signed<192, 3>;

/// 200-bit signed integer type.
pub type I200 = Signed<200, 4>;

/// 208-bit signed integer type.
pub type I208 = Signed<208, 4>;

/// 216-bit signed integer type.
pub type I216 = Signed<216, 4>;

/// 224-bit signed integer type.
pub type I224 = Signed<224, 4>;

/// 232-bit signed integer type.
pub type I232 = Signed<232, 4>;

/// 240-bit signed integer type.
pub type I240 = Signed<240, 4>;

/// 248-bit signed integer type.
pub type I248 = Signed<248, 4>;

/// 256-bit signed integer type.
pub type I256 = Signed<256, 4>;

/// A block hash.
pub type BlockHash = B256;

/// A block number.
pub type BlockNumber = u64;

/// A transaction hash is a kecack hash of an RLP encoded signed transaction.
pub type TxHash = B256;

/// The sequence number of all existing transactions.
pub type TxNumber = u64;

/// The index of transaction in a block.
pub type TxIndex = u64;

/// Chain identifier type (introduced in EIP-155).
pub type ChainId = u64;

/// An account storage key.
pub type StorageKey = B256;

/// An account storage value.
pub type StorageValue = U256;

/// Solidity contract functions are addressed using the first four byte of the
/// Keccak-256 hash of their signature
pub type Selector = [u8; 4];
