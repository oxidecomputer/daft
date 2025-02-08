use daft::Diffable;

#[derive(Diffable)]
#[daft(leaf, leaf, leaf)]
struct MyStruct {
    a: i32,
    b: String,
}

#[derive(Diffable)]
#[daft(leaf)]
#[daft(leaf)]
#[daft(leaf)]
struct MyStruct2 {
    a: i32,
    b: String,
}

fn main() {
    // MyStruct and MyStruct2 should still exist, even though the Diffable impl
    // couldn't be generated.
    let _ = MyStruct { a: 0, b: "foo".to_string() };
    let _ = MyStruct2 { a: 0, b: "foo".to_string() };
}
