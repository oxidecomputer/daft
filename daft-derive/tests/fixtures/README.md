# daft-derive fixtures

## Valid fixtures

These fixtures ensure that:

* the macro's output is stable, via `snapshot_test.rs`
* the macro's output compiles, via `ui_test.rs`

Each file in `valid` is automatically picked up by the snapshot and UI tests.

`snapshot_test.rs` tests all macro invocations annotated with `#[derive(Diffable)]`.

## Invalid fixtures

These fixtures ensure that:

* the macro's success output, if any, is stable, via `snapshot_test.rs`.
* the macro's output fails with a good error message, via `ui_test.rs`.

Each file in `invalid` is automatically picked up by the snapshot and UI tests.

Like with valid fixtures, `snapshot_test.rs` tests all macro invocations annotated with `#[derive(Diffable)]`.
