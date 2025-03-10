#[non_exhaustive]
struct NonExhaustiveDiff<'__daft> {
    pub a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    pub b: <i32 as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::core::fmt::Debug for NonExhaustiveDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(NonExhaustiveDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish_non_exhaustive()
    }
}
impl<'__daft> ::core::cmp::PartialEq for NonExhaustiveDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'__daft> ::core::cmp::Eq for NonExhaustiveDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
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
