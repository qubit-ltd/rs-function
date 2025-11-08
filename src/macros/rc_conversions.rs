/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Rc Conversions Macro
//!
//! Generates common into_xxx() conversion methods for all Rc-based function
//! wrappers.
//!
//! This macro generates the standard conversion methods (`into_box`, `into_rc`,
//! `into_fn`, `into_once`) for all Rc-based function wrapper types using a
//! single unified pattern.
//!
//! # Author
//!
//! Haixing Hu

/// Public interface macro for Rc-based conversions.
///
/// This macro automatically detects the number of generic parameters and calls
/// the appropriate internal implementation.
///
/// # Parameters
///
/// * `$rc_type<generics>` - The Rc wrapper type (e.g., `RcConsumer<T>`,
///   `RcBiConsumer<T, U>`)
/// * `$box_type` - The corresponding Box wrapper type (e.g., `BoxConsumer`)
/// * `$once_type` - The corresponding once wrapper type (e.g.,
///   `BoxConsumerOnce`)
/// * `$fn_type:ty` - The complete function type (e.g., `impl Fn(&T)`,
///   `impl Fn(&T, &U) -> R`)
/// # Examples
///
/// ```ignore
/// // For Consumer types (single parameter, direct call)
/// impl_rc_conversions!(
///     RcConsumer<T>,
///     BoxConsumer,
///     BoxConsumerOnce,
///     impl Fn(&T)
/// );
///
/// // For StatefulConsumer types (single parameter, borrow_mut call)
/// impl_rc_conversions!(
///     RcStatefulConsumer<T>,
///     BoxStatefulConsumer,
///     BoxConsumerOnce,
///     impl FnMut(&T),
///     borrow_mut
/// );
///
/// // For BiConsumer types (two parameters, direct call)
/// impl_rc_conversions!(
///     RcBiConsumer<T, U>,
///     BoxBiConsumer,
///     BoxBiConsumerOnce,
///     impl Fn(&T, &U)
/// );
/// ```
///
/// # Author
///
/// Haixing Hu

macro_rules! impl_rc_conversions {
    // ==================== Internal Implementation Mode ====================
    // Unified internal implementation, using args($($arg:ident),*) to handle
    // arbitrary number of parameters

    // Direct call version (for RcConsumer, RcFunction, etc.)
    (
        @internal
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $once_type:ident,
        $fn_type:ty,
        direct,
        args($($arg:ident),*)
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $box_type::new_with_optional_name(
                move |$($arg),*| (self.function)($($arg),*),
                self.name,
            )
        }

        fn into_rc(self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        fn into_fn(self) -> $fn_type
        {
            move |$($arg),*| (self.function)($($arg),*)
        }

        fn into_once(self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $once_type::new_with_optional_name(
                move |$($arg),*| (self.function)($($arg),*),
                self.name
            )
        }

        fn to_box(&self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $box_type::new_with_optional_name(
                move |$($arg),*| (self_fn)($($arg),*),
                self_name
            )
        }

        fn to_rc(&self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self.clone()
        }

        fn to_fn(&self) -> $fn_type {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn)($($arg),*)
        }

        fn to_once(&self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $once_type::new_with_optional_name(
                move |$($arg),*| (self_fn)($($arg),*),
                self_name
            )
        }
    };

    // Borrow mut version (for RcStatefulConsumer, RcStatefulFunction, etc.)
    (
        @internal
        $rc_type:ident < $($generics:ident),* >,
        $box_type:ident,
        $once_type:ident,
        $fn_type:ty,
        borrow_mut,
        args($($arg:ident),*)
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $box_type::new_with_optional_name(
                move |$($arg),*| (self.function.borrow_mut())($($arg),*),
                self.name,
            )
        }

        fn into_rc(self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        fn into_fn(self) -> $fn_type
        {
            move |$($arg),*| (self.function.borrow_mut())($($arg),*)
        }

        fn into_once(self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $once_type::new_with_optional_name(
                move |$($arg),*| (self.function.borrow_mut())($($arg),*),
                self.name
            )
        }

        fn to_box(&self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $box_type::new_with_optional_name(
                move |$($arg),*| (self_fn.borrow_mut())($($arg),*),
                self_name
            )
        }

        fn to_rc(&self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self.clone()
        }

        fn to_fn(&self) -> $fn_type {
            let self_fn = self.function.clone();
            move |$($arg),*| (self_fn.borrow_mut())($($arg),*)
        }

        fn to_once(&self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            let self_fn = self.function.clone();
            let self_name = self.name.clone();
            $once_type::new_with_optional_name(
                move |$($arg),*| (self_fn.borrow_mut())($($arg),*),
                self_name
            )
        }
    };

    // ==================== Public Interface Modes ====================

    // Single parameter version (1 generic parameter) - default direct call
    (
        $rc_type:ident < $t:ident >,
        $box_type:ident,
        $once_type:ident,
        $fn_type:ty
    ) => {
        impl_rc_conversions!(
            @internal
            $rc_type<$t>,
            $box_type,
            $once_type,
            $fn_type,
            direct,
            args(t)
        );
    };

    // Single parameter version with borrow_mut
    (
        $rc_type:ident < $t:ident >,
        $box_type:ident,
        $once_type:ident,
        $fn_type:ty,
        borrow_mut
    ) => {
        impl_rc_conversions!(
            @internal
            $rc_type<$t>,
            $box_type,
            $once_type,
            $fn_type,
            borrow_mut,
            args(t)
        );
    };

    // Two parameter version (2 generic parameters) - default direct call
    (
        $rc_type:ident < $t:ident, $u:ident >,
        $box_type:ident,
        $once_type:ident,
        $fn_type:ty
    ) => {
        impl_rc_conversions!(
            @internal
            $rc_type<$t, $u>,
            $box_type,
            $once_type,
            $fn_type,
            direct,
            args(t, u)
        );
    };

    // Two parameter version with borrow_mut
    (
        $rc_type:ident < $t:ident, $u:ident >,
        $box_type:ident,
        $once_type:ident,
        $fn_type:ty,
        borrow_mut
    ) => {
        impl_rc_conversions!(
            @internal
            $rc_type<$t, $u>,
            $box_type,
            $once_type,
            $fn_type,
            borrow_mut,
            args(t, u)
        );
    };

    // // Three parameter version (3 generic parameters) - if needed
    // (
    //     $rc_type:ident < $t:ident, $u:ident, $v:ident >,
    //     $box_type:ident,
    //     $once_type:ident,
    //     $fn_type:ty
    // ) => {
    //     impl_rc_conversions!(
    //         @internal
    //         $rc_type<$t, $u, $v>,
    //         $box_type,
    //         $once_type,
    //         $fn_type,
    //         direct,
    //         args(t, u, v)
    //     );
    // };

    // // Three parameter version with borrow_mut
    // (
    //     $rc_type:ident < $t:ident, $u:ident, $v:ident >,
    //     $box_type:ident,
    //     $once_type:ident,
    //     $fn_type:ty,
    //     borrow_mut
    // ) => {
    //     impl_rc_conversions!(
    //         @internal
    //         $rc_type<$t, $u, $v>,
    //         $box_type,
    //         $once_type,
    //         $fn_type,
    //         borrow_mut,
    //         args(t, u, v)
    //     );
    // };
}

pub(crate) use impl_rc_conversions;
