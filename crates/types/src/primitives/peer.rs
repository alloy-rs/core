// TODO: should we use `PublicKey` for this? Even when dealing with public keys
// we should try to prevent misuse

/// Represents an uncompressed secp256k1 public key.
///
/// Encodes the concatenation of the x and y components of the affine point
/// in bytes.
pub type PeerId = ethers_primitives::B512;

/// Generic wrapper with peer id
#[derive(Debug)]
pub struct WithPeerId<T>(PeerId, pub T);

impl<T> From<(PeerId, T)> for WithPeerId<T> {
    fn from(value: (PeerId, T)) -> Self {
        Self(value.0, value.1)
    }
}

impl<T> WithPeerId<T> {
    /// Wraps the value with the peerid.
    pub fn new(peer: PeerId, value: T) -> Self {
        Self(peer, value)
    }

    /// Get the peer id
    pub fn peer_id(&self) -> PeerId {
        self.0
    }

    /// Get the underlying data
    pub fn data(&self) -> &T {
        &self.1
    }

    /// Returns ownership of the underlying data.
    pub fn into_data(self) -> T {
        self.1
    }

    /// Transform the data
    pub fn transform<F: From<T>>(self) -> WithPeerId<F> {
        WithPeerId(self.0, self.1.into())
    }

    /// Split the wrapper into [PeerId] and data tuple
    pub fn split(self) -> (PeerId, T) {
        (self.0, self.1)
    }

    /// Maps the inner value to a new value using the given function.
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> WithPeerId<U> {
        WithPeerId(self.0, op(self.1))
    }
}
