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
//! Defines the `BoxStatefulConsumer` public type.

use super::{
    BoxConditionalStatefulConsumer,
    BoxConsumerOnce,
    Predicate,
    RcStatefulConsumer,
    StatefulConsumer,
    impl_box_consumer_methods,
    impl_box_conversions,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};

// ============================================================================
// 2. BoxStatefulConsumer - Single Ownership Implementation
// ============================================================================

/// BoxStatefulConsumer struct
///
/// Consumer implementation based on `Box<dyn FnMut(&T)>` for single ownership
/// scenarios. When sharing is not needed, this is the simplest and most
/// efficient consumer type.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, transfers ownership when used
/// - **Zero Overhead**: No reference counting or lock overhead
/// - **Mutable State**: Can modify captured environment through `FnMut`
/// - **Builder Pattern**: Method chaining naturally consumes `self`
///
/// # Use Cases
///
/// Choose `BoxStatefulConsumer` when:
/// - Consumer is used only once or in a linear flow
/// - Building pipelines where ownership flows naturally
/// - No need to share consumers across contexts
/// - Performance critical and cannot accept sharing overhead
///
/// # Performance
///
/// `BoxStatefulConsumer` has the best performance among the three consumer types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrowing checks
/// - Direct function calls through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, BoxStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x);
/// });
/// consumer.accept(&5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
/// ```
///
pub struct BoxStatefulConsumer<T> {
    pub(super) function: Box<dyn FnMut(&T)>,
    pub(super) name: Option<String>,
}

impl<T> BoxStatefulConsumer<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(BoxStatefulConsumer<T>, (FnMut(&T) + 'static), |f| Box::new(
        f
    ));

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(
        BoxStatefulConsumer<T>,
        BoxConditionalStatefulConsumer,
        StatefulConsumer
    );
}

impl<T> StatefulConsumer<T> for BoxStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulConsumer<T>,
        RcStatefulConsumer,
        FnMut(&T),
        BoxConsumerOnce
    );
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxStatefulConsumer<T>);
