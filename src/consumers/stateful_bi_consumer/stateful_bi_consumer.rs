// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the shared interface for stateful operations on two borrowed values.

/// Provides a shared interface for operations on two borrowed values.
///
/// The `&mut self` receiver and `FnMut(&T, &U)` callback allow implementations
/// to update ordinary captured state. Shared references do not allow direct
/// mutation of either input, though callbacks may use interior mutability or
/// external side effects.
///
/// # Type Parameters
///
/// - `T`: Type of the first borrowed input.
/// - `U`: Type of the second borrowed input.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxStatefulBiConsumer, StatefulBiConsumer};
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// fn apply_bi_consumer<C: StatefulBiConsumer<i32, i32>>(
///     consumer: &mut C,
///     first: &i32,
///     second: &i32,
/// ) {
///     consumer.accept(first, second);
/// }
///
/// let log = Rc::new(RefCell::new(Vec::new()));
/// let shared_log = Rc::clone(&log);
/// let mut consumer = BoxStatefulBiConsumer::new(move |first: &i32, second: &i32| {
///     shared_log.borrow_mut().push(*first + *second);
/// });
/// apply_bi_consumer(&mut consumer, &5, &3);
/// assert_eq!(*log.borrow(), vec![8]);
/// ```
pub trait StatefulBiConsumer<T, U> {
    /// Applies the consumer operation to two shared references.
    ///
    /// The operation may produce side effects or update the consumer's own
    /// state, but it does not mutate either input.
    ///
    /// # Parameters
    ///
    /// - `first`: Shared reference to the first input value.
    /// - `second`: Shared reference to the second input value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let shared_log = Arc::clone(&log);
    /// let mut consumer = BoxStatefulBiConsumer::new(move |first: &i32, second: &i32| {
    ///     shared_log.lock().expect("mutex should not be poisoned")
    ///         .push(*first + *second);
    /// });
    /// consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn accept(&mut self, first: &T, second: &U);
}

crate::macros::impl_closure_trait!(
    StatefulBiConsumer<T, U>,
    accept,
    FnMut(first: &T, second: &U)
);
