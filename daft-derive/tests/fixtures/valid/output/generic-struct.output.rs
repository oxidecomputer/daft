struct StructWithGenericsDiff<'daft, 'd: 'daft, 'e: 'daft, T: 'daft, U: 'daft>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
{
    b: <usize as daft::Diffable>::Diff<'daft>,
    c: <&'d T as daft::Diffable>::Diff<'daft>,
    d: <&'e U as daft::Diffable>::Diff<'daft>,
}
impl<'daft, 'd: 'daft, 'e: 'daft, T: 'daft, U: 'daft> ::std::fmt::Debug
for StructWithGenericsDiff<'daft, 'd, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
    <usize as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <&'d T as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <&'e U as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(StructWithGenericsDiff))
            .field(stringify!(b), &self.b)
            .field(stringify!(c), &self.c)
            .field(stringify!(d), &self.d)
            .finish()
    }
}
impl<'daft, 'd: 'daft, 'e: 'daft, T: 'daft, U: 'daft> ::std::cmp::PartialEq
for StructWithGenericsDiff<'daft, 'd, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
    <usize as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <&'d T as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <&'e U as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.b == other.b && self.c == other.c && self.d == other.d
    }
}
impl<'daft, 'd: 'daft, 'e: 'daft, T: 'daft, U: 'daft> ::std::cmp::Eq
for StructWithGenericsDiff<'daft, 'd, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
    <usize as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <&'d T as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <&'e U as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
{}
impl<'d, 'e, T, U> daft::Diffable for StructWithGenerics<'d, 'e, T, U>
where
    T: daft::Diffable + 'd + ?Sized,
    U: daft::Diffable + 'e + ?Sized,
{
    type Diff<'daft> = StructWithGenericsDiff<'daft, 'd, 'e, T, U> where Self: 'daft;
    fn diff<'daft>(
        &'daft self,
        other: &'daft Self,
    ) -> StructWithGenericsDiff<'daft, 'd, 'e, T, U> {
        Self::Diff {
            b: daft::Diffable::diff(&self.b, &other.b),
            c: daft::Diffable::diff(&self.c, &other.c),
            d: daft::Diffable::diff(&self.d, &other.d),
        }
    }
}
