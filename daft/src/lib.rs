pub use daft_derive::*;
use newtype_uuid::{TypedUuid, TypedUuidKind};
use paste::paste;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
};

pub trait Diffable<'a>: PartialEq + Eq {
    type Diff: 'a + Eq + Debug;
    fn diff(&'a self, other: &'a Self) -> Self::Diff;
}

/// A primitive change
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Leaf<'a, T: Eq + Debug> {
    pub before: &'a T,
    pub after: &'a T,
}

#[macro_export]
macro_rules! leaf{
    ($($typ:ty),*) => {
        $(
            impl<'a> $crate::Diffable<'a> for $typ {
                type Diff = $crate::Leaf<'a, Self>;

                fn diff(&'a self, other: &'a Self) -> Self::Diff {
                    $crate::Leaf {
                        before: self,
                        after: other
                    }
                }
            }
        )*
    }
}

leaf! { i64, i32, i16, i8, u64, u32, u16, u8, char, bool, isize, usize, (), uuid::Uuid}
leaf! { IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6 }
leaf! { oxnet::IpNet, oxnet::Ipv4Net, oxnet::Ipv6Net }
leaf! { String }

impl<'a, T: Eq + Debug + 'a> Diffable<'a> for Option<T> {
    type Diff = Leaf<'a, Option<T>>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        Leaf { before: self, after: other }
    }
}

impl<'a, T: Eq + Debug + 'a, U: Eq + Debug + 'a> Diffable<'a> for Result<T, U> {
    type Diff = Leaf<'a, Result<T, U>>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        Leaf { before: self, after: other }
    }
}

impl<'a, T> Diffable<'a> for TypedUuid<T>
where
    T: TypedUuidKind + Diffable<'a>,
{
    type Diff = Leaf<'a, TypedUuid<T>>;

    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        Leaf { before: self, after: other }
    }
}

/// Create a type `<MapType>Diff` and `impl Diffable` on it.
///
/// This is supported for `BTreeMap` and `HashMap`
macro_rules! map_diff {
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
         paste! {

            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$typ Diff>]<'a, K: $key_constraint + Eq, V: Diffable<'a>> {
                pub unchanged: $typ<&'a K, &'a V>,
                pub added: $typ<&'a K, &'a V>,
                pub removed: $typ<&'a K, &'a V>,
                pub modified: $typ<&'a K, V::Diff>,
            }

            impl<'a, K: $key_constraint + Eq, V: Diffable<'a>> [<$typ Diff>]<'a, K, V> {
                pub fn new() -> Self {
                    Self {
                        unchanged: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                        modified: $typ::new(),
                    }
                }
            }

            impl<
                'a,
                 K: $key_constraint + Eq + Debug + 'a,
                 V: Diffable<'a> + Debug + 'a>
                     $crate::Diffable<'a> for $typ<K, V>
            {
                type Diff = [<$typ Diff>]<'a, K, V>;

                fn diff(&'a self, other: &'a Self) -> Self::Diff {
                    let mut diff = [<$typ Diff>]::new();
                    for (k, v) in self {
                        if let Some(other_v) = other.get(k) {
                            if v != other_v {
                                diff.modified.insert(k, v.diff(other_v));
                            } else {
                                diff.unchanged.insert(k, v);
                            }
                        } else {
                            diff.removed.insert(k, v);
                        }
                    }
                    for (k, v) in other {
                        if !self.contains_key(k) {
                            diff.added.insert(k, v);
                        }
                    }
                    diff
                }
            }

        }
        )*
    }
}

map_diff!((BTreeMap, Ord), (HashMap, Hash));

/// Create a type `<SetType>Diff` and `impl Diffable` on it.
///
/// This is supported for `BTreeSet` and `HashSet`
/// We use Vecs rather than sets internally to avoid requiring key constraints
/// on `Leafs`
macro_rules! set_diff{
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
         paste! {

            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$typ Diff>]<'a, K: Diffable<'a>>  {
                pub unchanged: Vec<&'a K>,
                pub added: Vec<&'a K>,
                pub removed: Vec<&'a K>,
            }

            impl<'a, K: Diffable<'a> + Debug> [<$typ Diff>]<'a, K> {
                pub fn new() -> Self {
                    Self {
                        unchanged: Vec::new(),
                        added: Vec::new(),
                        removed: Vec::new(),
                    }
                }
            }

            impl<'a, K: $key_constraint + Debug + Diffable<'a> + 'a>
                $crate::Diffable<'a> for $typ<K>
            {
                type Diff = [<$typ Diff>]<'a, K>;

                fn diff(&'a self, other: &'a Self) -> Self::Diff {
                    let mut diff = [<$typ Diff>]::new();
                    diff.removed = self.difference(other).collect();
                    diff.added = other.difference(self).collect();
                    diff.unchanged = self.intersection(other).collect();
                    diff
                }
            }

        }
        )*
    }
}

