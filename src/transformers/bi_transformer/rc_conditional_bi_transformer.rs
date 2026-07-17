// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalBiTransformer` public type.

use {
    crate::BiPredicate,
    crate::BiTransformer,
    crate::RcBiPredicate,
    crate::RcBiTransformer,
    crate::transformers::macros::impl_conditional_transformer_clone,
    crate::transformers::macros::impl_conditional_transformer_debug_display,
    crate::transformers::macros::impl_shared_conditional_transformer,
};

// ============================================================================
// RcConditionalBiTransformer - Rc-based Conditional BiTransformer
// ============================================================================

/// RcConditionalBiTransformer struct
///
/// A single-threaded conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `RcBiTransformer` and `RcBiPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when bi-predicate returns
///   `true`
/// - **Ownership Cost**: `Rc` uses non-atomic reference counting; `Arc` uses
///   atomic reference counting, and neither wrapper locks callback invocation
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiTransformer, RcBiTransformer};
///
/// let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = RcBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalBiTransformer<T, U, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: RcBiTransformer<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalBiTransformer
impl_shared_conditional_transformer!(
    RcConditionalBiTransformer<T, U, R>,
    RcBiTransformer,
    BiTransformer,
    callback_bounds = ('static)
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalBiTransformer<T, U, R>);

// Implement Clone for RcConditionalBiTransformer
impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, R>);
