/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcStatefulBiConsumer` public type.

#![allow(unused_imports)]

use super::*;

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
/// - **Non-Consuming API**: `and_then` borrows `&self`, original remains
///   usable
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
/// `ArcStatefulBiConsumer` has some overhead compared to `BoxStatefulBiConsumer`:
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
///     l.lock().unwrap().push(*x + *y);
/// });
/// let mut clone = consumer.clone();
///
/// consumer.accept(&5, &3);
/// assert_eq!(*log.lock().unwrap(), vec![8]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcStatefulBiConsumer<T, U> {
    pub(super) function: Arc<Mutex<dyn FnMut(&T, &U) + Send>>,
    pub(super) name: Option<String>,
}

impl<T, U> ArcStatefulBiConsumer<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(
        ArcStatefulBiConsumer<T, U>,
        (FnMut(&T, &U) + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when() and and_then() methods that borrow &self (Arc can clone)
    impl_shared_consumer_methods!(
        ArcStatefulBiConsumer<T, U>,
        ArcConditionalStatefulBiConsumer,
        into_arc,
        StatefulBiConsumer,
        Send + Sync + 'static
    );
}

impl<T, U> StatefulBiConsumer<T, U> for ArcStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        (self.function.lock())(first, second)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulBiConsumer<T, U>,
        BoxStatefulBiConsumer,
        RcStatefulBiConsumer,
        BoxBiConsumerOnce,
        FnMut(t: &T, u: &U)
    );
}

// Use macro to generate Clone implementation
impl_consumer_clone!(ArcStatefulBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(ArcStatefulBiConsumer<T, U>);

// =======================================================================
// 5. Implement BiConsumer trait for closures
// =======================================================================

// Implements BiConsumer for all FnMut(&T, &U)
impl_closure_trait!(
    StatefulBiConsumer<T, U>,
    accept,
    BoxBiConsumerOnce,
    FnMut(first: &T, second: &U)
);
