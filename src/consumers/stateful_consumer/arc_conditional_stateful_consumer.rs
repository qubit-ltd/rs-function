// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalStatefulConsumer` public type.

use crate::ArcPredicate;
use crate::ArcStatefulConsumer;
use crate::Predicate;
use crate::StatefulConsumer;
use crate::consumers::macros::impl_conditional_consumer_clone;
use crate::consumers::macros::impl_conditional_consumer_debug_display;
use crate::consumers::macros::impl_shared_conditional_consumer;

// ============================================================================
// 8. ArcConditionalStatefulConsumer - Arc-based Conditional Consumer
// ============================================================================

/// ArcConditionalStatefulConsumer struct
///
/// A thread-safe conditional consumer that only executes when a predicate is
/// satisfied. Uses `ArcStatefulConsumer` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulConsumer::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, ArcStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let conditional = ArcStatefulConsumer::new(move |x: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x);
/// })
/// .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.accept(&value);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
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
pub struct ArcConditionalStatefulConsumer<T> {
    /// The wrapped consumer callback.
    pub(super) consumer: ArcStatefulConsumer<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate and_then and or_else methods
impl_shared_conditional_consumer!(
    ArcConditionalStatefulConsumer<T>,
    ArcStatefulConsumer,
    StatefulConsumer,
    callback_bounds = (Send + 'static)
);

impl<T> StatefulConsumer<T> for ArcConditionalStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(ArcConditionalStatefulConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(ArcConditionalStatefulConsumer<T>);
