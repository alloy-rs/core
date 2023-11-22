use crate::{Result, Word};
use alloy_primitives::Log;

/// A collection of [`SolEvent`]s.
///
/// [`SolEvent`]: crate::SolEvent
///
/// # Implementer's Guide
///
/// It should not be necessary to implement this trait manually. Instead, use
/// the [`sol!`](crate::sol!) procedural macro to parse Solidity syntax into
/// types that implement this trait.
pub trait SolEventInterface: Sized {
    /// The name of this type.
    const NAME: &'static str;

    /// The number of variants.
    const COUNT: usize;

    /// Decode the events from the given log info.
    fn decode_log(topics: &[Word], data: &[u8], validate: bool) -> Result<Self>;

    /// Decode the events from the given log object.
    fn decode_log_object(log: &Log, validate: bool) -> Result<Self> {
        Self::decode_log(log.topics(), &log.data, validate)
    }
}
