// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcBiConsumer` public type.

use std::sync::Arc;

use super::internal::ThreadSafeBiConsumerFn;
use crate::ArcConditionalBiConsumer;
use crate::BiConsumer;
use crate::BiPredicate;
use crate::consumers::macros::impl_consumer_clone;
use crate::consumers::macros::impl_consumer_common_methods;
use crate::consumers::macros::impl_consumer_debug_display;
use crate::consumers::macros::impl_shared_consumer_methods;

// =======================================================================
// 4. ArcBiConsumer - Thread-Safe Shared Ownership
// =======================================================================

/// ArcBiConsumer struct
///
/// A bi-consumer implementation based on
/// `Arc<dyn Fn(&T, &U) + Send + Sync>` for thread-safe shared ownership
/// scenarios. The wrapper does not add a mutex around the shared `Fn`
/// callback; the callback itself may use synchronization or interior
/// mutability.
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Callback Storage**: The wrapper stores an `Arc<dyn Fn(&T, &U) + Send +
///   Sync>` and does not add a mutex around callback invocation
/// - **Non-Consuming API**: `and_then` borrows `&self`, original remains usable
///
/// # Use Cases
///
/// Choose `ArcBiConsumer` when:
/// - Need to share non-mutating bi-consumer across multiple threads
/// - Pure observation operations like logging, monitoring, notifications
/// - Need a shared callback whose type satisfies `Send + Sync`
///
/// A mutex-free wrapper does not guarantee that the callback has no locks or
/// contention.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, ArcBiConsumer};
///
/// let consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
///     println!("Sum: {}", x + y);
/// });
/// let clone = consumer.clone();
///
/// consumer.accept(&5, &3);
/// clone.accept(&10, &20);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcBiConsumer<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: Arc<ThreadSafeBiConsumerFn<T, U>>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U> ArcBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        ArcBiConsumer<T, U>,
        (Fn(&T, &U) + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: when() and and_then() methods that borrow &self (Arc can
    // clone)
    impl_shared_consumer_methods!(
        ArcBiConsumer<T, U>,
        ArcConditionalBiConsumer,
        ArcBiPredicate,
        BiConsumer,
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + Sync + 'static)
    );
}

impl<T, U> BiConsumer<T, U> for ArcBiConsumer<T, U> {
    /// Invokes the shared callback with both borrowed input values.
    #[inline(always)]
    fn accept(&self, first: &T, second: &U) {
        (self.function)(first, second)
    }
}

// Use macro to generate Clone implementation
impl_consumer_clone!(ArcBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(ArcBiConsumer<T, U>);
