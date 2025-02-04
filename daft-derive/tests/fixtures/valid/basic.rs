use daft::Diffable;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Diffable)]
struct Basic {
    a: i32,
    b: BTreeMap<Uuid, BTreeSet<usize>>,
}

fn main() {}
