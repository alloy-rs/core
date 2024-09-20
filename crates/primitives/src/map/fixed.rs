use super::*;
use crate::{Address, FixedBytes, Selector, B256};
use cfg_if::cfg_if;
use core::hash::{BuildHasher, Hasher};

/// [`HashMap`] optimized for hashing [fixed-size byte arrays](FixedBytes).
pub type FbHashMap<const N: usize, V, S = DefaultHashBuilder> =
    HashMap<FixedBytes<N>, V, FbBuildHasher<N, S>>;
/// [`HashSet`] optimized for hashing [fixed-size byte arrays](FixedBytes).
pub type FbHashSet<const N: usize, S = DefaultHashBuilder> =
    HashSet<FixedBytes<N>, FbBuildHasher<N, S>>;

cfg_if! {
    if #[cfg(feature = "map-indexmap")] {
        /// [`IndexMap`] optimized for hashing [fixed-size byte arrays](FixedBytes).
        pub type FbIndexMap<const N: usize, V, S = DefaultHashBuilder> =
            indexmap::IndexMap<FixedBytes<N>, V, FbBuildHasher<N, S>>;
        /// [`IndexSet`] optimized for hashing [fixed-size byte arrays](FixedBytes).
        pub type FbIndexSet<const N: usize, S = DefaultHashBuilder> =
            indexmap::IndexSet<FixedBytes<N>, FbBuildHasher<N, S>>;
    }
}

macro_rules! fb_alias_maps {
    ($($ty:ident < $n:literal >),* $(,)?) => { paste::paste! {
        $(
            #[doc = concat!("[`HashMap`] optimized for hashing [`", stringify!($ty), "`].")]
            pub type [<$ty HashMap>]<V, S = DefaultHashBuilder> =
                HashMap<$ty, V, FbBuildHasher<$n, S>>;
            #[doc = concat!("[`HashSet`] optimized for hashing [`", stringify!($ty), "`].")]
            pub type [<$ty HashSet>]<S = DefaultHashBuilder> =
                HashSet<$ty, FbBuildHasher<$n, S>>;

            cfg_if! {
                if #[cfg(feature = "map-indexmap")] {
                    #[doc = concat!("[`IndexMap`] optimized for hashing [`", stringify!($ty), "`].")]
                    pub type [<$ty IndexMap>]<V, S = DefaultHashBuilder> =
                        IndexMap<$ty, V, FbBuildHasher<$n, S>>;
                    #[doc = concat!("[`IndexSet`] optimized for hashing [`", stringify!($ty), "`].")]
                    pub type [<$ty IndexSet>]<S = DefaultHashBuilder> =
                        IndexSet<$ty, FbBuildHasher<$n, S>>;
                }
            }
        )*
    } };
}

fb_alias_maps!(Selector<4>, Address<20>, B256<32>);

#[allow(unused_macros)]
macro_rules! assert_unchecked {
    ($e:expr) => { assert_unchecked!($e,); };
    ($e:expr, $($t:tt)*) => {
        if cfg!(debug_assertions) {
            assert!($e, $($t)*);
        } else if !$e {
            unsafe { core::hint::unreachable_unchecked() }
        }
    };
}

macro_rules! assert_eq_unchecked {
    ($a:expr, $b:expr) => { assert_eq_unchecked!($a, $b,); };
    ($a:expr, $b:expr, $($t:tt)*) => {
        if cfg!(debug_assertions) {
            assert_eq!($a, $b, $($t)*);
        } else if $a != $b {
            unsafe { core::hint::unreachable_unchecked() }
        }
    };
}

/// [`BuildHasher`] optimized for hashing [fixed-size byte arrays](FixedBytes).
///
/// Works best with `fxhash`, enabled by default with the "map-fxhash" feature.
///
/// **NOTE:** this hasher accepts only `N`-length byte arrays! It is UB to hash anything else.
#[derive(Clone, Debug, Default)]
pub struct FbBuildHasher<const N: usize, S = DefaultHashBuilder> {
    inner: S,
    _marker: core::marker::PhantomData<[(); N]>,
}

impl<const N: usize, S: BuildHasher> BuildHasher for FbBuildHasher<N, S> {
    type Hasher = FbHasher<N, S::Hasher>;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FbHasher { inner: self.inner.build_hasher(), _marker: core::marker::PhantomData }
    }
}

/// [`Hasher`] optimized for hashing [fixed-size byte arrays](FixedBytes).
///
/// Works best with `fxhash`, enabled by default with the "map-fxhash" feature.
///
/// **NOTE:** this hasher accepts only `N`-length byte arrays! It is UB to hash anything else.
#[derive(Clone, Debug, Default)]
pub struct FbHasher<const N: usize, H> {
    inner: H,
    _marker: core::marker::PhantomData<[(); N]>,
}

impl<const N: usize, H: Hasher> Hasher for FbHasher<N, H> {
    #[inline]
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        assert_eq_unchecked!(bytes.len(), N);
        // Threshold decided by some basic micro-benchmarks with fxhash.
        if N > 32 {
            self.inner.write(bytes);
        } else {
            write_bytes(&mut self.inner, bytes);
        }
    }

    // We can just skip hashing the length prefix entirely since we know it's always `N`.

    // `write_length_prefix` calls `write_usize` by default.
    #[cfg(not(feature = "nightly"))]
    #[inline]
    fn write_usize(&mut self, i: usize) {
        assert_eq_unchecked!(i, N);
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn write_length_prefix(&mut self, len: usize) {
        assert_eq_unchecked!(len, N);
    }
}

#[inline(always)]
fn write_bytes(hasher: &mut impl Hasher, mut bytes: &[u8]) {
    while let Some((chunk, rest)) = bytes.split_first_chunk() {
        hasher.write_usize(usize::from_ne_bytes(*chunk));
        bytes = rest;
    }
    if usize::BITS > 64 {
        if let Some((chunk, rest)) = bytes.split_first_chunk() {
            hasher.write_u64(u64::from_ne_bytes(*chunk));
            bytes = rest;
        }
    }
    if usize::BITS > 32 {
        if let Some((chunk, rest)) = bytes.split_first_chunk() {
            hasher.write_u32(u32::from_ne_bytes(*chunk));
            bytes = rest;
        }
    }
    if usize::BITS > 16 {
        if let Some((chunk, rest)) = bytes.split_first_chunk() {
            hasher.write_u16(u16::from_ne_bytes(*chunk));
            bytes = rest;
        }
    }
    if usize::BITS > 8 {
        if let Some((chunk, rest)) = bytes.split_first_chunk() {
            hasher.write_u8(u8::from_ne_bytes(*chunk));
            bytes = rest;
        }
    }

    debug_assert!(bytes.is_empty());
}

#[cfg(all(test, any(feature = "std", feature = "map-fxhash")))]
mod tests {
    use super::*;

    fn hash_zero<const N: usize>() -> u64 {
        FbBuildHasher::<N>::default().hash_one(&FixedBytes::<N>::ZERO)
    }

    #[test]
    fn fb_hasher() {
        // Just by running it once we test that it compiles and that debug assertions are correct.
        ruint::const_for!(N in [ 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                32, 47, 48, 49, 63, 64, 127, 128, 256, 512, 1024, 2048, 4096] {
            assert_ne!(hash_zero::<N>(), 0);
        });
    }
}
