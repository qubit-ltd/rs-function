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
//! Defines the `BoxConditionalStatefulBiConsumer` public type.

use super::{
    BiPredicate,
    BoxBiPredicate,
    BoxStatefulBiConsumer,
    RcStatefulBiConsumer,
    StatefulBiConsumer,
    impl_box_conditional_consumer,
    impl_conditional_consumer_conversions,
    impl_conditional_consumer_debug_display,
};

// =======================================================================
// 7. BoxConditionalBiConsumer - Box-based Conditional BiConsumer
// =======================================================================

/// BoxConditionalBiConsumer struct
///
/// A conditional bi-consumer that only executes when a predicate is satisfied.
/// Uses `BoxStatefulBiConsumer` and `BoxBiPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulBiConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
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
///
pub struct BoxConditionalStatefulBiConsumer<T, U> {
    pub(super) consumer: BoxStatefulBiConsumer<T, U>,
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

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(
        BoxStatefulBiConsumer<T, U>,
        RcStatefulBiConsumer,
        FnMut
    );
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalStatefulBiConsumer<T, U>);
