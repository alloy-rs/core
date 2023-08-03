//! Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
//! place.
//!
//! Each method of the [`VisitMut`] trait is a hook that can be overridden to
//! customize the behavior when visiting the corresponding type of node. By
//! default, every method recursively visits the substructure of the input by
//! invoking the right visitor method of each of its fields.

#![allow(unused_variables)]

use super::*;

make_visitor! {
    /// Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
    /// place.
    ///
    /// See the [module documentation] for details.
    ///
    /// [module documentation]: self
    trait VisitMut is mut;
}
