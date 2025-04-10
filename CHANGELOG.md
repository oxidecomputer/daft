# Changelog

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

[0.1.3]: https://github.com/oxidecomputer/daft/releases/daft-0.1.3
[0.1.2]: https://github.com/oxidecomputer/daft/releases/daft-0.1.2
[0.1.1]: https://github.com/oxidecomputer/daft/releases/daft-0.1.1
[0.1.0]: https://github.com/oxidecomputer/daft/releases/daft-0.1.0
