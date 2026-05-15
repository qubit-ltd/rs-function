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
//! Defines the `ArcConditionalTransformer` public type.

use super::{
    ArcPredicate,
    ArcTransformer,
    Predicate,
    Transformer,
    impl_conditional_transformer_clone,
    impl_conditional_transformer_debug_display,
    impl_shared_conditional_transformer,
};

// ============================================================================
// ArcConditionalTransformer - Arc-based Conditional Transformer
// ============================================================================

/// ArcConditionalTransformer struct
///
/// A thread-safe conditional transformer that only executes when a predicate is
/// satisfied. Uses `ArcTransformer` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Transformer, ArcTransformer};
///
/// let double = ArcTransformer::new(|x: i32| x * 2);
/// let identity = ArcTransformer::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional_clone.apply(-5), -5);
/// ```
///
pub struct ArcConditionalTransformer<T, R> {
    pub(super) transformer: ArcTransformer<T, R>,
    pub(super) predicate: ArcPredicate<T>,
}

// Implement ArcConditionalTransformer
impl_shared_conditional_transformer!(
    ArcConditionalTransformer<T, R>,
    ArcTransformer,
    Transformer,
    into_arc,
    Send + Sync + 'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(ArcConditionalTransformer<T, R>);

// Implement Clone for ArcConditionalTransformer
impl_conditional_transformer_clone!(ArcConditionalTransformer<T, R>);
