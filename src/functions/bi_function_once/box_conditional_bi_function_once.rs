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
//! Defines the `BoxConditionalBiFunctionOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalBiFunctionOnce - Box-based Conditional BiFunction
// ============================================================================

/// BoxConditionalBiFunctionOnce struct
///
/// A conditional consuming bi-function that only executes when a bi-predicate
/// is satisfied. Uses `BoxBiFunctionOnce` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiFunctionOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiFunctionOnce, BoxBiFunctionOnce};
///
/// let add = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// assert_eq!(conditional.apply(&5, &3), 8); // when branch executed
///
/// let add2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
/// let multiply2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
/// let conditional2 = add2.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply2);
/// assert_eq!(conditional2.apply(&-5, &3), -15); // or_else branch executed
/// ```
///
pub struct BoxConditionalBiFunctionOnce<T, U, R> {
    pub(super) function: BoxBiFunctionOnce<T, U, R>,
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiFunctionOnce
impl_box_conditional_function!(
    BoxConditionalBiFunctionOnce<T, U, R>,
    BoxBiFunctionOnce,
    BiFunctionOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiFunctionOnce<T, U, R>);
