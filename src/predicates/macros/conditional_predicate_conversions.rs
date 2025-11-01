/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Conditional Predicate Conversions Macro
//!
//! Generates conversion methods for Conditional Predicate implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional predicate types. It handles both immutable (Predicate) cases using
//! the `#[allow(unused_mut)]` annotation.
//!
//! The macro works by always declaring variables as `mut`, which is necessary for
//! cases where the mutability might be needed, while suppressing unused_mut warnings
//! for Predicate cases where the mutability is not needed.
//!
//! # Parameters
//!
//! * `$box_type<$t:ident>` - The box-based predicate type (e.g., `BoxPredicate<T>`)
//! * `$rc_type:ident` - The rc-based predicate type name (e.g., `RcPredicate`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn`)
//!
//! # Usage Examples
//!
//! For Predicate (immutable):
//! ```ignore
//! impl<T> Predicate<T> for BoxConditionalPredicate<T>
//! where
//!     T: 'static,
//! {
//!     fn test(&self, value: &T) -> bool {
//!         self.condition.test(value) && self.predicate.test(value)
//!     }
//!
//!     impl_conditional_predicate_conversions!(
//!         BoxPredicate<T>,
//!         RcPredicate,
//!         Fn
//!     );
//! }
//! ```
//!
//! # Implementation Details
//!
//! - Uses `#[allow(unused_mut)]` to handle Predicate cases where `mut` is not needed
//! - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
//!   based on their internal operations
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!
//! # Author
//!
//! Haixing Hu

/// Generates conversion methods for Conditional Predicate implementations
///
/// This macro should be used inside an existing impl block (typically within
/// a trait implementation block). It generates individual conversion methods
/// but does not create a complete impl block itself. This macro generates the
/// conversion methods (`into_box`, `into_rc`, `into_fn`) for conditional predicate
/// types. It handles both immutable (Predicate) cases using the `#[allow(unused_mut)]`
/// annotation.
///
/// The macro works by always declaring variables as `mut`, which is necessary for
/// cases where the mutability might be needed, while suppressing unused_mut warnings
/// for Predicate cases where the mutability is not needed.
///
/// # Parameters
///
/// * `$box_type<$t:ident>` - The box-based predicate type (e.g., `BoxPredicate<T>`)
/// * `$rc_type:ident` - The rc-based predicate type name (e.g., `RcPredicate`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn`)
///
/// # Usage Examples
///
/// For Predicate (immutable):
/// ```ignore
/// impl<T> Predicate<T> for BoxConditionalPredicate<T>
/// where
///     T: 'static,
/// {
///     fn test(&self, value: &T) -> bool {
///         self.condition.test(value) && self.predicate.test(value)
///     }
///
///     impl_conditional_predicate_conversions!(
///         BoxPredicate<T>,
///         RcPredicate,
///         Fn
///     );
/// }
/// ```
///
/// # Implementation Details
///
/// - Uses `#[allow(unused_mut)]` to handle Predicate cases where `mut` is not needed
/// - The closures inside `into_box` and `into_rc` will automatically capture as `Fn`
///   based on their internal operations
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
macro_rules! impl_conditional_predicate_conversions {
    // Single generic parameter - Predicate
    (
        $box_type:ident < $t:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t> {
            let condition = self.condition;
            let mut predicate = self.predicate;
            $box_type::new(move |t| {
                condition.test(t) && predicate.test(t)
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t> {
            let condition = self.condition.into_rc();
            let mut predicate = self.predicate.into_rc();
            let mut predicate_fn = predicate;
            $rc_type::new(move |t| {
                condition.test(t) && predicate_fn.test(t)
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&$t) -> bool {
            let condition = self.condition;
            let mut predicate = self.predicate;
            move |t: &$t| {
                condition.test(t) && predicate.test(t)
            }
        }
    };

    // Two generic parameters - BiPredicate
    (
        $box_type:ident < $t:ident, $u:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        #[allow(unused_mut)]
        fn into_box(self) -> $box_type<$t, $u> {
            let condition = self.condition;
            let mut predicate = self.predicate;
            $box_type::new(move |t, u| {
                condition.test(t, u) && predicate.test(t, u)
            })
        }

        #[allow(unused_mut)]
        fn into_rc(self) -> $rc_type<$t, $u> {
            let condition = self.condition.into_rc();
            let mut predicate = self.predicate.into_rc();
            let mut predicate_fn = predicate;
            $rc_type::new(move |t, u| {
                condition.test(t, u) && predicate_fn.test(t, u)
            })
        }

        #[allow(unused_mut)]
        fn into_fn(self) -> impl $fn_trait(&$t, &$u) -> bool {
            let condition = self.condition;
            let mut predicate = self.predicate;
            move |t: &$t, u: &$u| {
                condition.test(t, u) && predicate.test(t, u)
            }
        }
    };
}

pub(crate) use impl_conditional_predicate_conversions;
