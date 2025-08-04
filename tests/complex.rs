#![allow(unused)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use wrapper_lite::*;

wrapper! {
    pub struct TestWrapperEmpty<'a, 'b> {
        inner: String,
        _a: ::core::marker::PhantomData<&'a ()>,
        _b: ::core::marker::PhantomData<&'b ()>
    }
}

wrapper! {
    pub struct TestWrapperEmptyWithTailing<'a, 'b> {
        inner: String,
        _a: ::core::marker::PhantomData<&'a ()>,
        _b: ::core::marker::PhantomData<&'b ()>,
    }
}

wrapper! {
    pub struct TestWrapperEmptyWithDefault<'a, 'b> {
        inner: String,
        _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
        _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData
    }
}

wrapper! {
    pub struct TestWrapperEmptyWithDefaultWithTailing<'a, 'b> {
        inner: String,
        _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
        _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData,
    }
}

wrapper! {
    #[wrapper_impl(AsMut)]
    #[wrapper_impl(AsRef)]
    #[wrapper_impl(Borrow)]
    #[wrapper_impl(Debug)]
    #[wrapper_impl(DerefMut)]
    #[wrapper_impl(From)]
    #[derive(Clone)]
    #[repr(transparent)]
    pub struct TestWrapperComplexConstFromInner<'a, 'b> {
        inner: String,
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
    #[wrapper_impl(Borrow)]
    #[wrapper_impl(Debug)]
    #[wrapper_impl(DerefMut)]
    #[wrapper_impl(From)]
    #[repr(transparent)]
    #[derive(Clone)]
    pub struct TestWrapperComplex<'a, 'b: 'a, P: Sized + Clone> {
        inner: P,
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
