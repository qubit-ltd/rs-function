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
//! Defines the `RcConditionalStatefulConsumer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 9. RcConditionalStatefulConsumer - Rc-based Conditional Consumer
// ============================================================================

/// RcConditionalStatefulConsumer struct
///
/// A single-threaded conditional consumer that only executes when a predicate is
/// satisfied. Uses `RcStatefulConsumer` and `RcPredicate` for shared ownership within a
/// single thread.
///
/// This type is typically created by calling `RcStatefulConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulConsumer`
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
///
pub struct RcConditionalStatefulConsumer<T> {
    pub(super) consumer: RcStatefulConsumer<T>,
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate and_then and or_else methods
impl_shared_conditional_consumer!(
    RcConditionalStatefulConsumer<T>,
    RcStatefulConsumer,
    StatefulConsumer,
    into_rc,
    'static
);

impl<T> StatefulConsumer<T> for RcConditionalStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(BoxStatefulConsumer<T>, RcStatefulConsumer, FnMut);
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalStatefulConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalStatefulConsumer<T>);
