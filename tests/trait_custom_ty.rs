#![allow(unused)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};

use wrapper_lite::*;

// === AsRef ===

wrapper!(
    #[wrapper_impl(AsRef<[u8]>)]
    pub struct TestWrapperAsRefStruct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsRef)]
    #[wrapper_impl(AsRef<[u8]>)]
    pub struct TestWrapperAsRefComplex0Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsRef<[u8]>)]
    #[wrapper_impl(AsRef)]
    pub struct TestWrapperAsRefComplex1Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsRef<[u8]>)]
    #[wrapper_impl(AsRef<Vec<u8>>)]
    pub struct TestWrapperAsRefComplex2Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsRef<[u8]>)]
    pub struct TestComplexWrapperAsRefStruct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(AsRef)]
    #[wrapper_impl(AsRef<[u8]>)]
    pub struct TestComplexWrapperAsRefComplex0Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(AsRef<[u8]>)]
    #[wrapper_impl(AsRef)]
    pub struct TestComplexWrapperAsRefComplex1Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(AsRef<[u8]>)]
    #[wrapper_impl(AsRef<Vec<u8>>)]
    pub struct TestComplexWrapperAsRefComplex2Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

#[test]
fn test_impl_AsRef() {
    _assert_impl_as_ref::<TestWrapperAsRefStruct, [u8]>();
    _assert_impl_as_ref::<TestWrapperAsRefComplex0Struct, [u8]>();
    _assert_impl_as_ref::<TestWrapperAsRefComplex0Struct, Vec<u8>>();
    _assert_impl_as_ref::<TestWrapperAsRefComplex1Struct, [u8]>();
    _assert_impl_as_ref::<TestWrapperAsRefComplex1Struct, Vec<u8>>();
    _assert_impl_as_ref::<TestWrapperAsRefComplex2Struct, [u8]>();
    _assert_impl_as_ref::<TestWrapperAsRefComplex2Struct, Vec<u8>>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefStruct, [u8]>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefComplex0Struct, [u8]>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefComplex0Struct, Vec<u8>>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefComplex1Struct, [u8]>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefComplex1Struct, Vec<u8>>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefComplex2Struct, [u8]>();
    _assert_impl_as_ref::<TestComplexWrapperAsRefComplex2Struct, Vec<u8>>();
}

fn _assert_impl_as_ref<T, U>()
where
    T: AsRef<U>,
    U: ?Sized,
{
}

// === AsRef ===

// === AsMut ===

wrapper!(
    #[wrapper_impl(AsMut<[u8]>)]
    pub struct TestWrapperAsMutStruct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsMut)]
    #[wrapper_impl(AsMut<[u8]>)]
    pub struct TestWrapperAsMutComplex0Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsMut<[u8]>)]
    #[wrapper_impl(AsMut)]
    pub struct TestWrapperAsMutComplex1Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsMut<[u8]>)]
    #[wrapper_impl(AsMut<Vec<u8>>)]
    pub struct TestWrapperAsMutComplex2Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(AsMut<[u8]>)]
    pub struct TestComplexWrapperAsMutStruct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(AsMut)]
    #[wrapper_impl(AsMut<[u8]>)]
    pub struct TestComplexWrapperAsMutComplex0Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(AsMut<[u8]>)]
    #[wrapper_impl(AsMut)]
    pub struct TestComplexWrapperAsMutComplex1Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(AsMut<[u8]>)]
    #[wrapper_impl(AsMut<Vec<u8>>)]
    pub struct TestComplexWrapperAsMutComplex2Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

#[test]
fn test_impl_AsMut() {
    _assert_impl_as_mut::<TestWrapperAsMutStruct, [u8]>();
    _assert_impl_as_mut::<TestWrapperAsMutComplex0Struct, [u8]>();
    _assert_impl_as_mut::<TestWrapperAsMutComplex0Struct, Vec<u8>>();
    _assert_impl_as_mut::<TestWrapperAsMutComplex1Struct, [u8]>();
    _assert_impl_as_mut::<TestWrapperAsMutComplex1Struct, Vec<u8>>();
    _assert_impl_as_mut::<TestWrapperAsMutComplex2Struct, [u8]>();
    _assert_impl_as_mut::<TestWrapperAsMutComplex2Struct, Vec<u8>>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutStruct, [u8]>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutComplex0Struct, [u8]>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutComplex0Struct, Vec<u8>>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutComplex1Struct, [u8]>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutComplex1Struct, Vec<u8>>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutComplex2Struct, [u8]>();
    _assert_impl_as_mut::<TestComplexWrapperAsMutComplex2Struct, Vec<u8>>();
}

fn _assert_impl_as_mut<T, U>()
where
    T: AsMut<U>,
    U: ?Sized,
{
}

// === AsMut ===

// === Borrow ===

wrapper!(
    #[wrapper_impl(Borrow<[u8]>)]
    pub struct TestWrapperBorrowStruct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(Borrow)]
    #[wrapper_impl(Borrow<[u8]>)]
    pub struct TestWrapperBorrowComplex0Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(Borrow<[u8]>)]
    #[wrapper_impl(Borrow)]
    pub struct TestWrapperBorrowComplex1Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(Borrow<[u8]>)]
    #[wrapper_impl(Borrow<Vec<u8>>)]
    pub struct TestWrapperBorrowComplex2Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(Borrow<[u8]>)]
    pub struct TestComplexWrapperBorrowStruct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(Borrow)]
    #[wrapper_impl(Borrow<[u8]>)]
    pub struct TestComplexWrapperBorrowComplex0Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(Borrow<[u8]>)]
    #[wrapper_impl(Borrow)]
    pub struct TestComplexWrapperBorrowComplex1Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(Borrow<[u8]>)]
    #[wrapper_impl(Borrow<Vec<u8>>)]
    pub struct TestComplexWrapperBorrowComplex2Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

