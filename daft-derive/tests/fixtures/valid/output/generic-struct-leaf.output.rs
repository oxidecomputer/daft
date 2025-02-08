impl<'d, 'e, T, U> ::daft::Diffable for StructWithGenerics<'d, 'e, T, U>
where
    T: 'd + ?Sized,
    U: 'e + ?Sized,
{
    type Diff<'__daft> = ::daft::Leaf<&'__daft Self> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> Self::Diff<'__daft> {
        ::daft::Leaf {
            before: self,
            after: other,
        }
    }
}
