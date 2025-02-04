pub use daft_derive::*;
use newtype_uuid::{TypedUuid, TypedUuidKind};
use paste::paste;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ffi::{OsStr, OsString},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub trait Diffable {
    type Diff<'daft>
    where
        Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft>;
}

/// A primitive change
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Leaf<'daft, T: ?Sized> {
    pub before: &'daft T,
    pub after: &'daft T,
}

#[macro_export]
macro_rules! leaf{
    ($($typ:ty),*) => {
        $(
            impl $crate::Diffable for $typ {
                type Diff<'daft> = $crate::Leaf<'daft, Self>;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
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
leaf! { String, str, PathBuf, Path, OsString, OsStr }

impl<T> Diffable for Option<T> {
    type Diff<'daft>
        = Leaf<'daft, Option<T>>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T, U> Diffable for Result<T, U> {
    type Diff<'daft>
        = Leaf<'daft, Result<T, U>>
    where
        T: 'daft,
        U: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T: Diffable + ?Sized> Diffable for Box<T> {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
    }
}

impl<'a, T: Diffable + ?Sized> Diffable for &'a T {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        &'a T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
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

// Can't express lifetimes due to `RefCell`'s limited borrows, so we must return
// a leaf node that can be recursively diffed.
impl<T: ?Sized> Diffable for RefCell<T> {
    type Diff<'daft>
        = Leaf<'daft, Self>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T: ?Sized> Diffable for PhantomData<T> {
    type Diff<'daft>
        = Leaf<'daft, PhantomData<T>>
    where
        Self: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T> Diffable for TypedUuid<T>
where
    T: TypedUuidKind + Diffable,
{
    type Diff<'daft> = Leaf<'daft, TypedUuid<T>>;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
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
            pub struct [<$typ Diff>]<'daft, K: $key_constraint + Eq, V: Diffable> {
                pub unchanged: $typ<&'daft K, &'daft V>,
                pub added: $typ<&'daft K, &'daft V>,
                pub removed: $typ<&'daft K, &'daft V>,
                pub modified: $typ<&'daft K, V::Diff<'daft>>,
            }

            impl<'daft, K: $key_constraint + Eq, V: Diffable> [<$typ Diff>]<'daft, K, V> {
                pub fn new() -> Self {
                    Self {
                        unchanged: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                        modified: $typ::new(),
                    }
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K or V to be Default.
            impl<'daft, K: $key_constraint + Eq, V: Diffable> Default for [<$typ Diff>]<'daft, K, V> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<
                 K: $key_constraint + Eq,
                 V: Diffable + Eq>
                     $crate::Diffable for $typ<K, V>
            {
                type Diff<'daft> = [<$typ Diff>]<'daft, K, V> where K: 'daft, V: 'daft;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
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
            pub struct [<$typ Diff>]<'daft, K: Diffable>  {
                pub unchanged: Vec<&'daft K>,
                pub added: Vec<&'daft K>,
                pub removed: Vec<&'daft K>,
            }

            impl<'daft, K: Diffable> [<$typ Diff>]<'daft, K> {
                pub fn new() -> Self {
                    Self {
                        unchanged: Vec::new(),
                        added: Vec::new(),
                        removed: Vec::new(),
                    }
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K to be Default.
            impl<'daft, K: Diffable> Default for [<$typ Diff>]<'daft, K> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq + Diffable>
                $crate::Diffable for $typ<K>
            {
                type Diff<'daft> = [<$typ Diff>]<'daft, K> where K: 'daft;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
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
impl<T: Diffable> Diffable for Vec<T> {
    type Diff<'daft>
        = Leaf<'daft, Vec<T>>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

/// Treat slices as leaf nodes.
impl<T: Diffable> Diffable for [T] {
    type Diff<'daft>
        = Leaf<'daft, [T]>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

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
        struct TestStructDiff<'daft> {
            id: Leaf<'daft, Uuid>,
            sled_state: BTreeMapDiff<'daft, Uuid, SledState>,
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