#[test]
fn test_impl_Borrow() {
    _assert_impl_borrow::<TestWrapperBorrowStruct, [u8]>();
    _assert_impl_borrow::<TestWrapperBorrowComplex0Struct, [u8]>();
    _assert_impl_borrow::<TestWrapperBorrowComplex0Struct, Vec<u8>>();
    _assert_impl_borrow::<TestWrapperBorrowComplex1Struct, [u8]>();
    _assert_impl_borrow::<TestWrapperBorrowComplex1Struct, Vec<u8>>();
    _assert_impl_borrow::<TestWrapperBorrowComplex2Struct, [u8]>();
    _assert_impl_borrow::<TestWrapperBorrowComplex2Struct, Vec<u8>>();
    _assert_impl_borrow::<TestComplexWrapperBorrowStruct, [u8]>();
    _assert_impl_borrow::<TestComplexWrapperBorrowComplex0Struct, [u8]>();
    _assert_impl_borrow::<TestComplexWrapperBorrowComplex0Struct, Vec<u8>>();
    _assert_impl_borrow::<TestComplexWrapperBorrowComplex1Struct, [u8]>();
    _assert_impl_borrow::<TestComplexWrapperBorrowComplex1Struct, Vec<u8>>();
    _assert_impl_borrow::<TestComplexWrapperBorrowComplex2Struct, [u8]>();
    _assert_impl_borrow::<TestComplexWrapperBorrowComplex2Struct, Vec<u8>>();
}

fn _assert_impl_borrow<T, U>()
where
    T: Borrow<U>,
    U: ?Sized,
{
}

// === Borrow ===

// === BorrowMut ===

wrapper!(
    #[wrapper_impl(BorrowMut<[u8]>)]
    pub struct TestWrapperBorrowMutStruct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(BorrowMut)]
    #[wrapper_impl(BorrowMut<[u8]>)]
    pub struct TestWrapperBorrowMutComplex0Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(BorrowMut<[u8]>)]
    #[wrapper_impl(BorrowMut)]
    pub struct TestWrapperBorrowMutComplex1Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(BorrowMut<[u8]>)]
    #[wrapper_impl(BorrowMut<Vec<u8>>)]
    pub struct TestWrapperBorrowMutComplex2Struct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(BorrowMut<[u8]>)]
    pub struct TestComplexWrapperBorrowMutStruct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(BorrowMut)]
    #[wrapper_impl(BorrowMut<[u8]>)]
    pub struct TestComplexWrapperBorrowMutComplex0Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(BorrowMut<[u8]>)]
    #[wrapper_impl(BorrowMut)]
    pub struct TestComplexWrapperBorrowMutComplex1Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

wrapper!(
    #[wrapper_impl(BorrowMut<[u8]>)]
    #[wrapper_impl(BorrowMut<Vec<u8>>)]
    pub struct TestComplexWrapperBorrowMutComplex2Struct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

#[test]
fn test_impl_BorrowMut() {
    _assert_impl_borrow_mut::<TestWrapperBorrowMutStruct, [u8]>();
    _assert_impl_borrow_mut::<TestWrapperBorrowMutComplex0Struct, [u8]>();
    _assert_impl_borrow_mut::<TestWrapperBorrowMutComplex0Struct, Vec<u8>>();
    _assert_impl_borrow_mut::<TestWrapperBorrowMutComplex1Struct, [u8]>();
    _assert_impl_borrow_mut::<TestWrapperBorrowMutComplex1Struct, Vec<u8>>();
    _assert_impl_borrow_mut::<TestWrapperBorrowMutComplex2Struct, [u8]>();
    _assert_impl_borrow_mut::<TestWrapperBorrowMutComplex2Struct, Vec<u8>>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutStruct, [u8]>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutComplex0Struct, [u8]>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutComplex0Struct, Vec<u8>>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutComplex1Struct, [u8]>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutComplex1Struct, Vec<u8>>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutComplex2Struct, [u8]>();
    _assert_impl_borrow_mut::<TestComplexWrapperBorrowMutComplex2Struct, Vec<u8>>();
}

fn _assert_impl_borrow_mut<T, U>()
where
    T: BorrowMut<U>,
    U: ?Sized,
{
}

// === BorrowMut ===

// === Deref ===

wrapper!(
    #[wrapper_impl(Deref<[u8]>)]
    pub struct TestWrapperDerefStruct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(Deref<[u8]>)]
    pub struct TestComplexWrapperDerefStruct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

#[test]
fn test_impl_Deref() {
    _assert_impl_deref::<TestWrapperDerefStruct, [u8]>();
    _assert_impl_deref::<TestComplexWrapperDerefStruct, [u8]>();
}

fn _assert_impl_deref<T, U>()
where
    T: Deref<Target = U>,
    U: ?Sized,
{
}

// === Deref ===

// === DerefMut ===

wrapper!(
    #[wrapper_impl(DerefMut<[u8]>)]
    pub struct TestWrapperDerefMutStruct(Vec<u8>);
);

wrapper!(
    #[wrapper_impl(DerefMut<[u8]>)]
    pub struct TestComplexWrapperDerefMutStruct {
        inner_can_be_any_name: Vec<u8>,
        _a: ::core::marker::PhantomData<&'static ()>,
    }
);

#[test]
fn test_impl_DerefMut() {
    _assert_impl_deref_mut::<TestWrapperDerefMutStruct, [u8]>();
    _assert_impl_deref_mut::<TestComplexWrapperDerefMutStruct, [u8]>();
}

fn _assert_impl_deref_mut<T, U>()
where
    T: DerefMut + Deref<Target = U>,
    U: ?Sized,
{
}

// === DerefMut ===
