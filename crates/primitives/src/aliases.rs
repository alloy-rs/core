//! Type aliases for common primitive types.

use crate::{FixedBytes, Signed};

pub use ruint::aliases::{
    U0, U1, U1024, U128, U16, U160, U192, U2048, U256, U32, U320, U384, U4096, U448, U512, U64, U8,
};

macro_rules! int_aliases {
    ($($name:ident<$BITS:literal, $LIMBS:literal>),* $(,)?) => {$(
        #[doc = concat!($BITS, "-bit [signed integer type][Signed], consisting of ", $LIMBS, ", 64-bit limbs.")]
        pub type $name = Signed<$BITS, $LIMBS>;
        const _: () = assert!($LIMBS == ruint::nlimbs($BITS));
    )*};
}

/// The 0-bit signed integer type, capable of representing 0.
pub type I0 = Signed<0, 0>;

/// The 1-bit signed integer type, capable of representing 0 and -1.
pub type I1 = Signed<1, 1>;

int_aliases! {
    I8<8, 1>,
    I16<16, 1>,
    I32<32, 1>,
    I64<64, 1>,
    I128<128, 2>,
    I160<160, 3>,
    I192<192, 3>,
    I256<256, 4>,
    I512<512, 8>,
}

macro_rules! fixed_bytes_aliases {
    ($($(#[$attr:meta])* $name:ident<$N:literal>),* $(,)?) => {$(
        #[doc = concat!($N, "-byte [fixed byte-array][FixedBytes] type.")]
        $(#[$attr])*
        pub type $name = FixedBytes<$N>;
    )*};
}

fixed_bytes_aliases! {
    B8<1>,
    B16<2>,
    B32<4>,
    B64<8>,
    B96<12>,
    B128<16>,
    /// See [`crate::B160`] as to why you likely want to use
    /// [`Address`](crate::Address) instead.
    #[doc(hidden)]
    B160<20>,
    B192<24>,
    B224<28>,
    B256<32>,
    B512<64>,
    B1024<128>,
    B2048<256>,
}

/// A block hash.
pub type BlockHash = B256;

/// A block number.
pub type BlockNumber = u64;

/// A transaction hash is a keccak hash of an RLP encoded signed transaction.
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

/// Solidity contract functions are addressed using the first four bytes of the
/// Keccak-256 hash of their signature.
pub type Selector = [u8; 4];
