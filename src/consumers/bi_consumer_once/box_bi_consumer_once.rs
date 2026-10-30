// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiConsumerOnce` public type.

use super::BiConsumerOnceFn;
use crate::BiConsumerOnce;
use crate::BiPredicate;
use crate::BoxConditionalBiConsumerOnce;
use crate::consumers::macros::impl_box_consumer_methods;
use crate::consumers::macros::impl_consumer_common_methods;
use crate::consumers::macros::impl_consumer_debug_display;
use crate::macros::impl_closure_once_trait;

// =======================================================================
// 2. BoxBiConsumerOnce - Single Ownership Implementation
// =======================================================================

/// Owns a one-time callback that consumes two borrowed inputs.
///
/// A one-time bi-consumer implementation based on
/// `Box<dyn FnOnce(&T, &U)>` for single ownership scenarios. This is the
/// simplest one-time bi-consumer type for truly one-time use.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` consumes `self`
/// - **One-Time Use**: Consumes self on first call
/// - **Builder Pattern**: Method chaining consumes `self` naturally
///
/// # Use Cases
///
/// Choose `BoxBiConsumerOnce` when:
/// - The bi-consumer is truly used only once
/// - Building pipelines where ownership naturally flows
/// - The consumer captures values that should be consumed
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
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
/// use std::sync::Arc;
/// use std::sync::Mutex;
///
/// let sum = Arc::new(Mutex::new(0));
/// let captured_sum = Arc::clone(&sum);
/// let consumer = BoxBiConsumerOnce::new(move |x: &i32, y: &i32| {
///     *captured_sum.lock().expect("mutex should not be poisoned") = x + y;
/// });
/// consumer.accept(&5, &3);
/// assert_eq!(*sum.lock().expect("mutex should not be poisoned"), 8);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiConsumerOnce<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: Box<BiConsumerOnceFn<T, U>>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// All methods require T: 'static and U: 'static because
// Box<dyn FnOnce(&T, &U)> requires it
impl<T, U> BoxBiConsumerOnce<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        BoxBiConsumerOnce<T, U>,
        (FnOnce(&T, &U) + 'static),
        |f| Box::new(f)
    );

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(
        BoxBiConsumerOnce<T, U>,
        BoxConditionalBiConsumerOnce,
        BiConsumerOnce
    );
}

impl<T, U> BiConsumerOnce<T, U> for BoxBiConsumerOnce<T, U> {
    #[inline(always)]
    fn accept(self, first: &T, second: &U) {
        (self.function)(first, second)
    }
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxBiConsumerOnce<T, U>);

// =======================================================================
// 3. Implement BiConsumerOnce trait for closures
// =======================================================================

// Implement BiConsumerOnce for all FnOnce(&T, &U) using macro
impl_closure_once_trait!(
    BiConsumerOnce<T, U>,
    accept,
    BoxBiConsumerOnce,
    FnOnce(first: &T, second: &U)
);
