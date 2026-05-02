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
//! Defines the `BoxConditionalBiConsumer` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 7. BoxConditionalBiConsumer - Box-based Conditional BiConsumer
// =======================================================================

/// BoxConditionalBiConsumer struct
///
/// A conditional non-mutating bi-consumer that only executes when a predicate is satisfied.
/// Uses `BoxBiConsumer` and `BoxBiPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxBiConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiConsumer**: Can be used anywhere a `BiConsumer` is expected
/// - **Non-mutating**: Neither modifies itself nor input values
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
///     println!("Both positive: {} + {} = {}", x, y, x + y);
/// });
/// let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// conditional.accept(&5, &3);  // Prints: Both positive: 5 + 3 = 8
/// conditional.accept(&-5, &3); // Does nothing
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
///
pub struct BoxConditionalBiConsumer<T, U> {
    pub(super) consumer: BoxBiConsumer<T, U>,
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
    fn accept(&self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(
        BoxBiConsumer<T, U>,
        RcBiConsumer,
        Fn
    );
}

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(BoxConditionalBiConsumer<T, U>);
