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
//! Defines the `BoxStatefulBiConsumer` public type.

use super::{
    BiPredicate,
    BoxBiConsumerOnce,
    BoxConditionalStatefulBiConsumer,
    RcStatefulBiConsumer,
    StatefulBiConsumer,
    impl_box_consumer_methods,
    impl_box_conversions,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};

type BoxStatefulBiConsumerFn<T, U> = Box<dyn FnMut(&T, &U)>;

// =======================================================================
// 2. BoxStatefulBiConsumer - Single Ownership Implementation
// =======================================================================

/// BoxStatefulBiConsumer struct
///
/// A bi-consumer implementation based on `Box<dyn FnMut(&T, &U)>` for
/// single ownership scenarios. This is the simplest and most efficient
/// bi-consumer type when sharing is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Builder Pattern**: Method chaining consumes `self` naturally
///
/// # Use Cases
///
/// Choose `BoxStatefulBiConsumer` when:
/// - The bi-consumer is used only once or in a linear flow
/// - Building pipelines where ownership naturally flows
/// - No need to share the consumer across contexts
/// - Performance is critical and sharing overhead is unacceptable
///
/// # Performance
///
/// `BoxStatefulBiConsumer` has the best performance among the three bi-consumer
/// types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxStatefulBiConsumer, StatefulBiConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let mut consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// consumer.accept(&5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
/// ```
///
pub struct BoxStatefulBiConsumer<T, U> {
    pub(super) function: BoxStatefulBiConsumerFn<T, U>,
    pub(super) name: Option<String>,
}

impl<T, U> BoxStatefulBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        BoxStatefulBiConsumer<T, U>,
        (FnMut(&T, &U) + 'static),
        |f| Box::new(f)
    );

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(
        BoxStatefulBiConsumer<T, U>,
        BoxConditionalStatefulBiConsumer,
        StatefulBiConsumer
    );
}

impl<T, U> StatefulBiConsumer<T, U> for BoxStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulBiConsumer<T, U>,
        RcStatefulBiConsumer,
        FnMut(&T, &U),
        BoxBiConsumerOnce
    );
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxStatefulBiConsumer<T, U>);
