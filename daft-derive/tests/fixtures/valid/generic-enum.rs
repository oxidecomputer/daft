use daft::Diff;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diff)]
enum EnumWithGenerics<'a, T: Eq + Debug, U: Eq + Debug> {
    A(T),
    B(&'a U),
}

fn main() {}
