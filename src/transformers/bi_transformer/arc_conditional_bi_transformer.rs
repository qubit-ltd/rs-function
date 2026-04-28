/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcConditionalBiTransformer - Arc-based Conditional BiTransformer
// ============================================================================

/// ArcConditionalBiTransformer struct
///
/// A thread-safe conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `ArcBiTransformer` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcBiTransformer::when()` and is
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
/// use qubit_function::{BiTransformer, ArcBiTransformer};
///
/// let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = ArcBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalBiTransformer<T, U, R> {
    pub(super) transformer: ArcBiTransformer<T, U, R>,
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiTransformer
impl_shared_conditional_transformer!(
    ArcConditionalBiTransformer<T, U, R>,
    ArcBiTransformer,
    BiTransformer,
    into_arc,
    Send + Sync + 'static
);

// Implement Debug and Display for ArcConditionalBiTransformer
impl_conditional_transformer_debug_display!(ArcConditionalBiTransformer<T, U, R>);

// Implement Clone for ArcConditionalBiTransformer
impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, R>);
