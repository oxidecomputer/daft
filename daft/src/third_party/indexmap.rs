use crate::Diffable;
use core::hash::Hash;
use indexmap::{IndexMap, IndexSet};

map_diff!(
    /// A diff of two [`IndexMap`] instances.
    ///
    /// The diff contains three elements:
    ///
    /// - `common`: Entries that are present in both maps, with their values
    ///   stored as a [`Leaf`][crate::Leaf].
    /// - `added`: Entries present in `after`, but not in `before`.
    /// - `removed`: Entries present in `before`, but not in `after`.
    ///
    /// If `V` implements `Eq`, `common` can be split into
    /// [`unchanged`][Self::unchanged] and [`modified`][Self::modified] entries.
    /// Additionally, if `V` implements [`Diffable`],
    /// [`modified_diff`][Self::modified_diff] can be used to recursively diff
    /// modified entries.
    ///
    /// [`IndexMap`]: indexmap::IndexMap
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "indexmap")] {
    /// use daft::{Diffable, IndexMapDiff, Leaf};
    /// use indexmap::IndexMap;
    ///
    /// let a: IndexMap<usize, &str> =
    ///     [(0, "lorem"), (1, "ipsum"), (2, "dolor")].into_iter().collect();
    /// let b: IndexMap<usize, &str> =
    ///    [(1, "ipsum"), (2, "sit"), (3, "amet")].into_iter().collect();
    ///
    /// let changes = a.diff(&b);
    /// let expected = IndexMapDiff {
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
    /// assert!(changes.is_unchanged(&1));
    /// assert!(changes.is_modified(&2));
    /// let unchanged = changes.unchanged().collect::<Vec<_>>();
    /// let modified = changes.modified().collect::<Vec<_>>();
    ///
    /// assert_eq!(unchanged, [(&1, &"ipsum")]);
    /// assert_eq!(modified, [(&2, Leaf { before: &"dolor", after: &"sit" })]);
    /// # }
    /// ```
    IndexMap, Hash
);
set_diff!(
    /// A diff of two [`IndexSet`] instances.
    ///
    /// The diff contains three elements:
    ///
    /// - `common`: Entries that are present in both sets.
    /// - `added`: Entries present in `after`, but not in `before`.
    /// - `removed`: Entries present in `before`, but not in `after`.
    ///
    /// [`IndexSet`]: indexmap::IndexSet
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "indexmap")] {
    /// use daft::{Diffable, IndexSetDiff};
    /// use indexmap::IndexSet;
    ///
    /// let a: IndexSet<usize> = [0, 1].into_iter().collect();
    /// let b: IndexSet<usize> = [1, 2].into_iter().collect();
    ///
    /// let changes = a.diff(&b);
    /// let expected = IndexSetDiff {
    ///     // Entries are stored by reference and matched by equality.
    ///     common: [&1].into_iter().collect(),
    ///     added: [&2].into_iter().collect(),
    ///     removed: [&0].into_iter().collect(),
    /// };
    ///
    /// assert_eq!(changes, expected);
    /// # }
    /// ```
    IndexSet, Hash
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Leaf;
    use alloc::vec::Vec;

    #[test]
    fn indexmap_set_diff() {
        let a: IndexSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: IndexSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let changes = a.diff(&b);
        let expected = IndexSetDiff {
            added: [&6, &7, &8].into_iter().collect(),
            removed: [&0, &1, &2].into_iter().collect(),
            common: [&3, &4, &5].into_iter().collect(),
        };
        assert_eq!(expected, changes);
    }

    #[test]
    fn indexmap_map_diff() {
        let a: IndexMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: IndexMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = IndexMapDiff {
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
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct K(i32);

        #[derive(Debug)]
        #[expect(dead_code)]
        struct V(f64);

        let floats_a: IndexMap<K, V> =
            [(K(0), V(1.0)), (K(1), V(1.0)), (K(2), V(1.0))]
                .into_iter()
                .collect();
        let floats_b: IndexMap<K, V> =
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
