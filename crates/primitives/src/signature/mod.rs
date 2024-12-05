mod error;
pub use error::SignatureError;

mod parity;
#[deprecated(since = "0.8.15", note = "see https://github.com/alloy-rs/core/pull/776")]
pub use parity::Parity;

mod sig;
#[deprecated(since = "0.8.15", note = "use PrimitiveSignature instead")]
pub use sig::Signature;

mod utils;
pub use utils::{normalize_v, to_eip155_v};

mod primitive_sig;
pub use primitive_sig::PrimitiveSignature;
