#![allow(unused)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use wrapper_lite::*;

wrapper! {
    #[repr(align(cache))]
    pub struct TestWrapperCachePadded<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()>,
        _b: ::core::marker::PhantomData<&'b ()>
    }
}

wrapper! {
    #[repr(align(cache))]
    pub struct TestWrapperCachePaddedWithDefault<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
        _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData
    }
}

#[test]
fn test_align_of_TestWrapperCachePadded() {
    use core::mem::align_of;

    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "powerpc64",
    ))]
    {
        assert_eq!(align_of::<TestWrapperCachePadded>(), 128);
        assert_eq!(align_of::<TestWrapperCachePaddedWithDefault>(), 128);
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "sparc",
        target_arch = "hexagon",
    ))]
    {
        assert_eq!(align_of::<TestWrapperCachePadded>(), 32);
        assert_eq!(align_of::<TestWrapperCachePaddedWithDefault>(), 32);
    }

    #[cfg(target_arch = "m68k")]
    {
        assert_eq!(align_of::<TestWrapperCachePadded>(), 16);
        assert_eq!(align_of::<TestWrapperCachePaddedWithDefault>(), 16);
    }

    #[cfg(target_arch = "s390x")]
    {
        assert_eq!(align_of::<TestWrapperCachePadded>(), 256);
        assert_eq!(align_of::<TestWrapperCachePaddedWithDefault>(), 256);
    }

    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "powerpc64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "sparc",
        target_arch = "hexagon",
        target_arch = "m68k",
        target_arch = "s390x",
    )))]
    {
        assert_eq!(align_of::<TestWrapperCachePadded>(), 64);
        assert_eq!(align_of::<TestWrapperCachePaddedWithDefault>(), 64);
    }
}

wrapper! {
    pub struct TestWrapperEmptyWithTailing<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()>,
        _b: ::core::marker::PhantomData<&'b ()>
    }
}

wrapper!(
    pub struct TestWrapperEmpty<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()>,
        _b: ::core::marker::PhantomData<&'b ()>,
    }
);

wrapper! {
    pub struct TestWrapperEmptyWithDefault<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
        _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData
    }
}

wrapper!(
    pub struct TestWrapperEmptyWithDefaultWithTailing<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
        _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData,
    }
);

wrapper! {
    #[wrapper_impl(AsMut)]
    #[wrapper_impl(AsRef)]
    // #[wrapper_impl(Borrow)]
    #[wrapper_impl(BorrowMut)]
    #[wrapper_impl(Debug)]
    // #[wrapper_impl(Deref)]
    #[wrapper_impl(DerefMut)]
    #[wrapper_impl(From)]
    #[derive(Clone)]
    #[repr(transparent)]
    pub struct TestWrapperComplexConstFromInner<'a, 'b> {
        inner_can_be_any_name: String,
        _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
        _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData
    }
}

#[test]
fn assert_impls_TestWrapperComplexConstFromInner() {
    _assert_impl_debug::<TestWrapperComplexConstFromInner<'_, '_>>();
    _assert_impl_as_ref::<TestWrapperComplexConstFromInner<'_, '_>, _>();
    _assert_impl_as_mut::<TestWrapperComplexConstFromInner<'_, '_>, _>();
    _assert_impl_borrow::<TestWrapperComplexConstFromInner<'_, '_>, String>();
    _assert_impl_borrow_mut::<TestWrapperComplexConstFromInner<'_, '_>, String>();
    _assert_impl_deref::<TestWrapperComplexConstFromInner<'_, '_>, _>();
    _assert_impl_deref_mut::<TestWrapperComplexConstFromInner<'_, '_>, _>();
    _assert_impl_from::<TestWrapperComplexConstFromInner<'_, '_>, String>();

    assert_eq!(
        core::mem::size_of::<TestWrapperComplexConstFromInner<'_, '_>>(),
        core::mem::size_of::<String>()
    );
}

wrapper! {
    #[wrapper_impl(AsMut)]
    #[wrapper_impl(AsRef)]
    // #[wrapper_impl(Borrow)]
    #[wrapper_impl(BorrowMut)]
    #[wrapper_impl(Debug)]
    // #[wrapper_impl(Deref)]
    #[wrapper_impl(DerefMut)]
    // #[wrapper_impl(From)]
    #[repr(transparent)]
    #[derive(Clone)]
    pub struct TestWrapperComplex<'a, 'b: 'a, P: Sized + Clone> {
        inner_can_be_any_name: P,
        _a: ::core::marker::PhantomData<&'a ()>,
        _b: ::core::marker::PhantomData<&'b ()>
    }
}

#[test]
fn assert_impls_TestWrapperComplex() {
    _assert_impl_debug::<TestWrapperComplex<'_, '_, String>>();
    _assert_impl_as_ref::<TestWrapperComplex<'_, '_, String>, _>();
    _assert_impl_as_mut::<TestWrapperComplex<'_, '_, String>, _>();
    _assert_impl_borrow::<TestWrapperComplex<'_, '_, String>, String>();
    _assert_impl_borrow_mut::<TestWrapperComplex<'_, '_, String>, String>();
    _assert_impl_deref::<TestWrapperComplex<'_, '_, String>, _>();
    _assert_impl_deref_mut::<TestWrapperComplex<'_, '_, String>, _>();
    // _assert_impl_from::<TestWrapperComplex<'_, '_, String>, String>();

    assert_eq!(
        core::mem::size_of::<TestWrapperComplex<'_, '_, String>>(),
        core::mem::size_of::<String>()
    );
}

// === utilities ===

fn _assert_impl_debug<T>()
where
    T: ::core::fmt::Debug,
{
}

fn _assert_impl_as_ref<T, U>()
where
    T: ::core::convert::AsRef<U>,
{
}

fn _assert_impl_as_mut<T, U>()
where
    T: ::core::convert::AsMut<U>,
{
}

fn _assert_impl_borrow<T, U>()
where
    T: ::core::borrow::Borrow<U>,
{
}

fn _assert_impl_borrow_mut<T, U>()
where
    T: ::core::borrow::BorrowMut<U>,
{
}

fn _assert_impl_deref<T, U>()
where
    T: ::core::ops::Deref<Target = U>,
{
}

fn _assert_impl_deref_mut<T, U>()
where
    T: ::core::ops::DerefMut<Target = U>,
{
}

fn _assert_impl_from<T, U>()
where
    T: ::core::convert::From<U>,
{
}
