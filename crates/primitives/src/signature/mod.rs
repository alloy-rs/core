mod error;
pub use error::SignatureError;

mod utils;
pub use utils::{normalize_v, to_eip155_v};

mod sig;
pub use sig::Signature;

/// Deprecated alias for [`Signature`].
#[deprecated(since = "0.9.0", note = "use Signature instead")]
pub type PrimitiveSignature = Signature;
