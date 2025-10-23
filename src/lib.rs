#![doc = include_str!("../README.md")]
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
///     // #[wrapper_impl(Deref)]
///     #[wrapper_impl(DerefMut)]
///     // #[wrapper_impl(From)]
///     #[wrapper_impl(Debug)]
///     #[derive(Clone, Copy, PartialEq, Eq)]
///     #[repr(transparent)]
///     pub struct ExampleWrapperComplex<'a, 'b, P> {
///         inner: P,
///         _a: ::core::marker::PhantomData<&'a ()>,
///         _b: ::core::marker::PhantomData<&'b ()>,
///     }
/// );
///
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(AsMut)]
///     #[wrapper_impl(AsRef)]
///     // #[wrapper_impl(Borrow)]
///     #[wrapper_impl(BorrowMut)]
///     // #[wrapper_impl(Deref)]
///     #[wrapper_impl(DerefMut)]
///     #[wrapper_impl(From)]
///     #[wrapper_impl(Debug)]
///     #[derive(Clone, Copy, PartialEq, Eq)]
///     #[repr(transparent)]
///     pub struct ExampleWrapperComplexWithDefault<'a, 'b, P> {
///         inner: P,
///         _a: ::core::marker::PhantomData<&'a ()> = ::core::marker::PhantomData,
///         _b: ::core::marker::PhantomData<&'b ()> = ::core::marker::PhantomData,
///     }
/// );
/// ```
///
/// There're some limitations:
///
/// - The inner field must be the first field declared in the struct.
/// - When there's no default value specified, we cannot implement the `From`
///   trait for the wrapper type.
/// - The macro does not know if other fields were zero-sized types (ZST), hence
///   we will not automatically apply `repr(transparent)` attribute.
///
/// ## Special usages
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
/// ### `ConstAsMut`
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
/// ### `AsRef<T>`, `AsMut<T>`, `Borrow<T>`, `BorrowMut<T>`, `Deref<T>`, `DerefMut<T>`
///
/// These attributes allow you to specify a target type `T` for the respective
/// traits (experimental). This is done by auto-dereferencing the inner type to
/// get a reference to `T`.
///
/// ```rust
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(AsRef<[u8]>)]
///     #[wrapper_impl(AsMut<[u8]>)]
///     // #[wrapper_impl(Borrow<[u8]>)]
///     #[wrapper_impl(BorrowMut<[u8]>)]
///     // #[wrapper_impl(Deref<[u8]>)]
///     #[wrapper_impl(DerefMut<[u8]>)]
///     pub struct ExampleWrapper(pub(crate) Vec<u8>);
/// );
/// ```
///
/// ### `repr(align(cache))`
///
/// You can use `#[repr(align(cache))]` to pad and align the wrapper type to the
/// cache line size. This is useful for performance optimization in certain
/// scenarios.
///
/// ```
/// wrapper_lite::wrapper!(
///     #[wrapper_impl(From)]
///     #[repr(align(cache))]
///     /// Example doc
///     pub struct ExampleWrapperCachePadded(u64);
/// );
/// #[cfg(target_arch = "x86_64")]
/// assert_eq!(core::mem::align_of::<ExampleWrapperCachePadded>(), 128);
/// ```
///
/// Credits: <https://docs.rs/crossbeam/latest/crossbeam/utils/struct.CachePadded.html>.
///
/// Notes that `repr(align(cache))` must be placed after other
/// `#[wrapper_impl(...)]` attributes and before any other attributes, including
/// docs.
///
/// ## Notes
///
/// - The `wrapper_impl` attribute must be on top of any other attributes.
/// - For `BorrowMut` and `DerefMut`, the macro will automatically implement the
///   corresponding `Borrow` and `Deref` traits, so the following two examples
///   will fail to compile:
///
///   ```rust,compile_fail
///   wrapper_lite::wrapper!(
///       #[wrapper_impl(Borrow)]
///       #[wrapper_impl(BorrowMut)]
///       pub struct ExampleWrapper<P>(pub(crate) P);
///   );
///   ```
///
///   ```rust,compile_fail
///   wrapper_lite::wrapper!(
///       #[wrapper_impl(Deref)]
///       #[wrapper_impl(DerefMut)]
///       pub struct ExampleWrapper<P>(pub(crate) P);
///   );
///   ```
macro_rules! wrapper {
    // To filter out the `wrapper_impl` attribute and extract the inner type.
    (
        @INTERNAL IMPL
        #[wrapper_impl(AsRef $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(AsMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(ConstAsMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(Borrow $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(BorrowMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(Deref $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL IMPL
            $($tt)*
        }
    };
    (
        @INTERNAL IMPL
        #[wrapper_impl(DerefMut $(<$target:ty>)? )]
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
        #[repr(align(cache))]
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        // Starting from Intel's Sandy Bridge, spatial prefetcher is now pulling pairs of 64-byte cache
        // lines at a time, so we have to align to 128 bytes rather than 64.
        //
        // Sources:
        // - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
        // - https://github.com/facebook/folly/blob/1b5288e6eea6df074758f877c849b6e73bbb9fbb/folly/lang/Align.h#L107
        //
        // aarch64/arm64ec's big.LITTLE architecture has asymmetric cores and "big" cores have 128-byte cache line size.
        //
        // Sources:
        // - https://www.mono-project.com/news/2016/09/12/arm64-icache/
        //
        // powerpc64 has 128-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_ppc64x.go#L9
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/powerpc/include/asm/cache.h#L26
        #[cfg_attr(
            any(
                target_arch = "x86_64",
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "powerpc64",
            ),
            repr(align(128))
        )]
        // arm, mips, mips64, sparc, and hexagon have 32-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_arm.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mipsle.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips64x.go#L9
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L17
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/hexagon/include/asm/cache.h#L12
        #[cfg_attr(
            any(
                target_arch = "arm",
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "sparc",
                target_arch = "hexagon",
            ),
            repr(align(32))
        )]
        // m68k has 16-byte cache line size.
        //
        // Sources:
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/m68k/include/asm/cache.h#L9
        #[cfg_attr(target_arch = "m68k", repr(align(16)))]
        // s390x has 256-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_s390x.go#L7
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/s390/include/asm/cache.h#L13
        #[cfg_attr(target_arch = "s390x", repr(align(256)))]
        // x86, wasm, riscv, and sparc64 have 64-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/dda2991c2ea0c5914714469c4defc2562a907230/src/internal/cpu/cpu_x86.go#L9
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_wasm.go#L7
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/riscv/include/asm/cache.h#L10
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L19
        //
        // All others are assumed to have 64-byte cache line size.
        #[cfg_attr(
            not(any(
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
            )),
            repr(align(64))
        )]
        $(#[$outer])*
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

    (
        @INTERNAL IMPL
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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

    // The actual implementation of the wrapper type: `pub struct Name<...> { ... }`
    // with field initial value provided, make `const_from` const.
    (
        @INTERNAL IMPL
        #[repr(align(cache))]
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty = $field_default: expr
            )*
            $(,)?
        }
    ) => {
        // Starting from Intel's Sandy Bridge, spatial prefetcher is now pulling pairs of 64-byte cache
        // lines at a time, so we have to align to 128 bytes rather than 64.
        //
        // Sources:
        // - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
        // - https://github.com/facebook/folly/blob/1b5288e6eea6df074758f877c849b6e73bbb9fbb/folly/lang/Align.h#L107
        //
        // aarch64/arm64ec's big.LITTLE architecture has asymmetric cores and "big" cores have 128-byte cache line size.
        //
        // Sources:
        // - https://www.mono-project.com/news/2016/09/12/arm64-icache/
        //
        // powerpc64 has 128-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_ppc64x.go#L9
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/powerpc/include/asm/cache.h#L26
        #[cfg_attr(
            any(
                target_arch = "x86_64",
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "powerpc64",
            ),
            repr(align(128))
        )]
        // arm, mips, mips64, sparc, and hexagon have 32-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_arm.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mipsle.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips64x.go#L9
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L17
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/hexagon/include/asm/cache.h#L12
        #[cfg_attr(
            any(
                target_arch = "arm",
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "sparc",
                target_arch = "hexagon",
            ),
            repr(align(32))
        )]
        // m68k has 16-byte cache line size.
        //
        // Sources:
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/m68k/include/asm/cache.h#L9
        #[cfg_attr(target_arch = "m68k", repr(align(16)))]
        // s390x has 256-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_s390x.go#L7
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/s390/include/asm/cache.h#L13
        #[cfg_attr(target_arch = "s390x", repr(align(256)))]
        // x86, wasm, riscv, and sparc64 have 64-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/dda2991c2ea0c5914714469c4defc2562a907230/src/internal/cpu/cpu_x86.go#L9
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_wasm.go#L7
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/riscv/include/asm/cache.h#L10
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L19
        //
        // All others are assumed to have 64-byte cache line size.
        #[cfg_attr(
            not(any(
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
            )),
            repr(align(64))
        )]
        $(#[$outer])*
        $vis struct $name$(<$($lt),+>)? {
            $(#[$field_inner_meta])*
            $inner_vis $inner: $inner_ty,
            $(
                $(#[$field_meta])*
                $field_vis $field: $field_ty
            ),*
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            #[doc = concat!("Creates a new instance of [`", stringify!($name), "`]")]
            $inner_vis const fn const_from($inner: $inner_ty) -> Self {
                Self {
                    $inner,
                    $(
                        $field: $field_default,
                    )*
                }
            }
        }
    };

    (
        @INTERNAL IMPL
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
            $inner_vis $inner: $inner_ty,
            $(
                $(#[$field_meta])*
                $field_vis $field: $field_ty
            ),*
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            #[doc = concat!("Creates a new instance of [`", stringify!($name), "`]")]
            $inner_vis const fn const_from($inner: $inner_ty) -> Self {
                Self {
                    $inner,
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
        #[repr(align(cache))]
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty
            )*
            $(,)?
        }
    ) => {
        // Starting from Intel's Sandy Bridge, spatial prefetcher is now pulling pairs of 64-byte cache
        // lines at a time, so we have to align to 128 bytes rather than 64.
        //
        // Sources:
        // - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
        // - https://github.com/facebook/folly/blob/1b5288e6eea6df074758f877c849b6e73bbb9fbb/folly/lang/Align.h#L107
        //
        // aarch64/arm64ec's big.LITTLE architecture has asymmetric cores and "big" cores have 128-byte cache line size.
        //
        // Sources:
        // - https://www.mono-project.com/news/2016/09/12/arm64-icache/
        //
        // powerpc64 has 128-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_ppc64x.go#L9
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/powerpc/include/asm/cache.h#L26
        #[cfg_attr(
            any(
                target_arch = "x86_64",
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "powerpc64",
            ),
            repr(align(128))
        )]
        // arm, mips, mips64, sparc, and hexagon have 32-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_arm.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mipsle.go#L7
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips64x.go#L9
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L17
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/hexagon/include/asm/cache.h#L12
        #[cfg_attr(
            any(
                target_arch = "arm",
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "sparc",
                target_arch = "hexagon",
            ),
            repr(align(32))
        )]
        // m68k has 16-byte cache line size.
        //
        // Sources:
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/m68k/include/asm/cache.h#L9
        #[cfg_attr(target_arch = "m68k", repr(align(16)))]
        // s390x has 256-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_s390x.go#L7
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/s390/include/asm/cache.h#L13
        #[cfg_attr(target_arch = "s390x", repr(align(256)))]
        // x86, wasm, riscv, and sparc64 have 64-byte cache line size.
        //
        // Sources:
        // - https://github.com/golang/go/blob/dda2991c2ea0c5914714469c4defc2562a907230/src/internal/cpu/cpu_x86.go#L9
        // - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_wasm.go#L7
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/riscv/include/asm/cache.h#L10
        // - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L19
        //
        // All others are assumed to have 64-byte cache line size.
        #[cfg_attr(
            not(any(
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
            )),
            repr(align(64))
        )]
        $(#[$outer])*
        $vis struct $name$(<$($lt),+>)? {
            $(#[$field_inner_meta])*
            $inner_vis $inner: $inner_ty
            $(
                ,
                $(#[$field_meta])*
                $field_vis $field: $field_ty
            )*
        }
    };

    (
        @INTERNAL IMPL
        $(#[$outer:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
            $inner_vis $inner: $inner_ty
            $(
                ,
                $(#[$field_meta])*
                $field_vis $field: $field_ty
            )*
        }
    };

    // === Process all `wrapper_impl` attributes, and generate impls. ===

    // Extract wrapper impl for `AsRef` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(AsRef $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_AS_REF $(<$target>)?
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
        #[wrapper_impl(AsMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_AS_MUT $(<$target>)?
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL
            $($tt)*
        }
    };

    // Extract wrapper impl for `AsMut` trait, const version.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(ConstAsMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_CONST_AS_MUT $(<$target>)?
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
        #[wrapper_impl(Borrow $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_BORROW $(<$target>)?
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
        #[wrapper_impl(BorrowMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_BORROW $(<$target>)?
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_BORROW_MUT $(<$target>)?
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

    // Extract wrapper impl for `Deref` trait.
    (
        @INTERNAL WRAPPER_IMPL
        #[wrapper_impl(Deref $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEREF $(<$target>)?
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
        #[wrapper_impl(DerefMut $(<$target:ty>)? )]
        $($tt:tt)*
    ) => {
        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEREF $(<$target>)?
            $($tt)*
        }

        $crate::wrapper! {
            @INTERNAL WRAPPER_IMPL_DEREF_MUT $(<$target>)?
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
        @INTERNAL WRAPPER_IMPL_AS_REF <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsRef<$target> for $name$(<$($lt),+>)? {
            fn as_ref(&self) -> &$target {
                &self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_AS_REF <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsRef<$target> for $name$(<$($lt),+>)? {
            fn as_ref(&self) -> &$target {
                &self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_AS_REF
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                &self.$inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Returns a reference to the inner value.
            #[inline(always)]
            pub const fn as_inner(&self) -> &$inner_ty {
                &self.$inner
            }
        }
    };
    // ================ Impl `AsRef` trait for the wrapper type. ================


    // ================ Impl `AsMut` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_AS_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$target> for $name$(<$($lt),+>)? {
            fn as_mut(&mut self) -> &mut $target {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_AS_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$target> for $name$(<$($lt),+>)? {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $target {
                &mut self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                &mut self.$inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            /// Returns a mutable reference to the inner value.
            pub fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_CONST_AS_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$target> for $name$(<$($lt),+>)? {
            fn as_mut(&mut self) -> &mut $target {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_CONST_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::AsMut<$target> for $name$(<$($lt),+>)? {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut $target {
                &mut self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_CONST_AS_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                &mut self.$inner
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            #[inline(always)]
            /// Returns a mutable reference to the inner value.
            pub const fn as_inner_mut(&mut self) -> &mut $inner_ty {
                &mut self.$inner
            }
        }
    };
    // ================ Impl `AsMut` trait for the wrapper type. ================

    // ================ Impl `Borrow` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_BORROW <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::Borrow<$target> for $name$(<$($lt),+>)? {
            fn borrow(&self) -> &$target {
                &self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_BORROW <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::Borrow<$target> for $name$(<$($lt),+>)? {
            fn borrow(&self) -> &$target {
                &self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_BORROW
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                &self.$inner
            }
        }
    };
    // ================ Impl `Borrow` trait for the wrapper type. ================

    // ================ Impl `BorrowMut` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_BORROW_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::BorrowMut<$target> for $name$(<$($lt),+>)? {
            fn borrow_mut(&mut self) -> &mut $target {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_BORROW_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::borrow::BorrowMut<$target> for $name$(<$($lt),+>)? {
            fn borrow_mut(&mut self) -> &mut $target {
                &mut self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_BORROW_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                &mut self.$inner
            }
        }
    };
    // ================ Impl `Borrow` trait for the wrapper type. ================

    // ================ Impl `Debug` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEBUG
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                self.$inner.fmt(f)
            }
        }
    };
    // ================ Impl `Debug` trait for the wrapper type. ================

    // ================ Impl `DebugName` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEBUG_NAME
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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

    // ================ Impl `Deref` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEREF <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEREF <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEREF
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
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
                &self.$inner
            }
        }
    };
    // ================ Impl `Deref` trait for the wrapper type. ================

    // ================ Impl `DerefMut` traits for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_DEREF_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::DerefMut for $name$(<$($lt),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEREF_MUT <$target:ty>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::DerefMut for $name$(<$($lt),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$inner
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_DEREF_MUT
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
    ) => {
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty$( = $field_default: expr)?
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::ops::DerefMut for $name$(<$($lt),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$inner
            }
        }
    };
    // ================ Impl `DerefMut` traits for the wrapper type. ================

    // ================ Impl `From` trait for the wrapper type. ================
    (
        @INTERNAL WRAPPER_IMPL_FROM
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner_ty:ty);
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
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty = $field_default:expr
            )*
            $(,)?
        }
    ) => {
        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? ::core::convert::From<$inner_ty> for $name$(<$($lt),+>)? {
            fn from($inner: $inner_ty) -> Self {
                Self::const_from($inner)
            }
        }

        impl$(<$($lt$(:$clt$(+$dlt)*)?),+>)? $name$(<$($lt),+>)? {
            /// Creates a new instance of the wrapper type from the inner value.
            #[allow(unreachable_pub)]
            #[inline(always)]
            pub const fn from($inner: $inner_ty) -> Self {
                Self::const_from($inner)
            }
        }
    };
    (
        @INTERNAL WRAPPER_IMPL_FROM
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
            $(#[$field_inner_meta:meta])*
            $inner_vis:vis $inner:ident: $inner_ty:ty
            $(
                ,
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty
            )*
            $(,)?
        }
    ) => {
        compile_error!(
            "Invalid usage of `wrapper!` macro, cannot implement \
            `From` trait for wrapper types with multiple fields\
            but no default values given."
        );
    };
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
