struct MyStructDiff<'__daft> {
    a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    b: <NonDiffable as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::core::fmt::Debug for MyStructDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
    <NonDiffable as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(MyStructDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for MyStructDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
    <NonDiffable as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'__daft> ::core::cmp::Eq for MyStructDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
    <NonDiffable as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
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
