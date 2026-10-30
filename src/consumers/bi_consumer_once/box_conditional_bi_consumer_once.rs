// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalBiConsumerOnce` public type.

use crate::BiConsumerOnce;
use crate::BiPredicate;
use crate::BoxBiConsumerOnce;
use crate::BoxBiPredicate;
use crate::consumers::macros::impl_box_conditional_consumer;
use crate::consumers::macros::impl_conditional_consumer_debug_display;

// =======================================================================
// 5. BoxConditionalBiConsumerOnce - Box-based Conditional BiConsumerOnce
// =======================================================================

/// A one-time bi-consumer that invokes its callback only when its predicate
/// accepts both input values.
///
/// The callback and predicate are owned by this value. Calling `accept`
/// consumes the conditional consumer, whether or not the predicate matches.
/// Construct it by calling [`BoxBiConsumerOnce::when`], and use `or_else` to
/// add a fallback callback.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` consumes `self`
/// - **Conditional Execution**: Consumes the wrapper on every call; invokes the
///   callback only when the predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiConsumerOnce**: Calling `accept` consumes the wrapper
///
/// # Type Parameters
///
/// - `T` is the type of the first input borrowed by the predicate and callback.
/// - `U` is the type of the second input borrowed by the predicate and
///   callback.
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let consumer = BoxBiConsumerOnce::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// conditional.accept(&5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]); // Executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let consumer = BoxBiConsumerOnce::new(move |x: &i32, y: &i32| {
///     l1.lock().expect("mutex should not be poisoned").push(*x + *y);
/// }).when(|x: &i32, y: &i32| *x > 0 && *y > 0)
///   .or_else(move |x: &i32, y: &i32| {
///     l2.lock().expect("mutex should not be poisoned").push(*x * *y);
/// });
///
/// consumer.accept(&5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]); // when branch executed
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalBiConsumerOnce<T, U> {
    /// The wrapped consumer callback.
    pub(super) consumer: BoxBiConsumerOnce<T, U>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Generate and_then and or_else methods using macro
impl_box_conditional_consumer!(
    BoxConditionalBiConsumerOnce<T, U>,
    BoxBiConsumerOnce,
    BiConsumerOnce
);

impl<T, U> BiConsumerOnce<T, U> for BoxConditionalBiConsumerOnce<T, U> {
    /// Consumes this wrapper and invokes its callback when the predicate
    /// matches.
    ///
    /// If the predicate returns `false`, the callback is dropped without being
    /// invoked.
    fn accept(self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalBiConsumerOnce<T, U>);
