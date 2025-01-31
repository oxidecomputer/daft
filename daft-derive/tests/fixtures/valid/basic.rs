use daft::Diff;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Diff)]
struct Basic {
    a: i32,
    b: BTreeMap<Uuid, BTreeSet<usize>>,
}

fn main() {}
