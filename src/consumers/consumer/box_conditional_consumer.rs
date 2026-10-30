// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalConsumer` public type.

use crate::BoxConsumer;
use crate::BoxPredicate;
use crate::Consumer;
use crate::Predicate;
use crate::consumers::macros::impl_box_conditional_consumer;
use crate::consumers::macros::impl_conditional_consumer_debug_display;

// ============================================================================
// 7. BoxConditionalConsumer - Box-based Conditional Consumer
// ============================================================================

/// A conditional non-mutating consumer that only executes when a predicate is
/// satisfied. Uses `BoxConsumer` and `BoxPredicate` for single ownership
/// semantics.
///
/// This type is typically created by calling `BoxConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` borrows `&self`
/// - **Conditional Execution**: Calls the callback only when the predicate
///   returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Shared-reference calls**: `accept` borrows `&self` and the input through
///   a shared reference
/// - **Implements Consumer**: Can be used anywhere a `Consumer` is expected
/// - **Shared access**: Receives `&self` and `&T`; interior mutability and
///   external side effects remain possible
///
/// # Type Parameters
///
/// * `T` - The input type borrowed by the predicate and consumer callbacks.
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use std::cell::Cell;
/// use std::rc::Rc;
/// use qubit_function::{BoxConsumer, Consumer};
///
/// let observed = Rc::new(Cell::new(0));
/// let observed_by_consumer = Rc::clone(&observed);
/// let consumer = BoxConsumer::new(move |x: &i32| {
///     observed_by_consumer.set(*x);
/// });
/// let conditional = consumer.when(|x: &i32| *x > 0);
///
/// conditional.accept(&5);
/// assert_eq!(observed.get(), 5);
/// conditional.accept(&-5);
/// assert_eq!(observed.get(), 5);
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use std::cell::Cell;
/// use std::rc::Rc;
/// use qubit_function::{BoxConsumer, Consumer};
///
/// let observed = Rc::new(Cell::new(0));
/// let observed_by_consumer = Rc::clone(&observed);
/// let observed_by_fallback = Rc::clone(&observed);
/// let consumer = BoxConsumer::new(move |x: &i32| {
///     observed_by_consumer.set(*x);
/// })
/// .when(|x: &i32| *x > 0)
/// .or_else(move |x: &i32| {
///     observed_by_fallback.set(*x);
/// });
///
/// consumer.accept(&5);
/// assert_eq!(observed.get(), 5);
/// consumer.accept(&-5);
/// assert_eq!(observed.get(), -5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalConsumer<T> {
    /// The wrapped consumer callback.
    pub(super) consumer: BoxConsumer<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional consumer implementations
impl_box_conditional_consumer!(BoxConditionalConsumer<T>, BoxConsumer, Consumer);

// Consumer trait implementation
impl<T> Consumer<T> for BoxConditionalConsumer<T> {
    fn accept(&self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalConsumer<T>);
