#[non_exhaustive]
struct NonExhaustiveDiff<'__daft> {
    pub a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    pub b: <i32 as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::std::fmt::Debug for NonExhaustiveDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(NonExhaustiveDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish_non_exhaustive()
    }
}
impl<'__daft> ::std::cmp::PartialEq for NonExhaustiveDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'__daft> ::std::cmp::Eq for NonExhaustiveDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl ::daft::Diffable for NonExhaustive {
    type Diff<'__daft> = NonExhaustiveDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> NonExhaustiveDiff<'__daft> {
        Self::Diff {
            a: ::daft::Diffable::diff(&self.a, &other.a),
            b: ::daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
impl ::daft::Diffable for NonExhaustiveEnum {
    type Diff<'__daft> = ::daft::Leaf<&'__daft Self> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> Self::Diff<'__daft> {
        ::daft::Leaf {
            before: self,
            after: other,
        }
    }
}
