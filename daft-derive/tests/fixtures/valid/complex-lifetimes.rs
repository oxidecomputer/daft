use daft::Diffable;
use std::{cell::Cell, fmt::Debug, marker::PhantomData};

#[derive(Debug, Eq, PartialEq, Diffable)]
// 'daft is named as such to test potential clashes with generated code.
struct S<'a, 'b, 'daft: 'a, 'inv, 'contra> {
    // Multiple lifetime params.
    multi_ref: &'a &'b Vec<u8>,

    // Lifetime param with a lifetime bound.
    bound_ref: &'daft Vec<u8>,

    // Invariant lifetime parameter.
    inv_ref: PhantomData<Cell<&'inv ()>>,

    // Contravariant lifetime parameter.
    contra_ref: PhantomData<fn(&'contra ())>,
}

fn main() {}
