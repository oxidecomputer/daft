struct MyStructDiff<'__daft> {
    a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    b: <NonDiffable as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::std::fmt::Debug for MyStructDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <NonDiffable as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(MyStructDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl<'__daft> ::std::cmp::PartialEq for MyStructDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <NonDiffable as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'__daft> ::std::cmp::Eq for MyStructDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <NonDiffable as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl ::daft::Diffable for MyStruct {
    type Diff<'__daft> = MyStructDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> MyStructDiff<'__daft> {
        Self::Diff {
            a: ::daft::Diffable::diff(&self.a, &other.a),
            b: ::daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
