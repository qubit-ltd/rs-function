// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BiConsumerOnce` interface.

/// A unified interface for one-time operations that accept two borrowed
/// values and return no result.
///
/// Implementations consume themselves while operating on shared references
/// to both inputs. These references do not permit direct mutation of the input
/// values; callbacks may still use interior mutability or produce external
/// side effects. Closures implementing `FnOnce(&T, &U)` and the boxed callback
/// types in this module implement this trait.
///
/// # Type Parameters
///
/// - `T` is the type of the first value passed to the consumer.
/// - `U` is the type of the second value passed to the consumer.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: BiConsumerOnce<i32, i32>>(
///     consumer: C,
///     a: &i32,
///     b: &i32,
/// ) {
///     consumer.accept(a, b);
/// }
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let boxed = BoxBiConsumerOnce::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// apply_consumer(boxed, &5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
/// ```
pub trait BiConsumerOnce<T, U> {
    /// Consumes this consumer while operating on references to both inputs.
    ///
    /// The operation may read the values or produce side effects, but it does
    /// not modify the input values themselves.
    ///
    /// # Parameters
    ///
    /// * `first` is a reference to the first value.
    /// * `second` is a reference to the second value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
    ///
    /// let consumer = BoxBiConsumerOnce::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// consumer.accept(&5, &3);
    /// ```
    fn accept(self, first: &T, second: &U);
}
