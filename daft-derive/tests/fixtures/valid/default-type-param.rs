use daft::{Diff, Diffable};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diff)]
struct StructWithDefaultTypeParam<T: Diffable = ()> {
    field: T,
}

fn main() {}
