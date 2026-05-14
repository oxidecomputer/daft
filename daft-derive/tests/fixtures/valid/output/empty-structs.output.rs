struct UnitStructDiff<'__daft> {
    _phantom: ::core::marker::PhantomData<fn() -> &'__daft UnitStruct>,
}
impl<'__daft> ::core::fmt::Debug for UnitStructDiff<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(UnitStructDiff)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for UnitStructDiff<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for UnitStructDiff<'__daft> {}
impl ::daft::Diffable for UnitStruct {
    type Diff<'__daft> = UnitStructDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> UnitStructDiff<'__daft> {
        Self::Diff {
            _phantom: ::core::marker::PhantomData,
        }
    }
}
struct EmptyNamedDiff<'__daft> {
    _phantom: ::core::marker::PhantomData<fn() -> &'__daft EmptyNamed>,
}
impl<'__daft> ::core::fmt::Debug for EmptyNamedDiff<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(EmptyNamedDiff)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for EmptyNamedDiff<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for EmptyNamedDiff<'__daft> {}
impl ::daft::Diffable for EmptyNamed {
    type Diff<'__daft> = EmptyNamedDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> EmptyNamedDiff<'__daft> {
        Self::Diff {
            _phantom: ::core::marker::PhantomData,
        }
    }
}
struct EmptyTupleDiff<'__daft>(::core::marker::PhantomData<fn() -> &'__daft EmptyTuple>);
impl<'__daft> ::core::fmt::Debug for EmptyTupleDiff<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!(EmptyTupleDiff)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for EmptyTupleDiff<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for EmptyTupleDiff<'__daft> {}
impl ::daft::Diffable for EmptyTuple {
    type Diff<'__daft> = EmptyTupleDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> EmptyTupleDiff<'__daft> {
        Self::Diff {
            0: ::core::marker::PhantomData,
        }
    }
}
struct AllIgnoredNamedDiff<'__daft> {
    _phantom: ::core::marker::PhantomData<fn() -> &'__daft AllIgnoredNamed>,
}
impl<'__daft> ::core::fmt::Debug for AllIgnoredNamedDiff<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(AllIgnoredNamedDiff)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for AllIgnoredNamedDiff<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for AllIgnoredNamedDiff<'__daft> {}
impl ::daft::Diffable for AllIgnoredNamed {
    type Diff<'__daft> = AllIgnoredNamedDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(
        &'__daft self,
        other: &'__daft Self,
    ) -> AllIgnoredNamedDiff<'__daft> {
        Self::Diff {
            _phantom: ::core::marker::PhantomData,
        }
    }
}
struct AllIgnoredTupleDiff<'__daft>(
    ::core::marker::PhantomData<fn() -> &'__daft AllIgnoredTuple>,
);
impl<'__daft> ::core::fmt::Debug for AllIgnoredTupleDiff<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!(AllIgnoredTupleDiff)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for AllIgnoredTupleDiff<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for AllIgnoredTupleDiff<'__daft> {}
impl ::daft::Diffable for AllIgnoredTuple {
    type Diff<'__daft> = AllIgnoredTupleDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(
        &'__daft self,
        other: &'__daft Self,
    ) -> AllIgnoredTupleDiff<'__daft> {
        Self::Diff {
            0: ::core::marker::PhantomData,
        }
    }
}
struct GenericAllIgnoredDiff<'__daft, T: '__daft> {
    _phantom: ::core::marker::PhantomData<fn() -> &'__daft GenericAllIgnored<T>>,
}
impl<'__daft, T: '__daft> ::core::fmt::Debug for GenericAllIgnoredDiff<'__daft, T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(GenericAllIgnoredDiff)).finish()
    }
}
impl<'__daft, T: '__daft> ::core::cmp::PartialEq for GenericAllIgnoredDiff<'__daft, T> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft, T: '__daft> ::core::cmp::Eq for GenericAllIgnoredDiff<'__daft, T> {}
impl<T> ::daft::Diffable for GenericAllIgnored<T> {
    type Diff<'__daft> = GenericAllIgnoredDiff<'__daft, T> where Self: '__daft;
    fn diff<'__daft>(
        &'__daft self,
        other: &'__daft Self,
    ) -> GenericAllIgnoredDiff<'__daft, T> {
        Self::Diff {
            _phantom: ::core::marker::PhantomData,
        }
    }
}
