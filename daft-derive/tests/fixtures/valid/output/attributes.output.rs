struct WithAttrsDiff<'__daft> {
    a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::Diffable>::Diff<'__daft>,
    d: ::daft::Leaf<'__daft, Lazy>,
    e: ::daft::Leaf<'__daft, usize>,
    f: <usize as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::std::fmt::Debug for WithAttrsDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    ::daft::Leaf<'__daft, Lazy>: ::std::fmt::Debug,
    ::daft::Leaf<'__daft, usize>: ::std::fmt::Debug,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(WithAttrsDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .field(stringify!(d), &self.d)
            .field(stringify!(e), &self.e)
            .field(stringify!(f), &self.f)
            .finish()
    }
}
impl<'__daft> ::std::cmp::PartialEq for WithAttrsDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    ::daft::Leaf<'__daft, Lazy>: ::std::cmp::PartialEq,
    ::daft::Leaf<'__daft, usize>: ::std::cmp::PartialEq,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.d == other.d && self.e == other.e
            && self.f == other.f
    }
}
impl<'__daft> ::std::cmp::Eq for WithAttrsDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    ::daft::Leaf<'__daft, Lazy>: ::std::cmp::Eq,
    ::daft::Leaf<'__daft, usize>: ::std::cmp::Eq,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl ::daft::Diffable for WithAttrs {
    type Diff<'__daft> = WithAttrsDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> WithAttrsDiff<'__daft> {
        Self::Diff {
            a: ::daft::Diffable::diff(&self.a, &other.a),
            b: ::daft::Diffable::diff(&self.b, &other.b),
            d: ::daft::Leaf {
                before: &self.d,
                after: &other.d,
            },
            e: ::daft::Leaf {
                before: &self.e,
                after: &other.e,
            },
            f: ::daft::Diffable::diff(&self.f, &other.f),
        }
    }
}
struct LazyDiff<'__daft> {
    x: <usize as ::daft::Diffable>::Diff<'__daft>,
    y: <usize as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::std::fmt::Debug for LazyDiff<'__daft>
where
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(LazyDiff))
            .field(stringify!(x), &self.x)
            .field(stringify!(y), &self.y)
            .finish()
    }
}
impl<'__daft> ::std::cmp::PartialEq for LazyDiff<'__daft>
where
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl<'__daft> ::std::cmp::Eq for LazyDiff<'__daft>
where
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl ::daft::Diffable for Lazy {
    type Diff<'__daft> = LazyDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> LazyDiff<'__daft> {
        Self::Diff {
            x: ::daft::Diffable::diff(&self.x, &other.x),
            y: ::daft::Diffable::diff(&self.y, &other.y),
        }
    }
}
