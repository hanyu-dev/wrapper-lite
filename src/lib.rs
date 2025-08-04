//! Helper macro for creating a wrapper over any type (new-type idiom).

#![no_std]

// For test only
extern crate alloc;

#[macro_export]
/// Helper macro for creating a wrapper over any type (new-type idiom).
///
/// This is a shortcut for using the `wrapper!` macro with the most common impls
/// (`AsRef`, `Borrow`, `From`).
///
/// ```rust
/// # use wrapper_lite::*;
/// general_wrapper! {
///     #[derive(Debug, Clone, Copy)]
///     pub ExampleWrapper<'a, P>(pub(crate) &'a P)
/// }
/// # fn assert_impls_ExampleWrapper<P: core::fmt::Debug>() {
/// #    _assert_impl_debug::<ExampleWrapper<'_, P>>();
/// #    _assert_impl_as_ref::<ExampleWrapper<'_, P>, _>();
/// #    // _assert_impl_as_mut::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_borrow::<ExampleWrapper<'_, P>, &P>();
/// #    // _assert_impl_deref::<ExampleWrapper<'_, P>, _>();
/// #    // _assert_impl_deref_mut::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_from::<ExampleWrapper<'_, P>, &P>();
/// # }
/// ```
///
/// This is equivalent to using the `wrapper!` macro with the following
/// attributes:
///
/// ```rust
/// # use wrapper_lite::*;
/// wrapper! {
///     #[wrapper_impl(AsRef)]
///     #[wrapper_impl(Borrow)]
///     #[wrapper_impl(From)]
///     #[derive(Debug, Clone, Copy)]
///     pub ExampleWrapper<'a, P>(pub(crate) &'a P)
/// }
/// ```
///
/// You can certainly add attributes like `#[wrapper_impl(Deref)]` to implement
/// other traits based on the preset ones.
///
/// ```rust
/// # use wrapper_lite::*;
/// general_wrapper! {
///     #[wrapper_impl(Deref)]
///     #[derive(Debug, Clone, Copy)]
///     pub ExampleWrapper<'a, P>(pub(crate) &'a P)
/// }
/// # fn assert_impls_ExampleWrapper<P: core::fmt::Debug>() {
/// #    _assert_impl_debug::<ExampleWrapper<'_, P>>();
/// #    _assert_impl_as_ref::<ExampleWrapper<'_, P>, _>();
/// #    // _assert_impl_as_mut::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_borrow::<ExampleWrapper<'_, P>, &P>();
/// #    _assert_impl_deref::<ExampleWrapper<'_, P>, _>();
/// #    // _assert_impl_deref_mut::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_from::<ExampleWrapper<'_, P>, &P>();
/// # }
/// ```
///
/// See [`wrapper!`] for more details.
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
/// Helper macro for creating a wrapper over any type (new-type idom).
///
/// # Usage: basic
///
/// ```rust
/// # use wrapper_lite::*;
/// wrapper! {
///     #[wrapper_impl(AsRef)]
///     #[wrapper_impl(AsMut)]
///     #[wrapper_impl(Borrow)]
///     // #[wrapper_impl(Deref)]
///     #[wrapper_impl(DerefMut)]
///     #[wrapper_impl(From)]
///     #[derive(Debug, Clone, Copy)]
///     pub ExampleWrapper<'a, P>(pub(crate) &'a P)
/// }
/// # fn assert_impls_ExampleWrapper<P: core::fmt::Debug>() {
/// #    _assert_impl_debug::<ExampleWrapper<'_, P>>();
/// #    _assert_impl_as_ref::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_as_mut::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_borrow::<ExampleWrapper<'_, P>, &P>();
/// #    _assert_impl_deref::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_deref_mut::<ExampleWrapper<'_, P>, _>();
/// #    _assert_impl_from::<ExampleWrapper<'_, P>, &P>();
/// # }
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
/// especially when some lifetime markers or generics markers are needed.
///
/// ```rust
/// # use wrapper_lite::*;
/// wrapper! {
///     #[wrapper_impl(AsMut)]
///     #[wrapper_impl(AsRef)]
///     #[wrapper_impl(Borrow)]
///     #[wrapper_impl(DerefMut)]
///     #[wrapper_impl(From)]
///     #[derive(Debug)]
///     #[repr(transparent)]
///     pub struct ExampleWrapperComplex<'a, 'b, P> {
///         inner: P,
///         _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
///         _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData
///     }
/// }
/// ```
///
/// There're some limitations:
///
/// - The inner field must be named as `inner` (e.g. `inner: P`).
/// - When no default value is specified, the wrapper type will not implement
///   the `From` trait.
/// - Does **NOT** automatically apply `repr(transparent)` attribute, since the
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
/// # use wrapper_lite::*;
/// #
/// wrapper! {
///     #[wrapper_impl(Debug)]
///     #[derive(Clone, Copy)]
///     pub ExampleWrapperDebug<'a, P>(&'a P)
/// }
///
/// wrapper! {
///     #[wrapper_impl(DebugName)]
///     #[derive(Clone, Copy)]
///     pub ExampleWrapperDebugName<'a, P>(&'a P)
/// }
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
/// ## Notice
///
/// - The `wrapper_impl` attribute must be on top of any other attributes.
/// - Should **NOT** implement `Deref` and `DerefMut` at the same time (when
///   `DerefMut` is implemented, `Deref` would be implemented, too).
macro_rules! wrapper {
    // To filter out the `wrapper_impl` attribute and extract the inner type.
    (
        @INTERNEL IMPL
        #[wrapper_impl(AsRef)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(AsMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(Borrow)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(Deref)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(DerefMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(From)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(Debug)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };
    (
        @INTERNEL IMPL
        #[wrapper_impl(DebugName)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL IMPL
            $($tt)*
        }
    };

    // The actual implementation of the wrapper type: `pub Name<...>(...)`
    (
        @INTERNEL IMPL
        $(#[$outer:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty) $(;)?
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
        @INTERNEL IMPL
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
        @INTERNEL IMPL
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
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(AsRef)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_AS_REF
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `AsMut` trait.
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(AsMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_AS_MUT
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Borrow` trait.
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(Borrow)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_BORROW
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Debug` trait.
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(Debug)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_DEBUG
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Debug` trait  printing its name only.
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(DebugName)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_DEBUG_NAME
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `DerefMut` trait (and `Deref`).
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(DerefMut)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_DEREF_MUT
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `Deref` trait.
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(Deref)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_DEREF
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `From` trait.
    (
        @INTERNEL WRAPPER_IMPL
        #[wrapper_impl(From)]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL_FROM
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNEL WRAPPER_IMPL
            $($tt)*
        }
    };

    // ================ Impl `AsRef` trait for the wrapper type. ================
    (
        @INTERNEL WRAPPER_IMPL_AS_REF
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_AS_REF
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
        @INTERNEL WRAPPER_IMPL_AS_MUT
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
        ($inner_vis:vis $inner_ty:ty)
        $($tt:tt)*
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$inner_ty> for $name$(<$($lt),+>)? {
            fn as_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Returns a reference to the inner value.
            #[inline(always)]
            pub const fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    (
        @INTERNEL WRAPPER_IMPL_AS_MUT
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
            fn as_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Returns a reference to the inner value.
            #[cfg_attr(feature = "const-mut-method", rustversion::attr(since(1.83), const))]
            #[inline(always)]
            pub fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.inner
            }
        }
    };
    // ================ Impl `AsMut` trait for the wrapper type. ================

    // ================ Impl `Borrow` trait for the wrapper type. ================
    (
        @INTERNEL WRAPPER_IMPL_BORROW
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_BORROW
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

    // ================ Impl `Debug` trait for the wrapper type. ================
    (
        @INTERNEL WRAPPER_IMPL_DEBUG
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_DEBUG
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
        @INTERNEL WRAPPER_IMPL_DEBUG_NAME
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_DEBUG_NAME
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
        @INTERNEL WRAPPER_IMPL_DEREF_MUT
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_DEREF_MUT
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
        @INTERNEL WRAPPER_IMPL_DEREF
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_DEREF
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
        @INTERNEL WRAPPER_IMPL_FROM
        $(#[$meta:meta])*
        $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)?
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
        @INTERNEL WRAPPER_IMPL_FROM
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
    (@INTERNEL WRAPPER_IMPL_FROM $($tt:tt)*) => {};
    // ================ Impl `From` trait for the wrapper type. ================

    // No other wrapper_impl meta
    (@INTERNEL WRAPPER_IMPL $($tt:tt)*) => {};

    // Catch-all for invalid usage of the macro.
    (@INTERNEL $($tt:tt)*) => {
        compile_error!(
            "Invalid usage of `wrapper!` macro. @INTERNEL \
            Please refer to the documentation for the correct syntax."
        );
    };

    // Core macro for the wrapper type.
    ($($tt:tt)*) => {
        $crate::wrapper!(@INTERNEL IMPL $($tt)*);
        $crate::wrapper!(@INTERNEL WRAPPER_IMPL $($tt)*);
    };
}

#[doc(hidden)]
pub fn _assert_impl_debug<T>()
where
    T: ::core::fmt::Debug,
{
}

#[doc(hidden)]
pub fn _assert_impl_as_ref<T, U>()
where
    T: ::core::convert::AsRef<U>,
{
}

#[doc(hidden)]
pub fn _assert_impl_as_mut<T, U>()
where
    T: ::core::convert::AsMut<U>,
{
}

#[doc(hidden)]
pub fn _assert_impl_borrow<T, U>()
where
    T: ::core::borrow::Borrow<U>,
{
}

#[doc(hidden)]
pub fn _assert_impl_deref<T, U>()
where
    T: ::core::ops::Deref<Target = U>,
{
}

#[doc(hidden)]
pub fn _assert_impl_deref_mut<T, U>()
where
    T: ::core::ops::DerefMut<Target = U>,
{
}

#[doc(hidden)]
pub fn _assert_impl_from<T, U>()
where
    T: ::core::convert::From<U>,
{
}

// #[cfg(any(test, doctest))]
mod simple {
    #![allow(unused)]
    #![allow(unreachable_pub)]
    #![allow(dead_code)]
    #![allow(non_snake_case)]

    use ::alloc::string::String;

    use super::*;

    #[rustfmt::skip]
    wrapper! {
        #[wrapper_impl(AsMut)]
        #[wrapper_impl(AsRef)]
        #[wrapper_impl(Borrow)]
        #[wrapper_impl(Debug)]
        #[wrapper_impl(DerefMut)]
        #[wrapper_impl(From)] // Multiple wrapper_impl attributes
        #[derive(Clone, Default)]
        #[derive(PartialEq)] // Multiple derive attributes
        /// Test Docs
        pub TestWrapperAllMixed(String); // trailing semicolon
    }

    wrapper! {
        #[wrapper_impl(AsRef)]
        pub TestWrapperImplAsRef(String)
    }

    fn assert_impls_TestWrapperImplAsRef() {
        _assert_impl_as_ref::<TestWrapperImplAsRef, _>();
    }

    #[test]
    fn assert_size_eq() {
        use core::mem::size_of;

        assert_eq!(size_of::<TestWrapperImplAsRef>(), size_of::<String>());
    }

    wrapper! {
        #[wrapper_impl(AsRef)]
        #[derive(Debug)]
        pub TestWrapperImplAsRefMixed(String)
    }

    fn assert_impls_TestWrapperImplAsRefMixed() {
        _assert_impl_debug::<TestWrapperImplAsRefMixed>();
        _assert_impl_as_ref::<TestWrapperImplAsRefMixed, _>();
    }

    wrapper! {
        #[wrapper_impl(Borrow)]
        pub TestWrapperImplBorrow(String)
    }

    fn assert_impls_TestWrapperImplBorrow() {
        _assert_impl_borrow::<TestWrapperImplBorrow, String>();
    }

    wrapper! {
        #[wrapper_impl(Borrow)]
        #[derive(Debug)]
        pub TestWrapperImplBorrowMixed(String)
    }

    fn assert_impls_TestWrapperImplBorrowMixed() {
        _assert_impl_debug::<TestWrapperImplBorrowMixed>();
        _assert_impl_borrow::<TestWrapperImplBorrowMixed, String>();
    }

    wrapper! {
        #[wrapper_impl(Debug)]
        pub TestWrapperImplDebug(String)
    }

    wrapper! {
        #[wrapper_impl(DebugName)]
        pub TestWrapperImplDebugName(String)
    }

    fn assert_impls_TestWrapperImplDebug() {
        _assert_impl_debug::<TestWrapperImplDebug>();
        _assert_impl_debug::<TestWrapperImplDebugName>();
    }

    wrapper! {
        #[wrapper_impl(Debug)]
        #[derive(Default)]
        pub TestWrapperImplDebugMixed(String)
    }

    wrapper! {
        #[wrapper_impl(DebugName)]
        #[derive(Default)]
        pub TestWrapperImplDebugNameMixed(String)
    }

    fn assert_impls_TestWrapperImplDebugNameMixed() {
        _assert_impl_debug::<TestWrapperImplDebugMixed>();
        _assert_impl_debug::<TestWrapperImplDebugNameMixed>();
    }

    wrapper! {
        #[wrapper_impl(Deref)]
        pub TestWrapperImplDeref(String)
    }

    fn assert_impls_TestWrapperImplDeref() {
        _assert_impl_deref::<TestWrapperImplDeref, _>();
    }

    wrapper! {
        #[wrapper_impl(Deref)]
        #[derive(Debug)]
        pub TestWrapperImplDerefMixed(String)
    }

    fn assert_impls_TestWrapperImplDerefMixed() {
        _assert_impl_debug::<TestWrapperImplDerefMixed>();
        _assert_impl_deref::<TestWrapperImplDerefMixed, _>();
    }

    wrapper! {
        #[wrapper_impl(DerefMut)]
        pub TestWrapperImplDerefMut(String)
    }

    fn assert_impls_TestWrapperImplDerefMut() {
        _assert_impl_deref_mut::<TestWrapperImplDerefMut, _>();
    }

    wrapper! {
        #[wrapper_impl(DerefMut)]
        #[derive(Debug)]
        pub TestWrapperImplDerefMutMixed(String)
    }

    fn assert_impls_TestWrapperImplDerefMutMixed() {
        _assert_impl_debug::<TestWrapperImplDerefMutMixed>();
        _assert_impl_deref_mut::<TestWrapperImplDerefMutMixed, _>();
    }

    wrapper! {
        #[wrapper_impl(From)]
        pub TestWrapperImplFrom(String)
    }

    fn assert_impls_TestWrapperImplFrom() {
        _assert_impl_from::<TestWrapperImplFrom, String>();
    }

    wrapper! {
        #[wrapper_impl(From)]
        #[derive(Debug)]
        pub TestWrapperImplFromMixed(String)
    }

    fn assert_impls_TestWrapperImplFromMixed() {
        _assert_impl_debug::<TestWrapperImplFromMixed>();
        _assert_impl_from::<TestWrapperImplFromMixed, String>();
    }

    // Test multiple
    wrapper! {
        #[wrapper_impl(Deref)]
        #[wrapper_impl(From)]
        #[derive(Debug)]
        pub TestWrapperImplFromDeref(String)
    }

    fn assert_impls_TestWrapperImplFromDeref() {
        _assert_impl_debug::<TestWrapperImplFromDeref>();
        _assert_impl_deref::<TestWrapperImplFromDeref, _>();
        _assert_impl_from::<TestWrapperImplFromDeref, String>();
    }

    // Test multiple with lifetimes
    wrapper! {
        #[wrapper_impl(Deref)]
        #[wrapper_impl(From)]
        #[derive(Debug)]
        pub TestWrapperImplFromDerefMixed<'a, P>(pub(crate) &'a P)
    }

    fn assert_impls_TestWrapperImplFromDerefMixed<P: core::fmt::Debug>() {
        _assert_impl_debug::<TestWrapperImplFromDerefMixed<'_, P>>();
        _assert_impl_deref::<TestWrapperImplFromDerefMixed<'_, P>, _>();
        _assert_impl_from::<TestWrapperImplFromDerefMixed<'_, P>, &P>();
    }
}

// #[cfg(any(test, doctest))]
mod complex {
    #![allow(unused)]
    #![allow(unreachable_pub)]
    #![allow(dead_code)]
    #![allow(non_snake_case)]

    use alloc::string::String;

    use super::*;

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
}

mod test_expand {
    #![allow(unused)]
    #![allow(unreachable_pub)]
    #![allow(dead_code)]
    #![allow(non_snake_case)]

    use alloc::string::String;
    use core::fmt::Debug;

    use super::*;

    wrapper! {
        #[wrapper_impl(AsMut)]
        #[wrapper_impl(AsRef)]
        #[wrapper_impl(Borrow)]
        #[wrapper_impl(Debug)]
        #[wrapper_impl(DerefMut)]
        #[wrapper_impl(From)]
        #[repr(transparent)]
        #[derive(Clone)]
        pub struct TestWrapperComplex<'a, 'b: 'a, P: Debug + Clone> {
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
}
