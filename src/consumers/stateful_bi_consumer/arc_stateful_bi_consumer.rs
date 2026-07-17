// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulBiConsumer` public type.

use {
    crate::ArcConditionalStatefulBiConsumer,
    crate::BiPredicate,
};
use {
    crate::StatefulBiConsumer,
    crate::consumers::macros::impl_consumer_clone,
    crate::consumers::macros::impl_consumer_common_methods,
    crate::consumers::macros::impl_consumer_debug_display,
    crate::consumers::macros::impl_shared_consumer_methods,
    parking_lot::Mutex,
    std::sync::Arc,
};

/// The erased callback representation used by this implementation.
type ArcStatefulBiConsumerFn<T, U> = Arc<Mutex<dyn FnMut(&T, &U) + Send>>;

// =======================================================================
// 4. ArcStatefulBiConsumer - Thread-Safe Shared Ownership Implementation
// =======================================================================

/// ArcStatefulBiConsumer struct
///
/// A bi-consumer implementation based on
/// `Arc<Mutex<dyn FnMut(&T, &U) + Send>>` for thread-safe shared
/// ownership scenarios. This consumer can be safely cloned and shared
/// across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Interior Mutability**: Uses `Mutex` for safe mutable access
/// - **Non-Consuming API**: `and_then` borrows `&self`, original remains usable
/// - **Cross-Thread Sharing**: Can be sent to and used by other threads
///
/// # Use Cases
///
/// Choose `ArcStatefulBiConsumer` when:
/// - Need to share bi-consumer across multiple threads
/// - Concurrent task processing (e.g., thread pools)
/// - Using the same consumer in multiple places simultaneously
/// - Thread safety (Send + Sync) is required
///
/// # Performance Considerations
///
/// `ArcStatefulBiConsumer` has some overhead compared to
/// `BoxStatefulBiConsumer`:
/// - **Reference Counting**: Atomic operations on clone/drop
/// - **Mutex Locking**: Each `accept` call requires lock acquisition
/// - **Lock Contention**: High concurrency may cause contention
///
/// These overheads are necessary for safe concurrent access. If thread
/// safety is not needed, consider using `RcStatefulBiConsumer` for lower
/// overhead in single-threaded sharing.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let mut consumer = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// let mut clone = consumer.clone();
///
/// consumer.accept(&5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
/// ```
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared state
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcStatefulBiConsumer<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: ArcStatefulBiConsumerFn<T, U>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U> ArcStatefulBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        ArcStatefulBiConsumer<T, U>,
        (FnMut(&T, &U) + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when() and and_then() methods that borrow &self (Arc can
    // clone)
    impl_shared_consumer_methods!(
        ArcStatefulBiConsumer<T, U>,
        ArcConditionalStatefulBiConsumer,
        ArcBiPredicate,
        StatefulBiConsumer,
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + 'static)
    );
}

impl<T, U> StatefulBiConsumer<T, U> for ArcStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        let mut function = self.function.lock();
        function(first, second)
    }
}

// Use macro to generate Clone implementation
impl_consumer_clone!(ArcStatefulBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(ArcStatefulBiConsumer<T, U>);
