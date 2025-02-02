use daft::Diff;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Diff)]
struct WithAttrs {
    a: i32,
    b: BTreeMap<Uuid, BTreeSet<usize>>,
    #[daft(ignore)]
    c: std::time::Instant,
    #[daft(leaf)]
    d: Lazy,
    #[daft(leaf)]
    e: usize,
    f: usize,
}

#[derive(Debug, Eq, PartialEq, Diff)]
struct Lazy {
    x: usize,
    y: usize,
}

fn main() {}
