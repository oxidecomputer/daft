use daft::Diffable;
use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq, Diffable)]
struct UnitStruct;

#[derive(Debug, Eq, PartialEq, Diffable)]
struct EmptyNamed {}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct EmptyTuple();

#[derive(Debug, Eq, PartialEq, Diffable)]
struct AllIgnoredNamed {
    #[daft(ignore)]
    _a: i32,
    #[daft(ignore)]
    _b: String,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct AllIgnoredTuple(#[daft(ignore)] i32, #[daft(ignore)] String);

#[derive(Debug, Eq, PartialEq, Diffable)]
struct GenericAllIgnored<T> {
    #[daft(ignore)]
    _phantom: PhantomData<T>,
}

fn main() {}
