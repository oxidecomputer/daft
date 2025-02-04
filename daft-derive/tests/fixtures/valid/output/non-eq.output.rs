struct InnerDiff<'daft> {
    a: <i32 as daft::Diffable>::Diff<'daft>,
    b: <i32 as daft::Diffable>::Diff<'daft>,
}
impl<'daft> ::std::fmt::Debug for InnerDiff<'daft>
where
    <i32 as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <i32 as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(InnerDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl<'daft> ::std::cmp::PartialEq for InnerDiff<'daft>
where
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'daft> ::std::cmp::Eq for InnerDiff<'daft>
where
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
{}
impl daft::Diffable for Inner {
    type Diff<'daft> = InnerDiff<'daft> where Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> InnerDiff<'daft> {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
struct OuterDiff<'daft> {
    inner: <Inner as daft::Diffable>::Diff<'daft>,
    c: <i32 as daft::Diffable>::Diff<'daft>,
}
impl<'daft> ::std::fmt::Debug for OuterDiff<'daft>
where
    <Inner as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <i32 as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(OuterDiff))
            .field(stringify!(inner), &self.inner)
            .field(stringify!(c), &self.c)
            .finish()
    }
}
impl<'daft> ::std::cmp::PartialEq for OuterDiff<'daft>
where
    <Inner as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.c == other.c
    }
}
impl<'daft> ::std::cmp::Eq for OuterDiff<'daft>
where
    <Inner as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
{}
impl daft::Diffable for Outer {
    type Diff<'daft> = OuterDiff<'daft> where Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> OuterDiff<'daft> {
        Self::Diff {
            inner: daft::Diffable::diff(&self.inner, &other.inner),
            c: daft::Diffable::diff(&self.c, &other.c),
        }
    }
}
