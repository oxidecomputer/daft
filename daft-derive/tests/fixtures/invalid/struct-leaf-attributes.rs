use daft::Diffable;

#[derive(Diffable)]
#[daft(leaf)]
struct MyStruct {
    // Attributes on leaf-struct fields are banned.
    #[daft(ignore)]
    a: i32,
    #[daft(leaf)]
    b: i32,
}

fn main() {
    // MyStruct and its Diffable should exist.
    let before = MyStruct { a: 0, b: 0 };
    let after = MyStruct { a: 0, b: 1 };

    let _diff = before.diff(&after);
}
