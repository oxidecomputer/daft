//! Implementations for types from the `alloc` crate.

use crate::{Diffable, Leaf};
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};

leaf_deref! { String => str }

impl<T: Diffable + ?Sized> Diffable for Box<T> {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
    }
}

impl<T: Diffable + ToOwned + ?Sized> Diffable for Cow<'_, T> {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        Self: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        self.as_ref().diff(other.as_ref())
    }
}

impl<T: Diffable + ?Sized> Diffable for Arc<T> {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
    }
}

impl<T: Diffable + ?Sized> Diffable for Rc<T> {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
    }
}

map_diff!(
    /// A diff of two [`BTreeMap`] instances.
    ///
    /// The diff contains three elements:
    ///
    /// - `common`: Entries that are present in both maps, with their values
    ///   stored as a [`Leaf`].
    /// - `added`: Entries present in `after`, but not in `before`.
    /// - `removed`: Entries present in `before`, but not in `after`.
    ///
    /// If `V` implements `Eq`, `common` can be split into
    /// [`unchanged`][Self::unchanged] and [`modified`][Self::modified] entries.
    /// Additionally, if `V` implements [`Diffable`],
    /// [`modified_diff`][Self::modified_diff] can be used to recursively diff
    /// modified entries.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// use daft::{BTreeMapDiff, Diffable, Leaf};
    /// use std::collections::BTreeMap;
    ///
    /// let a: BTreeMap<usize, &str> =
    ///     [(0, "lorem"), (1, "ipsum"), (2, "dolor")].into_iter().collect();
    /// let b: BTreeMap<usize, &str> =
    ///    [(1, "ipsum"), (2, "sit"), (3, "amet")].into_iter().collect();
    ///
    /// let changes = a.diff(&b);
    /// let expected = BTreeMapDiff {
    ///     // Keys are stored by reference and matched by equality.
    ///     common: [
    ///         (&1, Leaf { before: &"ipsum", after: &"ipsum" }),
    ///         (&2, Leaf { before: &"dolor", after: &"sit" }),
    ///     ].into_iter().collect(),
    ///     added: [(&3, &"amet")].into_iter().collect(),
    ///     removed: [(&0, &"lorem")].into_iter().collect(),
    /// };
    ///
    /// assert_eq!(changes, expected);
    ///
    /// // If the values are `Eq`, it's also possible to get lists of
    /// // modified and unchanged entries.
    /// let unchanged = changes.unchanged().collect::<Vec<_>>();
    /// let modified = changes.modified().collect::<Vec<_>>();
    ///
    /// assert_eq!(unchanged, [(&1, &"ipsum")]);
    /// assert_eq!(modified, [(&2, Leaf { before: &"dolor", after: &"sit" })]);
    /// # }
    /// ```
    BTreeMap, Ord
);
set_diff!(
    /// A diff of two [`BTreeSet`] instances.
    ///
    /// The diff contains three elements:
    ///
    /// - `common`: Entries that are present in both sets.
    /// - `added`: Entries present in `after`, but not in `before`.
    /// - `removed`: Entries present in `before`, but not in `after`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// use daft::{BTreeSetDiff, Diffable};
    /// use std::collections::BTreeSet;
    ///
    /// let a: BTreeSet<usize> = [0, 1].into_iter().collect();
    /// let b: BTreeSet<usize> = [1, 2].into_iter().collect();
    ///
    /// let changes = a.diff(&b);
    /// let expected = BTreeSetDiff {
    ///     // Entries are stored by reference and matched by equality.
    ///     common: [&1].into_iter().collect(),
    ///     added: [&2].into_iter().collect(),
    ///     removed: [&0].into_iter().collect(),
    /// };
    ///
    /// assert_eq!(changes, expected);
    /// # }
    /// ```
    BTreeSet, Ord
);

/// Treat Vecs as Leafs
//
// We plan to add opt in diff functionality: set-like, reordered, etc...
impl<T: Diffable> Diffable for Vec<T> {
    type Diff<'daft>
        = Leaf<&'daft [T]>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btree_set_diff() {
        let a: BTreeSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: BTreeSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let changes = a.diff(&b);
        let expected = BTreeSetDiff {
            added: [&6, &7, &8].into_iter().collect(),
            removed: [&0, &1, &2].into_iter().collect(),
            common: [&3, &4, &5].into_iter().collect(),
        };
        assert_eq!(expected, changes);
    }

    #[test]
    fn btree_map_diff() {
        let a: BTreeMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: BTreeMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = BTreeMapDiff {
            common: [
                (&0, Leaf { before: &1, after: &2 }),
                (&2, Leaf { before: &1, after: &1 }),
            ]
            .into_iter()
            .collect(),
            added: [(&3, &1)].into_iter().collect(),
            removed: [(&1, &1)].into_iter().collect(),
        };

        assert_eq!(changes, expected);

        // Ensure that keys don't need to be Diffable, and values don't need to
        // be Eq or Diffable.
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct K(i32);

        #[derive(Debug)]
        #[expect(dead_code)]
        struct V(f64);

        let floats_a: BTreeMap<K, V> =
            [(K(0), V(1.0)), (K(1), V(1.0)), (K(2), V(1.0))]
                .into_iter()
                .collect();
        let floats_b: BTreeMap<K, V> =
            [(K(0), V(2.0)), (K(2), V(1.0)), (K(3), V(1.0))]
                .into_iter()
                .collect();

        let diff = floats_a.diff(&floats_b);
        assert_eq!(diff.added.keys().copied().collect::<Vec<_>>(), [&K(3)]);
        assert_eq!(diff.removed.keys().copied().collect::<Vec<_>>(), [&K(1)]);
        assert_eq!(
            diff.common.keys().copied().collect::<Vec<_>>(),
            [&K(0), &K(2)]
        );
    }
}
