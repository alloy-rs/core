use crate::FixedBytes;
use core::borrow::Borrow;

wrap_fixed_bytes! {
    /// An Ethereum ABI function pointer, 24 bytes in length.
    ///
    /// An address (20 bytes), followed by a function selector (4 bytes).
    /// Encoded identical to `bytes24`.
    pub struct Function<24>;
}

impl<A, S> From<(A, S)> for Function
where
    A: Borrow<[u8; 20]>,
    S: Borrow<[u8; 4]>,
{
    #[inline]
    fn from((address, selector): (A, S)) -> Self {
        Self::from_address_and_selector(address, selector)
    }
}

impl Function {
    /// Creates an Ethereum function from an EVM word's lower 24 bytes
    /// (`word[..24]`).
    ///
    /// Note that this is different from `Address::from_word`, which uses the
    /// upper 20 bytes.
    #[inline]
    #[must_use]
    pub fn from_word(word: FixedBytes<32>) -> Self {
        Self(FixedBytes(word[..24].try_into().unwrap()))
    }

    /// Right-pads the function to 32 bytes (EVM word size).
    ///
    /// Note that this is different from `Address::into_word`, which left-pads
    /// the address.
    #[inline]
    #[must_use]
    pub fn into_word(&self) -> FixedBytes<32> {
        let mut word = [0; 32];
        word[..24].copy_from_slice(self.as_slice());
        FixedBytes(word)
    }

    /// Creates an Ethereum function from an address and selector.
    #[inline]
    pub fn from_address_and_selector<A, S>(address: A, selector: S) -> Self
    where
        A: Borrow<[u8; 20]>,
        S: Borrow<[u8; 4]>,
    {
        let mut bytes = [0; 24];
        bytes[..20].copy_from_slice(address.borrow());
        bytes[20..].copy_from_slice(selector.borrow());
        Self(FixedBytes(bytes))
    }
}
