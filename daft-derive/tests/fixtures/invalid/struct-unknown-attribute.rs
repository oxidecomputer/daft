use daft::Diffable;

#[derive(Diffable)]
#[daft(ignore)]
struct MyStruct {
    a: i32,
    b: String,
}

fn main() {
    // MyStruct should still exist, even though the Diffable impl couldn't be
    // generated.
    let _ = MyStruct { a: 0, b: "foo".to_string() };
}
