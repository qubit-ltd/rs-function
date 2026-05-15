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
//! Defines the `FnBiConsumerOnceOps` public type.

use super::{
    BiConsumerOnce,
    BoxBiConsumerOnce,
};

// =======================================================================
// 4. Provide extension methods for closures
// =======================================================================

/// Extension trait providing one-time bi-consumer composition methods for
/// closures
///
/// Provides `and_then` and other composition methods for all closures
/// implementing `FnOnce(&T, &U)`, enabling direct method chaining on
/// closures without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxBiConsumerOnce**: Composition results can be further
///   chained
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `FnOnce(&T, &U)` closures get
///   these methods automatically
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, FnBiConsumerOnceOps};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let chained = (move |x: &i32, y: &i32| {
///     l1.lock().unwrap().push(*x + *y);
/// }).and_then(move |x: &i32, y: &i32| {
///     l2.lock().unwrap().push(*x * *y);
/// });
/// chained.accept(&5, &3);
/// assert_eq!(*log.lock().unwrap(), vec![8, 15]);
/// ```
///
pub trait FnBiConsumerOnceOps<T, U>: FnOnce(&T, &U) + Sized {
    /// Chains another one-time bi-consumer in sequence
    ///
    /// Returns a new consumer executing the current operation first, then
    /// the next operation. Consumes the current closure and returns
    /// `BoxBiConsumerOnce<T, U>`.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the next consumer
    ///
    /// # Parameters
    ///
    /// * `next` - The consumer to execute after the current operation. **Note:
    ///   This parameter is passed by value and will transfer ownership.** Since
    ///   `BoxBiConsumerOnce` cannot be cloned, the parameter will be consumed.
    ///   Can be:
    ///   - A closure: `|x: &T, y: &U|`
    ///   - A `BoxBiConsumerOnce<T, U>`
    ///   - Any type implementing `BiConsumerOnce<T, U>`
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxBiConsumerOnce<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumerOnce, FnBiConsumerOnceOps};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l1 = log.clone();
    /// let l2 = log.clone();
    /// let chained = (move |x: &i32, y: &i32| {
    ///     l1.lock().unwrap().push(*x + *y);
    /// }).and_then(move |x: &i32, y: &i32| {
    ///     l2.lock().unwrap().push(*x * *y);
    /// }).and_then(|x: &i32, y: &i32| {
    ///     println!("Result: {}, {}", x, y);
    /// });
    ///
    /// chained.accept(&5, &3);
    /// assert_eq!(*log.lock().unwrap(), vec![8, 15]);
    /// ```
    fn and_then<C>(self, next: C) -> BoxBiConsumerOnce<T, U>
    where
        Self: 'static,
        C: BiConsumerOnce<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let first = self;
        let second = next;
        BoxBiConsumerOnce::new(move |t, u| {
            first(t, u);
            second.accept(t, u);
        })
    }
}

/// Implements FnBiConsumerOnceOps for all closure types
impl<T, U, F> FnBiConsumerOnceOps<T, U> for F where F: FnOnce(&T, &U) {}
