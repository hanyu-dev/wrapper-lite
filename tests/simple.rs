#![allow(unused)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use wrapper_lite::*;

wrapper!(
    #[wrapper_impl(From)]
    #[repr(align(cache))]
    pub struct TestWrapperCachePadded(String);
);

#[test]
fn test_align_of_TestWrapperCachePadded() {
    use core::mem::align_of;

    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "powerpc64",
    ))]
    assert_eq!(align_of::<TestWrapperCachePadded>(), 128);

    #[cfg(any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "sparc",
        target_arch = "hexagon",
    ))]
    assert_eq!(align_of::<TestWrapperCachePadded>(), 32);

    #[cfg(target_arch = "m68k")]
    assert_eq!(align_of::<TestWrapperCachePadded>(), 16);

    #[cfg(target_arch = "s390x")]
    assert_eq!(align_of::<TestWrapperCachePadded>(), 256);

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
    assert_eq!(align_of::<TestWrapperCachePadded>(), 64);
}

wrapper!(
    #[wrapper_impl(AsMut)]
    #[wrapper_impl(AsRef)]
    // #[wrapper_impl(Borrow)]
    #[wrapper_impl(BorrowMut)]
    #[wrapper_impl(Debug)]
    #[wrapper_impl(DerefMut)]
    #[wrapper_impl(From)] // Multiple wrapper_impl attributes
    #[derive(Clone, Default)]
    #[derive(PartialEq)] // Multiple derive attributes
    /// Test Docs
    pub struct TestWrapperAllMixed(String);
);

#[test]
fn assert_impls_TestWrapperAllMixed() {
    _assert_impl_debug::<TestWrapperAllMixed>();
    _assert_impl_as_ref::<TestWrapperAllMixed, _>();
    _assert_impl_as_mut::<TestWrapperAllMixed, _>();
    _assert_impl_borrow::<TestWrapperAllMixed, String>();
    _assert_impl_borrow_mut::<TestWrapperAllMixed, String>();
    _assert_impl_deref_mut::<TestWrapperAllMixed, _>();
    _assert_impl_from::<TestWrapperAllMixed, String>();

    assert_eq!(
        core::mem::size_of::<TestWrapperAllMixed>(),
        core::mem::size_of::<String>()
    );
}

wrapper!(
    #[wrapper_impl(AsRef)]
    pub struct TestWrapperImplAsRef(String);
);

fn assert_impls_TestWrapperImplAsRef() {
    _assert_impl_as_ref::<TestWrapperImplAsRef, _>();
}

#[test]
fn assert_size_eq() {
    use core::mem::size_of;

    assert_eq!(size_of::<TestWrapperImplAsRef>(), size_of::<String>());
}

wrapper!(
    #[wrapper_impl(AsRef)]
    #[derive(Debug)]
    pub struct TestWrapperImplAsRefMixed(String);
);

fn assert_impls_TestWrapperImplAsRefMixed() {
    _assert_impl_debug::<TestWrapperImplAsRefMixed>();
    _assert_impl_as_ref::<TestWrapperImplAsRefMixed, _>();
}

wrapper!(
    #[wrapper_impl(Borrow)]
    pub struct TestWrapperImplBorrow(String);
);

fn assert_impls_TestWrapperImplBorrow() {
    _assert_impl_borrow::<TestWrapperImplBorrow, String>();
}

wrapper!(
    #[wrapper_impl(BorrowMut)]
    pub struct TestWrapperImplBorrowMut(String);
);

fn assert_impls_TestWrapperImplBorrowMut() {
    _assert_impl_borrow::<TestWrapperImplBorrowMut, String>();
    _assert_impl_borrow_mut::<TestWrapperImplBorrowMut, String>();
}

wrapper!(
    #[wrapper_impl(Borrow)]
    #[derive(Debug)]
    pub struct TestWrapperImplBorrowMixed(String);
);

fn assert_impls_TestWrapperImplBorrowMixed() {
    _assert_impl_debug::<TestWrapperImplBorrowMixed>();
    _assert_impl_borrow::<TestWrapperImplBorrowMixed, String>();
}

