use daft::Diffable;

#[derive(Diffable)]
#[daft(leaf)]
enum MyEnum {
    #[daft(leaf)]
    A(#[daft(ignore)] i32),
    #[daft(leaf)]
    B {
        #[daft(ignore)]
        data: usize,
    },
}

#[derive(Diffable)]
#[daft(ignore)]
enum MyEnum2 {
    A(i32),
}

fn main() {
    // MyEnum/MyEnum2 and its Diffable impl should all exist.
    let before = MyEnum::A(0);
    let after = MyEnum::B { data: 1 };

    let _diff = before.diff(&after);

    let before = MyEnum2::A(0);
    let after = MyEnum2::A(1);

    let _diff = before.diff(&after);
}
