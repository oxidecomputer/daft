use daft::{Diff, Diffable};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Eq, PartialEq, Diff)]
struct S<'a, T, U>
where
    T: Diffable + Debug + Eq + 'a,
    U: Diffable + Debug + Eq + 'a,
{
    a: BTreeMap<usize, T>,
    b: usize,
    c: &'a U,
    d: &'a str,
}

fn main() {}
