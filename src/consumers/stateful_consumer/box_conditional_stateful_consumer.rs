// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalStatefulConsumer` public type.

use crate::BoxPredicate;
use crate::BoxStatefulConsumer;
use crate::Predicate;
use crate::StatefulConsumer;
use crate::consumers::macros::impl_box_conditional_consumer;
use crate::consumers::macros::impl_conditional_consumer_debug_display;

// ============================================================================
// 7. BoxConditionalStatefulConsumer - Box-based Conditional Consumer
// ============================================================================

/// BoxConditionalStatefulConsumer struct
///
/// A conditional consumer that only executes when a predicate is satisfied.
/// Uses `BoxStatefulConsumer` and `BoxPredicate` for single ownership
/// semantics.
///
/// This type is typically created by calling `BoxStatefulConsumer::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` borrows `&mut self`
/// - **Conditional Execution**: Calls the callback only when the predicate
///   returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements StatefulConsumer**: Can be used where a `StatefulConsumer` is
///   expected
///
/// The conditional wrapper is reusable; `accept` mutably borrows it and
/// borrows the input by shared reference.
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use std::sync::Arc;
/// use std::sync::Mutex;
///
/// use qubit_function::BoxConditionalStatefulConsumer;
/// use qubit_function::BoxStatefulConsumer;
/// use qubit_function::Consumer;
/// use qubit_function::StatefulConsumer;
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let consumer = BoxStatefulConsumer::new(move |x: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x);
/// });
/// let mut conditional: BoxConditionalStatefulConsumer<i32> =
///     consumer.when(|x: &i32| *x > 0);
///
/// conditional.accept(&5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]); // Executed
///
/// conditional.accept(&-5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]); // Not executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use std::sync::Arc;
/// use std::sync::Mutex;
///
/// use qubit_function::BoxConditionalStatefulConsumer;
/// use qubit_function::BoxStatefulConsumer;
/// use qubit_function::Consumer;
/// use qubit_function::StatefulConsumer;
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let mut consumer: BoxStatefulConsumer<i32> =
///     BoxStatefulConsumer::new(move |x: &i32| {
///         l1.lock().expect("mutex should not be poisoned").push(*x);
///     })
///     .when(|x: &i32| *x > 0)
///     .or_else(move |x: &i32| {
///         l2.lock().expect("mutex should not be poisoned").push(-*x);
///     });
///
/// consumer.accept(&5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]); // when branch executed
///
/// consumer.accept(&-5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5, 5]); // or_else branch executed
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalStatefulConsumer<T> {
    /// The wrapped consumer callback.
    pub(super) consumer: BoxStatefulConsumer<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate and_then and or_else methods
impl_box_conditional_consumer!(BoxConditionalStatefulConsumer<T>, BoxStatefulConsumer, StatefulConsumer);

impl<T> StatefulConsumer<T> for BoxConditionalStatefulConsumer<T> {
    /// Forwards the borrowed value to the wrapped consumer when the predicate
    /// matches.
    fn accept(&mut self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalStatefulConsumer<T>);
