// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalTransformer` public type.

use {
    crate::Predicate,
    crate::RcPredicate,
    crate::RcTransformer,
    crate::Transformer,
    crate::transformers::macros::impl_conditional_transformer_clone,
    crate::transformers::macros::impl_conditional_transformer_debug_display,
    crate::transformers::macros::impl_shared_conditional_transformer,
};

// ============================================================================
// RcConditionalTransformer - Rc-based Conditional Transformer
// ============================================================================

/// RcConditionalTransformer struct
///
/// A single-threaded conditional transformer that only executes when a
/// predicate is satisfied. Uses `RcTransformer` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Ownership Cost**: `Rc` uses non-atomic reference counting; `Arc` uses
///   atomic reference counting, and neither wrapper locks callback invocation
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Transformer, RcTransformer};
///
/// let double = RcTransformer::new(|x: i32| x * 2);
/// let identity = RcTransformer::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional_clone.apply(-5), -5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalTransformer<T, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: RcTransformer<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

// Implement RcConditionalTransformer
impl_shared_conditional_transformer!(
    RcConditionalTransformer<T, R>,
    RcTransformer,
    Transformer,
    callback_bounds = ('static)
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalTransformer<T, R>);

// Implement Clone for RcConditionalTransformer
impl_conditional_transformer_clone!(RcConditionalTransformer<T, R>);
