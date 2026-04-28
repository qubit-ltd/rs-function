/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # FunctionOnce Types
//!
//! Provides Rust implementations of consuming function traits similar to
//! Rust's `FnOnce(&T) -> R` trait, for computing output from input references.
//!
//! This module provides the `FunctionOnce<T, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxFunctionOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu
use crate::functions::macros::{
    impl_box_conditional_function,
    impl_box_function_methods,
    impl_conditional_function_debug_display,
    impl_fn_ops_trait,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
    impl_function_identity_method,
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

mod box_function_once;
pub use box_function_once::BoxFunctionOnce;
mod box_conditional_function_once;
pub use box_conditional_function_once::BoxConditionalFunctionOnce;
mod fn_function_once_ops;
pub use fn_function_once_ops::FnFunctionOnceOps;

// ============================================================================
// Core Trait
// ============================================================================

/// FunctionOnce trait - consuming function that takes ownership
///
/// Defines the behavior of a consuming function: computing a value of
/// type `R` from a reference to type `T` by taking ownership of self.
/// This trait is analogous to `FnOnce(&T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait FunctionOnce<T, R> {
    /// Applies the function to the input reference, consuming self
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, t: &T) -> R;

    /// Converts to BoxFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let boxed = double.into_box();
    /// assert_eq!(boxed.apply(&21), 42);
    /// ```
    fn into_box(self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxFunctionOnce::new(move |input: &T| self.apply(input))
    }

    /// Converts function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let func = double.into_fn();
    /// assert_eq!(func(&21), 42);
    /// ```
    fn into_fn(self) -> impl FnOnce(&T) -> R
    where
        Self: Sized + 'static,
    {
        move |input: &T| self.apply(input)
    }

    /// Converts to BoxFunctionOnce without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxFunctionOnce` that
    /// captures a clone. Types implementing `Clone` can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let boxed = double.to_box();
    /// assert_eq!(boxed.apply(&21), 42);
    /// ```
    fn to_box(&self) -> BoxFunctionOnce<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts function to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures a
    /// clone of `self` and calls its `apply` method. Types can
    /// override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let func = double.to_fn();
    /// assert_eq!(func(&21), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