wrapper!(
    #[wrapper_impl(Debug)]
    pub struct TestWrapperImplDebug(String);
);

wrapper!(
    #[wrapper_impl(DebugName)]
    pub struct TestWrapperImplDebugName(String);
);

fn assert_impls_TestWrapperImplDebug() {
    _assert_impl_debug::<TestWrapperImplDebug>();
    _assert_impl_debug::<TestWrapperImplDebugName>();
}

wrapper!(
    #[wrapper_impl(Debug)]
    #[derive(Default)]
    pub struct TestWrapperImplDebugMixed(String);
);

wrapper!(
    #[wrapper_impl(DebugName)]
    #[derive(Default)]
    pub struct TestWrapperImplDebugNameMixed(String);
);

fn assert_impls_TestWrapperImplDebugNameMixed() {
    _assert_impl_debug::<TestWrapperImplDebugMixed>();
    _assert_impl_debug::<TestWrapperImplDebugNameMixed>();
}

wrapper!(
    #[wrapper_impl(Deref)]
    pub struct TestWrapperImplDeref(String);
);

fn assert_impls_TestWrapperImplDeref() {
    _assert_impl_deref::<TestWrapperImplDeref, _>();
}

wrapper!(
    #[wrapper_impl(Deref)]
    #[derive(Debug)]
    pub struct TestWrapperImplDerefMixed(String);
);

fn assert_impls_TestWrapperImplDerefMixed() {
    _assert_impl_debug::<TestWrapperImplDerefMixed>();
    _assert_impl_deref::<TestWrapperImplDerefMixed, _>();
}

wrapper!(
    #[wrapper_impl(DerefMut)]
    pub struct TestWrapperImplDerefMut(String);
);

fn assert_impls_TestWrapperImplDerefMut() {
    _assert_impl_deref::<TestWrapperImplDerefMut, _>();
    _assert_impl_deref_mut::<TestWrapperImplDerefMut, _>();
}

wrapper!(
    #[wrapper_impl(DerefMut)]
    #[derive(Debug)]
    pub struct TestWrapperImplDerefMutMixed(String);
);

fn assert_impls_TestWrapperImplDerefMutMixed() {
    _assert_impl_debug::<TestWrapperImplDerefMutMixed>();
    _assert_impl_deref::<TestWrapperImplDerefMutMixed, _>();
    _assert_impl_deref_mut::<TestWrapperImplDerefMutMixed, _>();
}

wrapper!(
    #[wrapper_impl(From)]
    pub struct TestWrapperImplFrom(String);
);

fn assert_impls_TestWrapperImplFrom() {
    _assert_impl_from::<TestWrapperImplFrom, String>();
}

wrapper!(
    #[wrapper_impl(From)]
    #[derive(Debug)]
    pub struct TestWrapperImplFromMixed(String);
);

fn assert_impls_TestWrapperImplFromMixed() {
    _assert_impl_debug::<TestWrapperImplFromMixed>();
    _assert_impl_from::<TestWrapperImplFromMixed, String>();
}

// Test multiple
wrapper!(
    #[wrapper_impl(Deref)]
    #[wrapper_impl(From)]
    #[derive(Debug)]
    pub struct TestWrapperImplFromDeref(String);
);

fn assert_impls_TestWrapperImplFromDeref() {
    _assert_impl_debug::<TestWrapperImplFromDeref>();
    _assert_impl_deref::<TestWrapperImplFromDeref, _>();
    _assert_impl_from::<TestWrapperImplFromDeref, String>();
}

// Test multiple with lifetimes
wrapper!(
    #[wrapper_impl(Deref)]
    #[wrapper_impl(From)]
    #[derive(Debug)]
    pub struct TestWrapperImplFromDerefMixed<'a, P>(pub(crate) &'a P);
);

fn assert_impls_TestWrapperImplFromDerefMixed<P: core::fmt::Debug>() {
    _assert_impl_debug::<TestWrapperImplFromDerefMixed<'_, P>>();
    _assert_impl_deref::<TestWrapperImplFromDerefMixed<'_, P>, _>();
    _assert_impl_from::<TestWrapperImplFromDerefMixed<'_, P>, &P>();
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
