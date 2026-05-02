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
//! Defines the `BoxConditionalMutatingFunctionOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalMutatingFunctionOnce - Box-based Conditional Mutating Function
// ============================================================================

/// BoxConditionalMutatingFunctionOnce struct
///
/// A conditional consuming transformer that only executes when a predicate is
/// satisfied. Uses `BoxMutatingFunctionOnce` and `BoxPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxMutatingFunctionOnce::when()` and
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
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// let double = BoxMutatingFunctionOnce::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let negate = BoxMutatingFunctionOnce::new(|x: &mut i32| {
///     *x = -*x;
///     *x
/// });
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// let mut positive = 5;
/// assert_eq!(conditional.apply(&mut positive), 10); // when branch executed
///
/// let double2 = BoxMutatingFunctionOnce::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let negate2 = BoxMutatingFunctionOnce::new(|x: &mut i32| {
///     *x = -*x;
///     *x
/// });
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// let mut negative = -5;
/// assert_eq!(conditional2.apply(&mut negative), 5); // or_else branch executed
/// ```
///
pub struct BoxConditionalMutatingFunctionOnce<T, R> {
    pub(super) function: BoxMutatingFunctionOnce<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalMutatingFunctionOnce<T, R>,
    BoxMutatingFunctionOnce,
    MutatingFunctionOnce
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalMutatingFunctionOnce<T, R>);
