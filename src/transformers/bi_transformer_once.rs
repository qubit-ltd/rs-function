/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # BiTransformerOnce Types
//!
//! Provides Rust implementations of consuming bi-transformer traits similar to
//! Rust's `FnOnce` trait, but with value-oriented semantics for functional
//! programming patterns with two inputs.
//!
//! This module provides the `BiTransformerOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiTransformerOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::bi_predicate::{
    BiPredicate,
    BoxBiPredicate,
};
use crate::transformers::{
    macros::{
        impl_box_conditional_transformer,
        impl_box_transformer_methods,
        impl_conditional_transformer_debug_display,
        impl_transformer_common_methods,
        impl_transformer_constant_method,
        impl_transformer_debug_display,
    },
    transformer_once::TransformerOnce,
};

mod box_bi_transformer_once;
pub use box_bi_transformer_once::BoxBiTransformerOnce;
mod fn_bi_transformer_once_ops;
pub use fn_bi_transformer_once_ops::FnBiTransformerOnceOps;
mod binary_operator_once;
pub use binary_operator_once::BinaryOperatorOnce;
mod box_binary_operator_once;
pub use box_binary_operator_once::BoxBinaryOperatorOnce;
mod box_conditional_bi_transformer_once;
pub use box_conditional_bi_transformer_once::BoxConditionalBiTransformerOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// BiTransformerOnce trait - consuming bi-transformation that takes ownership
///
/// Defines the behavior of a consuming bi-transformer: converting two values of
/// types `T` and `U` to a value of type `R` by taking ownership of self and
/// both inputs. This trait is analogous to `FnOnce(T, U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (consumed)
/// * `U` - The type of the second input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiTransformerOnce<T, U, R> {
    /// Transforms two input values, consuming self and both inputs
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value (consumed)
    /// * `second` - The second input value (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(self, first: T, second: U) -> R;

    /// Converts to BoxBiTransformerOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiTransformerOnce<T, U, R>`
    fn into_box(self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiTransformerOnce::new(move |t: T, u: U| self.apply(t, u))
    }

    /// Converts bi-transformer to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(T, U) -> R`
    fn into_fn(self) -> impl FnOnce(T, U) -> R
    where
        Self: Sized + 'static,
    {
        move |t: T, u: U| self.apply(t, u)
    }

    /// Converts bi-transformer to a boxed function pointer
    ///
    /// **📌 Borrows `&self`**: The original bi-transformer remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a boxed function pointer that implements `FnOnce(T, U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiTransformerOnce;
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let func = add.to_fn();
    /// assert_eq!(func(20, 22), 42);
    /// ```
    fn to_box(&self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts bi-transformer to a closure
    ///
    /// **📌 Borrows `&self`**: The original bi-transformer remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(T, U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiTransformerOnce;
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let func = add.to_fn();
    /// assert_eq!(func(20, 22), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(T, U) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
