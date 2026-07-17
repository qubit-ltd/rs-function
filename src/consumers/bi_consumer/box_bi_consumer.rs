// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiConsumer` public type.

use {
    super::BiConsumerFn,
    crate::BiConsumer,
    crate::consumers::macros::impl_box_consumer_methods,
    crate::consumers::macros::impl_consumer_common_methods,
    crate::consumers::macros::impl_consumer_debug_display,
};
use {
    crate::BiPredicate,
    crate::BoxConditionalBiConsumer,
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
/// - **Runtime cost**: One heap allocation and dynamic dispatch; no reference
///   counting or locking
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
    fn accept(&self, first: &T, second: &U) {
        (self.function)(first, second)
    }
}

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(BoxBiConsumer<T, U>);
