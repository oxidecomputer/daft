use daft::{Diffable, Leaf};

#[derive(Diffable)]
union Inner {
    a: i32,
    b: u32,
}

fn main() {
    let before = Inner { a: 1 };
    let after = Inner { b: 2 };

    let diff: Leaf<&Inner> = before.diff(&after);
    assert_eq!(unsafe { diff.before.b }, 1);
    assert_eq!(unsafe { diff.after.a }, 2);
}
