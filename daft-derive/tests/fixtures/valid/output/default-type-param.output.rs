struct StructWithDefaultTypeParamDiff<'__daft, T: Diffable + '__daft = ()> {
    field: <T as daft::Diffable>::Diff<'__daft>,
}
impl<'__daft, T: Diffable + '__daft> ::std::fmt::Debug
for StructWithDefaultTypeParamDiff<'__daft, T>
where
    <T as daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(StructWithDefaultTypeParamDiff))
            .field(stringify!(field), &self.field)
            .finish()
    }
}
impl<'__daft, T: Diffable + '__daft> ::std::cmp::PartialEq
for StructWithDefaultTypeParamDiff<'__daft, T>
where
    <T as daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}
impl<'__daft, T: Diffable + '__daft> ::std::cmp::Eq
for StructWithDefaultTypeParamDiff<'__daft, T>
where
    <T as daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl<T: Diffable> daft::Diffable for StructWithDefaultTypeParam<T> {
    type Diff<'__daft> = StructWithDefaultTypeParamDiff<'__daft, T> where Self: '__daft;
    fn diff<'__daft>(
        &'__daft self,
        other: &'__daft Self,
    ) -> StructWithDefaultTypeParamDiff<'__daft, T> {
        Self::Diff {
            field: daft::Diffable::diff(&self.field, &other.field),
        }
    }
}
