use crate::{Panic, Result, Revert, SolError};
use alloc::vec::Vec;

/// A collection of ABI-encoded call-like types. This currently includes
/// [`SolCall`] and [`SolError`].
///
/// This trait assumes that the implementing type always has a selector, and
/// thus encoded/decoded data is always at least 4 bytes long.
///
/// [`SolCall`]: crate::SolCall
/// [`SolError`]: crate::SolError
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity contract
/// definition.
pub trait SolCalls: Sized {
    /// The name of this type.
    const NAME: &'static str;

    /// The minimum length of the data for this type.
    ///
    /// This does *not* include the selector's length (4).
    const MIN_DATA_LENGTH: usize;

    /// The number of variants.
    const COUNT: usize;

    /// The selector of this type.
    fn selector(&self) -> [u8; 4];

    /// Checks if the given selector is known to this type.
    fn type_check(selector: [u8; 4]) -> Result<()>;

    /// ABI-decodes the given data into one of the variants of `self`.
    fn decode_raw(selector: [u8; 4], data: &[u8], validate: bool) -> Result<Self>;

    /// The size of the encoded data, *without* any selectors.
    fn encoded_size(&self) -> usize;

    /// ABI-encodes `self` into the given buffer, *without* any selectors.
    fn encode_raw(&self, out: &mut Vec<u8>);

    /// ABI-encodes `self` into the given buffer.
    #[inline]
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.encoded_size());
        out.extend(self.selector());
        self.encode_raw(&mut out);
        out
    }

    /// ABI-decodes the given data into one of the variants of `self`.
    #[inline]
    fn decode(data: &[u8], validate: bool) -> Result<Self> {
        if data.len() < Self::MIN_DATA_LENGTH + 4 {
            Err(crate::Error::type_check_fail(data, Self::NAME))
        } else {
            let (selector, data) = crate::impl_core::split_array_ref(data);
            Self::decode_raw(*selector, data, validate)
        }
    }
}

/// A generic contract error.
///
/// Contains a [`Revert`] or [`Panic`] error, or a custom error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError<T> {
    /// A contract's custom error.
    CustomError(T),
    /// A generic revert. See [`Revert`] for more information.
    Revert(Revert),
    /// A panic. See [`Panic`] for more information.
    Panic(Panic),
}

impl<T: SolCalls> SolCalls for ContractError<T> {
    const NAME: &'static str = "ContractError";

    // revert is 64, panic is 32
    const MIN_DATA_LENGTH: usize = if T::MIN_DATA_LENGTH < 32 {
        T::MIN_DATA_LENGTH
    } else {
        32
    };

    const COUNT: usize = T::COUNT + 2;

    #[inline]
    fn selector(&self) -> [u8; 4] {
        match self {
            Self::CustomError(error) => error.selector(),
            Self::Panic(_) => Panic::SELECTOR,
            Self::Revert(_) => Revert::SELECTOR,
        }
    }

    #[inline]
    fn type_check(selector: [u8; 4]) -> Result<()> {
        match selector {
            Revert::SELECTOR | Panic::SELECTOR => Ok(()),
            s => T::type_check(s),
        }
    }

    #[inline]
    fn decode_raw(selector: [u8; 4], data: &[u8], validate: bool) -> Result<Self> {
        match selector {
            Revert::SELECTOR => Revert::decode_raw(data, validate).map(Self::Revert),
            Panic::SELECTOR => Panic::decode_raw(data, validate).map(Self::Panic),
            _ => T::decode(data, validate).map(Self::CustomError),
        }
    }

    #[inline]
    fn encoded_size(&self) -> usize {
        match self {
            Self::CustomError(error) => error.encoded_size(),
            Self::Panic(panic) => panic.encoded_size(),
            Self::Revert(revert) => revert.encoded_size(),
        }
    }

    #[inline]
    fn encode_raw(&self, out: &mut Vec<u8>) {
        match self {
            Self::CustomError(error) => error.encode_raw(out),
            Self::Panic(panic) => panic.encode_raw(out),
            Self::Revert(revert) => revert.encode_raw(out),
        }
    }
}

impl<T> ContractError<T> {
    /// Returns `true` if `self` matches [`CustomError`](Self::CustomError).
    #[inline]
    pub const fn is_custom_error(&self) -> bool {
        matches!(self, Self::CustomError(_))
    }

    /// Returns an immutable reference to the inner custom error if `self`
    /// matches [`CustomError`](Self::CustomError).
    #[inline]
    pub const fn as_custom_error(&self) -> Option<&T> {
        match self {
            Self::CustomError(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner custom error if `self`
    /// matches [`CustomError`](Self::CustomError).
    #[inline]
    pub fn as_custom_error_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::CustomError(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns `true` if `self` matches [`Revert`](Self::Revert).
    #[inline]
    pub const fn is_revert(&self) -> bool {
        matches!(self, Self::Revert(_))
    }

    /// Returns an immutable reference to the inner [`Revert`] if `self` matches
    /// [`Revert`](Self::Revert).
    #[inline]
    pub const fn as_revert(&self) -> Option<&Revert> {
        match self {
            Self::Revert(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner [`Revert`] if `self` matches
    /// [`Revert`](Self::Revert).
    #[inline]
    pub fn as_revert_mut(&mut self) -> Option<&mut Revert> {
        match self {
            Self::Revert(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns `true` if `self` matches [`Panic`](Self::Panic).
    #[inline]
    pub const fn is_panic(&self) -> bool {
        matches!(self, Self::Panic(_))
    }

    /// Returns an immutable reference to the inner [`Panic`] if `self` matches
    /// [`Panic`](Self::Panic).
    #[inline]
    pub const fn as_panic(&self) -> Option<&Panic> {
        match self {
            Self::Panic(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner [`Panic`] if `self` matches
    /// [`Panic`](Self::Panic).
    #[inline]
    pub fn as_panic_mut(&mut self) -> Option<&mut Panic> {
        match self {
            Self::Panic(inner) => Some(inner),
            _ => None,
        }
    }
}

impl<T> From<Revert> for ContractError<T> {
    #[inline]
    fn from(value: Revert) -> Self {
        Self::Revert(value)
    }
}

impl<T> TryFrom<ContractError<T>> for Revert {
    type Error = ContractError<T>;

    #[inline]
    fn try_from(value: ContractError<T>) -> Result<Revert, Self::Error> {
        match value {
            ContractError::Revert(inner) => Ok(inner),
            _ => Err(value),
        }
    }
}

impl<T> From<Panic> for ContractError<T> {
    #[inline]
    fn from(value: Panic) -> Self {
        Self::Panic(value)
    }
}

impl<T> TryFrom<ContractError<T>> for Panic {
    type Error = ContractError<T>;

    #[inline]
    fn try_from(value: ContractError<T>) -> Result<Panic, Self::Error> {
        match value {
            ContractError::Panic(inner) => Ok(inner),
            _ => Err(value),
        }
    }
}
