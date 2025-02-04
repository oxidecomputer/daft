struct StructWithGenericsDiff<'__daft, 'd: '__daft, 'e: '__daft, T: '__daft, U: '__daft>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
{
    b: <usize as ::daft::Diffable>::Diff<'__daft>,
    c: <&'d T as ::daft::Diffable>::Diff<'__daft>,
    d: <&'e U as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft, 'd: '__daft, 'e: '__daft, T: '__daft, U: '__daft> ::std::fmt::Debug
for StructWithGenericsDiff<'__daft, 'd, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <&'d T as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <&'e U as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(StructWithGenericsDiff))
            .field(stringify!(b), &self.b)
            .field(stringify!(c), &self.c)
            .field(stringify!(d), &self.d)
            .finish()
    }
}
impl<'__daft, 'd: '__daft, 'e: '__daft, T: '__daft, U: '__daft> ::std::cmp::PartialEq
for StructWithGenericsDiff<'__daft, 'd, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <&'d T as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <&'e U as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.b == other.b && self.c == other.c && self.d == other.d
    }
}
impl<'__daft, 'd: '__daft, 'e: '__daft, T: '__daft, U: '__daft> ::std::cmp::Eq
for StructWithGenericsDiff<'__daft, 'd, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
    <usize as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <&'d T as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <&'e U as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl<'d, 'e, T, U> ::daft::Diffable for StructWithGenerics<'d, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
{
    type Diff<'__daft> = StructWithGenericsDiff<'__daft, 'd, 'e, T, U>
    where
        Self: '__daft;
    fn diff<'__daft>(
        &'__daft self,
        other: &'__daft Self,
    ) -> StructWithGenericsDiff<'__daft, 'd, 'e, T, U> {
        Self::Diff {
            b: ::daft::Diffable::diff(&self.b, &other.b),
            c: ::daft::Diffable::diff(&self.c, &other.c),
            d: ::daft::Diffable::diff(&self.d, &other.d),
        }
    }
}
