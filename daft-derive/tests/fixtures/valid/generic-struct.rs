use daft::Diffable;

#[derive(Debug, Eq, PartialEq, Diffable)]
struct StructWithGenerics<'d, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
{
    b: usize,
    c: &'d T,
    d: &'e U,
}

fn main() {}
