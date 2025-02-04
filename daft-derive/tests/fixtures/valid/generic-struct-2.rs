use daft::{Diff, Diffable};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Eq, PartialEq, Diff)]
struct S<'a, T, U>
where
    for<'x> T: Diffable<'x> + Debug + Eq + 'x,
    U: Diffable<'a> + Debug + Eq,
{
    a: BTreeMap<usize, T>,
    b: usize,
    c: &'a U,
}

fn main() {}
