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
//! Defines the `BoxConditionalStatefulFunction` public type.

use super::{
    BoxPredicate,
    BoxStatefulFunction,
    Predicate,
    StatefulFunction,
    impl_box_conditional_function,
    impl_conditional_function_debug_display,
};

// ============================================================================
// BoxConditionalStatefulFunction - Box-based Conditional StatefulFunction
// ============================================================================

/// BoxConditionalStatefulFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxStatefulFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
/// - **Implements StatefulFunction**: Can be used anywhere a `StatefulFunction` is expected
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulFunction, BoxStatefulFunction};
///
/// let mut high_count = 0;
/// let mut low_count = 0;
///
/// let mut function = BoxStatefulFunction::new(move |x: &i32| {
///     high_count += 1;
///     x * 2
/// })
/// .when(|x: &i32| *x >= 10)
/// .or_else(move |x: &i32| {
///     low_count += 1;
///     x + 1
/// });
///
/// assert_eq!(function.apply(&15), 30); // when branch executed
/// assert_eq!(function.apply(&5), 6);   // or_else branch executed
/// ```
///
pub struct BoxConditionalStatefulFunction<T, R> {
    pub(super) function: BoxStatefulFunction<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalStatefulFunction<T, R>,
    BoxStatefulFunction,
    StatefulFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalStatefulFunction<T, R>);
