// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Predicate Common Methods Macro
//!
//! Generates common Predicate methods (new, new_with_name, name,
//! set_name, always_true, always_false)
//!
//! Generates constructor methods, name management methods, and constructors
//! for predicates that always return `true` or `false`. This macro should be
//! called inside an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter predicates.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
//!   `Fn(&T) -> bool + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Predicate
//! impl_predicate_common_methods!(
//!     BoxPredicate<T>,
//!     (Fn(&T) -> bool + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Two generic parameters - BiPredicate
//! impl_predicate_common_methods!(
//!     BoxBiPredicate<T, U>,
//!     (Fn(&T, &U) -> bool + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new predicate
//! * `new_with_name()` - Creates a named predicate
//! * `name()` - Gets the name of the predicate
//! * `set_name()` - Sets the name of the predicate
//! * `always_true()` - Creates a predicate that always returns true
//! * `always_false()` - Creates a predicate that always returns false

/// Routes stateful predicate variants through the shared constructor macro.
///
/// The adapters preserve each variant's `FnMut` behavior and add the
/// thread-safety bound required by `Arc`-backed predicates.
#[cfg(feature = "stateful")]
macro_rules! impl_stateful_predicate_new_methods {
    (BoxStatefulPredicate<$t:ident>, |$f:ident| $w:expr) => { $crate::predicates::macros::impl_stateful_predicate_new_methods!(@one $t, ('static), |$f| $w); };
    (RcStatefulPredicate<$t:ident>, |$f:ident| $w:expr) => { $crate::predicates::macros::impl_stateful_predicate_new_methods!(@one $t, ('static), |$f| $w); };
    (ArcStatefulPredicate<$t:ident>, |$f:ident| $w:expr) => { $crate::predicates::macros::impl_stateful_predicate_new_methods!(@one $t, (Send + 'static), |$f| $w); };
    (BoxStatefulBiPredicate<$t:ident, $u:ident>, |$f:ident| $w:expr) => { $crate::predicates::macros::impl_stateful_predicate_new_methods!(@two $t, $u, ('static), |$f| $w); };
    (RcStatefulBiPredicate<$t:ident, $u:ident>, |$f:ident| $w:expr) => { $crate::predicates::macros::impl_stateful_predicate_new_methods!(@two $t, $u, ('static), |$f| $w); };
    (ArcStatefulBiPredicate<$t:ident, $u:ident>, |$f:ident| $w:expr) => { $crate::predicates::macros::impl_stateful_predicate_new_methods!(@two $t, $u, (Send + 'static), |$f| $w); };
    (@one $t:ident, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic_mut (StatefulPredicate<$t> + $($b)+), |source| move |value: &$t| source.test(value), |$f| $w, "predicate"); };
    (@two $t:ident, $u:ident, ($($b:tt)+), |$f:ident| $w:expr) => { crate::macros::impl_common_new_methods!(semantic_mut (StatefulBiPredicate<$t, $u> + $($b)+), |source| move |first: &$t, second: &$u| source.test(first, second), |$f| $w, "bi-predicate"); };
}

/// Generates common predicate methods for a target struct's existing `impl`.
///
/// The macro adds constructors, name management, and always-true/always-false
/// constructors. It accepts either one predicate parameter or two
/// bi-predicate parameters and adapts the supplied callback wrapper expression.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with its generic parameters.
/// * `$fn_trait_with_bounds` - Closure trait and its bounds, such as `Fn(&T) ->
///   bool + 'static`.
/// * `$wrapper_expr` - Expression that wraps the closure bound to `f`.
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Predicate
/// impl_predicate_common_methods!(
///     BoxPredicate<T>,
///     (Fn(&T) -> bool + 'static),
///     |f| Box::new(f)
/// );
///
/// // Two generic parameters - BiPredicate
/// impl_predicate_common_methods!(
///     BoxBiPredicate<T, U>,
///     (Fn(&T, &U) -> bool + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new predicate.
/// * `new_with_name()` - Creates a named predicate.
/// * `name()` - Gets the predicate's optional name.
/// * `set_name()` - Sets the predicate's name.
/// * `always_true()` - Creates a predicate that always returns `true`.
/// * `always_false()` - Creates a predicate that always returns `false`.
macro_rules! impl_predicate_common_methods {
    // The Rc stateful bi-predicate emits its restricted constructor at the end
    // of its inherent impl, after all public methods.
    (
        RcStatefulBiPredicate<$t:ident, $u:ident>,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        defer_metadata
    ) => {
        crate::macros::impl_common_new_public_methods!(
            semantic_mut (StatefulBiPredicate<$t, $u> + 'static),
            |source| move |first: &$t, second: &$u| source.test(first, second),
            |$f| $wrapper_expr,
            "bi-predicate"
        );
        /// Creates a bi-predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!(RcStatefulBiPredicate), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_: &$t, _: &$u| true)
        }

        /// Creates a bi-predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!(RcStatefulBiPredicate), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_: &$t, _: &$u| false)
        }

        crate::macros::impl_common_new_metadata_method!(
            semantic_mut(StatefulBiPredicate<$t, $u> + 'static),
            |source| move |first: &$t, second: &$u| source.test(first, second),
            |$f| $wrapper_expr,
            "bi-predicate"
        );

        crate::macros::impl_common_name_methods!("bi-predicate");
    };

    // Single generic parameter with a semantic Predicate adapter.
    (
        $struct_name:ident < $t:ident >,
        semantic ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr
    ) => {
        crate::macros::impl_common_new_public_methods!(
            semantic ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            "predicate"
        );
        /// Creates a predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_: &$t| true)
        }

        /// Creates a predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_: &$t| false)
        }

        crate::macros::impl_common_name_methods!("predicate");

        crate::macros::impl_common_new_metadata_method!(
            semantic ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            "predicate"
        );
    };

    // Two generic parameters with a semantic BiPredicate adapter.
    (
        $struct_name:ident < $t:ident, $u:ident >,
        semantic ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr
    ) => {
        crate::macros::impl_common_new_public_methods!(
            semantic ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            "bi-predicate"
        );
        /// Creates a bi-predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_: &$t, _: &$u| true)
        }

        /// Creates a bi-predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_: &$t, _: &$u| false)
        }

        crate::macros::impl_common_name_methods!("bi-predicate");

        crate::macros::impl_common_new_metadata_method!(
            semantic ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            "bi-predicate"
        );
    };

    // Single generic parameter - Predicate types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::predicates::macros::impl_stateful_predicate_new_methods!($struct_name<$t>, |$f| $wrapper_expr);
        /// Creates a predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_: &$t| true)
        }

        /// Creates a predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_: &$t| false)
        }

        crate::macros::impl_common_name_methods!("predicate");

    };

    // Two generic parameters - BiPredicate types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::predicates::macros::impl_stateful_predicate_new_methods!($struct_name<$t, $u>, |$f| $wrapper_expr);
        /// Creates a bi-predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_: &$t, _: &$u| true)
        }

        /// Creates a bi-predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_: &$t, _: &$u| false)
        }

        crate::macros::impl_common_name_methods!("bi-predicate");

    };
}

pub(crate) use impl_predicate_common_methods;
#[cfg(feature = "stateful")]
pub(crate) use impl_stateful_predicate_new_methods;
