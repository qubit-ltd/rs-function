/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Predicate Common Methods Macro
//!
//! Generates common Predicate methods (new, new_with_name, name,
//! set_name, always_true)
//!
//! Generates constructor methods, name management methods and always_true
//! constructor for Predicate structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter predicates.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn(&T) -> bool + 'static`)
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
//!

/// Generates common Predicate methods (new, new_with_name, name,
/// set_name, always_true, always_false)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and always_true/always_false constructors for Predicate structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter predicates.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn(&T) -> bool + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
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
/// * `new()` - Creates a new predicate
/// * `new_with_name()` - Creates a named predicate
/// * `name()` - Gets the name of the predicate
/// * `set_name()` - Sets the name of the predicate
/// * `always_true()` - Creates a predicate that always returns true
/// * `always_false()` - Creates a predicate that always returns false
///
macro_rules! impl_predicate_common_methods {
    // Single generic parameter - Predicate types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        crate::macros::impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "predicate"
        );
        crate::macros::impl_common_name_methods!("predicate");

        /// Creates a predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_| true)
        }

        /// Creates a predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_| false)
        }
    };

    // Two generic parameters - BiPredicate types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        crate::macros::impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-predicate"
        );
        crate::macros::impl_common_name_methods!("bi-predicate");

        /// Creates a bi-predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        #[inline]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_, _| true)
        }

        /// Creates a bi-predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        #[inline]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_, _| false)
        }
    };
}

pub(crate) use impl_predicate_common_methods;
