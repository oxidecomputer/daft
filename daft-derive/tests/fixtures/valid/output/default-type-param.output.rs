struct StructWithDefaultTypeParamDiff<'daft, T: Diffable + 'daft = ()> {
    field: <T as daft::Diffable>::Diff<'daft>,
}
impl<'daft, T: Diffable + 'daft> ::std::fmt::Debug
for StructWithDefaultTypeParamDiff<'daft, T>
where
    <T as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(StructWithDefaultTypeParamDiff))
            .field(stringify!(field), &self.field)
            .finish()
    }
}
impl<'daft, T: Diffable + 'daft> ::std::cmp::PartialEq
for StructWithDefaultTypeParamDiff<'daft, T>
where
    <T as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}
impl<'daft, T: Diffable + 'daft> ::std::cmp::Eq
for StructWithDefaultTypeParamDiff<'daft, T>
where
    <T as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
{}
impl<T: Diffable> daft::Diffable for StructWithDefaultTypeParam<T> {
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
