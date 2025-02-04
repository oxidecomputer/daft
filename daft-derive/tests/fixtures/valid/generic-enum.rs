use daft::Diff;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diff)]
enum EnumWithGenerics<'a, T, U> {
    A(T),
    B(&'a U),
}

fn main() {}
