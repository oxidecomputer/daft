use daft::Diffable;

struct NonDiffable {}

#[derive(Diffable)]
struct MyStruct {
    a: i32,
    b: NonDiffable,
}

fn main() {
    // MyStruct should still exist, even though the Diffable impl has errors.
    let _ = MyStruct { a: 0, b: NonDiffable {} };
}
