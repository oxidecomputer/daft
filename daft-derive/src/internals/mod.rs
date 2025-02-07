//! Internals for daft-derive.
//!
//! This is imported both by this crate's lib.rs and by tests/snapshot_test.rs.

mod error_store;
mod imp;

pub use imp::*;
