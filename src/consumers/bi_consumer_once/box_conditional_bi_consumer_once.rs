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
//! Defines the `BoxConditionalBiConsumerOnce` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 5. BoxConditionalBiConsumerOnce - Box-based Conditional BiConsumerOnce
// =======================================================================

/// BoxConditionalBiConsumerOnce struct
///
/// A conditional one-time bi-consumer that only executes when a predicate is satisfied.
/// Uses `BoxBiConsumerOnce` and `BoxBiPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxBiConsumerOnce::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiConsumerOnce**: Can be used anywhere a `BiConsumerOnce` is expected
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
///     l.lock().unwrap().push(*x + *y);
/// });
/// let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// conditional.accept(&5, &3);
/// assert_eq!(*log.lock().unwrap(), vec![8]); // Executed
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
///     l1.lock().unwrap().push(*x + *y);
/// }).when(|x: &i32, y: &i32| *x > 0 && *y > 0)
///   .or_else(move |x: &i32, y: &i32| {
///     l2.lock().unwrap().push(*x * *y);
/// });
///
/// consumer.accept(&5, &3);
/// assert_eq!(*log.lock().unwrap(), vec![8]); // when branch executed
/// ```
///
pub struct BoxConditionalBiConsumerOnce<T, U> {
    pub(super) consumer: BoxBiConsumerOnce<T, U>,
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Generate and_then and or_else methods using macro
impl_box_conditional_consumer!(
    BoxConditionalBiConsumerOnce<T, U>,
    BoxBiConsumerOnce,
    BiConsumerOnce
);

impl<T, U> BiConsumerOnce<T, U> for BoxConditionalBiConsumerOnce<T, U> {
    fn accept(self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }

    fn into_fn(self) -> impl FnOnce(&T, &U) {
        let pred = self.predicate;
        let consumer = self.consumer;
        move |t: &T, u: &U| {
            if pred.test(t, u) {
                consumer.accept(t, u);
            }
        }
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalBiConsumerOnce<T, U>);
