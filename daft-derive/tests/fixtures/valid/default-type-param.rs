use daft::{Diff, Diffable};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diff)]
struct StructWithDefaultTypeParam<T: Eq + Debug = ()>
where
    for<'x> T: Diffable<'x>,
{
    field: T,
}

fn main() {}
