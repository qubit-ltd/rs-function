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
//! Defines the `BoxConditionalMutatingFunction` public type.

use super::{
    BoxMutatingFunction,
    BoxPredicate,
    MutatingFunction,
    Predicate,
    impl_box_conditional_function,
    impl_conditional_function_debug_display,
};

// ============================================================================
// BoxConditionalMutatingFunction - Box-based Conditional Mutating Function
// ============================================================================

/// BoxConditionalMutatingFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxMutatingFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxMutatingFunction::when()` and is
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
/// use qubit_function::{MutatingFunction, BoxMutatingFunction};
///
/// let double = BoxMutatingFunction::new(|x: &mut i32| *x * 2);
/// let negate = BoxMutatingFunction::new(|x: &mut i32| -*x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// let mut positive = 5;
/// assert_eq!(conditional.apply(&mut positive), 10); // when branch executed
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut negative), 5); // or_else branch executed
/// ```
///
pub struct BoxConditionalMutatingFunction<T, R> {
    pub(super) function: BoxMutatingFunction<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalMutatingFunction<T, R>,
    BoxMutatingFunction,
    MutatingFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalMutatingFunction<T, R>);
