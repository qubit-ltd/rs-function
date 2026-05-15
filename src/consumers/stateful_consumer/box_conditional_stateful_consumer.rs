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
//! Defines the `BoxConditionalStatefulConsumer` public type.

use super::{
    BoxPredicate,
    BoxStatefulConsumer,
    Predicate,
    RcStatefulConsumer,
    StatefulConsumer,
    impl_box_conditional_consumer,
    impl_conditional_consumer_conversions,
    impl_conditional_consumer_debug_display,
};

// ============================================================================
// 7. BoxConditionalStatefulConsumer - Box-based Conditional Consumer
// ============================================================================

/// BoxConditionalStatefulConsumer struct
///
/// A conditional consumer that only executes when a predicate is satisfied.
/// Uses `BoxStatefulConsumer` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Consumer**: Can be used anywhere a `Consumer` is expected
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, BoxStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let consumer = BoxStatefulConsumer::new(move |x: &i32| {
///     l.lock().unwrap().push(*x);
/// });
/// let mut conditional = consumer.when(|x: &i32| *x > 0);
///
/// conditional.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![5]); // Executed
///
/// conditional.accept(&-5);
/// assert_eq!(*log.lock().unwrap(), vec![5]); // Not executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, BoxStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
///     l1.lock().unwrap().push(*x);
/// })
/// .when(|x: &i32| *x > 0)
/// .or_else(move |x: &i32| {
///     l2.lock().unwrap().push(-*x);
/// });
///
/// consumer.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![5]); // when branch executed
///
/// consumer.accept(&-5);
/// assert_eq!(*log.lock().unwrap(), vec![5, 5]); // or_else branch executed
/// ```
///
pub struct BoxConditionalStatefulConsumer<T> {
    pub(super) consumer: BoxStatefulConsumer<T>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate and_then and or_else methods
impl_box_conditional_consumer!(
    BoxConditionalStatefulConsumer<T>,
    BoxStatefulConsumer,
    StatefulConsumer
);

impl<T> StatefulConsumer<T> for BoxConditionalStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(BoxStatefulConsumer<T>, RcStatefulConsumer, FnMut);
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalStatefulConsumer<T>);
