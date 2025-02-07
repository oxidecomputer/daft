//! Macros for internal implementations.

#[macro_export]
macro_rules! leaf {
    ($($typ:ty),*) => {
        $(
            impl $crate::Diffable for $typ {
                type Diff<'daft> = $crate::Leaf<&'daft Self>;

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

#[cfg(feature = "alloc")]
macro_rules! leaf_deref {
    ($($typ:ty => $target:ty),*) => {
        $(
            impl $crate::Diffable for $typ {
                type Diff<'daft> = $crate::Leaf<&'daft $target>;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    $crate::Leaf {
                        before: &**self,
                        after: &**other
                    }
                }
            }
        )*
    };
}

/// Create a type `<MapType>Diff` and `impl Diffable` on it.
///
/// This is supported for `BTreeMap` and `HashMap`
#[cfg(feature = "alloc")]
macro_rules! map_diff {
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
         paste::paste! {

            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$typ Diff>]<'daft, K: $key_constraint + Eq, V> {
                pub common: $typ<&'daft K, $crate::Leaf<&'daft V>>,
                pub added: $typ<&'daft K, &'daft V>,
                pub removed: $typ<&'daft K, &'daft V>,
            }

            impl<'daft, K: $key_constraint + Eq, V> [<$typ Diff>]<'daft, K, V> {
                pub fn new() -> Self {
                    Self {
                        common: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                    }
                }
            }

            impl<'daft, K: $key_constraint + Eq, V: Eq> [<$typ Diff>]<'daft, K, V> {
                /// Return an iterator over unchanged keys and values.
                pub fn unchanged(&self) -> impl Iterator<Item = (&'daft K, &'daft V)> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        (leaf.before == leaf.after).then_some((*k, leaf.before))
                    })
                }

                /// Return an iterator over unchanged keys.
                pub fn unchanged_keys(&self) -> impl Iterator<Item = &'daft K> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        (leaf.before == leaf.after).then_some(*k)
                    })
                }

                /// Return an iterator over unchanged values.
                pub fn unchanged_values(&self) -> impl Iterator<Item = &'daft V> + '_ {
                    self.common.iter().filter_map(|(_, leaf)| {
                        (leaf.before == leaf.after).then_some(leaf.before)
                    })
                }

                /// Return an iterator over modified keys and values.
                pub fn modified(&self) -> impl Iterator<Item = (&'daft K, $crate::Leaf<&'daft V>)> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        (leaf.before != leaf.after).then_some((*k, *leaf))
                    })
                }

                /// Return an iterator over modified keys.
                pub fn modified_keys(&self) -> impl Iterator<Item = &'daft K> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        (leaf.before != leaf.after).then_some(*k)
                    })
                }

                /// Return an iterator over modified values.
                pub fn modified_values(&self) -> impl Iterator<Item = $crate::Leaf<&'daft V>> + '_ {
                    self.common.iter().filter_map(|(_, leaf)| {
                        (leaf.before != leaf.after).then_some(*leaf)
                    })
                }

                /// Return an iterator over modified keys and values, performing
                /// a diff on the values.
                ///
                /// This is useful when `V::Diff` is a complex type, not just a
                /// [`Leaf`](crate::Leaf).
                #[allow(rustdoc::redundant_explicit_links)] // some macro use sites have Leaf available, some don't
                pub fn modified_diff(&self) -> impl Iterator<Item = (&'daft K, V::Diff<'daft>)> + '_
                where
                    V: Diffable,
                {
                    self.modified().map(|(k, leaf)| (k, leaf.before.diff(&leaf.after)))
                }

                /// Return an iterator over modified values, performing a diff on
                /// them.
                ///
                /// This is useful when `V::Diff` is a complex type, not just a
                /// [`Leaf`](crate::Leaf).
                #[allow(rustdoc::redundant_explicit_links)] // some macro use sites have Leaf available, some don't
                pub fn modified_values_diff(&self) -> impl Iterator<Item = V::Diff<'daft>> + '_
                where
                    V: Diffable,
                {
                    self.modified_values().map(|leaf| leaf.before.diff(&leaf.after))
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K or V to be Default.
            impl<'daft, K: $key_constraint + Eq, V> Default for [<$typ Diff>]<'daft, K, V> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq, V> $crate::Diffable for $typ<K, V>
            {
                type Diff<'daft> = [<$typ Diff>]<'daft, K, V> where K: 'daft, V: 'daft;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    let mut diff = [<$typ Diff>]::new();
                    for (k, v) in self {
                        if let Some(other_v) = other.get(k) {
                            diff.common.insert(k, $crate::Leaf { before: v, after: other_v });
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

/// Create a type `<SetType>Diff` and `impl Diffable` on it.
///
/// This is supported for `BTreeSet` and `HashSet`
/// We use Vecs rather than sets internally to avoid requiring key constraints
/// on `Leafs`
#[cfg(feature = "alloc")]
macro_rules! set_diff {
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
         paste::paste! {

            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$typ Diff>]<'daft, K: $key_constraint + Eq>  {
                pub common: $typ<&'daft K>,
                pub added: $typ<&'daft K>,
                pub removed: $typ<&'daft K>,
            }

            impl<'daft, K: $key_constraint + Eq> [<$typ Diff>]<'daft, K> {
                pub fn new() -> Self {
                    Self {
                        common: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                    }
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K to be Default.
            impl<'daft, K: $key_constraint + Eq> Default for [<$typ Diff>]<'daft, K> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq>
                $crate::Diffable for $typ<K>
            {
                type Diff<'daft> = [<$typ Diff>]<'daft, K> where K: 'daft;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    let mut diff = [<$typ Diff>]::new();
                    diff.removed = self.difference(other).collect();
                    diff.added = other.difference(self).collect();
                    diff.common = self.intersection(other).collect();
                    diff
                }
            }

        }
        )*
    }
}
