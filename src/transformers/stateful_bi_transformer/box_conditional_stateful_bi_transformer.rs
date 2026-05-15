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
//! Defines the `BoxConditionalStatefulBiTransformer` public type.

use super::{
    BiPredicate,
    BoxBiPredicate,
    BoxStatefulBiTransformer,
    StatefulBiTransformer,
    impl_box_conditional_transformer,
    impl_conditional_transformer_debug_display,
};

// ============================================================================
// BoxConditionalStatefulBiTransformer - Box-based Conditional StatefulBiTransformer
// ============================================================================

/// BoxConditionalStatefulBiTransformer struct
///
/// A conditional bi-transformer that only executes when a bi-predicate is
/// satisfied. Uses `BoxStatefulBiTransformer` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxStatefulBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements StatefulBiTransformer**: Can be used anywhere a `StatefulBiTransformer` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{StatefulBiTransformer, BoxStatefulBiTransformer};
///
/// let add = BoxStatefulBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = BoxStatefulBiTransformer::new(|x: i32, y: i32| x * y);
/// let mut conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(5, 3), 8);  // when branch executed
/// assert_eq!(conditional.apply(-5, 3), -15); // or_else branch executed
/// ```
///
pub struct BoxConditionalStatefulBiTransformer<T, U, R> {
    pub(super) transformer: BoxStatefulBiTransformer<T, U, R>,
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalStatefulBiTransformer
impl_box_conditional_transformer!(
    BoxConditionalStatefulBiTransformer<T, U, R>,
    BoxStatefulBiTransformer,
    StatefulBiTransformer
);

// Implement Debug and Display for BoxConditionalStatefulBiTransformer
impl_conditional_transformer_debug_display!(BoxConditionalStatefulBiTransformer<T, U, R>);
