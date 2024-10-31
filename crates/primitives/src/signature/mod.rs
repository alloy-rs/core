mod error;
pub use error::SignatureError;

mod parity;
pub use parity::Parity;

mod sig;
pub use sig::Signature;

mod utils;
pub use utils::{normalize_v, to_eip155_v};

/// Primitive signature type with boolean parity.
pub mod primitive_sig;
