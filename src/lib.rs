//! Helper macro for creating a wrapper over any type (new-type idiom).

#![no_std]

#[macro_export]
/// Helper macro for creating a wrapper over any type (new-type idiom).
///
/// This is a shortcut for using the [`wrapper!`] macro with the most common
/// impls (`AsRef`, `Borrow`, `From`).
///
/// ```rust
/// wrapper_lite::general_wrapper!(
///     #[derive(Debug, Clone, Copy)]
///     pub struct ExampleWrapper<'a, P>(pub(crate) &'a P);
/// );
/// ```
///
/// This is equivalent to using the `wrapper!` macro with the following
/// attributes:
///
/// ```rust
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(AsRef)]
///     #[wrapper_impl(Borrow)]
///     #[wrapper_impl(From)]
///     #[derive(Debug, Clone, Copy)]
///     pub struct ExampleWrapper<'a, P>(pub(crate) &'a P);
/// );
/// ```
///
/// You can certainly add attributes like `#[wrapper_impl(Deref)]` other than
/// the preset ones. See [`wrapper!`] for more details.
///
/// ```rust
/// wrapper_lite::general_wrapper!(
///     #[wrapper_impl(Deref)]
///     #[derive(Debug, Clone, Copy)]
///     pub struct ExampleWrapper<'a, P>(pub(crate) &'a P);
/// );
/// ```
macro_rules! general_wrapper {
    ($($tt:tt)+) => {
        $crate::wrapper! {
            #[wrapper_impl(AsRef)]
            #[wrapper_impl(Borrow)]
            #[wrapper_impl(From)]
            $($tt)+
        }
    };
}

