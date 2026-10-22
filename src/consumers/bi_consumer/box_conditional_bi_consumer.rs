// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalBiConsumer` public type.

use crate::BiConsumer;
use crate::BiPredicate;
use crate::BoxBiConsumer;
use crate::BoxBiPredicate;
use crate::consumers::macros::impl_box_conditional_consumer;
use crate::consumers::macros::impl_conditional_consumer_debug_display;

// =======================================================================
// 7. BoxConditionalBiConsumer - Box-based Conditional BiConsumer
// =======================================================================

/// A single-owner bi-consumer that invokes its callback only when its
/// predicate accepts both borrowed inputs. The callback and predicate are
/// stored in boxes, and the wrapper does not provide shared ownership.
///
/// This type is typically created by calling `BoxBiConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
/// - **Shared access**: Receives shared references; interior mutability and
///   external side effects remain possible
///
/// # Examples
///
/// ## Conditional Execution
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// use std::cell::Cell;
/// use std::rc::Rc;
///
/// let total = Rc::new(Cell::new(0));
/// let callback_total = Rc::clone(&total);
/// let consumer = BoxBiConsumer::new(move |x: &i32, y: &i32| {
///     callback_total.set(x + y);
/// });
/// let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// conditional.accept(&5, &3);
/// assert_eq!(total.get(), 8);
/// conditional.accept(&-5, &3);
/// assert_eq!(total.get(), 8); // The callback is skipped.
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
///     println!("Both positive: {} + {} = {}", x, y, x + y);
/// })
/// .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
/// .or_else(|x: &i32, y: &i32| {
///     println!("Not both positive: {} and {}", x, y);
/// });
///
/// consumer.accept(&5, &3);  // Prints: Both positive: 5 + 3 = 8
/// consumer.accept(&-5, &3); // Prints: Not both positive: -5 and 3
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalBiConsumer<T, U> {
    /// The wrapped consumer callback.
    pub(super) consumer: BoxBiConsumer<T, U>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Use macro to generate conditional bi-consumer implementations
impl_box_conditional_consumer!(
    BoxConditionalBiConsumer<T, U>,
    BoxBiConsumer,
    BiConsumer
);

// Hand-written BiConsumer trait implementation
impl<T, U> BiConsumer<T, U> for BoxConditionalBiConsumer<T, U> {
    /// Invokes the wrapped callback when the predicate accepts both inputs.
    fn accept(&self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalBiConsumer<T, U>);
