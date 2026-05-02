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
//! Defines the `ArcStatefulConsumer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 4. ArcStatefulConsumer - Thread-Safe Shared Ownership Implementation
// ============================================================================

/// ArcStatefulConsumer struct
///
/// Consumer implementation based on `Arc<Mutex<dyn FnMut(&T) + Send>>` for
/// thread-safe shared ownership scenarios. This consumer can be safely cloned
/// and shared across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Arc`, allowing multiple owners
/// - **Thread Safety**: Implements `Send + Sync`, safe for concurrent use
/// - **Interior Mutability**: Uses `Mutex` for safe mutable access
/// - **Non-Consuming API**: `and_then` borrows `&self`, original object remains
///   usable
/// - **Cross-Thread Sharing**: Can be sent to other threads and used
///
/// # Use Cases
///
/// Choose `ArcStatefulConsumer` when:
/// - Need to share consumers across multiple threads
/// - Concurrent task processing (e.g., thread pools)
/// - Using the same consumer in multiple places simultaneously
/// - Need thread safety (Send + Sync)
///
/// # Performance Considerations
///
/// `ArcStatefulConsumer` has some performance overhead compared to `BoxStatefulConsumer`:
/// - **Reference Counting**: Atomic operations on clone/drop
/// - **Mutex Locking**: Each `accept` call requires lock acquisition
/// - **Lock Contention**: High concurrency may cause contention
///
/// These overheads are necessary for safe concurrent access. If thread safety
/// is not needed, consider using `RcStatefulConsumer` for less single-threaded sharing
/// overhead.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, ArcStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
///     l.lock().unwrap().push(*x * 2);
/// });
/// let mut clone = consumer.clone();
///
/// consumer.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![10]);
/// ```
///
pub struct ArcStatefulConsumer<T> {
    pub(super) function: Arc<Mutex<dyn FnMut(&T) + Send>>,
    pub(super) name: Option<String>,
}

impl<T> ArcStatefulConsumer<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(ArcStatefulConsumer<T>, (FnMut(&T) + Send + 'static), |f| {
        Arc::new(Mutex::new(f))
    });

    // Generates: when() and and_then() methods that borrow &self (Arc can clone)
    impl_shared_consumer_methods!(
        ArcStatefulConsumer<T>,
        ArcConditionalStatefulConsumer,
        into_arc,
        StatefulConsumer,
        Send + Sync + 'static
    );
}

impl<T> StatefulConsumer<T> for ArcStatefulConsumer<T> {
    fn accept(&mut self, value: &T) {
        (self.function.lock())(value)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulConsumer<T>,
        BoxStatefulConsumer,
        RcStatefulConsumer,
        BoxConsumerOnce,
        FnMut(t: &T)
    );
}

// Use macro to generate Clone implementation
impl_consumer_clone!(ArcStatefulConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(ArcStatefulConsumer<T>);

// ============================================================================
// 5. Implement Consumer trait for closures
// ============================================================================

// Implement Consumer for all FnMut(&T)
impl_closure_trait!(
    StatefulConsumer<T>,
    accept,
    BoxConsumerOnce,
    FnMut(value: &T)
);
