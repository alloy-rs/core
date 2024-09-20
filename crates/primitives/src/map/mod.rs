//! Re-exports of map types and utilities.

use cfg_if::cfg_if;

mod fixed;
pub use fixed::*;

use hashbrown as _;

// Use `hashbrown` if requested with "map-hashbrown" or required by `no_std`.
cfg_if! {
    if #[cfg(any(feature = "map-hashbrown", not(feature = "std")))] {
        use hashbrown as imp;
    } else {
        use std::collections as imp;
    }
}

#[doc(no_inline)]
pub use imp::{hash_map, hash_map::Entry, hash_set};

/// A [`HashMap`](imp::HashMap) using the [default hasher](DefaultHasher).
///
/// See [`HashMap`](imp::HashMap) for more information.
pub type HashMap<K, V, S = DefaultHashBuilder> = imp::HashMap<K, V, S>;
/// A [`HashSet`](imp::HashSet) using the [default hasher](DefaultHasher).
///
/// See [`HashSet`](imp::HashSet) for more information.
pub type HashSet<V, S = DefaultHashBuilder> = imp::HashSet<V, S>;

// Faster hasher.
cfg_if! {
    if #[cfg(feature = "map-fxhash")] {
        #[doc(no_inline)]
        pub use rustc_hash::{self, FxHasher};

        cfg_if! {
            if #[cfg(all(feature = "std", feature = "rand"))] {
                #[doc(no_inline)]
                pub use rustc_hash::FxRandomState as FxBuildHasher;
            } else {
                #[doc(no_inline)]
                pub use rustc_hash::FxBuildHasher;
            }
        }

        /// A [`HashMap`] using [`FxHasher`] as its hasher.
        pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;
        /// A [`HashSet`] using [`FxHasher`] as its hasher.
        pub type FxHashSet<V> = HashSet<V, FxBuildHasher>;
    }
}

// Default hasher.
cfg_if! {
    if #[cfg(feature = "map-fxhash")] {
        type DefaultHashBuilderInner = FxBuildHasher;
    } else if #[cfg(feature = "std")] {
        type DefaultHashBuilderInner = std::collections::hash_map::RandomState;
    } else {
        type DefaultHashBuilderInner = hashbrown::hash_map::DefaultHashBuilder;
    }
}
/// The default [`BuildHasher`](core::hash::BuildHasher) used by [`HashMap`] and [`HashSet`].
///
/// This hasher prioritizes speed over security, even if it is still secure enough for most
/// applications thanks to the use of a random seed.
pub type DefaultHashBuilder = DefaultHashBuilderInner;
/// The default [`Hasher`](core::hash::Hasher) used by [`HashMap`] and [`HashSet`].
///
/// This hasher prioritizes speed over security, even if it is still secure enough for most
/// applications thanks to the use of a random seed.
pub type DefaultHasher = <DefaultHashBuilder as core::hash::BuildHasher>::Hasher;

// `indexmap` re-exports.
cfg_if! {
    if #[cfg(feature = "map-indexmap")] {
        #[doc(no_inline)]
        pub use indexmap::{self, map::Entry as IndexEntry};

        /// [`IndexMap`](indexmap::IndexMap) using the [default hasher](DefaultHasher).
        ///
        /// See [`IndexMap`](indexmap::IndexMap) for more information.
        pub type IndexMap<K, V, S = DefaultHashBuilder> = indexmap::IndexMap<K, V, S>;
        /// [`IndexSet`](indexmap::IndexSet) using the [default hasher](DefaultHasher).
        ///
        /// See [`IndexSet`](indexmap::IndexSet) for more information.
        pub type IndexSet<V, S = DefaultHashBuilder> = indexmap::IndexSet<V, S>;

        cfg_if! {
            if #[cfg(feature = "map-fxhash")] {
                /// An [`IndexMap`] using [`FxHasher`] as its hasher.
                pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
                /// An [`IndexSet`] using [`FxHasher`] as its hasher.
                pub type FxIndexSet<V> = IndexSet<V, FxBuildHasher>;
            }
        }
    }
}
