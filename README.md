Daft is a library and proc macro for generating differences between types.

`daft` is inspired by [diffus](https://github.com/distil/diffus) but
simplified significantly. The primary usage is to `#[derive(Diff)]`
on types so that they are semantically diffable. See the
[integration_test](https://github.com/oxidecomputer/daft/blob/main/daft-derive/tests/integration_test.rs)
for an example of how to use `daft`.

Using `#[derive(Diff)]` on a struct creates a new struct named `<Ident>Diff`
that represents the difference between two types named `Ident`. A `Diffable`
implementation is crated for `Ident` that returns the generated type as a
`Diff`.

We significantly depart with `diffus` when we `#[derive(Diff)]` on `Enum`s. We
do not attempt to actually recurse into data carrying enums and instead return a
`Leaf` type with the `before` and `after` values. This significantly simplifies
the implementation and makes `daft` easier to work with in practice. Enums that
carry data across multiple variants must generate an `Enum` return type for a
diff, as in diffus. In practice this means that the user is matching on an enum
anyway, and they may as well do it on the original type. It's not much more work
to call `diff` on the associated data if desired.

We depart from `diffus` in a few other significant ways:

* `daft` doesn't wrap types in an outer `Edit` type, but instead returns the
actual associated `Diff` type for `diff` implementations.
* All types that implement `Diffable` are required to implement `Eq`, rather
than implementing `Same`. This simplifies things further and means that
types can be compared directly rather than requiring a wrapper informing of a
`Change` or `Copy`. However, we also plan to generate methods for derived type
equivalence since daft also allows ignoring fields when performing diffs,
and shouldn't report equivalence for ignored fields.
* Since all types must implement `Eq`, daft contains a `Leaf` type that contains
the `before` and `after` values for primitives, enums, and other types like
`String` and `Uuid` that don't make sense to diff. We are building type based
diffs, not text based diffs here.
* For sets and maps we provide specific types that report changes as we
would like to see them. For example, for maps we return separate collections
containing which values were inserted, removed, modified, and unchangedy. In
practice this is what we want to know, and so we don't force the user to compile
this data by walking and matching the values.
