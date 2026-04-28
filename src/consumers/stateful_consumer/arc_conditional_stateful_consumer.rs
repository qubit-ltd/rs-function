/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalStatefulConsumer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 8. ArcConditionalStatefulConsumer - Arc-based Conditional Consumer
// ============================================================================

/// ArcConditionalStatefulConsumer struct
///
/// A thread-safe conditional consumer that only executes when a predicate is
/// satisfied. Uses `ArcStatefulConsumer` and `ArcPredicate` for shared ownership across
/// threads.
///
/// This type is typically created by calling `ArcStatefulConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, ArcStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let conditional = ArcStatefulConsumer::new(move |x: &i32| {
///     l.lock().unwrap().push(*x);
/// })
/// .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.accept(&value);
/// assert_eq!(*log.lock().unwrap(), vec![5]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulConsumer<T> {
    pub(super) consumer: ArcStatefulConsumer<T>,
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate and_then and or_else methods
impl_shared_conditional_consumer!(
    ArcConditionalStatefulConsumer<T>,
    ArcStatefulConsumer,
    StatefulConsumer,
    into_arc,
    Send + Sync + 'static
);

impl<T> StatefulConsumer<T> for ArcConditionalStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        if self.predicate.test(value) {
            self.consumer.accept(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(BoxStatefulConsumer<T>, RcStatefulConsumer, FnMut);
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(ArcConditionalStatefulConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(ArcConditionalStatefulConsumer<T>);
