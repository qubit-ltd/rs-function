// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalStatefulBiConsumer` public type.

use crate::BiPredicate;
use crate::BoxBiPredicate;
use crate::BoxStatefulBiConsumer;
use crate::StatefulBiConsumer;
use crate::consumers::macros::impl_box_conditional_consumer;
use crate::consumers::macros::impl_conditional_consumer_debug_display;

// =======================================================================
// 7. BoxConditionalStatefulBiConsumer - Box-based Conditional Stateful
//    BiConsumer
// =======================================================================

/// A single-owner stateful bi-consumer that invokes its callback only when its
/// predicate accepts both borrowed inputs. The callback can update its own
/// captured state, while the wrapper stores the callback and predicate in
/// boxes without shared ownership.
///
/// This type is typically created by calling `BoxStatefulBiConsumer::when()`
/// and can be chained with `or_else()` to create if-then-else logic.
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
///
/// # Features
///
/// - **Single Ownership**: Owns its callback and predicate without shared
///   ownership
/// - **Conditional Execution**: Invokes the callback only when the predicate
///   returns `true`
/// - **Stateful Callback**: The callback may update its captured state between
///   calls
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements StatefulBiConsumer**: Can be used wherever that trait is
///   expected
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxStatefulBiConsumer, StatefulBiConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let mut consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// let mut conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// conditional.accept(&5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]); // Executed
///
/// conditional.accept(&-5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]); // Not executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxStatefulBiConsumer, StatefulBiConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let mut consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l1.lock().expect("mutex should not be poisoned").push(*x + *y);
/// }).when(|x: &i32, y: &i32| *x > 0 && *y > 0)
///   .or_else(move |x: &i32, y: &i32| {
///     l2.lock().expect("mutex should not be poisoned").push(*x * *y);
/// });
///
/// consumer.accept(&5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]); // when branch executed
///
/// consumer.accept(&-5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, -15]); // or_else branch executed
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalStatefulBiConsumer<T, U> {
    /// The wrapped consumer callback.
    pub(super) consumer: BoxStatefulBiConsumer<T, U>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Use macro to generate conditional bi-consumer implementations
impl_box_conditional_consumer!(
    BoxConditionalStatefulBiConsumer<T, U>,
    BoxStatefulBiConsumer,
    StatefulBiConsumer
);

impl<T, U> StatefulBiConsumer<T, U> for BoxConditionalStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalStatefulBiConsumer<T, U>);
