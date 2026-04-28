/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiMutatingFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-mutating-function traits similar to
//! Rust's `FnOnce(&mut T, &mut U) -> R` trait, but with value-oriented semantics for functional
//! programming patterns with two mutable input references.
//!
//! This module provides the `BiMutatingFunctionOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiMutatingFunctionOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu
use crate::functions::{
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_debug_display,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
    },
    mutating_function_once::MutatingFunctionOnce,
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::bi_predicate::{
    BiPredicate,
    BoxBiPredicate,
};

mod box_bi_mutating_function_once;
pub use box_bi_mutating_function_once::BoxBiMutatingFunctionOnce;
mod fn_bi_mutating_function_once_ops;
pub use fn_bi_mutating_function_once_ops::FnBiMutatingFunctionOnceOps;
mod box_conditional_bi_mutating_function_once;
pub use box_conditional_bi_mutating_function_once::BoxConditionalBiMutatingFunctionOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// BiMutatingFunctionOnce trait - consuming bi-mutating-function that takes
/// mutable references
///
/// Defines the behavior of a consuming bi-mutating-function: computing a value of
/// type `R` from mutable references to types `T` and `U` by taking ownership of self.
/// This trait is analogous to `FnOnce(&mut T, &mut U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (mutable reference)
/// * `U` - The type of the second input value (mutable reference)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiMutatingFunctionOnce<T, U, R> {
    /// Computes output from two mutable references, consuming self
    ///
    /// # Parameters
    ///
    /// * `first` - Mutable reference to the first input value
    /// * `second` - Mutable reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, first: &mut T, second: &mut U) -> R;

    /// Converts to BoxBiMutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiMutatingFunctionOnce<T, U, R>`
    fn into_box(self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiMutatingFunctionOnce::new(move |t: &mut T, u: &mut U| self.apply(t, u))
    }

    /// Converts bi-mutating-function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&mut T, &mut U) -> R`
    fn into_fn(self) -> impl FnOnce(&mut T, &mut U) -> R
    where
        Self: Sized + 'static,
    {
        move |t: &mut T, u: &mut U| self.apply(t, u)
    }

    /// Converts bi-mutating-function to a boxed function pointer
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a boxed function pointer that implements `FnOnce(&mut T, &mut U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiMutatingFunctionOnce;
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let func = swap_and_sum.to_box();
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(func.apply(&mut a, &mut b), 42);
    /// ```
    fn to_box(&self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts bi-mutating-function to a closure
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&mut T, &mut U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiMutatingFunctionOnce;
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let func = swap_and_sum.to_fn();
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(func(&mut a, &mut b), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&mut T, &mut U) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
