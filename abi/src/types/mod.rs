/// A solidity error
mod error;
pub use error::{Panic, Revert, SolError};

/// A Solidity call
mod call;
pub use call::SolCall;

/// A solidity type that can be encoded/decoded via ABI
mod r#type;
pub use r#type::SolType;

/// A solidity struct
mod r#struct;
pub use r#struct::SolStruct;

/// Solidity Primitives. These are the types that are built in to solidity.
pub mod data_type;

/// Solidity user-defined value types
// no export needed as only item is a macro
mod udt;
