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
//! Defines the `BoxBiConsumer` public type.

use super::{
    BiConsumer,
    BiConsumerFn,
    BiPredicate,
    BoxBiConsumerOnce,
    BoxConditionalBiConsumer,
    RcBiConsumer,
    impl_box_consumer_methods,
    impl_box_conversions,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};

// =======================================================================
// 2. BoxBiConsumer - Single Ownership Implementation
// =======================================================================

/// BoxBiConsumer struct
///
/// A non-mutating bi-consumer implementation based on `Box<dyn Fn(&T, &U)>`
/// for single ownership scenarios.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Shared-reference API**: Invoked through `&self` and shared input
///   references
/// - **No Wrapper Interior Mutability**: No need for Mutex or RefCell in the
///   wrapper
///
/// # Use Cases
///
/// Choose `BoxBiConsumer` when:
/// - The non-mutating bi-consumer is used only once or in a linear flow
/// - No need to share the consumer across contexts
/// - Pure observation operations like logging
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
///     println!("Sum: {}", x + y);
/// });
/// consumer.accept(&5, &3);
/// ```
///
pub struct BoxBiConsumer<T, U> {
    pub(super) function: Box<BiConsumerFn<T, U>>,
    pub(super) name: Option<String>,
}

impl<T, U> BoxBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        BoxBiConsumer<T, U>,
        (Fn(&T, &U) + 'static),
        |f| Box::new(f)
    );

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(
        BoxBiConsumer<T, U>,
        BoxConditionalBiConsumer,
        BiConsumer
    );
}

impl<T, U> BiConsumer<T, U> for BoxBiConsumer<T, U> {
    fn accept(&self, first: &T, second: &U) {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiConsumer<T, U>,
        RcBiConsumer,
        Fn(&T, &U),
        BoxBiConsumerOnce
    );
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxBiConsumer<T, U>);
