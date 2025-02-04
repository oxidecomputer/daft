struct InnerDiff<'__daft> {
    a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    b: <i32 as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::std::fmt::Debug for InnerDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(InnerDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl<'__daft> ::std::cmp::PartialEq for InnerDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'__daft> ::std::cmp::Eq for InnerDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl ::daft::Diffable for Inner {
    type Diff<'__daft> = InnerDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> InnerDiff<'__daft> {
        Self::Diff {
            a: ::daft::Diffable::diff(&self.a, &other.a),
            b: ::daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
struct OuterDiff<'__daft> {
    inner: <Inner as ::daft::Diffable>::Diff<'__daft>,
    c: <i32 as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::std::fmt::Debug for OuterDiff<'__daft>
where
    <Inner as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(OuterDiff))
            .field(stringify!(inner), &self.inner)
            .field(stringify!(c), &self.c)
            .finish()
    }
}
impl<'__daft> ::std::cmp::PartialEq for OuterDiff<'__daft>
where
    <Inner as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.c == other.c
    }
}
impl<'__daft> ::std::cmp::Eq for OuterDiff<'__daft>
where
    <Inner as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl ::daft::Diffable for Outer {
    type Diff<'__daft> = OuterDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> OuterDiff<'__daft> {
        Self::Diff {
            inner: ::daft::Diffable::diff(&self.inner, &other.inner),
            c: ::daft::Diffable::diff(&self.c, &other.c),
        }
    }
}
