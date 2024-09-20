use super::*;
use crate::{Address, FixedBytes, Selector, B256};
use cfg_if::cfg_if;
use core::hash::{BuildHasher, Hasher};

/// [`HashMap`] optimized for hashing [fixed-size byte arrays](FixedBytes).
pub type FbHashMap<const N: usize, V, S = DefaultHashBuilder> =
    HashMap<FixedBytes<N>, V, BuildFbHasher<N, S>>;
/// [`HashSet`] optimized for hashing [fixed-size byte arrays](FixedBytes).
pub type FbHashSet<const N: usize, S = DefaultHashBuilder> =
    HashSet<FixedBytes<N>, BuildFbHasher<N, S>>;

cfg_if! {
    if #[cfg(feature = "map-indexmap")] {
        /// [`IndexMap`] optimized for hashing [fixed-size byte arrays](FixedBytes).
        pub type FbIndexMap<const N: usize, V, S = DefaultHashBuilder> =
            indexmap::IndexMap<FixedBytes<N>, V, BuildFbHasher<N, S>>;
        /// [`IndexSet`] optimized for hashing [fixed-size byte arrays](FixedBytes).
        pub type FbIndexSet<const N: usize, S = DefaultHashBuilder> =
            indexmap::IndexSet<FixedBytes<N>, BuildFbHasher<N, S>>;
    }
}

macro_rules! fb_alias_maps {
    ($($ty:ident < $n:literal >),* $(,)?) => { paste::paste! {
        $(
            #[doc = concat!("[`HashMap`] optimized for hashing [`", stringify!($ty), "`].")]
            pub type [<$ty HashMap>]<V, S = DefaultHashBuilder> =
                HashMap<$ty, V, BuildFbHasher<$n, S>>;
            #[doc = concat!("[`HashSet`] optimized for hashing [`", stringify!($ty), "`].")]
            pub type [<$ty HashSet>]<S = DefaultHashBuilder> =
                HashSet<$ty, BuildFbHasher<$n, S>>;

            cfg_if! {
                if #[cfg(feature = "map-indexmap")] {
                    #[doc = concat!("[`IndexMap`] optimized for hashing [`", stringify!($ty), "`].")]
                    pub type [<$ty IndexMap>]<V, S = DefaultHashBuilder> =
                        IndexMap<$ty, V, BuildFbHasher<$n, S>>;
                    #[doc = concat!("[`IndexSet`] optimized for hashing [`", stringify!($ty), "`].")]
                    pub type [<$ty IndexSet>]<S = DefaultHashBuilder> =
                        IndexSet<$ty, BuildFbHasher<$n, S>>;
                }
            }
        )*
    } };
}

fb_alias_maps!(Selector<4>, Address<20>, B256<32>);

macro_rules! assert_unchecked {
    ($e:expr) => {
        if cfg!(debug_assertions) {
            assert!($e);
        } else if !$e {
            unsafe { core::hint::unreachable_unchecked() }
        }
    };
}

/// [`BuildHasher`] optimized for hashing [fixed-size byte arrays](FixedBytes).
///
/// **NOTE:** this hasher accepts only `N`-length byte arrays! It is UB to hash anything else.
#[derive(Clone, Debug, Default)]
pub struct BuildFbHasher<const N: usize, S = DefaultHashBuilder> {
    inner: S,
    _marker: core::marker::PhantomData<[(); N]>,
}

impl<const N: usize, S: BuildHasher> BuildHasher for BuildFbHasher<N, S> {
    type Hasher = FbHasher<N, S::Hasher>;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FbHasher { inner: self.inner.build_hasher(), _marker: core::marker::PhantomData }
    }
}

/// [`Hasher`] optimized for hashing [fixed-size byte arrays](FixedBytes).
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
    fn write(&mut self, mut bytes: &[u8]) {
        assert_unchecked!(bytes.len() == N);

        while let Some((chunk, rest)) = bytes.split_first_chunk() {
            self.inner.write_usize(usize::from_ne_bytes(*chunk));
            bytes = rest;
        }
        if usize::BITS > 64 {
            if let Some((chunk, rest)) = bytes.split_first_chunk() {
                self.inner.write_u64(u64::from_ne_bytes(*chunk));
                bytes = rest;
            }
        }
        if usize::BITS > 32 {
            if let Some((chunk, rest)) = bytes.split_first_chunk() {
                self.inner.write_u32(u32::from_ne_bytes(*chunk));
                bytes = rest;
            }
        }
        if usize::BITS > 16 {
            if let Some((chunk, rest)) = bytes.split_first_chunk() {
                self.inner.write_u16(u16::from_ne_bytes(*chunk));
                bytes = rest;
            }
        }
        if usize::BITS > 8 {
            if let Some((chunk, rest)) = bytes.split_first_chunk() {
                self.inner.write_u8(u8::from_ne_bytes(*chunk));
                bytes = rest;
            }
        }

        debug_assert!(bytes.is_empty());
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn write_length_prefix(&mut self, len: usize) {
        assert_unchecked!(len == N);
        // We can just skip hashing the length prefix entirely since we know it's always `N`.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hash_zero {
        ($n:expr) => {
            BuildFbHasher::<$n>::default().hash_one(&FixedBytes::<$n>::ZERO)
        };
    }

    #[test]
    fn fb_hasher() {
        // Just by running it once we test that it compiles and that debug assertions are correct.
        ruint::const_for!(N in [ 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
                                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47] {
            assert_ne!(hash_zero!(N), 0);
        });
    }
}
