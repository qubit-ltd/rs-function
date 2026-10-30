// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ConsumerOnce` trait.

/// ConsumerOnce trait - Unified one-time consumer interface
///
/// It is similar to the `FnOnce(&T)` trait in the standard library.
///
/// Defines the core behavior of all one-time consumer types. Similar to
/// consumers implementing `FnOnce(&T)`, it operates on a shared input reference
/// and returns no result. The reference does not permit direct mutation of the
/// input; the callback may still use interior mutability or produce external
/// side effects. The consumer is consumed by the operation.
///
/// # Automatic Implementation
///
/// - All closures implementing `FnOnce(&T)`
/// - `BoxConsumerOnce<T>`
/// - `BoxConditionalConsumerOnce<T>`
///
/// # Features
///
/// - **Unified Interface**: All consumer types share the same `accept` method
///   signature
/// - **Automatic Implementation**: Closures implement this trait directly,
///   without allocating an adapter
/// - **Generic Programming**: Write functions that work with any one-time
///   consumer type
///
/// # Type Parameters
///
/// * `T` - The type of value borrowed by the one-time operation.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
///     consumer.accept(value);
/// }
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let box_con = BoxConsumerOnce::new(move |x: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x);
/// });
/// apply_consumer(box_con, &5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
/// ```
pub trait ConsumerOnce<T> {
    /// Execute one-time consumption operation
    ///
    /// Executes an operation on the given shared reference and consumes `self`.
    /// The shared reference does not permit direct mutation of the input, but
    /// interior mutability may still change its state. The operation may also
    /// produce external side effects.
    ///
    /// # Parameters
    ///
    /// * `value` - Reference to the value to be consumed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
    ///
    /// let consumer = BoxConsumerOnce::new(|x: &i32| println!("{}", x));
    /// consumer.accept(&5);
    /// ```
    fn accept(self, value: &T);
}
