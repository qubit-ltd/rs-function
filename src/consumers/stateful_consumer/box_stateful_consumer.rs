// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxStatefulConsumer` public type.

use crate::BoxConditionalStatefulConsumer;
use crate::Predicate;
use crate::StatefulConsumer;
use crate::consumers::macros::impl_box_consumer_methods;
use crate::consumers::macros::impl_consumer_common_methods;
use crate::consumers::macros::impl_consumer_debug_display;

// ============================================================================
// 2. BoxStatefulConsumer - Single Ownership Implementation
// ============================================================================

/// BoxStatefulConsumer struct
///
/// Consumer implementation based on `Box<dyn FnMut(&T)>` for single ownership
/// scenarios. Choose it when the callback should use single-owner `Box`
/// storage.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` borrows `&mut self`
/// - **Mutable State**: Can modify captured environment through `FnMut`
/// - **Builder Pattern**: Method chaining naturally consumes `self`
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
/// * `T` - The type of values borrowed by the callback when it is invoked.
///
/// # Use Cases
///
/// Choose `BoxStatefulConsumer` when:
/// - Consumer is used only once or in a linear flow
/// - Building pipelines where ownership flows naturally
/// - No need to share consumers across contexts
/// - The callback should use single-owner `Box` storage
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulConsumer, BoxStatefulConsumer};
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
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxStatefulConsumer<T> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnMut(&T)>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T> BoxStatefulConsumer<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(BoxStatefulConsumer<T>, (FnMut(&T) + 'static), |f| Box::new(f));

    // Generates: when() and and_then() methods that consume self
    impl_box_consumer_methods!(BoxStatefulConsumer<T>, BoxConditionalStatefulConsumer, StatefulConsumer);
}

impl<T> StatefulConsumer<T> for BoxStatefulConsumer<T> {
    #[inline(always)]
    fn accept(&mut self, value: &T) {
        (self.function)(value)
    }
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxStatefulConsumer<T>);
