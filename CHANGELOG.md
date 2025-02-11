# Changelog

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

[0.1.0]: https://github.com/oxidecomputer/daft/releases/daft-0.1.0
