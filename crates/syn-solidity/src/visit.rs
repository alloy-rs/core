//! Syntax tree traversal to walk a shared borrow of a syntax tree.
//!
//! Each method of the [`Visit`] trait is a hook that can be overridden to
//! customize the behavior when visiting the corresponding type of node. By
//! default, every method recursively visits the substructure of the input by
//! invoking the right visitor method of each of its fields.

#![allow(unused_variables)]

use super::*;

make_visitor! {
    /// Syntax tree traversal to walk a shared borrow of a syntax tree.
    ///
    /// See the [module documentation] for details.
    ///
    /// [module documentation]: self
    trait Visit;
}
