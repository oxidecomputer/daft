use daft::Diffable;

#[non_exhaustive]
#[derive(Diffable)]
struct NonExhaustive {
    pub a: i32,
    pub b: i32,
}

#[non_exhaustive]
#[derive(Diffable)]
enum NonExhaustiveEnum {
    A(i32),
    #[non_exhaustive]
    B {
        b: i32,
    },
}

fn main() {}
