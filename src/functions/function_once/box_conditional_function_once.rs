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
//! Defines the `BoxConditionalFunctionOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalFunctionOnce - Box-based Conditional Function
// ============================================================================

/// BoxConditionalFunctionOnce struct
///
/// A conditional consuming transformer that only executes when a predicate is
/// satisfied. Uses `BoxFunctionOnce` and `BoxPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxFunctionOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{FunctionOnce, BoxFunctionOnce};
///
/// let double = BoxFunctionOnce::new(|x: &i32| x * 2);
/// let negate = BoxFunctionOnce::new(|x: &i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// assert_eq!(conditional.apply(&5), 10); // when branch executed
///
/// let double2 = BoxFunctionOnce::new(|x: &i32| x * 2);
/// let negate2 = BoxFunctionOnce::new(|x: &i32| -x);
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// assert_eq!(conditional2.apply(&-5), 5); // or_else branch executed
/// ```
///
pub struct BoxConditionalFunctionOnce<T, R> {
    pub(super) function: BoxFunctionOnce<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalFunctionOnce<T, R>,
    BoxFunctionOnce,
    FunctionOnce
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalFunctionOnce<T, R>);