set_diff!((BTreeSet, Ord), (HashSet, Hash));

/// Treat Vecs as Leafs
//
// We plan to add opt in diff functionality: set-like, reordered, etc...
impl<'a, T: Diffable<'a> + 'a + Debug> Diffable<'a> for Vec<T> {
    type Diff = Leaf<'a, Vec<T>>;

    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        Leaf { before: self, after: other }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_sets() {
        let a: BTreeSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: BTreeSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let changes = a.diff(&b);
        let expected = BTreeSetDiff {
            added: [&6, &7, &8].into_iter().collect(),
            removed: [&0, &1, &2].into_iter().collect(),
            unchanged: [&3, &4, &5].into_iter().collect(),
        };
        assert_eq!(expected, changes);

        let a: HashSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: HashSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let mut changes = a.diff(&b);
        // HashSet output must be sorted for comparison
        changes.unchanged.sort_unstable();
        changes.added.sort_unstable();
        changes.removed.sort_unstable();

        let expected = HashSetDiff {
            added: [&6, &7, &8].into_iter().collect(),
            removed: [&0, &1, &2].into_iter().collect(),
            unchanged: [&3, &4, &5].into_iter().collect(),
        };
        assert_eq!(expected, changes);
    }

    #[test]
    fn test_maps() {
        let a: BTreeMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: BTreeMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = BTreeMapDiff {
            unchanged: [(&2, &1)].into_iter().collect(),
            added: [(&3, &1)].into_iter().collect(),
            removed: [(&1, &1)].into_iter().collect(),
            modified: [(&0, Leaf { before: &1, after: &2 })]
                .into_iter()
                .collect(),
        };

        assert_eq!(changes, expected);

        let a: HashMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: HashMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = HashMapDiff {
            unchanged: [(&2, &1)].into_iter().collect(),
            added: [(&3, &1)].into_iter().collect(),
            removed: [(&1, &1)].into_iter().collect(),
            modified: [(&0, Leaf { before: &1, after: &2 })]
                .into_iter()
                .collect(),
        };

        assert_eq!(changes, expected);
    }

    #[test]
    fn example_struct() {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum SledState {
            Active,
            Decommissioned,
        }
        leaf!(SledState);

        #[derive(Debug, Clone)]
        struct TestStruct {
            id: Uuid,
            sled_state: BTreeMap<Uuid, SledState>,
        }

        // This is what daft-derive should generate
        // for `TestStruct`
        #[derive(Debug)]
        struct TestStructDiff<'a> {
            id: Leaf<'a, Uuid>,
            sled_state: BTreeMapDiff<'a, Uuid, SledState>,
        }

        let sled_states = vec![
            (Uuid::new_v4(), SledState::Active),
            (Uuid::new_v4(), SledState::Active),
            (Uuid::new_v4(), SledState::Decommissioned),
        ];

        let a = TestStruct {
            id: Uuid::new_v4(),
            sled_state: sled_states.clone().into_iter().collect(),
        };
        let mut b = a.clone();
        b.id = Uuid::new_v4();
        *(b.sled_state.get_mut(&sled_states[0].0).unwrap()) =
            SledState::Decommissioned;
        b.sled_state.insert(Uuid::new_v4(), SledState::Active);

        let diff = TestStructDiff {
            id: a.id.diff(&b.id),
            sled_state: a.sled_state.diff(&b.sled_state),
        };

        assert_ne!(diff.id.before, diff.id.after);
        assert_eq!(diff.sled_state.unchanged.len(), 2);
        assert_eq!(diff.sled_state.added.len(), 1);
        assert_eq!(diff.sled_state.removed.len(), 0);
        assert_eq!(diff.sled_state.modified.len(), 1);
    }
}
