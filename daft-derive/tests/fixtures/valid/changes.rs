//! Exercises `#[daft(changes)]` opt-in: emission of the `*Changes` struct, the
//! `IntoChanges` impl, and (under the `serde` feature) the hand-written
//! `Serialize` impl. Covers named, tuple, empty, and `#[daft(ignore)]`-only
//! shapes plus a `#[daft(leaf)]` field, since the leaf path threads through
//! `Leaf::IntoChanges` rather than the generated `*Changes` type.

use daft::Diffable;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Diffable)]
#[daft(changes)]
struct Config {
    name: String,
    retries: u32,
    tags: BTreeMap<u32, String>,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
#[daft(changes)]
struct Pair(u32, String);

#[derive(Debug, Eq, PartialEq, Diffable)]
#[daft(changes)]
struct OnlyIgnored {
    #[daft(ignore)]
    _scratch: u32,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct Inner {
    value: u32,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
#[daft(changes)]
struct Wrapper {
    #[daft(leaf)]
    inner: Inner,
    label: String,
}

fn main() {}
