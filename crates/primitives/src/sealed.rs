use derive_more::Deref;

use crate::B256;

/// A consensus hashable item, with its memoized hash.
///
/// We do not implement any specific hashing algorithm here. Instead types
/// implement the [`Sealable`] trait to provide define their own hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(proptest_derive::Arbitrary))]
pub struct Sealed<T> {
    /// The inner item
    #[deref]
    inner: T,
    /// Its hash.
    seal: B256,
}

impl<T> Sealed<T> {
    /// Instantiate without performing the hash. This should be used carefully.
    pub const fn new_unchecked(inner: T, seal: B256) -> Self {
        Self { inner, seal }
    }

    /// Decompose into parts.
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn into_parts(self) -> (T, B256) {
        (self.inner, self.seal)
    }

    /// Decompose into parts. Alias for [`Self::into_parts`].
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn split(self) -> (T, B256) {
        self.into_parts()
    }

    /// Get the inner item.
    #[inline(always)]
    pub const fn inner(&self) -> &T {
        &self.inner
    }

    /// Get the hash.
    #[inline(always)]
    pub const fn seal(&self) -> B256 {
        self.seal
    }

    /// Unseal the inner item, discarding the hash.
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Unseal the inner item, discarding the hash. Alias for
    /// [`Self::into_inner`].
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn unseal(self) -> T {
        self.into_inner()
    }
}

impl<T> Default for Sealed<T>
where
    T: Sealable + Default,
{
    fn default() -> Self {
        T::default().seal_slow()
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T> arbitrary::Arbitrary<'a> for Sealed<T>
where
    T: for<'b> arbitrary::Arbitrary<'b> + Sealable,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(T::arbitrary(u)?.seal_slow())
    }
}

/// Sealeable objects.
pub trait Sealable: Sized {
    /// Calculate the seal hash, this may be slow.
    fn hash_slow(&self) -> B256;

    /// Seal the object by calculating the hash. This may be slow.
    fn seal_slow(self) -> Sealed<Self> {
        let seal = self.hash_slow();
        Sealed::new_unchecked(self, seal)
    }

    /// Instantiate an unchecked seal. This should be used with caution.
    fn seal_unchecked(self, seal: B256) -> Sealed<Self> {
        Sealed::new_unchecked(self, seal)
    }
}
