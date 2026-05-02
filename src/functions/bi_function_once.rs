/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # BiFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-function traits similar to
//! Rust's `FnOnce(&T, &U) -> R` trait, but with value-oriented semantics for functional
//! programming patterns with two input references.
//!
//! This module provides the `BiFunctionOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiFunctionOnce`]: Single ownership, one-time use
//!
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::bi_predicate::{
    BiPredicate,
    BoxBiPredicate,
};
use crate::{
    functions::function_once::FunctionOnce,
    functions::macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_debug_display,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
    },
};

mod box_bi_function_once;
pub use box_bi_function_once::BoxBiFunctionOnce;
mod fn_bi_function_once_ops;
pub use fn_bi_function_once_ops::FnBiFunctionOnceOps;
mod box_conditional_bi_function_once;
pub use box_conditional_bi_function_once::BoxConditionalBiFunctionOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// BiFunctionOnce trait - consuming bi-function that takes references
///
/// Defines the behavior of a consuming bi-function: computing a value of
/// type `R` from references to types `T` and `U` by taking ownership of self.
/// This trait is analogous to `FnOnce(&T, &U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (borrowed)
/// * `U` - The type of the second input value (borrowed)
/// * `R` - The type of the output value
///
pub trait BiFunctionOnce<T, U, R> {
    /// Computes output from two input references, consuming self
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, first: &T, second: &U) -> R;

    /// Converts to BoxBiFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiFunctionOnce<T, U, R>`
    fn into_box(self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiFunctionOnce::new(move |t: &T, u: &U| self.apply(t, u))
    }

    /// Converts bi-function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T, &U) -> R`
    fn into_fn(self) -> impl FnOnce(&T, &U) -> R
    where
        Self: Sized + 'static,
    {
        move |t: &T, u: &U| self.apply(t, u)
    }

    /// Converts bi-function to a boxed function pointer
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a boxed function pointer that implements `FnOnce(&T, &U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiFunctionOnce;
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let func = add.to_box();
    /// assert_eq!(func.apply(&20, &22), 42);
    /// ```
    fn to_box(&self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts bi-function to a closure
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T, &U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiFunctionOnce;
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let func = add.to_fn();
    /// assert_eq!(func(&20, &22), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T, &U) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
