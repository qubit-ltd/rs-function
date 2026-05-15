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
//! Defines the `ArcConsumer` public type.

use super::{
    Arc,
    ArcConditionalConsumer,
    BoxConsumer,
    BoxConsumerOnce,
    Consumer,
    Predicate,
    RcConsumer,
    impl_arc_conversions,
    impl_closure_trait,
    impl_consumer_clone,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
    impl_shared_consumer_methods,
};

// ============================================================================
// 4. ArcConsumer - Thread-safe Shared Ownership Implementation
// ============================================================================

/// ArcConsumer struct
///
/// Non-mutating consumer implementation based on `Arc<dyn Fn(&T) + Send + Sync>`,
/// for thread-safe shared ownership scenarios. The wrapper does not need
/// `Mutex` because it only invokes a shared `Fn`.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable through `Arc`, allows multiple owners
/// - **Thread Safe**: Implements `Send + Sync`, can be safely used concurrently
/// - **Lock-free Wrapper**: No Mutex protection needed by the wrapper
/// - **Non-consuming API**: `and_then` borrows `&self`, original object remains
///   usable
///
/// # Use Cases
///
/// Choose `ArcConsumer` when:
/// - Need to share non-mutating consumer across multiple threads
/// - Pure observation operations, such as logging, monitoring, notifications
/// - Need high-concurrency reads with no lock overhead
///
/// # Performance Advantages
///
/// Compared to `ArcStatefulConsumer`, `ArcConsumer` has no Mutex lock overhead,
/// performing better in high-concurrency observation scenarios.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, ArcConsumer};
///
/// let consumer = ArcConsumer::new(|x: &i32| {
///     println!("Observed: {}", x);
/// });
/// let clone = consumer.clone();
///
/// consumer.accept(&5);
/// clone.accept(&10);
/// ```
///
pub struct ArcConsumer<T> {
    pub(super) function: Arc<dyn Fn(&T) + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T> ArcConsumer<T> {
    // Generates: new(), new_with_name(), name(), set_name(), noop()
    impl_consumer_common_methods!(ArcConsumer<T>, (Fn(&T) + Send + Sync + 'static), |f| {
        Arc::new(f)
    });

    // Generates: when() and and_then() methods that borrow &self (Arc can clone)
    impl_shared_consumer_methods!(
        ArcConsumer<T>,
        ArcConditionalConsumer,
        into_arc,
        Consumer,
        Send + Sync + 'static
    );
}

impl<T> Consumer<T> for ArcConsumer<T> {
    fn accept(&self, value: &T) {
        (self.function)(value)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcConsumer<T>,
        BoxConsumer,
        RcConsumer,
        BoxConsumerOnce,
        Fn(t: &T)
    );
}

// Use macro to generate Clone implementation
impl_consumer_clone!(ArcConsumer<T>);

// Use macro to generate Debug and Display implementations
impl_consumer_debug_display!(ArcConsumer<T>);

// ============================================================================
// 5. Implement Consumer trait for closures
// ============================================================================

// Implement Consumer for all Fn(&T)
impl_closure_trait!(
    Consumer<T>,
    accept,
    BoxConsumerOnce,
    Fn(value: &T)
);
