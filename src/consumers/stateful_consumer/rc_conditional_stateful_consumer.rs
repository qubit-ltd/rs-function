// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalStatefulConsumer` public type.

use crate::Predicate;
use crate::RcPredicate;
use crate::RcStatefulConsumer;
use crate::StatefulConsumer;
use crate::consumers::macros::impl_conditional_consumer_clone;
use crate::consumers::macros::impl_conditional_consumer_debug_display;
use crate::consumers::macros::impl_shared_conditional_consumer;

// ============================================================================
// 9. RcConditionalStatefulConsumer - Rc-based Conditional Consumer
// ============================================================================

/// Runs its callback only when an input satisfies a predicate, sharing both
/// through `Rc` within a single thread.
///
/// Clones share the `RcStatefulConsumer` callback and `RcPredicate` condition.
///
/// This type is typically created by calling `RcStatefulConsumer::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Callback sharing**: Uses `Rc` and checks mutable callback borrows with
///   `RefCell`; it does not use a mutex.
///
/// # Type Parameters
///
/// * `T` - The input type borrowed by the predicate and consumer callback.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, RcStatefulConsumer};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let log = Rc::new(RefCell::new(Vec::new()));
/// let l = log.clone();
/// let conditional = RcStatefulConsumer::new(move |x: &i32| {
///     l.borrow_mut().push(*x);
/// })
/// .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.accept(&value);
/// assert_eq!(*log.borrow(), vec![5]);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalStatefulConsumer<T> {
    /// The wrapped consumer callback.
    pub(super) consumer: RcStatefulConsumer<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate and_then and or_else methods
impl_shared_conditional_consumer!(
    RcConditionalStatefulConsumer<T>,
    RcStatefulConsumer,
    StatefulConsumer,
    callback_bounds = ('static)
);

impl<T> StatefulConsumer<T> for RcConditionalStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalStatefulConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalStatefulConsumer<T>);
