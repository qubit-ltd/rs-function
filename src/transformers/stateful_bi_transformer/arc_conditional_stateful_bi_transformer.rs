// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalStatefulBiTransformer` public type.

use crate::ArcBiPredicate;
use crate::ArcStatefulBiTransformer;
use crate::BiPredicate;
use crate::StatefulBiTransformer;
use crate::transformers::macros::impl_conditional_transformer_clone;
use crate::transformers::macros::impl_conditional_transformer_debug_display;
use crate::transformers::macros::impl_shared_conditional_transformer;

// ============================================================================
// ArcConditionalStatefulBiTransformer - Arc-based Conditional
// StatefulBiTransformer
// ============================================================================

/// ArcConditionalStatefulBiTransformer struct
///
/// A thread-safe conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `ArcStatefulBiTransformer` and
/// `ArcBiPredicate` for shared ownership across threads.
///
/// This type is typically created by calling `ArcStatefulBiTransformer::when()`
/// and is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when bi-predicate returns
///   `true`
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
/// # Locking and reentrancy
///
/// When the wrapped stateful callback executes, the underlying
/// `parking_lot::Mutex` remains locked until that callback returns.
/// Synchronous re-entry through the same shared state deadlocks. The mutex is
/// not poisoned after a panic, and mutations completed before a panic are not
/// rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalStatefulBiTransformer<T, U, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: ArcStatefulBiTransformer<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcBiPredicate<T, U>,
}

impl_shared_conditional_transformer!(
    ArcConditionalStatefulBiTransformer<T, U, R>,
    ArcStatefulBiTransformer,
    StatefulBiTransformer,
    callback_bounds = (Send + 'static)
);

// Implement Debug and Display for ArcConditionalStatefulBiTransformer
impl_conditional_transformer_debug_display!(ArcConditionalStatefulBiTransformer<T, U, R>);

// Implement Clone for ArcConditionalStatefulBiTransformer
impl_conditional_transformer_clone!(ArcConditionalStatefulBiTransformer<T, U, R>);
