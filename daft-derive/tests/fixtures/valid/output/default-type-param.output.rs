#[derive(Debug, PartialEq, Eq)]
struct StructWithDefaultTypeParamDiff<'daft, T: Eq + Debug + 'daft = ()>
where
    for<'x> T: Diffable<'x>,
{
    field: <T as daft::Diffable<'daft>>::Diff,
}
impl<'daft, T: Eq + Debug + 'daft> daft::Diffable<'daft>
for StructWithDefaultTypeParam<T>
where
    for<'x> T: Diffable<'x>,
{
    type Diff = StructWithDefaultTypeParamDiff<'daft, T>;
    fn diff(&'daft self, other: &'daft Self) -> Self::Diff {
        Self::Diff {
            field: daft::Diffable::diff(&self.field, &other.field),
        }
    }
}
