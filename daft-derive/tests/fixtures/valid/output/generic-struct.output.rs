#[derive(Debug, PartialEq, Eq)]
struct StructWithGenericsDiff<
    'daft,
    'd: 'daft,
    'e: 'daft,
    T: Eq + Debug + 'daft,
    U: Eq + Debug + 'daft,
>
where
    T: daft::Diffable<'d>,
    U: daft::Diffable<'e>,
{
    b: <usize as daft::Diffable<'daft>>::Diff,
    c: <T as daft::Diffable<'d>>::Diff,
    d: <U as daft::Diffable<'e>>::Diff,
}
impl<
    'daft,
    'd: 'daft,
    'e: 'daft,
    T: Eq + Debug + 'daft,
    U: Eq + Debug + 'daft,
> daft::Diffable<'daft> for StructWithGenerics<'d, 'e, T, U>
where
    T: daft::Diffable<'d>,
    U: daft::Diffable<'e>,
{
    type Diff = StructWithGenericsDiff<'daft, 'd, 'e, T, U>;
    fn diff(&'daft self, other: &'daft Self) -> Self::Diff {
        Self::Diff {
            b: daft::Diffable::diff(&self.b, &other.b),
            c: daft::Diffable::diff(&*self.c, &*other.c),
            d: daft::Diffable::diff(&*self.d, &*other.d),
        }
    }
}
