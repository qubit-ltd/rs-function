// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # BiMutatingFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-mutating-function traits
//! similar to Rust's `FnOnce(&mut T, &mut U) -> R` trait, but with
//! value-oriented semantics for functional programming patterns with two
//! mutable input references.
//!
//! This module provides the `BiMutatingFunctionOnce<T, U, R>` trait and
//! one-time use implementations:
//!
//! - [`BoxBiMutatingFunctionOnce`]: Single ownership, one-time use

mod box_bi_mutating_function_once;
pub use box_bi_mutating_function_once::BoxBiMutatingFunctionOnce;
mod box_conditional_bi_mutating_function_once;
pub use box_conditional_bi_mutating_function_once::BoxConditionalBiMutatingFunctionOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// BiMutatingFunctionOnce trait - consuming bi-mutating-function that takes
/// mutable references
///
/// Defines the behavior of a consuming bi-mutating-function: computing a value
/// of type `R` from mutable references to types `T` and `U` by taking ownership
/// of self. This trait is analogous to `FnOnce(&mut T, &mut U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (mutable reference)
/// * `U` - The type of the second input value (mutable reference)
/// * `R` - The type of the output value
///
/// # Examples
///
/// ```
/// use qubit_function::BiMutatingFunctionOnce;
///
/// let add_second = |first: &mut i32, second: &mut i32| {
///     *first += *second;
///     *first
/// };
/// let mut first = 2;
/// let mut second = 3;
///
/// assert_eq!(add_second.apply(&mut first, &mut second), 5);
/// assert_eq!(first, 5);
/// assert_eq!(second, 3);
/// ```
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
    #[must_use = "the computed callback result should be used"]
    fn apply(self, first: &mut T, second: &mut U) -> R;
}
