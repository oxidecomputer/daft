use std::collections::{BTreeMap, BTreeSet};

pub trait Diffable<'a>: PartialEq + Eq {
    type Diff: 'a;
    fn diff(&'a self, other: &'a Self) -> Self::Diff;
}

/// A primitive change
#[derive(Debug, PartialEq, Eq)]
pub struct Leaf<'a, T: PartialEq + Eq> {
    pub before: &'a T,
    pub after: &'a T,
}

// TODO: macro for primitives
impl<'a> Diffable<'a> for i32 {
    type Diff = Leaf<'a, Self>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        Leaf {
            before: self,
            after: other,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumChange<'a, T: ?Sized, Diff> {
    // Variant changes mean that we never diff the variant data if there is any
    Variant { before: &'a T, after: &'a T },

    // Associated data changes mean that we recurse further
    AssociatedData(Diff),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MapChange<'a, K, V: Diffable<'a>> {
    Insert((&'a K, &'a V)),
    Remove((&'a K, &'a V)),
    Change((&'a K, V::Diff)),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SetChange<'a, T: 'a> {
    Insert(&'a T),
    Remove(&'a T),
}

impl<'a, K: Ord + 'a, V: Diffable<'a> + 'a> Diffable<'a> for BTreeMap<K, V> {
    type Diff = Vec<MapChange<'a, K, V>>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        let mut changes = vec![];
        for (k, v) in self {
            if let Some(other_v) = other.get(k) {
                if v != other_v {
                    changes.push(MapChange::Change((k, v.diff(other_v))));
                }
            } else {
                changes.push(MapChange::Remove((k, v)));
            }
        }
        for (k, v) in other {
            if !self.contains_key(k) {
                changes.push(MapChange::Insert((k, v)));
            }
        }
        changes
    }
}

impl<'a, T: Diffable<'a> + 'a> Diffable<'a> for Option<T> {
    type Diff = Option<EnumChange<'a, Option<T>, T::Diff>>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        match (self, other) {
            (None, None) => None,
            (Some(a), Some(b)) => {
                if a == b {
                    None
                } else {
                    Some(EnumChange::AssociatedData(a.diff(b)))
                }
            }
            _ => Some(EnumChange::Variant {
                before: self,
                after: other,
            }),
        }
    }
}

impl<'a, T: Ord + 'a> Diffable<'a> for BTreeSet<T> {
    type Diff = Vec<SetChange<'a, T>>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        let mut changes: Vec<_> = self
            .difference(other)
            .map(|e| SetChange::Remove(e))
            .collect();
        changes.extend(other.difference(self).map(SetChange::Insert));
        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let a: BTreeSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: BTreeSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let changes = a.diff(&b);
        let expected = vec![
            SetChange::Remove(&0),
            SetChange::Remove(&1),
            SetChange::Remove(&2),
            SetChange::Insert(&6),
            SetChange::Insert(&7),
            SetChange::Insert(&8),
        ];
        assert_eq!(expected, changes);
    }

    #[test]
    fn test_map() {
        let a: BTreeMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: BTreeMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = vec![
            MapChange::Change((
                &0,
                Leaf {
                    before: &1,
                    after: &2,
                },
            )),
            MapChange::Remove((&1, &1)),
            MapChange::Insert((&3, &1)),
        ];

        assert_eq!(changes, expected);
    }

    #[test]
    fn test_option() {
        let a = Some(4);
        let b = None;
        let diff = a.diff(&b);
        let expected = Some(EnumChange::Variant {
            before: &Some(4),
            after: &None,
        });
        assert_eq!(diff, expected);

        let a = Some(4);
        let b = Some(5);
        let diff = a.diff(&b);
        let expected = Some(EnumChange::AssociatedData(Leaf {
            before: &4,
            after: &5,
        }));
        assert_eq!(diff, expected);

        let a: Option<i32> = None;
        let b = None;
        let diff = a.diff(&b);
        assert!(diff.is_none());

        let a = Some(4);
        let b = Some(4);
        let diff = a.diff(&b);
        assert!(diff.is_none());
    }
}
