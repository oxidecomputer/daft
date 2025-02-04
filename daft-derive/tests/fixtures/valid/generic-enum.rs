use daft::Diffable;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diffable)]
enum EnumWithGenerics<'a, T, U> {
    A(T),
    B(&'a U),
}

fn main() {}
