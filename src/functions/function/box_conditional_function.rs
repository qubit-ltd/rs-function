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
//! Defines the `BoxConditionalFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalFunction - Box-based Conditional Function
// ============================================================================

/// BoxConditionalFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Function**: Can be used anywhere a `Function` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{Function, BoxFunction};
///
/// let double = BoxFunction::new(|x: &i32| x * 2);
/// let negate = BoxFunction::new(|x: &i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(&5), 10); // when branch executed
/// assert_eq!(conditional.apply(&-5), 5); // or_else branch executed
/// ```
///
pub struct BoxConditionalFunction<T, R> {
    pub(super) function: BoxFunction<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalFunction<T, R>,
    BoxFunction,
    Function
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalFunction<T, R>);
