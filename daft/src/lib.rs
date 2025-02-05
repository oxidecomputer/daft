#![cfg_attr(doc_cfg, feature(doc_auto_cfg))]

//! Daft is a library to perform semantic diffs of Rust data structures.
//!
//! Daft consists of a trait called [`Diffable`], along with [a derive
//! macro][macro@Diffable] by the same name. This trait represents the
//! notion of a type for which two members can be simultaneously compared.
//!
//! # Usage
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! // Annotate your struct with `#[derive(Diffable)]`:
//! #[derive(Diffable)]
//! struct MyStruct {
//!     a: i32,
//!     b: &'static str,
//! }
//!
//! // This generates a type called MyStructDiff, which looks like:
//! # /*
//! #[automatically_derived]
//! struct MyStructDiff<'daft> {
//!     a: Leaf<'daft, i32>,
//!     b: Leaf<'daft, &'static str>,
//! }
//! # */
//!
//! // Then, with two instances of MyStruct:
//! let before = MyStruct { a: 1, b: "hello" };
//! let after = MyStruct { a: 2, b: "world" };
//!
//! // You can diff them like so:
//! let diff = before.diff(&after);
//!
//! // And compare the results:
//! assert_eq!(*diff.a.before, 1);
//! assert_eq!(*diff.a.after, 2);
//! assert_eq!(diff.b.before, "hello");
//! assert_eq!(diff.b.after, "world");
//! ```
//!
//! This crate assigns one side the name *before*, and the other side *after*.
//! These labels are arbitrary: if *before* and *after* are swapped, the diff is
//! reversed.
//!
//! ## Diff types
//!
//! Currently, daft comes with a few kinds of diff types:
//!
//! ### [`Leaf`] instances
//!
//! A [`Leaf`] represents a logical *leaf node* or *base case* in a diff, i.e. a
//! point at which diffing stops. [`Leaf`] instances are used for:
//!
//! * *Scalar* or *primitive types* like `i32`, `String`, `bool`, etc.
//! * *Enums*, since diffing across variants is usually not meaningful.
//! * Vector and slice types, since there are several reasonable ways to diff
//!   vectors (e.g. set-like, ordered, etc.) and we don't want to make assumptions.
//! * Any point at which you want to terminate recursion, via the `#[daft(leaf)]` attribute.
//!
//! #### Example
//!
//! A contrived example for integers:
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! let diff: Leaf<'_, i32> = 1_i32.diff(&2);
//! assert_eq!(*diff.before, 1);
//! assert_eq!(*diff.after, 2);
//! ```
//!
//! Enums also use `Leaf`:
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! // Option<T> uses Leaf:
//! let diff: Leaf<'_, Option<i32>> = Some(1_i32).diff(&Some(2));
//! assert_eq!(*diff.before, Some(1));
//! assert_eq!(*diff.after, Some(2));
//!
//! // Automatically derived enums also use Leaf:
//! #[derive(Debug, PartialEq, Eq, Diffable)]
//! enum MyEnum {
//!     A(i32),
//!     B(String),
//! }
//!
//! let before = MyEnum::A(1);
//! let after = MyEnum::B("hello".to_string());
//!
//! let diff: Leaf<'_, MyEnum> = before.diff(&after);
//! assert_eq!(diff.before, &before);
//! assert_eq!(diff.after, &after);
//! ```
//!
//! Vectors use `Leaf` as well:
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! let before = vec![1, 2, 3];
//! let after = vec![4, 5, 6];
//! let diff: Leaf<'_, Vec<i32>> = before.diff(&after);
//! assert_eq!(diff.before, &before);
//! assert_eq!(diff.after, &after);
//! ```
//!
//! ### Map diffs
//!
//! For [`BTreeMap`] and [`HashMap`], daft has corresponding [`BTreeMapDiff`]
//! and [`HashMapDiff`] types. These types have fields for *unchanged*, *added*,
//! *removed*, and *modified* entries.
//!
//! Map diffs are performed eagerly.
//!
//! #### Example
//!
//! ```rust
//! use daft::{Diffable, Leaf, BTreeMapDiff};
//! use std::collections::BTreeMap;
//!
//! let mut a = BTreeMap::new();
//! a.insert(1, "one");
//! a.insert(2, "two");
//! a.insert(3, "three");
//!
//! let mut b = BTreeMap::new();
//! b.insert(2, "two");
//! b.insert(3, "THREE");
//! b.insert(4, "four");
//!
//! let diff: BTreeMapDiff<'_, i32, &str> = a.diff(&b);
//!
//! // Unchanged, added, and removed entries are stored as maps:
//! assert_eq!(diff.unchanged, [(&2, &"two")].into_iter().collect());
//! assert_eq!(diff.added, [(&4, &"four")].into_iter().collect());
//! assert_eq!(diff.removed, [(&1, &"one")].into_iter().collect());
//!
//! // Modified entries are stored via the values' diff types:
//! assert_eq!(
//!     diff.modified,
//!     [(&3, Leaf { before: "three", after: "THREE" })].into_iter().collect(),
//! );
//! ```
//!
//! ### Set diffs
//!
//! For [`BTreeSet`] and [`HashSet`], daft has corresponding [`BTreeSetDiff`]
//! and [`HashSetDiff`] types. These types have fields for unchanged, added and
//! removed entries.
//!
//! Set diffs are performed eagerly.
//!
//! #### Example
//!
//! ```rust
//! use daft::{Diffable, Leaf, BTreeSetDiff};
//! use std::collections::BTreeSet;
//!
//! let a: BTreeSet<i32> = [0, 1, 2, 3, 4, 5].into_iter().collect();
//! let b: BTreeSet<i32> = [3, 4, 5, 6, 7, 8].into_iter().collect();
//! let diff: BTreeSetDiff<'_, i32> = a.diff(&b);
//!
//! assert_eq!(diff.unchanged, [&3, &4, &5].into_iter().collect::<Vec<_>>());
//! assert_eq!(diff.added, [&6, &7, &8].into_iter().collect::<Vec<_>>());
//! assert_eq!(diff.removed, [&0, &1, &2].into_iter().collect::<Vec<_>>());
//! ```
//!
//! ### Recursive diffs
//!
//! For structs, the [`Diffable`][macro@Diffable] derive macro generates
//! a diff type with a field corresponding to each field type. Each field must
//! implement [`Diffable`].
//!
//! A struct `Foo` gets a corresponding `FooDiff` struct, which has fields
//! corresponding to each field in `Foo`.
//!
//! #### Example
//!
//! For an example of structs with named fields, see [*Usage*](#usage) above.
//!
//! Tuple-like structs produce tuple-like diff structs:
//!
//! ```rust
//! use daft::Diffable;
//! use std::collections::BTreeMap;
//!
//! #[derive(Diffable)]
//! struct MyTuple(BTreeMap<i32, &'static str>, i32);
//!
//! let before = MyTuple(BTreeMap::new(), 1);
//! let after = MyTuple([(1, "hello")].into_iter().collect(), 2);
//! let diff = before.diff(&after);
//!
//! // The generated type is MyTupleDiff(BTreeMapDiff<i32, &str>, Leaf<i32>).
//! assert_eq!(**diff.0.added.get(&1).unwrap(), "hello");
//! assert_eq!(*diff.1.before, 1);
//! assert_eq!(*diff.1.after, 2);
//! ```
//!
//! ### Custom diff types
//!
//! The [`Diffable`] trait can also be implemented manually for custom behavior.
//!
//! In general, most custom implementations will likely use one of the built-in
//! diff types directly.
//!
//! ### Example
//!
//! Some structs like identifiers should be treated as leaf nodes:
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! struct Identifier(String);
//!
//! impl Diffable for Identifier {
//!     type Diff<'daft> = Leaf<'daft, Self>;
//!
//!     fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
//!         Leaf {
//!             before: self,
//!             after: other,
//!         }
//!     }
//! }
//! ```
//!
//! ## Type and lifetime parameters
//!
//! If a type parameter is specified, the [`Diffable`][macro@Diffable]
//! derive macro for structs normally requires that the type parameter implement
//! `Diffable`. An exception is if the field is annotated with `#[daft(leaf)]`.
//!
//! Daft fully supports types with arbitrary lifetimes. Automatically generated
//! diff structs will have an additional `'daft` lifetime parameter at the
//! beginning, with the requirement that all other lifetime and type parameters
//! outlive it.
//!
//! ### Example
//!
//! ```rust
//! use daft::Diffable;
//!
//! #[derive(Diffable)]
//! struct BorrowedData<'a, 'b, T: Diffable + ?Sized> {
//!     a: &'a str,
//!     b: &'b T,
//!     // TODO: example with daft(leaf)
//! }
//!
//! // This generates a struct that looks like:
//! # /*
//! #[automatically_derived]
//! struct BorrowedDataDiff<'daft, 'a: 'daft, 'b: 'daft, T: ?Sized + 'daft> {
//!     a: Leaf<'daft, &'a str>,
//!     b: T::Diff<'daft>,
//! }
//! # */
//! ```
//!
//! # Optional features
//!
//! Implementations for foreign types:
//!
//! * `uuid1`: Enable diffing for [`uuid::Uuid`]. *Disabled by default.*
//! * `oxnet01`: Enable diffing for network types from the [`oxnet`] crate. *Disabled by default.*
//! * `newtype-uuid1`: Enable diffing for [`newtype_uuid::TypedUuid`]. *Disabled by default.*
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The minimum supported Rust version is **1.81.0**. At any time, at least the
//! last three stable versions of Rust are supported.
//!
//! While this crate is a pre-release (0.x.x) it may have its MSRV bumped in a
//! patch release. Once this crate has reached 1.x, any MSRV bump will be
//! accompanied with a new minor version.
//!
//! # Related work
//!
//! [Diffus](https://crates.io/crates/diffus) is the original inspiration for
//! this crate and a great alternative. Daft diverges from diffus in a few ways:
//!
//! * Daft's derive macro does not attempt to diff enums with different variants.
//!   In practice, we've found that diffing enums across different variants is less
//!   useful than it first appears.
//!
//! * Daft has the notion of a [`Leaf`] type, which represents an atomic unit.
//!   (For example, the [`Diffable`] implementation for `i32` is a [`Leaf`].)
//!   [`Leaf`]s are also used for enums, as well as in any other place where lazy
//!   diffing is desired.
//!
//! * Diffus has a `Same` trait, which is like `Eq` except it's also implemented
//!   for floats. Daft doesn't have the `Same` trait, and in fact mostly forgoes
//!   any trait requirements: the only places where `Eq` is required is for maps
//!   (both keys and values) and sets.
//!
//!   For a primitive scalar like `f64`, you'll get a `Leaf` struct which you can
//!   compare with whatever notion of equality you want.
//!
//! * Daft uses a [generic associated type (GAT)][GAT] so that the `Diffable`
//!   trait no longer needs a lifetime parameter. This leads to simpler usage.
//!   (Diffus was written before GATs were available in stable Rust.)
//!
//! * Daft uses fewer types in general. For example, diffus wraps its return values
//!   in an outer `Edit` type, but daft does not.
//!
//! [GAT]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html

