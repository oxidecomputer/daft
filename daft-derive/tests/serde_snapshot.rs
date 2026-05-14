//! Snapshot tests for the JSON form of projected diffs.
//!
//! Each test builds two values, computes their diff, projects it through
//! `IntoChanges`, serializes the result to pretty-printed JSON, and compares
//! the result against a file in `tests/fixtures/serde/`. Regenerate the
//! expected files with `EXPECTORATE=overwrite`.
//!
//! These tests complement the unit-style ones in `integration_test.rs` by
//! producing reviewable artifacts: the pretty JSON makes it easy to inspect
//! exactly which subtrees survive the projection and which are elided.

#![cfg(feature = "serde")]

use daft::{Diffable, IntoChanges};
use expectorate::assert_contents;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Compare `value` serialized as pretty JSON against
/// `tests/fixtures/serde/{name}.json`.
fn assert_json_snapshot<T: Serialize>(name: &str, value: &T) {
    let mut pretty = serde_json::to_string_pretty(value)
        .expect("serializing changes to JSON");
    pretty.push('\n');
    let path = format!("tests/fixtures/serde/{name}.json");
    assert_contents(&path, &pretty);
}

#[test]
fn simple_struct() {
    #[derive(Diffable)]
    #[daft(changes)]
    struct Config {
        name: String,
        retries: u32,
        timeout_ms: u32,
    }

    let before =
        Config { name: "alpha".to_owned(), retries: 3, timeout_ms: 1000 };
    let after =
        Config { name: "alpha".to_owned(), retries: 5, timeout_ms: 1000 };

    let changes = before.diff(&after).into_changes().expect("retries changed");
    assert_json_snapshot("simple_struct", &changes);
}

#[test]
fn nested_struct() {
    #[derive(Diffable)]
    #[daft(changes)]
    struct Inner {
        value: u32,
        note: String,
    }

    #[derive(Diffable)]
    #[daft(changes)]
    struct Outer {
        inner: Inner,
        tag: u32,
    }

    let before =
        Outer { inner: Inner { value: 1, note: "stable".to_owned() }, tag: 7 };
    let after =
        Outer { inner: Inner { value: 2, note: "stable".to_owned() }, tag: 7 };

    let changes =
        before.diff(&after).into_changes().expect("inner.value changed");
    assert_json_snapshot("nested_struct", &changes);
}

#[test]
fn map_field() {
    #[derive(Diffable)]
    #[daft(changes)]
    struct Cache {
        entries: BTreeMap<u32, &'static str>,
        version: u32,
    }

    let before = Cache {
        entries: [(1, "alpha"), (2, "beta"), (3, "gamma")]
            .into_iter()
            .collect(),
        version: 1,
    };
    let after = Cache {
        entries: [(1, "alpha"), (2, "BETA"), (4, "delta")]
            .into_iter()
            .collect(),
        version: 1,
    };

    let changes = before.diff(&after).into_changes().expect("entries changed");
    assert_json_snapshot("map_field", &changes);
}

#[test]
fn set_field() {
    #[derive(Diffable)]
    #[daft(changes)]
    struct Tags {
        labels: BTreeSet<&'static str>,
    }

    let before =
        Tags { labels: ["alpha", "beta", "gamma"].into_iter().collect() };
    let after =
        Tags { labels: ["beta", "gamma", "delta"].into_iter().collect() };

    let changes = before.diff(&after).into_changes().expect("labels changed");
    assert_json_snapshot("set_field", &changes);
}

#[test]
fn tuple_struct() {
    #[derive(Diffable)]
    #[daft(changes)]
    struct Pair(u32, String, bool);

    let before = Pair(1, "same".to_owned(), false);
    let after = Pair(2, "same".to_owned(), true);

    let changes = before.diff(&after).into_changes().expect("0 and 2 changed");
    assert_json_snapshot("tuple_struct", &changes);
}

#[test]
fn leaf_field() {
    // `#[daft(leaf)]` short-circuits a struct field to a `Leaf<&Inner>` even
    // when `Inner` itself implements `Diffable`. The Changes projection
    // surfaces it as `{ "before": ..., "after": ... }`.
    #[derive(Diffable, Serialize, Eq, PartialEq)]
    struct Inner {
        value: u32,
    }

    #[derive(Diffable)]
    #[daft(changes)]
    struct Wrapper {
        #[daft(leaf)]
        inner: Inner,
        label: String,
    }

    let before =
        Wrapper { inner: Inner { value: 1 }, label: "unchanged".to_owned() };
    let after =
        Wrapper { inner: Inner { value: 2 }, label: "unchanged".to_owned() };

    let changes = before.diff(&after).into_changes().expect("inner changed");
    assert_json_snapshot("leaf_field", &changes);
}

#[test]
fn mixed_fields() {
    // A single struct exercising every shape at once: a `Leaf` field, a map,
    // a set, a nested struct, and an unchanged field that should drop out.
    #[derive(Diffable)]
    #[daft(changes)]
    struct Profile {
        name: String,
        weight: u32,
    }

    #[derive(Diffable)]
    #[daft(changes)]
    struct Snapshot {
        version: u32,
        profile: Profile,
        attrs: BTreeMap<String, u32>,
        tags: BTreeSet<String>,
        unchanged: u32,
    }

    let before = Snapshot {
        version: 1,
        profile: Profile { name: "alice".to_owned(), weight: 70 },
        attrs: [("a".to_owned(), 1), ("b".to_owned(), 2)].into_iter().collect(),
        tags: ["x".to_owned(), "y".to_owned()].into_iter().collect(),
        unchanged: 42,
    };
    let after = Snapshot {
        version: 2,
        profile: Profile { name: "alice".to_owned(), weight: 71 },
        attrs: [("a".to_owned(), 1), ("b".to_owned(), 20), ("c".to_owned(), 3)]
            .into_iter()
            .collect(),
        tags: ["x".to_owned(), "z".to_owned()].into_iter().collect(),
        unchanged: 42,
    };

    let changes = before.diff(&after).into_changes().expect("multiple changes");
    assert_json_snapshot("mixed_fields", &changes);
}

#[test]
fn no_changes_yields_none() {
    // Diffing a value against itself must yield `None` so callers can skip
    // serialization entirely instead of emitting an empty payload.
    #[derive(Diffable)]
    #[daft(changes)]
    struct Config {
        name: String,
        retries: u32,
    }

    let value = Config { name: "alpha".to_owned(), retries: 3 };
    assert!(value.diff(&value).into_changes().is_none());
}
