struct MyStructDiff<'__daft> {
    a: ::daft::Leaf<&'__daft i32>,
}
impl<'__daft> ::core::fmt::Debug for MyStructDiff<'__daft>
where
    ::daft::Leaf<&'__daft i32>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(MyStructDiff)).field(stringify!(a), &self.a).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for MyStructDiff<'__daft>
where
    ::daft::Leaf<&'__daft i32>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}
impl<'__daft> ::core::cmp::Eq for MyStructDiff<'__daft>
where
    ::daft::Leaf<&'__daft i32>: ::core::cmp::Eq,
{}
impl ::daft::Diffable for MyStruct {
    type Diff<'__daft> = MyStructDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> MyStructDiff<'__daft> {
        Self::Diff {
            a: ::daft::Leaf {
                before: &self.a,
                after: &other.a,
            },
        }
    }
}
