/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalStatefulBiConsumer` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 8. ArcConditionalStatefulBiConsumer - Arc-based Conditional BiConsumer
// =======================================================================

/// ArcConditionalStatefulBiConsumer struct
///
/// A thread-safe conditional bi-consumer that only executes when a predicate is
/// satisfied. Uses `ArcStatefulBiConsumer` and `ArcBiPredicate` for shared ownership across
/// threads.
///
/// This type is typically created by calling `ArcStatefulBiConsumer::when()` and is
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
/// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let conditional = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.lock().unwrap().push(*x + *y);
/// }).when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.accept(&value, &3);
/// assert_eq!(*log.lock().unwrap(), vec![8]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulBiConsumer<T, U> {
    pub(super) consumer: ArcStatefulBiConsumer<T, U>,
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Use macro to generate and_then and or_else methods
impl_shared_conditional_consumer!(
    ArcConditionalStatefulBiConsumer<T, U>,
    ArcStatefulBiConsumer,
    StatefulBiConsumer,
    into_arc,
    Send + Sync + 'static
);

impl<T, U> StatefulBiConsumer<T, U> for ArcConditionalStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(
        BoxStatefulBiConsumer<T, U>,
        RcStatefulBiConsumer,
        FnMut
    );
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(ArcConditionalStatefulBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(ArcConditionalStatefulBiConsumer<T, U>);
