use daft::{Diff, Diffable};
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Diff)]
struct S<'a, T, U>
where
    T: Diffable + Eq + 'a,
    U: Diffable + 'a,
{
    a: BTreeMap<usize, T>,
    b: usize,
    c: &'a U,
    d: &'a str,
}

fn main() {}
