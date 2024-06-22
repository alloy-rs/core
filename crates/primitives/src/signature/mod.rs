mod error;
pub use error::SignatureError;

mod parity;
pub use parity::Parity;

mod sig;
#[cfg(feature = "unstable-doc")]
pub use sig::Signature;
#[cfg(not(feature = "unstable-doc"))]
pub(crate) use sig::Signature;

mod utils;
pub use utils::to_eip155_v;
