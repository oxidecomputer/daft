use daft::Diff;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Diff)]
struct StructWithGenerics<'d, 'e, T: Eq + Debug, U: Eq + Debug>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
{
    b: usize,
    c: &'d T,
    d: &'e U,
}

fn main() {}
