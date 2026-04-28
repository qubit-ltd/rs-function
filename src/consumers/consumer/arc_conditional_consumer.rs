/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalConsumer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 9. ArcConditionalConsumer - Arc-based Conditional Consumer
// ============================================================================

/// ArcConditionalConsumer struct
///
/// A conditional non-mutating consumer that only executes when a predicate is satisfied.
/// Uses `ArcConsumer` and `ArcPredicate` for thread-safe shared ownership semantics.
///
/// This type is typically created by calling `ArcConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Arc`, allows multiple owners
/// - **Thread Safe**: Implements `Send + Sync`, can be safely used concurrently
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
/// use qubit_function::{Consumer, ArcConsumer};
///
/// let consumer = ArcConsumer::new(|x: &i32| {
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
/// use qubit_function::{Consumer, ArcConsumer};
///
/// let consumer = ArcConsumer::new(|x: &i32| {
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
pub struct ArcConditionalConsumer<T> {
    pub(super) consumer: ArcConsumer<T>,
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional consumer implementations
impl_shared_conditional_consumer!(
    ArcConditionalConsumer<T>,
    ArcConsumer,
    Consumer,
    into_arc,
    Send + Sync + 'static
);

// Hand-written Consumer trait implementation
impl<T> Consumer<T> for ArcConditionalConsumer<T> {
    fn accept(&self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(BoxConsumer<T>, RcConsumer, Fn);
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(ArcConditionalConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(ArcConditionalConsumer<T>);
