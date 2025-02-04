struct SDiff<'__daft, 'a: '__daft, T: '__daft, U: '__daft>
where
    T: Diffable + Eq + 'a,
    U: Diffable + 'a,
{
    a: <BTreeMap<usize, T> as ::daft::Diffable>::Diff<'__daft>,
    b: <usize as ::daft::Diffable>::Diff<'__daft>,
    c: <&'a U as ::daft::Diffable>::Diff<'__daft>,
    d: <&'a str as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft, 'a: '__daft, T: '__daft, U: '__daft> ::std::fmt::Debug
for SDiff<'__daft, 'a, T, U>
where
    T: Diffable + Eq + 'a,
    U: Diffable + 'a,
    <BTreeMap<usize, T> as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <&'a U as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <&'a str as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(SDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .field(stringify!(c), &self.c)
            .field(stringify!(d), &self.d)
            .finish()
    }
}
impl<'__daft, 'a: '__daft, T: '__daft, U: '__daft> ::std::cmp::PartialEq
for SDiff<'__daft, 'a, T, U>
where
    T: Diffable + Eq + 'a,
    U: Diffable + 'a,
    <BTreeMap<usize, T> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <&'a U as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <&'a str as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}
impl<'__daft, 'a: '__daft, T: '__daft, U: '__daft> ::std::cmp::Eq
for SDiff<'__daft, 'a, T, U>
where
    T: Diffable + Eq + 'a,
    U: Diffable + 'a,
    <BTreeMap<usize, T> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <&'a U as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <&'a str as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl<'a, T, U> ::daft::Diffable for S<'a, T, U>
where
    T: Diffable + Eq + 'a,
    U: Diffable + 'a,
{
    type Diff<'__daft> = SDiff<'__daft, 'a, T, U> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> SDiff<'__daft, 'a, T, U> {
        Self::Diff {
            a: ::daft::Diffable::diff(&self.a, &other.a),
            b: ::daft::Diffable::diff(&self.b, &other.b),
            c: ::daft::Diffable::diff(&self.c, &other.c),
            d: ::daft::Diffable::diff(&self.d, &other.d),
        }
    }
}
