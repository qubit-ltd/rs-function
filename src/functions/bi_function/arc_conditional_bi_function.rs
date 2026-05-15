/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `ArcConditionalBiFunction` public type.

use super::{
    ArcBiFunction,
    ArcBiPredicate,
    BiFunction,
    BiPredicate,
    impl_conditional_function_clone,
    impl_conditional_function_debug_display,
    impl_shared_conditional_function,
};

// ============================================================================
// ArcConditionalBiFunction - Arc-based Conditional BiFunction
// ============================================================================

/// ArcConditionalBiFunction struct
///
/// A thread-safe conditional bi-function that only executes when a
/// bi-predicate is satisfied. Uses `ArcBiFunction` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcBiFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiFunction, ArcBiFunction};
///
/// let add = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = ArcBiFunction::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(&5, &3), 8);
/// assert_eq!(conditional_clone.apply(&-5, &3), -15);
/// ```
///
pub struct ArcConditionalBiFunction<T, U, R> {
    pub(super) function: ArcBiFunction<T, U, R>,
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiFunction
impl_shared_conditional_function!(
    ArcConditionalBiFunction<T, U, R>,
    ArcBiFunction,
    BiFunction,
    into_arc,
    Send + Sync + 'static
);

// Implement Debug and Display for ArcConditionalBiFunction
impl_conditional_function_debug_display!(ArcConditionalBiFunction<T, U, R>);

// Implement Clone for ArcConditionalBiFunction
impl_conditional_function_clone!(ArcConditionalBiFunction<T, U, R>);
