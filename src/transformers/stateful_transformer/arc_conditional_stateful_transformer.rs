// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalStatefulTransformer` public type.

use crate::ArcPredicate;
use crate::ArcStatefulTransformer;
use crate::Predicate;
use crate::StatefulTransformer;
use crate::transformers::macros::impl_conditional_transformer_clone;
use crate::transformers::macros::impl_conditional_transformer_debug_display;
use crate::transformers::macros::impl_shared_conditional_transformer;

// ============================================================================
// ArcConditionalStatefulTransformer - Arc-based Conditional StatefulTransformer
// ============================================================================

/// ArcConditionalStatefulTransformer struct
///
/// A thread-safe conditional transformer that only executes when a predicate
/// is satisfied. Uses `ArcStatefulTransformer` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulTransformer::when()`
/// and is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send`, safe for concurrent use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
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
/// # Locking and reentrancy
///
/// When the wrapped stateful callback executes, the underlying
/// `parking_lot::Mutex` remains locked until that callback returns.
/// Synchronous re-entry through the same shared state deadlocks. The mutex is
/// not poisoned after a panic, and mutations completed before a panic are not
/// rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalStatefulTransformer<T, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: ArcStatefulTransformer<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcPredicate<T>,
}

// Implement ArcConditionalStatefulTransformer
impl_shared_conditional_transformer!(
    ArcConditionalStatefulTransformer<T, R>,
    ArcStatefulTransformer,
    StatefulTransformer,
    callback_bounds = (Send + 'static)
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(ArcConditionalStatefulTransformer<T, R>);

// Implement Clone for ArcConditionalStatefulTransformer
impl_conditional_transformer_clone!(ArcConditionalStatefulTransformer<T, R>);
