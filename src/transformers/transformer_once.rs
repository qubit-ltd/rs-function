/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # TransformerOnce Types
//!
//! Provides Rust implementations of consuming transformer traits similar to
//! Rust's `FnOnce` trait, but with value-oriented semantics for functional
//! programming patterns.
//!
//! This module provides the `TransformerOnce<T, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxTransformerOnce`]: Single ownership, one-time use
//!

use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};
use crate::transformers::macros::{
    impl_box_conditional_transformer,
    impl_box_transformer_methods,
    impl_conditional_transformer_debug_display,
    impl_transformer_common_methods,
    impl_transformer_constant_method,
    impl_transformer_debug_display,
};

mod box_transformer_once;
pub use box_transformer_once::BoxTransformerOnce;
mod fn_transformer_once_ops;
pub use fn_transformer_once_ops::FnTransformerOnceOps;
mod unary_operator_once;
pub use unary_operator_once::UnaryOperatorOnce;
mod box_unary_operator_once;
pub use box_unary_operator_once::BoxUnaryOperatorOnce;
mod box_conditional_transformer_once;
pub use box_conditional_transformer_once::BoxConditionalTransformerOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// TransformerOnce trait - consuming transformation that takes ownership
///
/// Defines the behavior of a consuming transformer: converting a value of
/// type `T` to a value of type `R` by taking ownership of both self and the
/// input. This trait is analogous to `FnOnce(T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
pub trait TransformerOnce<T, R> {
    /// Transforms the input value, consuming both self and input
    ///
    /// # Parameters
    ///
    /// * `input` - The input value (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(self, input: T) -> R;

    /// Converts to BoxTransformerOnce
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxTransformerOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let boxed = double.into_box();
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn into_box(self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxTransformerOnce::new(move |input: T| self.apply(input))
    }

    /// Converts transformer to a closure
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let func = double.into_fn();
    /// assert_eq!(func(21), 42);
    /// ```
    fn into_fn(self) -> impl FnOnce(T) -> R
    where
        Self: Sized + 'static,
    {
        move |input: T| self.apply(input)
    }

    /// Converts to BoxTransformerOnce without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxTransformerOnce` that
    /// captures a clone. Types implementing `Clone` can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxTransformerOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let boxed = double.to_box();
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn to_box(&self) -> BoxTransformerOnce<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts transformer to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures a
    /// clone of `self` and calls its `transform` method. Types can
    /// override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let func = double.to_fn();
    /// assert_eq!(func(21), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(T) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
