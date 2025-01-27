use std::collections::{BTreeMap, BTreeSet};
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
};

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

#[macro_export]
macro_rules! leaf{
    ($($typ:ty),*) => {
        $(
            impl<'a> Diffable<'a> for $typ {
                type Diff = Leaf<'a, Self>;

                fn diff(&'a self, other: &'a Self) -> Self::Diff {
                    Leaf {
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

#[derive(Debug, PartialEq, Eq)]
pub struct MapDiff<'a, K, V: Diffable<'a>> {
    pub unchanged: Vec<(&'a K, &'a V)>,
    pub added: Vec<(&'a K, &'a V)>,
    pub removed: Vec<(&'a K, &'a V)>,
    pub modified: Vec<(&'a K, V::Diff)>,
}

impl<'a, K, V: Diffable<'a>> MapDiff<'a, K, V> {
    pub fn new() -> MapDiff<'a, K, V> {
        MapDiff {
            unchanged: vec![],
            added: vec![],
            removed: vec![],
            modified: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SetDiff<'a, T: 'a> {
    pub unchanged: Vec<&'a T>,
    pub added: Vec<&'a T>,
    pub removed: Vec<&'a T>,
}

impl<'a, T: 'a> SetDiff<'a, T> {
    pub fn new() -> SetDiff<'a, T> {
        SetDiff { unchanged: vec![], added: vec![], removed: vec![] }
    }
}

impl<'a, K: Ord + 'a, V: Diffable<'a> + 'a> Diffable<'a> for BTreeMap<K, V> {
    type Diff = MapDiff<'a, K, V>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        let mut diff = MapDiff::new();
        for (k, v) in self {
            if let Some(other_v) = other.get(k) {
                if v != other_v {
                    diff.modified.push((k, v.diff(other_v)));
                } else {
                    diff.unchanged.push((k, v));
                }
            } else {
                diff.removed.push((k, v));
            }
        }
        for (k, v) in other {
            if !self.contains_key(k) {
                diff.added.push((k, v));
            }
        }
        diff
    }
}

impl<'a, T: Ord + 'a> Diffable<'a> for BTreeSet<T> {
    type Diff = SetDiff<'a, T>;
    fn diff(&'a self, other: &'a Self) -> Self::Diff {
        let mut diff = SetDiff::new();
        diff.removed = self.difference(other).collect();
        diff.added = other.difference(self).collect();
        diff.unchanged = self.intersection(other).collect();
        diff
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_set() {
        let a: BTreeSet<_> = [0, 1, 2, 3, 4, 5].into_iter().collect();
        let b: BTreeSet<_> = [3, 4, 5, 6, 7, 8].into_iter().collect();
        let changes = a.diff(&b);
        let expected = SetDiff {
            added: vec![&6, &7, &8],
            removed: vec![&0, &1, &2],
            unchanged: vec![&3, &4, &5],
        };
        assert_eq!(expected, changes);
    }

    #[test]
    fn test_map() {
        let a: BTreeMap<_, _> = [(0, 1), (1, 1), (2, 1)].into_iter().collect();
        let b: BTreeMap<_, _> = [(0, 2), (2, 1), (3, 1)].into_iter().collect();

        let changes = a.diff(&b);
        let expected = MapDiff {
            unchanged: vec![(&2, &1)],
            added: vec![(&3, &1)],
            removed: vec![(&1, &1)],
            modified: vec![(&0, Leaf { before: &1, after: &2 })],
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
            sled_state: MapDiff<'a, Uuid, SledState>,
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

pub use daft_derive::*;
