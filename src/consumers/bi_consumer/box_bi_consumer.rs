// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiConsumer` public type.

use super::internal::BiConsumerFn;
use crate::BiConsumer;
use crate::BiPredicate;
use crate::BoxConditionalBiConsumer;
use crate::consumers::macros::impl_box_consumer_methods;
use crate::consumers::macros::impl_consumer_common_methods;
use crate::consumers::macros::impl_consumer_debug_display;

// =======================================================================
// 2. BoxBiConsumer - Single Ownership Implementation
// =======================================================================

/// BoxBiConsumer struct
///
/// A non-mutating bi-consumer implementation based on `Box<dyn Fn(&T, &U)>`
/// for single ownership scenarios.
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
/// * `T` - Type of the first value passed to the callback.
/// * `U` - Type of the second value passed to the callback.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; `accept` borrows `&self`
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
/// use std::cell::Cell;
/// use std::rc::Rc;
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// let observed_sum = Rc::new(Cell::new(0));
/// let callback_sum = Rc::clone(&observed_sum);
/// let consumer = BoxBiConsumer::new(move |x: &i32, y: &i32| {
///     callback_sum.set(x + y);
/// });
/// consumer.accept(&5, &3);
/// assert_eq!(observed_sum.get(), 8);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiConsumer<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: Box<BiConsumerFn<T, U>>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
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
    #[inline(always)]
    fn accept(&self, first: &T, second: &U) {
        (self.function)(first, second)
    }
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxBiConsumer<T, U>);
