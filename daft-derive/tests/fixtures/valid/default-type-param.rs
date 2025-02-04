use daft::Diffable;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diffable)]
struct StructWithDefaultTypeParam<T: Diffable = ()> {
    field: T,
}

fn main() {}
