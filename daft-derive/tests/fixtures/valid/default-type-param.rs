use daft::{Diff, Diffable};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diff)]
struct StructWithDefaultTypeParam<T: Eq + Debug + Diffable = ()> {
    field: T,
}

fn main() {}
