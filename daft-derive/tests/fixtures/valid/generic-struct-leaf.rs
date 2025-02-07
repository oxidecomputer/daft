use daft::Diffable;

#[derive(Debug, Eq, PartialEq, Diffable)]
#[daft(leaf)]
struct StructWithGenerics<'d, 'e, T, U>
where
    // daft(leaf) on the struct means Diffable shouldn't be necessary.
    T: 'd + ?Sized,
    U: 'e + ?Sized,
{
    b: usize,
    c: &'d T,
    d: &'e U,
}

fn main() {}
