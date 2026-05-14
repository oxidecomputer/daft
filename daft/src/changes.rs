//! Projecting a diff to the subset of nodes that actually changed.
//!
//! See [`IntoChanges`] for an overview.

use crate::Leaf;

/// Project a diff to the subset of nodes that have changed.
///
/// Daft's diff types preserve the full *before*/*after* structure regardless
/// of whether anything changed. That is the right representation for
/// in-memory analysis, but a poor fit for storage and transport: a diff of
/// two large structures with one altered field still serializes both sides
/// in full.
///
/// `IntoChanges` describes the dual representation. For each diff type
/// there is a corresponding *changes* type — `Leaf<&T>` maps to itself,
/// [`BTreeMapDiff`](crate::BTreeMapDiff) maps to
/// [`BTreeMapChanges`](crate::BTreeMapChanges), a derived `FooDiff` maps to a
/// generated `FooChanges`, and so on. Calling [`into_changes`][Self::into_changes]
/// drops every unchanged subtree and returns [`Some`] only if anything
/// remained. [`None`] is reserved for "no changes at all".
///
/// The trait requires `Eq` at the leaves: there is no way to tell whether a
/// `Leaf<&T>` represents a change without comparing the two sides.
///
/// With the `serde` feature, the `*Changes` types implement [`Serialize`].
/// `None` fields are skipped at serialization time, so unchanged subtrees
/// are omitted entirely from the serialized output.
///
/// [`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
///
/// # Example
///
/// ```
/// # #[cfg(feature = "std")] {
/// use daft::{Diffable, IntoChanges, Leaf};
/// use std::collections::BTreeMap;
///
/// let before: BTreeMap<u32, &str> =
///     [(1, "alpha"), (2, "beta"), (3, "gamma")].into_iter().collect();
/// let after: BTreeMap<u32, &str> =
///     [(1, "alpha"), (2, "BETA"), (4, "delta")].into_iter().collect();
///
/// let changes =
///     before.diff(&after).into_changes().expect("the map changed");
///
/// // `common` only retains entries whose value differs.
/// assert_eq!(
///     changes.common,
///     [(&2, Leaf { before: &"beta", after: &"BETA" })]
///         .into_iter()
///         .collect(),
/// );
/// assert_eq!(changes.added, [(&4, &"delta")].into_iter().collect());
/// assert_eq!(changes.removed, [(&3, &"gamma")].into_iter().collect());
///
/// // Diffing a map against itself yields no changes at all.
/// assert!(before.diff(&before).into_changes().is_none());
/// # }
/// ```
pub trait IntoChanges {
    /// The "changes-only" representation of this diff.
    ///
    /// For terminal types (`Leaf<&T>`, primitive diffs) this is typically
    /// `Self`. For composite diffs, it is a parallel struct in which each
    /// field is the corresponding child's [`Changes`][Self::Changes] wrapped
    /// in [`Option`] so unchanged subtrees can be skipped.
    type Changes;

    /// Drop unchanged subtrees from the diff.
    ///
    /// Returns [`None`] if every leaf is unchanged, and [`Some`] containing
    /// the projected representation otherwise.
    fn into_changes(self) -> Option<Self::Changes>;
}

impl<T: Eq + ?Sized> IntoChanges for Leaf<&T> {
    type Changes = Self;

    #[inline]
    fn into_changes(self) -> Option<Self::Changes> {
        if self.before == self.after {
            None
        } else {
            Some(self)
        }
    }
}

impl<T: Eq> IntoChanges for Leaf<Option<&T>> {
    type Changes = Self;

    #[inline]
    fn into_changes(self) -> Option<Self::Changes> {
        if self.before == self.after {
            None
        } else {
            Some(self)
        }
    }
}

impl<T: Eq, U: Eq> IntoChanges for Leaf<Result<&T, &U>> {
    type Changes = Self;

    #[inline]
    fn into_changes(self) -> Option<Self::Changes> {
        if self.before == self.after {
            None
        } else {
            Some(self)
        }
    }
}

macro_rules! tuple_into_changes {
    ($(($($name:ident $ix:tt),+)),+ $(,)?) => {
        $(
            impl<$($name: IntoChanges),+> IntoChanges for ($($name,)+) {
                type Changes = ($(Option<$name::Changes>,)+);

                fn into_changes(self) -> Option<Self::Changes> {
                    let tup = ($(self.$ix.into_changes(),)+);
                    if $(tup.$ix.is_none())&&+ {
                        None
                    } else {
                        Some(tup)
                    }
                }
            }
        )+
    }
}

tuple_into_changes! {
    (A 0),
    (A 0, B 1),
    (A 0, B 1, C 2),
    (A 0, B 1, C 2, D 3),
    (A 0, B 1, C 2, D 3, E 4),
    (A 0, B 1, C 2, D 3, E 4, F 5),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Diffable;

    #[test]
    fn leaf_changes() {
        let leaf: Leaf<&i32> = 1.diff(&1);
        assert_eq!(leaf.into_changes(), None);

        let leaf: Leaf<&i32> = 1.diff(&2);
        assert_eq!(leaf.into_changes(), Some(Leaf { before: &1, after: &2 }));
    }

    #[test]
    fn tuple_changes() {
        let before = (1, 2);
        let after = (1, 2);
        let diff = before.diff(&after);
        assert_eq!(diff.into_changes(), None);

        let before = (1, 2);
        let after = (1, 3);
        let diff = before.diff(&after);
        assert_eq!(
            diff.into_changes(),
            Some((None, Some(Leaf { before: &2, after: &3 }))),
        );
    }
}
