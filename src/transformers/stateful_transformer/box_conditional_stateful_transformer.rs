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
//! Defines the `BoxConditionalStatefulTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalStatefulTransformer - Box-based Conditional StatefulTransformer
// ============================================================================

/// BoxConditionalStatefulTransformer struct
///
/// A conditional transformer that only executes when a predicate is satisfied.
/// Uses `BoxStatefulTransformer` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
/// - **Implements StatefulTransformer**: Can be used anywhere a `StatefulTransformer` is expected
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulTransformer, BoxStatefulTransformer};
///
/// let mut high_count = 0;
/// let mut low_count = 0;
///
/// let mut transformer = BoxStatefulTransformer::new(move |x: i32| {
///     high_count += 1;
///     x * 2
/// })
/// .when(|x: &i32| *x >= 10)
/// .or_else(move |x| {
///     low_count += 1;
///     x + 1
/// });
///
/// assert_eq!(transformer.apply(15), 30); // when branch executed
/// assert_eq!(transformer.apply(5), 6);   // or_else branch executed
/// ```
///
pub struct BoxConditionalStatefulTransformer<T, R> {
    pub(super) transformer: BoxStatefulTransformer<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Implement BoxConditionalTransformer
impl_box_conditional_transformer!(
    BoxConditionalStatefulTransformer<T, R>,
    BoxStatefulTransformer,
    StatefulTransformer
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalStatefulTransformer<T, R>);
