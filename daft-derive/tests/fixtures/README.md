# daft-derive fixtures

## Valid fixtures

These fixtures ensure that:

* the macro's output is stable, via `snapshot_test.rs`
* the macro's output compiles, via `ui_test.rs`

Each file in `valid` is automatically picked up by the snapshot and UI tests.

Currently, `snapshot_test.rs` only tests the first struct or enum in the file.
The test can be extended to test multiple macro invocations per file if
necessary.