#[macro_export]
/// Helper macro for creating a wrapper over any type (new-type idiom).
///
/// # Usage
///
/// It's worth noting that `wrapper! { ... }` is almost equivalent to
/// `wrapper!( ... );` but lacking cargo-fmt support. We recommend using the
/// latter.
///
/// ## Usage: basic
///
/// ```rust
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(AsRef)]
///     #[wrapper_impl(AsMut)]
///     // #[wrapper_impl(Borrow)]
///     #[wrapper_impl(BorrowMut)]
///     // #[wrapper_impl(Deref)]
///     #[wrapper_impl(DerefMut)]
///     #[wrapper_impl(From)]
///     #[derive(Debug, Clone, Copy)]
///     pub struct ExampleWrapper<'a, P>(pub(crate) &'a P);
/// );
/// ```
///
/// Generates const accessor methods for wrapper types implementing `AsRef` and
/// `AsMut` traits.
///
/// For types implementing `AsRef`, this creates a const method `as_inner` that
/// returns a reference to the wrapped value. For types implementing `AsMut`,
/// this creates a const method `as_inner_mut` that returns a mutable reference
/// to the wrapped value.
///
/// Additionally generates a const constructor method `const_from` for the
/// wrapper type, using the same visibility as the inner field. When the `From`
/// trait is implemented, also generates a public const method `from`.
///
/// ## Usage: advanced
///
/// You can also create a wrapper type with a struct with multiple fields,
/// especially when some lifetime markers or generics markers are too complex,
/// or you need some custom fields.
///
/// Here's an complex example:
///
/// ```rust
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(AsMut)]
///     #[wrapper_impl(AsRef)]
///     // #[wrapper_impl(Borrow)]
///     #[wrapper_impl(BorrowMut)]
///     //     #[wrapper_impl(Deref)]
///     #[wrapper_impl(DerefMut)]
///     #[wrapper_impl(From)]
///     #[wrapper_impl(Debug)]
///     #[derive(Clone, Copy, PartialEq, Eq)]
///     #[repr(transparent)]
///     pub struct ExampleWrapperComplex<'a, 'b, P> {
///         inner: P,
///         _a: ::core::marker::PhantomData<&'a ()>,
///         _b: ::core::marker::PhantomData<&'b ()>,
///     }
/// );
/// ```
///
/// There're some limitations:
///
/// - The inner field must be named as `inner`.
/// - When no default value is specified, the wrapper type will not implement
///   the `From` trait. Will also not generate the `const_from` method.
/// - Does not automatically apply `repr(transparent)` attribute, since the
///   macro doesn't know if other fields were zero-sized types (ZST).
///
/// ## Special attributes
///
/// ### `Debug` and `DebugName`
///
/// We offer `Debug` and `DebugName` attributes to control how the wrapper type
/// is printed when using the `Debug` trait, instead of `#[derive(Debug)]`.
///
/// - `#[wrapper_impl(Debug)]`: transparently implements the `Debug` trait if
///   the inner type implements it. The debug output is the same as the inner
///   one.
/// - `#[wrapper_impl(DebugName)]`: implements the `Debug` trait, but only
///   prints the name of the wrapper type.
///
/// ```rust
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(Debug)]
///     #[derive(Clone, Copy)]
///     pub struct ExampleWrapperDebug<'a, P>(&'a P);
/// );
///
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(DebugName)]
///     #[derive(Clone, Copy)]
///     pub struct ExampleWrapperDebugName<'a, P>(&'a P);
/// );
///
/// let data = "Hello".to_string();
///
/// // Here we transparently print the debug output of the inner type.
/// assert_eq!(
///     format!("{:?}", ExampleWrapperDebug { inner: &data }),
///     "\"Hello\""
/// );
/// // Here we only print the name of the wrapper type.
/// assert_eq!(
///     format!("{:?}", ExampleWrapperDebugName { inner: &data }),
///     "ExampleWrapperDebugName"
/// );
/// ```
///
/// ## `ConstAsMut`
///
/// Like `AsMut`, but instead generates a const version of `as_inner_mut` method
/// (stable since Rust 1.83.0+).
///
/// ```rust,no_run
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(ConstAsMut)]
///     #[derive(Debug, Clone, Copy)]
///     pub struct ExampleWrapper<P>(pub(crate) P);
/// );
///
/// const fn const_fn_example<P>(w: &mut ExampleWrapper<P>) -> &mut P {
///     w.as_inner_mut()
/// }
/// ```
///
/// ## Notice
///
/// - The `wrapper_impl` attribute must be on top of any other attributes.
/// - For `BorrowMut` and `DerefMut`, the macro will automatically implement the
///   corresponding `Borrow` and `Deref` traits so you don't need to add them
///   manually.
macro_rules! wrapper {
    // To filter out the `wrapper_impl` attribute and extract the inner type.
    (
        @INTERNAL IMPL
        #[wrapper_impl(AsRef)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(AsMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(ConstAsMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(Borrow)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(BorrowMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(Deref)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(DerefMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(From)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(Debug)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(DebugName)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };

    // The actual implementation of the wrapper type: `pub Name<...>(...)`
    (
        @INTERNAL IMPL
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty) $(;)?
    ) => {
        $(#[$outer])*
        #[repr(transparent)]
        $vis struct $name$(<$($lt),+>)? {
            /// Inner value
            $inner_vis inner: $inner_ty,
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            #[doc = concat!("Creates a new instance of [`", stringify!($name), "`]")]
            $inner_vis const fn const_from(inner: $inner_ty) -> Self {
                Self {
                    inner,
                }
            }
        }
    };

    // The actual implementation of the wrapper type: `pub struct Name<...> { ... }`, with field initial value provided, make `const_from` const.
    (
        @INTERNAL IMPL
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty = $field_default: expr
            )*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis struct $name$(<$($lt),+>)? {
            $(#[$field_inner_meta])*
            $inner_vis inner: $inner_ty,
            $(
                $(#[$field_meta])*
                $field_vis $field: $field_ty
            ),*
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            #[doc = concat!("Creates a new instance of [`", stringify!($name), "`]")]
            $inner_vis const fn const_from(inner: $inner_ty) -> Self {
                Self {
                    inner,
                    $(
                        $field: $field_default,
                    )*
                }
            }
        }
    };

    // The actual implementation of the wrapper type with fields: `pub struct Name<...> { ... }`
    (
        @INTERNAL IMPL
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty
            )*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis struct $name$(<$($lt),+>)? {
            $(#[$field_inner_meta])*
            $inner_vis inner: $inner_ty
            $(
                ,
                $(#[$field_meta])*
                $field_vis $field: $field_ty
            )*
        }
    };

    // Extract wrapper impl for `AsRef` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(AsRef)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_AS_REF
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `AsMut` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(AsMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_AS_MUT
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(ConstAsMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_CONST_AS_MUT
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Borrow` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(Borrow)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_BORROW
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `BorrowMut` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(BorrowMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_BORROW
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_BORROW_MUT
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Debug` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(Debug)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEBUG
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Debug` trait  printing its name only.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(DebugName)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEBUG_NAME
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `DerefMut` trait (and `Deref`).
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(DerefMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEREF_MUT
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Deref` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(Deref)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEREF
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `From` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(From)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_FROM
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // ================ Impl `AsRef` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_AS_REF
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsRef<$inner_ty> for $name$(<$($lt),+>)? {
            fn as_ref(&self) -> &$inner_ty {
                &self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Returns a reference to the inner value.
            #[inline(always)]
            pub const fn as_inner(&self) -> &$inner_ty {
                &self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_AS_REF
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsRef<$inner_ty> for $name$(<$($lt),+>)? {
            fn as_ref(&self) -> &$inner_ty {
                &self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Returns a reference to the inner value.
            #[inline(always)]
            pub const fn as_inner(&self) -> &$inner_ty {
                &self.inner
            }
        }
    };
    // ================ Impl `AsRef` trait for the wrapper type. ================


    // ================ Impl `AsMut` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$inner_ty> for $name$(<$($lt),+>)? {
            fn as_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            /// Returns a mutable reference to the inner value.
            pub fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$inner_ty> for $name$(<$($lt),+>)? {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            /// Returns a mutable reference to the inner value.
            pub fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };

    // Const version
    (
        @INTERNAL WRAPPER_IMPL_CONST_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$inner_ty> for $name$(<$($lt),+>)? {
            fn as_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            /// Returns a mutable reference to the inner value.
            pub const fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_CONST_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$inner_ty> for $name$(<$($lt),+>)? {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            /// Returns a mutable reference to the inner value.
            pub const fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    // ================ Impl `AsMut` trait for the wrapper type. ================

    // ================ Impl `Borrow` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_BORROW
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::Borrow<$inner_ty> for $name$(<$($lt),+>)? {
            fn borrow(&self) -> &$inner_ty {
                &self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_BORROW
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::Borrow<$inner_ty> for $name$(<$($lt),+>)? {
            fn borrow(&self) -> &$inner_ty {
                &self.inner
            }
        }
    };
    // ================ Impl `Borrow` trait for the wrapper type. ================

    // ================ Impl `BorrowMut` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_BORROW_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::BorrowMut<$inner_ty> for $name$(<$($lt),+>)? {
            fn borrow_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_BORROW_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::BorrowMut<$inner_ty> for $name$(<$($lt),+>)? {
            fn borrow_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    // ================ Impl `Borrow` trait for the wrapper type. ================

    // ================ Impl `Debug` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEBUG
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::fmt::Debug for $name$(<$($lt),+>)?
        where
            $inner_ty: ::core::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.inner.fmt(f)
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEBUG
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::fmt::Debug for $name$(<$($lt),+>)?
        where
            $inner_ty: ::core::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.inner.fmt(f)
            }
        }
    };
    // ================ Impl `Debug` trait for the wrapper type. ================

    // ================ Impl `DebugName` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEBUG_NAME
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::fmt::Debug for $name$(<$($lt),+>)? {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEBUG_NAME
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::fmt::Debug for $name$(<$($lt),+>)? {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
    // ================ Impl `DebugName` trait for the wrapper type. ================

    // ================ Impl `DerefMut` traits for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEREF_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $inner_ty;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::DerefMut for $name$(<$($lt),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEREF_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $inner_ty;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::DerefMut for $name$(<$($lt),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
    // ================ Impl `DerefMut` traits for the wrapper type. ================

    // ================ Impl `Deref` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEREF
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $inner_ty;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEREF
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $inner_ty;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
    // ================ Impl `Deref` trait for the wrapper type. ================

    // ================ Impl `From` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_FROM
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? $(;)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::From<$inner_ty> for $name$(<$($lt),+>)? {
            fn from(inner: $inner_ty) -> Self {
                Self::const_from(inner)
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Creates a new instance of the wrapper type from the inner value.
            #[allow(unreachable_pub)]
            #[inline(always)]
            pub const fn from(inner: $inner_ty) -> Self {
                Self::const_from(inner)
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_FROM
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis inner: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty = $field_default:expr
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::From<$inner_ty> for $name$(<$($lt),+>)? {
            fn from(inner: $inner_ty) -> Self {
                Self::const_from(inner)
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Creates a new instance of the wrapper type from the inner value.
            #[allow(unreachable_pub)]
            #[inline(always)]
            pub const fn from(inner: $inner_ty) -> Self {
                Self::const_from(inner)
            }
        }
    };
    (@INTERNAL WRAPPER_IMPL_FROM $($tt:tt)*) => {};
    // ================ Impl `From` trait for the wrapper type. ================

    // No other wrapper_impl meta
    (@INTERNAL WRAPPER_IMPL $($tt:tt)*) => {};

    // Catch-all for invalid usage of the macro.
    (@INTERNAL $($tt:tt)*) => {
        compile_error!(
            "Invalid usage of `wrapper!` macro. @INTERNAL \
            Please refer to the documentation for the correct syntax."
        );
    };

    // Core macro for the wrapper type.
    ($($tt:tt)*) => {
        $crate::wrapper!(@INTERNAL IMPL $($tt)*);
        $crate::wrapper!(@INTERNAL WRAPPER_IMPL $($tt)*);
    };
}
