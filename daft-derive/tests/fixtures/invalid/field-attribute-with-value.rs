use daft::Diffable;

#[derive(Diffable)]
struct MyStruct {
    // We accept just `leaf`, not `leaf = true`.
    #[daft(leaf = true)]
    a: i32,
    #[daft(ignore = "yes")]
    b: String,
}

fn main() {
    // MyStruct should still exist, even though the Diffable impl couldn't be
    // generated.
    let _ = MyStruct { a: 0, b: "foo".to_string() };
}
