//! Implementations for types in std.

use crate::Diffable;
use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    hash::Hash,
    path::{Path, PathBuf},
};

leaf! { Path, OsStr }
leaf_deref! { PathBuf => Path, OsString => OsStr }

map_diff!((HashMap, Hash));
set_diff!((HashSet, Hash));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Leaf;

    #[test]
    fn hash_set_diff() {
        let a: HashSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: HashSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let changes = a.diff(&b);

        let expected = HashSetDiff {
            added: [&6, &7, &8].into_iter().collect(),
            removed: [&0, &1, &2].into_iter().collect(),
            common: [&3, &4, &5].into_iter().collect(),
        };
        assert_eq!(expected, changes);
    }

    #[test]
    fn hash_map_diff() {
        let a: HashMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: HashMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = HashMapDiff {
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

        let floats_a: HashMap<K, V> =
            [(K(0), V(1.0)), (K(1), V(1.0)), (K(2), V(1.0))]
                .into_iter()
                .collect();
        let floats_b: HashMap<K, V> =
            [(K(0), V(2.0)), (K(2), V(1.0)), (K(3), V(1.0))]
                .into_iter()
                .collect();

        let diff = floats_a.diff(&floats_b);
        assert_eq!(diff.added.keys().copied().collect::<Vec<_>>(), [&K(3)]);
        assert_eq!(diff.removed.keys().copied().collect::<Vec<_>>(), [&K(1)]);
        // HashMaps have non-deterministic order, so they should be sorted
        // before comparison.
        let mut common = diff.common.keys().copied().collect::<Vec<_>>();
        common.sort();
        assert_eq!(common, [&K(0), &K(2)]);
    }
}
