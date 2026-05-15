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
//! Defines the `BoxConditionalBiTransformer` public type.

use super::{
    BiPredicate,
    BiTransformer,
    BoxBiPredicate,
    BoxBiTransformer,
    impl_box_conditional_transformer,
    impl_conditional_transformer_debug_display,
};

// ============================================================================
// BoxConditionalBiTransformer - Box-based Conditional BiTransformer
// ============================================================================

/// BoxConditionalBiTransformer struct
///
/// A conditional bi-transformer that only executes when a bi-predicate is
/// satisfied. Uses `BoxBiTransformer` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiTransformer**: Can be used anywhere a `BiTransformer` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiTransformer, BoxBiTransformer};
///
/// let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = BoxBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(5, 3), 8);  // when branch executed
/// assert_eq!(conditional.apply(-5, 3), -15); // or_else branch executed
/// ```
///
pub struct BoxConditionalBiTransformer<T, U, R> {
    pub(super) transformer: BoxBiTransformer<T, U, R>,
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalTransformer
impl_box_conditional_transformer!(
    BoxConditionalBiTransformer<T, U, R>,
    BoxBiTransformer,
    BiTransformer
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalBiTransformer<T, U, R>);
