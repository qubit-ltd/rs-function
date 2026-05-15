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
//! Defines the `BoxConsumer` public type.

use super::{
    BoxConditionalConsumer,
    BoxConsumerOnce,
    Consumer,
    Predicate,
    RcConsumer,
    impl_box_consumer_methods,
    impl_box_conversions,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};

// ============================================================================
// 2. BoxConsumer - Single Ownership Implementation
// ============================================================================

/// BoxConsumer struct
///
/// Non-mutating consumer implementation based on `Box<dyn Fn(&T)>` for single
/// ownership scenarios.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, transfers ownership when used
/// - **Zero Overhead**: No reference counting or lock overhead
/// - **Shared-reference API**: Invoked through `&self` and shared input
///   references
/// - **No Wrapper Interior Mutability**: No need for Mutex or RefCell in the
///   wrapper
///
/// # Use Cases
///
/// Choose `BoxConsumer` when:
/// - Non-mutating consumer is used once or in a linear flow
/// - No need to share consumer across contexts
/// - Pure observation operations, such as logging
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, BoxConsumer};
///
/// let consumer = BoxConsumer::new(|x: &i32| {
///     println!("Observed value: {}", x);
/// });
/// consumer.accept(&5);
/// ```
///
pub struct BoxConsumer<T> {
    pub(super) function: Box<dyn Fn(&T)>,
    pub(super) name: Option<String>,
}

impl<T> BoxConsumer<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(BoxConsumer<T>, (Fn(&T) + 'static), |f| Box::new(f));

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(BoxConsumer<T>, BoxConditionalConsumer, Consumer);
}

impl<T> Consumer<T> for BoxConsumer<T> {
    fn accept(&self, value: &T) {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(BoxConsumer<T>, RcConsumer, Fn(&T), BoxConsumerOnce);
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxConsumer<T>);
