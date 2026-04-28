/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcConditionalConsumer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 8. RcConditionalConsumer - Rc-based Conditional Consumer
// ============================================================================

/// RcConditionalConsumer struct
///
/// A conditional non-mutating consumer that only executes when a predicate is satisfied.
/// Uses `RcConsumer` and `RcPredicate` for single-threaded shared ownership semantics.
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
/// - **Non-mutating**: Neither modifies itself nor input values
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
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalConsumer<T> {
    pub(super) consumer: RcConsumer<T>,
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate conditional consumer implementations
impl_shared_conditional_consumer!(
    RcConditionalConsumer<T>,
    RcConsumer,
    Consumer,
    into_rc,
    'static
);

// Hand-written Consumer trait implementation
impl<T> Consumer<T> for RcConditionalConsumer<T> {
    fn accept(&self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(BoxConsumer<T>, RcConsumer, Fn);
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalConsumer<T>);
