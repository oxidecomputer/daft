//! Integration tests for daft-derive.
//!
//! Structuring integration tests this way results in a single binary so new
//! test modules don't have to build a separate binary.

mod basic;
mod default_field_values;
mod generics;
