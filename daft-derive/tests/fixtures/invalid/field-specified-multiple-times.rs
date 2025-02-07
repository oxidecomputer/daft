use daft::Diffable;

#[derive(Diffable)]
struct MyStruct {
    #[daft(leaf, leaf, leaf)]
    a: i32,
    #[daft(ignore)]
    #[daft(ignore)]
    #[daft(ignore)]
    b: String,
}

fn main() {
    // MyStruct should still exist, even though the Diffable impl couldn't be
    // generated.
    let _ = MyStruct { a: 0, b: "foo".to_string() };
}
