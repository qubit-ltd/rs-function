// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalBiConsumer` public type.

use crate::BiConsumer;
use crate::BiPredicate;
use crate::RcBiConsumer;
use crate::RcBiPredicate;
use crate::consumers::macros::impl_conditional_consumer_clone;
use crate::consumers::macros::impl_conditional_consumer_debug_display;
use crate::consumers::macros::impl_shared_conditional_consumer;

// =======================================================================
// 9. RcConditionalBiConsumer - Rc-based Conditional BiConsumer
// =======================================================================

/// A conditional bi-consumer that shares its callback and predicate through
/// `Rc`, invoking the callback only when the predicate accepts both inputs.
/// The callback and predicate must be `'static`; this single-threaded wrapper
/// is not thread-safe.
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Rc`, allows multiple owners
/// - **Single-Threaded**: Uses non-atomic `Rc`; choose `Arc` when thread-safe
///   sharing is required, and benchmark the real workload when cost matters
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
/// - **Shared access**: Receives shared references; interior mutability and
///   external side effects remain possible
///
/// # Examples
///
/// ```rust
/// use std::cell::Cell;
/// use std::rc::Rc;
///
/// use qubit_function::BiConsumer;
/// use qubit_function::RcConditionalBiConsumer;
/// use qubit_function::RcBiConsumer;
///
/// let total = Rc::new(Cell::new(0));
/// let callback_total = Rc::clone(&total);
/// let consumer: RcConditionalBiConsumer<i32, i32> =
///     RcBiConsumer::new(move |first: &i32, second: &i32| {
///         callback_total.set(callback_total.get() + first + second);
///     })
///     .when(|first: &i32, second: &i32| *first > 0 && *second > 0);
///
/// consumer.accept(&2, &3);
/// consumer.accept(&-1, &5);
/// assert_eq!(total.get(), 5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalBiConsumer<T, U> {
    /// The wrapped consumer callback.
    pub(super) consumer: RcBiConsumer<T, U>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Use macro to generate conditional bi-consumer implementations
impl_shared_conditional_consumer!(
    RcConditionalBiConsumer<T, U>,
    RcBiConsumer,
    BiConsumer,
    callback_bounds = ('static)
);

// Hand-written BiConsumer trait implementation
impl<T, U> BiConsumer<T, U> for RcConditionalBiConsumer<T, U> {
    /// Invokes the wrapped callback when the predicate accepts both inputs.
    fn accept(&self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalBiConsumer<T, U>);
