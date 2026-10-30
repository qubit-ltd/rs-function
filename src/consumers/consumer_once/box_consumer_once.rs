// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConsumerOnce` public type.

use crate::BoxConditionalConsumerOnce;
use crate::ConsumerOnce;
use crate::Predicate;
use crate::consumers::macros::impl_box_consumer_methods;
use crate::consumers::macros::impl_consumer_common_methods;
use crate::consumers::macros::impl_consumer_debug_display;
use crate::macros::impl_closure_once_trait;

// ============================================================================
// 2. BoxConsumerOnce - Single Ownership Implementation
// ============================================================================

/// Owns a callback that consumes itself when invoked once.
///
/// One-time consumer implementation based on `Box<dyn FnOnce(&T)>` for single
/// ownership scenarios. This is the simplest consumer type for truly one-time
/// use.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` consumes `self`
/// - **One-time Use**: `accept` consumes `self` on its single call
/// - **Builder Pattern**: Method chaining naturally consumes `self`
///
/// # Use Cases
///
/// Choose `BoxConsumerOnce` when:
/// - Consumer is truly used only once
/// - Building pipelines where ownership flows naturally
/// - Consumer captures values that should be consumed
/// - The callback does not need shared ownership
///
/// # Runtime Cost
///
/// The wrapper stores a type-erased callback and diagnostic metadata. Calls use
/// dynamic dispatch. Callback ownership does not add reference counting or a
/// shared-wrapper lock; an optional diagnostic name uses shared `Arc<str>`
/// storage. Memory usage and allocation depend on the capture and name. No
/// fixed pointer count or unconditional performance ranking is guaranteed.
///
/// # Type Parameters
///
/// * `T` - The input type borrowed by the callback when it is invoked.
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
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConsumerOnce<T> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnOnce(&T)>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// All methods require T: 'static because Box<dyn FnOnce(&T)> requires it
impl<T> BoxConsumerOnce<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(BoxConsumerOnce<T>, (FnOnce(&T) + 'static), |f| Box::new(f));

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(BoxConsumerOnce<T>, BoxConditionalConsumerOnce, ConsumerOnce);
}

impl<T> ConsumerOnce<T> for BoxConsumerOnce<T> {
    /// Consumes this wrapper and invokes its callback with `value`.
    #[inline(always)]
    fn accept(self, value: &T) {
        (self.function)(value)
    }
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
