/// A solidity struct trait
mod sol_struct;
pub use sol_struct::SolStruct;

/// Solidity Types
pub mod sol_type;
pub use sol_type::SolType;

/// Solidity user-defined value types
mod sol_udt;
// no export needed as only item is a macro
