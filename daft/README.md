<!-- cargo-sync-rdme title [[ -->
# daft
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/daft.svg?)
[![crates.io](https://img.shields.io/crates/v/daft.svg?logo=rust)](https://crates.io/crates/daft)
[![docs.rs](https://img.shields.io/docsrs/daft.svg?logo=docs.rs)](https://docs.rs/daft)
[![Rust: ^1.81.0](https://img.shields.io/badge/rust-^1.81.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme rustdoc [[ -->
Daft is a library to perform semantic diffs of Rust data structures.

Daft consists of a trait called [`Diffable`](https://docs.rs/daft/0.1.0/daft/trait.Diffable.html), along with \[a derive
macro\]\[macro@Diffable\] by the same name. This trait represents the
notion of a type for which two members can be simultaneously compared.

## Usage

````rust
use daft::{Diffable, Leaf};

// Annotate your struct with `#[derive(Diffable)]`:
struct MyStruct {
    a: i32,
    b: &'static str,
}

// This generates a type called MyStructDiff, which looks like:
struct MyStructDiff<'daft> {
    a: Leaf<'daft, i32>,
    b: Leaf<'daft, &'static str>,
}

// Then, with two instances of MyStruct:
let before = MyStruct { a: 1, b: "hello" };
let after = MyStruct { a: 2, b: "world" };

// You can diff them like so:
let diff = before.diff(&after);

// And compare the results:
assert_eq!(*diff.a.before, 1);
assert_eq!(*diff.a.after, 2);
assert_eq!(diff.b.before, "hello");
assert_eq!(diff.b.after, "world");
````

This crate assigns one side the name *before*, and the other side *after*.
These labels are arbitrary: if *before* and *after* are swapped, the diff is
reversed.

### Diff types

Currently, daft comes with a few kinds of diff types:

#### [`Leaf`](https://docs.rs/daft/0.1.0/daft/struct.Leaf.html) instances

A [`Leaf`](https://docs.rs/daft/0.1.0/daft/struct.Leaf.html) represents a logical *leaf node* or *base case* in a diff, i.e. a
point at which diffing stops. [`Leaf`](https://docs.rs/daft/0.1.0/daft/struct.Leaf.html) instances are used for:

* *Scalar* or *primitive types* like `i32`, `String`, `bool`, etc.
* *Enums*, since diffing across variants is usually not meaningful.
* Vector and slice types, since there are several reasonable ways to diff
  vectors (e.g. set-like, ordered, etc.) and we don’t want to make assumptions.
* Any point at which you want to terminate recursion, via the `#[daft(leaf)]` attribute.

##### Example

A contrived example for integers:

````rust
use daft::{Diffable, Leaf};

let diff: Leaf<'_, i32> = 1_i32.diff(&2);
assert_eq!(*diff.before, 1);
assert_eq!(*diff.after, 2);
````

Enums also use `Leaf`:

````rust
use daft::{Diffable, Leaf};

// Option<T> uses Leaf:
let diff: Leaf<'_, Option<i32>> = Some(1_i32).diff(&Some(2));
assert_eq!(*diff.before, Some(1));
assert_eq!(*diff.after, Some(2));

// Automatically derived enums also use Leaf:
enum MyEnum {
    A(i32),
    B(String),
}

let before = MyEnum::A(1);
let after = MyEnum::B("hello".to_string());

let diff: Leaf<'_, MyEnum> = before.diff(&after);
assert_eq!(diff.before, &before);
assert_eq!(diff.after, &after);
````

Vectors use `Leaf` as well:

````rust
use daft::{Diffable, Leaf};

let before = vec![1, 2, 3];
let after = vec![4, 5, 6];
let diff: Leaf<'_, Vec<i32>> = before.diff(&after);
assert_eq!(diff.before, &before);
assert_eq!(diff.after, &after);
````

#### Map diffs

For [`BTreeMap`](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html) and [`HashMap`](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html), daft has corresponding [`BTreeMapDiff`](https://docs.rs/daft/0.1.0/daft/struct.BTreeMapDiff.html)
and [`HashMapDiff`](https://docs.rs/daft/0.1.0/daft/struct.HashMapDiff.html) types. These types have fields for *unchanged*, *added*,
*removed*, and *modified* entries.

Map diffs are performed eagerly.

##### Example

````rust
use daft::{Diffable, Leaf, BTreeMapDiff};
use std::collections::BTreeMap;

let mut a = BTreeMap::new();
a.insert(1, "one");
a.insert(2, "two");
a.insert(3, "three");

let mut b = BTreeMap::new();
b.insert(2, "two");
b.insert(3, "THREE");
b.insert(4, "four");

let diff: BTreeMapDiff<'_, i32, &str> = a.diff(&b);

// Unchanged, added, and removed entries are stored as maps:
assert_eq!(diff.unchanged, [(&2, &"two")].into_iter().collect());
assert_eq!(diff.added, [(&4, &"four")].into_iter().collect());
assert_eq!(diff.removed, [(&1, &"one")].into_iter().collect());

// Modified entries are stored via the values' diff types:
assert_eq!(
    diff.modified,
    [(&3, Leaf { before: "three", after: "THREE" })].into_iter().collect(),
);
````

#### Set diffs

For [`BTreeSet`](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html) and [`HashSet`](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html), daft has corresponding [`BTreeSetDiff`](https://docs.rs/daft/0.1.0/daft/struct.BTreeSetDiff.html)
and [`HashSetDiff`](https://docs.rs/daft/0.1.0/daft/struct.HashSetDiff.html) types. These types have fields for unchanged, added and
removed entries.

Set diffs are performed eagerly.

##### Example

````rust
use daft::{Diffable, Leaf, BTreeSetDiff};
use std::collections::BTreeSet;

let a: BTreeSet<i32> = [0, 1, 2, 3, 4, 5].into_iter().collect();
let b: BTreeSet<i32> = [3, 4, 5, 6, 7, 8].into_iter().collect();
let diff: BTreeSetDiff<'_, i32> = a.diff(&b);

assert_eq!(diff.unchanged, [&3, &4, &5].into_iter().collect::<Vec<_>>());
assert_eq!(diff.added, [&6, &7, &8].into_iter().collect::<Vec<_>>());
assert_eq!(diff.removed, [&0, &1, &2].into_iter().collect::<Vec<_>>());
````

#### Recursive diffs

For structs, the \[`Diffable`\]\[macro@Diffable\] derive macro generates
a diff type with a field corresponding to each field type. Each field must
implement [`Diffable`](https://docs.rs/daft/0.1.0/daft/trait.Diffable.html).

A struct `Foo` gets a corresponding `FooDiff` struct, which has fields
corresponding to each field in `Foo`.

##### Example

For an example of structs with named fields, see [*Usage*](#usage) above.

Tuple-like structs produce tuple-like diff structs:

````rust
use daft::Diffable;
use std::collections::BTreeMap;

struct MyTuple(BTreeMap<i32, &'static str>, i32);

let before = MyTuple(BTreeMap::new(), 1);
let after = MyTuple([(1, "hello")].into_iter().collect(), 2);
let diff = before.diff(&after);

// The generated type is MyTupleDiff(BTreeMapDiff<i32, &str>, Leaf<i32>).
assert_eq!(**diff.0.added.get(&1).unwrap(), "hello");
assert_eq!(*diff.1.before, 1);
assert_eq!(*diff.1.after, 2);
````

#### Custom diff types

The [`Diffable`](https://docs.rs/daft/0.1.0/daft/trait.Diffable.html) trait can also be implemented manually for custom behavior.

In general, most custom implementations will likely use one of the built-in
diff types directly.

#### Example

Some structs like identifiers should be treated as leaf nodes:

````rust
use daft::{Diffable, Leaf};

struct Identifier(String);

impl Diffable for Identifier {
    type Diff<'daft> = Leaf<'daft, Self>;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf {
            before: self,
            after: other,
        }
    }
}
````

### Type and lifetime parameters

If a type parameter is specified, the \[`Diffable`\]\[macro@Diffable\]
derive macro for structs normally requires that the type parameter implement
`Diffable`. An exception is if the field is annotated with `#[daft(leaf)]`.

Daft fully supports types with arbitrary lifetimes. Automatically generated
diff structs will have an additional `'daft` lifetime parameter at the
beginning, with the requirement that all other lifetime and type parameters
outlive it.

#### Example

````rust
use daft::Diffable;

struct BorrowedData<'a, 'b, T: Diffable + ?Sized> {
    a: &'a str,
    b: &'b T,
    // TODO: example with daft(leaf)
}

// This generates a struct that looks like:
struct BorrowedDataDiff<'daft, 'a: 'daft, 'b: 'daft, T: ?Sized + 'daft> {
    a: Leaf<'daft, &'a str>,
    b: T::Diff<'daft>,
}
````

## Optional features

Implementations for foreign types:

* `uuid1`: Enable diffing for [`uuid::Uuid`](https://docs.rs/uuid/1.12.1/uuid/struct.Uuid.html). *Disabled by default.*
* `oxnet01`: Enable diffing for network types from the \[`oxnet`\] crate. *Disabled by default.*
* `newtype-uuid1`: Enable diffing for [`newtype_uuid::TypedUuid`](https://docs.rs/newtype-uuid/1.2.1/newtype_uuid/struct.TypedUuid.html). *Disabled by default.*

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **1.81.0**. At any time, at least the
last three stable versions of Rust are supported.

While this crate is a pre-release (0.x.x) it may have its MSRV bumped in a
patch release. Once this crate has reached 1.x, any MSRV bump will be
accompanied with a new minor version.

## Related work

[Diffus](https://crates.io/crates/diffus) is the original inspiration for
this crate and a great alternative. Daft diverges from diffus in a few ways:

* Daft’s derive macro does not attempt to diff enums with different variants.
  In practice, we’ve found that diffing enums across different variants is less
  useful than it first appears.

* Daft has the notion of a [`Leaf`](https://docs.rs/daft/0.1.0/daft/struct.Leaf.html) type, which represents an atomic unit.
  (For example, the [`Diffable`](https://docs.rs/daft/0.1.0/daft/trait.Diffable.html) implementation for `i32` is a [`Leaf`](https://docs.rs/daft/0.1.0/daft/struct.Leaf.html).)
  [`Leaf`](https://docs.rs/daft/0.1.0/daft/struct.Leaf.html)s are also used for enums, as well as in any other place where lazy
  diffing is desired.

* Diffus has a `Same` trait, which is like `Eq` except it’s also implemented
  for floats. Daft doesn’t have the `Same` trait, and in fact mostly forgoes
  any trait requirements: the only places where `Eq` is required is for maps
  (both keys and values) and sets.
  
  For a primitive scalar like `f64`, you’ll get a `Leaf` struct which you can
  compare with whatever notion of equality you want.

* Daft uses a [generic associated type (GAT)][GAT] so that the `Diffable`
  trait no longer needs a lifetime parameter. This leads to simpler usage.
  (Diffus was written before GATs were available in stable Rust.)

* Daft uses fewer types in general. For example, diffus wraps its return values
  in an outer `Edit` type, but daft does not.

[GAT]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
