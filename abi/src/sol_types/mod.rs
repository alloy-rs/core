/// A solidity error
mod sol_error;
pub use sol_error::{Panic, Revert, SolError};

/// A Solidity call
mod sol_call;
pub use sol_call::SolCall;

/// A solidity type that can be encoded/decoded via ABI
mod sol_type;
pub use sol_type::SolType;

/// A solidity struct
mod sol_struct;
pub use sol_struct::SolStruct;

/// Solidity Primitives. These are the types that are built in to solidity.
pub mod sol_data;
pub use sol_data::SolDataType;

/// Solidity user-defined value types
// no export needed as only item is a macro
mod sol_udt;