pub use daft_derive::*;
use paste::paste;
use std::{
    borrow::Cow,
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

/// Represents a type which can be diffed.
///
/// For more information, see the [crate-level documentation](crate).
pub trait Diffable {
    /// The type of the diff.
    ///
    /// This is a [generic associated type][GAT], also known as a GAT. The
    /// `'daft` lifetime is used in the `diff` method to ensure that the
    /// returned diff is valid for the lifetime of the input values.
    ///
    /// [GAT]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html
    type Diff<'daft>
    where
        Self: 'daft;

    /// Compute the diff between two values.
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft>;
}

/// A primitive or atomic change.
///
/// For more information, see the [crate-level documentation](crate).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Leaf<'daft, T: ?Sized> {
    pub before: &'daft T,
    pub after: &'daft T,
}

#[macro_export]
macro_rules! leaf{
    ($($(#[$($outer:meta)*])* $typ:ty),*) => {
        $(
            $(#[$($outer)*])*
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

leaf! { i64, i32, i16, i8, u64, u32, u16, u8, char, bool, isize, usize, () }
leaf! { IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6 }
leaf! { String, str, PathBuf, Path, OsString, OsStr }

// Use attributes inside the macro (rather than a single cfg(feature = ...)
// outside the macro) to allow `doc_auto_cfg` to work properly.
leaf! {
    #[cfg(feature = "oxnet01")]
    oxnet::IpNet,
    #[cfg(feature = "oxnet01")]
    oxnet::Ipv4Net,
    #[cfg(feature = "oxnet01")]
    oxnet::Ipv6Net
}
leaf! { #[cfg(feature = "uuid1")] uuid::Uuid }

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

#[cfg(feature = "newtype-uuid1")]
impl<T> Diffable for newtype_uuid::TypedUuid<T>
where
    T: newtype_uuid::TypedUuidKind + Diffable,
{
    type Diff<'daft> = Leaf<'daft, newtype_uuid::TypedUuid<T>>;

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

    #[cfg(feature = "uuid1")]
    #[test]
    fn example_struct() {
        use uuid::Uuid;

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
