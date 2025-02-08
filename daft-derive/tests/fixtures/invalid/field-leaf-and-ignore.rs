use daft::Diffable;

#[derive(Diffable)]
struct MyStruct {
    // Having both `leaf` and `ignore` should result in an error.
    #[daft(leaf, ignore)]
    a: i32,
    #[daft(ignore)]
    #[daft(leaf)]
    b: String,
}

fn main() {
    // MyStruct should still exist, even though the Diffable impl couldn't be
    // generated.
    let _ = MyStruct { a: 0, b: "foo".to_string() };
}
