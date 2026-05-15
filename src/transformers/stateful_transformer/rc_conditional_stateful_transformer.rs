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
//! Defines the `RcConditionalStatefulTransformer` public type.

use super::{
    Predicate,
    RcPredicate,
    RcStatefulTransformer,
    StatefulTransformer,
    impl_conditional_transformer_clone,
    impl_conditional_transformer_debug_display,
    impl_shared_conditional_transformer,
};

// ============================================================================
// RcConditionalStatefulTransformer - Rc-based Conditional StatefulTransformer
// ============================================================================

/// RcConditionalStatefulTransformer struct
///
/// A single-threaded conditional transformer that only executes when a
/// predicate is satisfied. Uses `RcStatefulTransformer` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulTransformer`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulTransformer, RcStatefulTransformer};
///
/// let mut transformer = RcStatefulTransformer::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut transformer_clone = transformer.clone();
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer_clone.apply(-5), 5);
/// ```
///
pub struct RcConditionalStatefulTransformer<T, R> {
    pub(super) transformer: RcStatefulTransformer<T, R>,
    pub(super) predicate: RcPredicate<T>,
}

// Implement RcConditionalStatefulTransformer
impl_shared_conditional_transformer!(
    RcConditionalStatefulTransformer<T, R>,
    RcStatefulTransformer,
    StatefulTransformer,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalStatefulTransformer<T, R>);

// Implement Clone for RcConditionalStatefulTransformer
impl_conditional_transformer_clone!(RcConditionalStatefulTransformer<T, R>);
