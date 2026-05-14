# Changelog

<!-- next-header -->
## Unreleased - ReleaseDate

### Added

- New `IntoChanges` trait and a parallel `*Changes` type per diff, projecting a diff to the subset of nodes that actually changed. Built-in implementations cover `Leaf`, the map/set diffs, and tuples.
- The `Diffable` derive macro now recognizes `#[daft(changes)]` on structs as opt-in to generating a parallel `FooChanges` type and an `IntoChanges` impl. Opting in is required because the projection needs `Eq` (and, with the new `serde` feature, `Serialize`) on every leaf type — constraints existing users may not satisfy.
- New `serde` feature emits `Serialize` impls for `Leaf` and every `*Changes` type, so a projected diff can be written to JSON (or any other serde format) with unchanged subtrees omitted entirely.

## [0.1.6] - 2026-05-14

### Added

Support for indexmap under the optional `indexmap` feature. Thanks [scoopr](https://github.com/scoopr) for your first contribution!

### Fixed

- The `Diffable` derive macro now works properly if there are no struct fields to compare (either empty structs or all fields marked `#[daft(ignore)]`.)

## [0.1.5] - 2025-09-29

### Fixed

Replaced obsolete `doc_auto_cfg` with `doc_cfg`, to fix Rust nightly builds with the `doc_cfg` flag enabled.

## [0.1.4] - 2025-06-29

### Added

- Implement `Diffable` for `NonZero` types. Thanks [itsjunetime](https://github.com/itsjunetime) for your first contribution!

## [0.1.3] - 2025-04-01

### Fixed

The `Diffable` derive macro now produces both code and errors when the only errors are duplicate struct and field attribute errors.

Thanks to [schneems](https://github.com/schneems) for your first contribution; be sure to read [his blog post](https://www.schneems.com/2025/03/26/a-daft-procmacro-trick-how-to-emit-partialcode-errors/) about Daft's error handling!

## [0.1.2] - 2025-03-10

### Fixed

- Semantic errors (e.g. a field not implementing `Diffable`) are better annotated.
- The `Diffable` derive macro now produces references to `core` rather than to `std`, meaning that the code it generates is no-std compatible.

## [0.1.1] - 2025-02-10

### Added

- Add `Leaf::is_unchanged` and `Leaf::is_modified` when the stored type is `Eq`.
- Add `BTreeMapDiff::is_unchanged`, `BTreeMapDiff::is_modified`, `BTreeMapDiff::get_unchanged`, `BTreeMapDiff::get_modified`, and similar methods for `HashMapDiff` when map values are `Eq`.

## [0.1.0] - 2025-02-10

Initial release with support for:

- Diffing maps, sets, and structs recursively
- Eager and lazy diffing
- No-std support
- The `Diffable` derive macro
- Implementations for `oxnet`, `uuid`, and `newtype-uuid`

<!-- next-url -->
[0.1.6]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.6
[0.1.5]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.5
[0.1.4]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.4
[0.1.3]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.3
[0.1.2]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.2
[0.1.1]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.1
[0.1.0]: https://github.com/oxidecomputer/daft/releases/tag/daft-0.1.0
