use daft::Diffable;

#[derive(Diffable)]
struct MyStruct {
    #[daft(foo)]
    a: i32,
    #[daft(bar)]
    b: String,
}

fn main() {
    // MyStruct should still exist, even though the Diffable impl couldn't be
    // generated.
    let _ = MyStruct { a: 0, b: "foo".to_string() };
}
