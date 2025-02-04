use daft::{Diffable, Leaf};

#[derive(Diffable)]
struct Inner {
    a: i32,
    b: i32,
}

#[derive(Diffable)]
struct Outer {
    inner: Inner,
    c: i32,
}

fn main() {
    // Ensure that diffing works even though the structs don't implement Eq.
    let a = Outer { inner: Inner { a: 1, b: 2 }, c: 3 };
    let b = Outer { inner: Inner { a: 4, b: 5 }, c: 6 };

    let diff = a.diff(&b);
    assert_eq!(
        diff.inner,
        InnerDiff {
            a: Leaf { before: &1, after: &4 },
            b: Leaf { before: &2, after: &5 },
        }
    );
}
