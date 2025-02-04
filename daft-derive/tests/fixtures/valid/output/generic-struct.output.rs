#[derive(Debug, PartialEq, Eq)]
struct StructWithGenericsDiff<
    'daft,
    'd: 'daft,
    'e: 'daft,
    T: Eq + Debug + 'daft,
    U: Eq + Debug + 'daft,
>
where
    T: daft::Diffable + 'd,
    U: daft::Diffable + 'e,
{
    b: <usize as daft::Diffable>::Diff<'daft>,
    c: <&'d T as daft::Diffable>::Diff<'daft>,
    d: <&'e U as daft::Diffable>::Diff<'daft>,
}
impl<'d, 'e, T: Eq + Debug, U: Eq + Debug> daft::Diffable
for StructWithGenerics<'d, 'e, T, U>
where
    T: daft::Diffable + 'd,
    U: daft::Diffable + 'e,
{
    type Diff<'daft> = StructWithGenericsDiff<'daft, 'd, 'e, T, U> where Self: 'daft;
    fn diff<'daft>(
        &'daft self,
        other: &'daft Self,
    ) -> StructWithGenericsDiff<'daft, 'd, 'e, T, U> {
        Self::Diff {
            b: daft::Diffable::diff(&self.b, &other.b),
            c: daft::Diffable::diff(&self.c, &other.c),
            d: daft::Diffable::diff(&self.d, &other.d),
        }
    }
}
