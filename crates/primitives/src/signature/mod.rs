mod error;
pub use error::SignatureError;

#[expect(deprecated)]
mod parity;
#[expect(deprecated)]
pub use parity::Parity;

#[expect(deprecated)]
mod sig;
#[expect(deprecated)]
pub use sig::Signature;

mod utils;
pub use utils::{normalize_v, to_eip155_v};

mod primitive_sig;
pub use primitive_sig::PrimitiveSignature;
