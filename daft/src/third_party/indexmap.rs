use crate::Diffable;
use core::hash::Hash;
use indexmap::{IndexMap, IndexSet};

map_diff!(IndexMap, Hash);
set_diff!(IndexSet, Hash);

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
