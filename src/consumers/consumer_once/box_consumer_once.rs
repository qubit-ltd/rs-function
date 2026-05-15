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
//! Defines the `BoxConsumerOnce` public type.

use super::{
    BoxConditionalConsumerOnce,
    ConsumerOnce,
    Predicate,
    impl_box_consumer_methods,
    impl_box_once_conversions,
    impl_closure_once_trait,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};

// ============================================================================
// 2. BoxConsumerOnce - Single Ownership Implementation
// ============================================================================

/// BoxConsumerOnce struct
///
/// One-time consumer implementation based on `Box<dyn FnOnce(&T)>` for single ownership scenarios.
/// This is the simplest consumer type for truly one-time use.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, transfers ownership on use
/// - **Zero Overhead**: No reference counting or lock overhead
/// - **One-time Use**: Consumes self on first call
/// - **Builder Pattern**: Method chaining naturally consumes `self`
///
/// # Use Cases
///
/// Choose `BoxConsumerOnce` when:
/// - Consumer is truly used only once
/// - Building pipelines where ownership flows naturally
/// - Consumer captures values that should be consumed
/// - Performance critical and cannot accept shared overhead
///
/// # Performance
///
/// `BoxConsumerOnce` has the best performance:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
///
/// let consumer = BoxConsumerOnce::new(|x: &i32| {
///     println!("Value: {}", x);
/// });
/// consumer.accept(&5);
/// ```
///
pub struct BoxConsumerOnce<T> {
    pub(super) function: Box<dyn FnOnce(&T)>,
    pub(super) name: Option<String>,
}

// All methods require T: 'static because Box<dyn FnOnce(&T)> requires it
impl<T> BoxConsumerOnce<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(BoxConsumerOnce<T>, (FnOnce(&T) + 'static), |f| Box::new(f));

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(BoxConsumerOnce<T>, BoxConditionalConsumerOnce, ConsumerOnce);
}

impl<T> ConsumerOnce<T> for BoxConsumerOnce<T> {
    fn accept(self, value: &T) {
        (self.function)(value)
    }

    impl_box_once_conversions!(BoxConsumerOnce<T>, ConsumerOnce, FnOnce(&T));
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxConsumerOnce<T>);

// ============================================================================
// 3. Implement ConsumerOnce trait for closures
// ============================================================================

// Implement ConsumerOnce for all FnOnce(&T) using macro
impl_closure_once_trait!(
    ConsumerOnce<T>,
    accept,
    BoxConsumerOnce,
    FnOnce(value: &T)
);
