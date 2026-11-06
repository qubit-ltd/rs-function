// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalConsumer` public type.

use crate::Consumer;
use crate::Predicate;
use crate::RcConsumer;
use crate::RcPredicate;
use crate::consumers::macros::impl_conditional_consumer_clone;
use crate::consumers::macros::impl_conditional_consumer_debug_display;

// ============================================================================
// 8. RcConditionalConsumer - Rc-based Conditional Consumer
// ============================================================================

/// A single-threaded shared consumer that runs only when its predicate accepts
/// the input.
///
/// It shares its callback and predicate through `Rc`, and can be created with
/// [`RcConsumer::when`]. Use [`or_else`](Self::or_else) to add a callback for
/// inputs rejected by the predicate. Since it uses `Rc`, this type is not
/// thread-safe and cannot be sent between threads.
///
/// This type is typically created by calling `RcConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Rc`, allows multiple owners
/// - **Single-threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
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
/// use qubit_function::{Consumer, RcConsumer};
///
/// let consumer = RcConsumer::new(|x: &i32| {
///     println!("Positive: {}", x);
/// });
/// let conditional = consumer.when(|x: &i32| *x > 0);
///
/// conditional.accept(&5);  // Prints: Positive: 5
/// conditional.accept(&-5); // Does nothing
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{Consumer, RcConsumer};
///
/// let consumer = RcConsumer::new(|x: &i32| {
///     println!("Positive: {}", x);
/// })
/// .when(|x: &i32| *x > 0)
/// .or_else(|x: &i32| {
///     println!("Non-positive: {}", x);
/// });
///
/// consumer.accept(&5);  // Prints: Positive: 5
/// consumer.accept(&-5); // Prints: Non-positive: -5
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalConsumer<T> {
    /// The wrapped consumer callback.
    pub(super) consumer: RcConsumer<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

impl<T> RcConditionalConsumer<T> {
    /// Chains another consumer after this conditional consumer.
    ///
    /// The original consumer runs only when its predicate accepts the input;
    /// `next` runs afterward for every input.
    ///
    /// # Type Parameters
    ///
    /// * `C` - Type of the consumer to run after this conditional consumer.
    ///
    /// # Parameters
    ///
    /// * `next` - Consumer that always runs after the conditional action.
    ///
    /// # Returns
    ///
    /// A consumer that applies the conditional action and then `next`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use std::rc::Rc;
    /// use qubit_function::{Consumer, RcConsumer};
    ///
    /// let calls = Rc::new(Cell::new(0));
    /// let observed = Rc::clone(&calls);
    /// let first = RcConsumer::new(|_: &i32| {});
    /// let conditional = first.when(|value: &i32| *value > 0);
    /// let next = RcConsumer::new(move |_: &i32| observed.set(observed.get() + 1));
    /// let combined = conditional.and_then(next);
    /// combined.accept(&-1);
    /// assert_eq!(calls.get(), 1);
    /// ```
    pub fn and_then<C>(&self, next: C) -> RcConsumer<T>
    where
        T: 'static,
        C: Consumer<T> + 'static,
    {
        let first_predicate = self.predicate.clone();
        let first_consumer = self.consumer.clone();
        RcConsumer::new(move |value: &T| {
            if first_predicate.test(value) {
                first_consumer.accept(value);
            }
            next.accept(value);
        })
    }

    /// Adds a fallback consumer for inputs rejected by this consumer's
    /// predicate.
    ///
    /// # Type Parameters
    ///
    /// * `C` - Type of the fallback consumer.
    ///
    /// # Parameters
    ///
    /// * `else_consumer` - Consumer to run when the predicate rejects the
    ///   input.
    ///
    /// # Returns
    ///
    /// A consumer that runs this wrapper's callback or the fallback callback.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use std::rc::Rc;
    /// use qubit_function::{Consumer, RcConsumer};
    ///
    /// let observed = Rc::new(Cell::new(0));
    /// let fallback_observed = Rc::clone(&observed);
    /// let first = RcConsumer::new(|_: &i32| {});
    /// let conditional = first.when(|value: &i32| *value > 0);
    /// let fallback = RcConsumer::new(move |value: &i32| fallback_observed.set(*value));
    /// let combined = conditional.or_else(fallback);
    /// combined.accept(&-1);
    /// assert_eq!(observed.get(), -1);
    /// ```
    pub fn or_else<C>(&self, else_consumer: C) -> RcConsumer<T>
    where
        T: 'static,
        C: Consumer<T> + 'static,
    {
        let predicate = self.predicate.clone();
        let then_consumer = self.consumer.clone();
        RcConsumer::new(move |value: &T| {
            if predicate.test(value) {
                then_consumer.accept(value);
            } else {
                else_consumer.accept(value);
            }
        })
    }
}

// Hand-written Consumer trait implementation
impl<T> Consumer<T> for RcConditionalConsumer<T> {
    /// Runs the wrapped consumer only when the predicate accepts `value`.
    ///
    /// The input is borrowed, and a rejected value leaves the wrapped consumer
    /// untouched. Calling through `&self` preserves the shared, single-threaded
    /// access model of this `Rc`-backed wrapper.
    ///
    /// # Parameters
    ///
    /// * `value` - Input borrowed by the predicate and, when accepted, by the
    ///   wrapped consumer.
    fn accept(&self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalConsumer<T>);
