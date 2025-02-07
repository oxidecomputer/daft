#![cfg_attr(doc_cfg, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! Daft is a library to perform semantic diffs of Rust data structures.
//!
//! Daft consists of a trait called [`Diffable`], along with [a derive
//! macro][macro@Diffable] by the same name. This trait represents the
//! notion of a type for which two members can be simultaneously compared.
//!
//! # Features
//!
//! - **Recursive diffing** of structs, sets, and maps
//! - **Derive macro** for automatically generating diff types
//! - Choose between **eager** and **lazy** diffing
//! - **No-std compatible**, both with and without `alloc`
//!
//! # Usage
//!
//! ```rust
//! # #[cfg(feature = "std")] {
//! use daft::{Diffable, Leaf};
//!
//! // Annotate your struct with `#[derive(Diffable)]`:
//! #[derive(Diffable)]
//! struct MyStruct {
//!     a: i32,
//!     b: String,
//! }
//!
//! // This generates a type called MyStructDiff, which looks like:
//! # /*
//! #[automatically_derived]
//! struct MyStructDiff<'daft> {
//!     a: Leaf<&'daft i32>,
//!     b: Leaf<&'daft str>,
//! }
//! # */
//!
//! // Then, with two instances of MyStruct:
//! let before = MyStruct { a: 1, b: "hello".to_owned() };
//! let after = MyStruct { a: 2, b: "world".to_owned() };
//!
//! // You can diff them like so:
//! let diff = before.diff(&after);
//!
//! // And compare the results:
//! assert_eq!(*diff.a.before, 1);
//! assert_eq!(*diff.a.after, 2);
//! assert_eq!(diff.b.before, "hello");
//! assert_eq!(diff.b.after, "world");
//! # }
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
//! * As an opt-in mechanism for struct fields: see
//!   [*Recursive diffs*](#recursive-diffs) below for more.
//!
//! #### Example
//!
//! A contrived example for integers:
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! let diff: Leaf<&i32> = 1_i32.diff(&2);
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
//! let diff: Leaf<Option<&i32>> = Some(1_i32).diff(&Some(2));
//! assert_eq!(diff.before, Some(&1));
//! assert_eq!(diff.after, Some(&2));
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
//! let diff: Leaf<&MyEnum> = before.diff(&after);
//! assert_eq!(diff.before, &before);
//! assert_eq!(diff.after, &after);
//! ```
//!
//! Vectors use `Leaf` as well:
//!
//! ```rust
//! # #[cfg(feature = "std")] {
//! use daft::{Diffable, Leaf};
//!
//! let before = vec![1, 2, 3];
//! let after = vec![4, 5, 6];
//! let diff: Leaf<&[i32]> = before.diff(&after);
//! assert_eq!(diff.before, &before);
//! assert_eq!(diff.after, &after);
//! # }
//! ```
//!
//! ### Map diffs
//!
//! For [`BTreeMap`] and [`HashMap`], daft has corresponding [`BTreeMapDiff`]
//! and [`HashMapDiff`] types. These types have fields for *common*, *added*,
//! and *removed* entries.
//!
//! Map diffs are performed eagerly for keys, but values are stored as leaf
//! nodes.
//!
//! #### Example
//!
//! ```rust
//! # #[cfg(feature = "std")] {
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
//! // Added and removed entries are stored as maps:
//! assert_eq!(diff.added, [(&4, &"four")].into_iter().collect());
//! assert_eq!(diff.removed, [(&1, &"one")].into_iter().collect());
//!
//! // Common entries are stored as leaf nodes.
//! assert_eq!(
//!     diff.common,
//!     [
//!         (&2, Leaf { before: &"two", after: &"two" }),
//!         (&3, Leaf { before: &"three", after: &"THREE" })
//!     ]
//!     .into_iter().collect(),
//! );
//!
//! // If `V` implements `Eq`, unchanged and modified iterators become
//! // available. `unchanged` and `modified` return key-value pairs;
//! // `unchanged_keys` and `modified_keys` return keys; and
//! // `unchanged_values` and `modified_values` return values.
//! //
//! // Here's `unchanged_keys` to get the keys of unchanged entries:
//! assert_eq!(diff.unchanged_keys().collect::<Vec<_>>(), [&2]);
//!
//! // `modified_values` returns leaf nodes for modified entries.
//! assert_eq!(
//!     diff.modified_values().collect::<Vec<_>>(),
//!     [Leaf { before: &"three", after: &"THREE" }],
//! );
//! # }
//! ```
//!
//! ### Set diffs
//!
//! For [`BTreeSet`] and [`HashSet`], daft has corresponding [`BTreeSetDiff`]
//! and [`HashSetDiff`] types. These types have fields for *common*, *added*,
//! and *removed* entries.
//!
//! Set diffs are performed eagerly.
//!
//! #### Example
//!
//! ```rust
//! # #[cfg(feature = "std")] {
//! use daft::{Diffable, Leaf, BTreeSetDiff};
//! use std::collections::BTreeSet;
//!
//! let a: BTreeSet<i32> = [0, 1, 2, 3, 4, 5].into_iter().collect();
//! let b: BTreeSet<i32> = [3, 4, 5, 6, 7, 8].into_iter().collect();
//! let diff: BTreeSetDiff<'_, i32> = a.diff(&b);
//!
//! assert_eq!(diff.common, [&3, &4, &5].into_iter().collect());
//! assert_eq!(diff.added, [&6, &7, &8].into_iter().collect());
//! assert_eq!(diff.removed, [&0, &1, &2].into_iter().collect());
//! # }
//! ```
//!
//! ### Tuple diffs
//!
//! For a tuple like `(A, B, C)`, the [`Diffable`] implementation is recursive:
//! the diff resolves to `(A::Diff, B::Diff, C::Diff)`.
//!
//! #### Example
//!
//! ```rust
//! # #[cfg(feature = "std")] {
//! use daft::{BTreeSetDiff, Diffable, Leaf};
//! use std::collections::BTreeSet;
//!
//! let before: (usize, String, BTreeSet<usize>) = (1, "hello".to_owned(), [1, 2, 3].into_iter().collect());
//! let after = (2, "world".to_owned(), [2, 3, 4].into_iter().collect());
//!
//! let diff = before.diff(&after);
//! assert_eq!(
//!     diff,
//!     (
//!         Leaf { before: &1, after: &2 },
//!         Leaf { before: "hello", after: "world" },
//!         BTreeSetDiff {
//!             common: [&2, &3].into_iter().collect(),
//!             added: [&4].into_iter().collect(),
//!             removed: [&1].into_iter().collect(),
//!         }
//!     ),
//! );
//! # }
//! ```
//!
//! ### Struct diffs
//!
//! For structs, the [`Diffable`][macro@Diffable] derive macro generates
//! a diff type with a field corresponding to each field type. Each field must
//! implement [`Diffable`].
//!
//! A struct `Foo` gets a corresponding `FooDiff` struct, which has fields
//! corresponding to each field in `Foo`.
//!
//! Structs can be annotated with `#[daft(leaf)]` to treat the field as a leaf
//! node, regardless of the field's `Diff` type or even whether it implements
//! [`Diffable`].
//!
//! #### Example
//!
//! For an example of structs with named fields, see [*Usage*](#usage) above.
//!
//! Tuple-like structs produce tuple-like diff structs:
//!
//! ```rust
//! # #[cfg(feature = "std")] {
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
//! # }
//! ```
//!
//! An example with `#[daft(leaf)]`:
//!
//! ```rust
//! use daft::{Diffable, Leaf};
//!
//! // A simple struct that implements Diffable.
//! #[derive(Debug, PartialEq, Eq, Diffable)]
//! struct InnerStruct {
//!     text: &'static str,
//! }
//!
//! // A struct that does not implement Diffable.
//! #[derive(Debug, PartialEq, Eq)]
//! struct PlainStruct(usize);
//!
//! #[derive(Diffable)]
//! struct OuterStruct {
//!     // Ordinarily, InnerStruct would be diffed recursively, but
//!     // with #[daft(leaf)], it is treated as a leaf node.
//!     #[daft(leaf)]
//!     inner: InnerStruct,
//!
//!     // PlainStruct does not implement Diffable, but using
//!     // daft(leaf) allows it to be diffed anyway.
//!     #[daft(leaf)]
//!     plain: PlainStruct,
//! }
//!
//! let before = OuterStruct { inner: InnerStruct { text: "hello" }, plain: PlainStruct(1) };
//! let after = OuterStruct { inner: InnerStruct { text: "world" }, plain: PlainStruct(2) };
//! let diff = before.diff(&after);
//!
//! // `OuterStructDiff` does *not* recursively diff `InnerStruct`, but instead
//! // returns a leaf node.
//! assert_eq!(
//!     diff.inner,
//!     Leaf { before: &InnerStruct { text: "hello" }, after: &InnerStruct { text: "world" } },
//! );
//!
//! // But you can continue the recursion anyway, since `InnerStruct` implements
//! // `Diffable`:
//! let inner_diff = diff.inner.diff_pair();
//! assert_eq!(
//!     inner_diff,
//!     InnerStructDiff { text: Leaf { before: "hello", after: "world" } },
//! );
//!
//! // `PlainStruct` can also be compared even though it doesn't implement `Diffable`.
//! assert_eq!(diff.plain, Leaf { before: &PlainStruct(1), after: &PlainStruct(2) });
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
//! # #[cfg(feature = "std")] {
//! use daft::{Diffable, Leaf};
//!
//! struct Identifier(String);
//!
//! impl Diffable for Identifier {
//!     type Diff<'daft> = Leaf<&'daft Self>;
//!
//!     fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
//!         Leaf {
//!             before: self,
//!             after: other,
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ## Type and lifetime parameters
//!
//! If a type parameter is specified, the [`Diffable`][macro@Diffable] derive
//! macro for structs normally requires that the type parameter implement
//! `Diffable`. This is not required if the field is annotated with
//! `#[daft(leaf)]`.
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
//! Implementations for standard library types, all **enabled** by default:
//!
//! * `alloc`: Enable diffing for types from the [`alloc`] crate.
//! * `std`: Enable diffing for types from the [`std`] crate.
//!
//! (With `default-features = false`, daft is no-std compatible.)
//!
//! Implementations for third-party types, all **disabled** by default:
//!
//! * `uuid1`: Enable diffing for [`uuid::Uuid`].
//! * `oxnet01`: Enable diffing for network types from the [`oxnet`] crate.
//! * `newtype-uuid1`: Enable diffing for [`newtype_uuid::TypedUuid`].
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
//!   for floats. Daft doesn't have the `Same` trait, and its core
//!   functionality forgoes the need for `Eq` entirely.
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
//! * Daft is no-std-compatible, while diffus requires std.
//!
//! [GAT]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html
//! [`BTreeMap`]: std::collections::BTreeMap
//! [`BTreeSet`]: std::collections::BTreeSet
//! [`HashMap`]: std::collections::HashMap
//! [`HashSet`]: std::collections::HashSet

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

#[cfg(feature = "alloc")]
mod alloc_impls;
mod core_impls;
mod diffable;
mod leaf;
#[cfg(feature = "std")]
mod std_impls;
mod third_party;

#[cfg(feature = "alloc")]
pub use alloc_impls::*;
pub use daft_derive::*;
pub use diffable::*;
pub use leaf::*;
#[cfg(feature = "std")]
pub use std_impls::*;
