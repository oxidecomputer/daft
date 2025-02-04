#[derive(Debug, PartialEq, Eq)]
struct StructWithDefaultTypeParamDiff<'daft, T: Eq + Debug + Diffable + 'daft = ()> {
    field: <T as daft::Diffable>::Diff<'daft>,
}
impl<T: Eq + Debug + Diffable> daft::Diffable for StructWithDefaultTypeParam<T> {
    type Diff<'daft> = StructWithDefaultTypeParamDiff<'daft, T> where Self: 'daft;
    fn diff<'daft>(
        &'daft self,
        other: &'daft Self,
    ) -> StructWithDefaultTypeParamDiff<'daft, T> {
        Self::Diff {
            field: daft::Diffable::diff(&self.field, &other.field),
        }
    }
}
