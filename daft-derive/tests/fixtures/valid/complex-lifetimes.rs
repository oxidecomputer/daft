use daft::Diff;
use std::{cell::Cell, fmt::Debug, marker::PhantomData};

#[derive(Debug, Eq, PartialEq, Diff)]
struct S<'a, 'b, 'c: 'a, 'inv, 'contra> {
    // Multiple lifetime params.
    multi_ref: &'a &'b Vec<u8>,

    // Lifetime param with a lifetime bound.
    bound_ref: &'c Vec<u8>,

    // Invariant lifetime parameter.
    inv_ref: PhantomData<Cell<&'inv ()>>,

    // Contravariant lifetime parameter.
    contra_ref: PhantomData<fn(&'contra ())>,
}

fn main() {}
