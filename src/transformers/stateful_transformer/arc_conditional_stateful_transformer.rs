/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalStatefulTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcConditionalStatefulTransformer - Arc-based Conditional StatefulTransformer
// ============================================================================

/// ArcConditionalStatefulTransformer struct
///
/// A thread-safe conditional transformer that only executes when a predicate
/// is satisfied. Uses `ArcStatefulTransformer` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send`, safe for concurrent use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulTransformer, ArcStatefulTransformer};
///
/// let mut transformer = ArcStatefulTransformer::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut transformer_clone = transformer.clone();
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer_clone.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulTransformer<T, R> {
    pub(super) transformer: ArcStatefulTransformer<T, R>,
    pub(super) predicate: ArcPredicate<T>,
}

// Implement ArcConditionalStatefulTransformer
impl_shared_conditional_transformer!(
    ArcConditionalStatefulTransformer<T, R>,
    ArcStatefulTransformer,
    StatefulTransformer,
    into_arc,
    Send + Sync + 'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(ArcConditionalStatefulTransformer<T, R>);

// Implement Clone for ArcConditionalStatefulTransformer
impl_conditional_transformer_clone!(ArcConditionalStatefulTransformer<T, R>);
