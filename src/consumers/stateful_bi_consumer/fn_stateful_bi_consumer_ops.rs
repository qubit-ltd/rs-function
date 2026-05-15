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
//! Defines the `FnStatefulBiConsumerOps` public type.

use super::{
    BoxStatefulBiConsumer,
    StatefulBiConsumer,
};

// =======================================================================
// 6. Provide extension methods for closures
// =======================================================================

/// Extension trait providing bi-consumer composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures
/// implementing `FnMut(&T, &U)`, enabling direct method chaining on
/// closures without explicit wrapper types.
///
/// # Design Rationale
///
/// This trait allows closures to be composed naturally using method
/// syntax, similar to iterator combinators. Composition methods consume
/// the closure and return `BoxStatefulBiConsumer<T, U>`, which can be further
/// chained.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxStatefulBiConsumer**: Composition results are
///   `BoxStatefulBiConsumer<T, U>` for continued chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `FnMut(&T, &U)` closures get
///   these methods automatically
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, FnStatefulBiConsumerOps, StatefulBiConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let mut chained = (move |x: &i32, y: &i32| {
///     l1.lock().unwrap().push(*x + *y);
/// }).and_then(move |x: &i32, y: &i32| {
///     l2.lock().unwrap().push(*x * *y);
/// });
/// chained.accept(&5, &3);
/// assert_eq!(*log.lock().unwrap(), vec![8, 15]);
/// ```
///
pub trait FnStatefulBiConsumerOps<T, U>: FnMut(&T, &U) + Sized {
    /// Chains another consumer in sequence
    ///
    /// Returns a new consumer executing the current operation first, then
    /// the next operation. Consumes the current closure and returns
    /// `BoxStatefulBiConsumer<T, U>`.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the next consumer
    ///
    /// # Parameters
    ///
    /// * `next` - The consumer to execute after the current operation. **Note:
    ///   This parameter is passed by value and will transfer ownership.** If you
    ///   need to preserve the original consumer, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: &T, y: &U|`
    ///   - A `BoxStatefulBiConsumer<T, U>`
    ///   - An `ArcStatefulBiConsumer<T, U>`
    ///   - An `RcStatefulBiConsumer<T, U>`
    ///   - Any type implementing `BiConsumer<T, U>`
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxStatefulBiConsumer<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, FnStatefulBiConsumerOps, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l1 = log.clone();
    /// let l2 = log.clone();
    /// let mut chained = (move |x: &i32, y: &i32| {
    ///     l1.lock().unwrap().push(*x + *y);
    /// }).and_then(move |x: &i32, y: &i32| {
    ///     l2.lock().unwrap().push(*x * *y);
    /// }).and_then(|x: &i32, y: &i32| println!("Result: {}, {}", x, y));
    ///
    /// chained.accept(&5, &3); // Prints: Result: 5, 3
    /// assert_eq!(*log.lock().unwrap(), vec![8, 15]);
    /// ```
    fn and_then<C>(self, next: C) -> BoxStatefulBiConsumer<T, U>
    where
        Self: 'static,
        C: StatefulBiConsumer<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let mut first = self;
        let mut second = next;
        BoxStatefulBiConsumer::new(move |t, u| {
            first(t, u);
            second.accept(t, u);
        })
    }
}

/// Implements FnStatefulBiConsumerOps for all closure types
impl<T, U, F> FnStatefulBiConsumerOps<T, U> for F where F: FnMut(&T, &U) {}
