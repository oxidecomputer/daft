use daft::Diffable;

#[derive(Diffable)]
#[daft(leaf)]
union MyUnion {
    #[daft(leaf)]
    a: i32,
    #[daft(ignore)]
    b: u32,
}

#[derive(Diffable)]
#[daft(ignore)]
union MyUnion2 {
    a: i32,
    b: u32,
}

fn main() {
    // MyEnum/MyEnum2 and its Diffable impl should all exist.
    let before = MyUnion { a: 0 };
    let after = MyUnion { b: 1 };

    let _diff = before.diff(&after);

    let before = MyUnion2 { a: 0 };
    let after = MyUnion2 { a: 1 };

    let _diff = before.diff(&after);
}
