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
//! Defines the `ArcConditionalStatefulBiTransformer` public type.

use super::{
    ArcBiPredicate,
    ArcStatefulBiTransformer,
    BiPredicate,
    StatefulBiTransformer,
    impl_conditional_transformer_clone,
    impl_conditional_transformer_debug_display,
    impl_shared_conditional_transformer,
};

// ============================================================================
// ArcConditionalStatefulBiTransformer - Arc-based Conditional StatefulBiTransformer
// ============================================================================

/// ArcConditionalStatefulBiTransformer struct
///
/// A thread-safe conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `ArcStatefulBiTransformer` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcStatefulBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulBiTransformer, ArcStatefulBiTransformer};
///
/// let add = ArcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = ArcStatefulBiTransformer::new(|x: i32, y: i32| x * y);
/// let mut conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let mut conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
///
pub struct ArcConditionalStatefulBiTransformer<T, U, R> {
    pub(super) transformer: ArcStatefulBiTransformer<T, U, R>,
    pub(super) predicate: ArcBiPredicate<T, U>,
}

impl_shared_conditional_transformer!(
    ArcConditionalStatefulBiTransformer<T, U, R>,
    ArcStatefulBiTransformer,
    StatefulBiTransformer,
    into_arc,
    Send + Sync + 'static
);

// Implement Debug and Display for ArcConditionalStatefulBiTransformer
impl_conditional_transformer_debug_display!(ArcConditionalStatefulBiTransformer<T, U, R>);

// Implement Clone for ArcConditionalStatefulBiTransformer
impl_conditional_transformer_clone!(ArcConditionalStatefulBiTransformer<T, U, R>);
