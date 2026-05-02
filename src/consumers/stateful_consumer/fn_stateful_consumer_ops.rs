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
//! Defines the `FnStatefulConsumerOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 6. Extension methods for closures
// ============================================================================

/// Extension trait providing consumer composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures
/// implementing `FnMut(&T)`, allowing direct method chaining on closures
/// without explicit wrapper types.
///
/// # Design Philosophy
///
/// This trait allows closures to be naturally composed using method syntax,
/// similar to iterator combinators. Composition methods consume the closure and
/// return `BoxStatefulConsumer<T>`, which can continue chaining.
///
/// # Features
///
/// - **Natural Syntax**: Direct method chaining on closures
/// - **Returns BoxStatefulConsumer**: Composition results in `BoxStatefulConsumer<T>`, can
///   continue chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `FnMut(&T)` closures automatically get
///   these methods
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, FnStatefulConsumerOps};
/// use std::sync::{Arc, Mutex};
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l1 = log.clone();
/// let l2 = log.clone();
/// let mut chained = (move |x: &i32| {
///     l1.lock().unwrap().push(*x * 2);
/// }).and_then(move |x: &i32| {
///     l2.lock().unwrap().push(*x + 10);
/// });
/// chained.accept(&5);
/// assert_eq!(*log.lock().unwrap(), vec![10, 15]);
/// // (5 * 2), (5 + 10)
/// ```
///
pub trait FnStatefulConsumerOps<T>: FnMut(&T) + Sized {
    /// Sequentially chain another consumer
    ///
    /// Returns a new consumer that executes the current operation first, then the
    /// next operation. Consumes the current closure and returns `BoxStatefulConsumer<T>`.
    ///
    /// # Type Parameters
    ///
    /// * `C` - Type of the next consumer
    ///
    /// # Parameters
    ///
    /// * `next` - Consumer to execute after the current operation. **Note: This
    ///   parameter is passed by value and will transfer ownership.** If you need
    ///   to preserve the original consumer, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: &T|`
    ///   - A `BoxStatefulConsumer<T>`
    ///   - An `RcStatefulConsumer<T>`
    ///   - An `ArcStatefulConsumer<T>`
    ///   - Any type implementing `Consumer<T>`
    ///
    /// # Return Value
    ///
    /// Returns a combined `BoxStatefulConsumer<T>`
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, FnStatefulConsumerOps, BoxStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l1 = log.clone();
    /// let l2 = log.clone();
    /// let second = BoxStatefulConsumer::new(move |x: &i32| {
    ///     l2.lock().unwrap().push(*x + 10);
    /// });
    ///
    /// // second is moved here
    /// let mut chained = (move |x: &i32| {
    ///     l1.lock().unwrap().push(*x * 2);
    /// }).and_then(second);
    ///
    /// chained.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10, 15]);
    /// // second.accept(&3); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, FnStatefulConsumerOps, RcStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l1 = log.clone();
    /// let l2 = log.clone();
    /// let mut second = RcStatefulConsumer::new(move |x: &i32| {
    ///     l2.lock().unwrap().push(*x + 10);
    /// });
    ///
    /// // Clone to preserve original
    /// let mut chained = (move |x: &i32| {
    ///     l1.lock().unwrap().push(*x * 2);
    /// }).and_then(second.clone());
    ///
    /// chained.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10, 15]);
    ///
    /// // Original still usable
    /// second.accept(&3);
    /// assert_eq!(*log.lock().unwrap(), vec![10, 15, 13]);
    /// ```
    fn and_then<C>(self, next: C) -> BoxStatefulConsumer<T>
    where
        Self: 'static,
        C: StatefulConsumer<T> + 'static,
        T: 'static,
    {
        let mut first = self;
        let mut second = next;
        BoxStatefulConsumer::new(move |t| {
            first(t);
            second.accept(t);
        })
    }
}

/// Implement FnStatefulConsumerOps for all closure types
impl<T, F> FnStatefulConsumerOps<T> for F where F: FnMut(&T) {}
