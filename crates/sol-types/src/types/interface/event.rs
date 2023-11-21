use crate::{abi::token::WordToken, Error, Panic, Result, Revert, SolError};
use alloc::vec::Vec;
use alloy_primitives::B256;
use core::{convert::Infallible, fmt, iter::FusedIterator, marker::PhantomData};

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

    /// The minimum length of the data for this type.
    ///
    /// This does *not* include the selector's length (4).
    const MIN_DATA_LENGTH: usize;

    /// The number of variants.
    const COUNT: usize;
}
