// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulConsumer` trait.

/// Stateful consumer interface for operations that may update their own state.
///
/// Implementations receive the input through a shared reference, so they can
/// inspect it without modifying it. They may mutate state captured by the
/// consumer, which makes this trait suitable for accumulation and event
/// handling.
///
/// Closures implementing `FnMut(&T)` implement this trait automatically.
/// The crate's `BoxStatefulConsumer<T>`, `ArcStatefulConsumer<T>`, and
/// feature-gated `RcStatefulConsumer<T>` types also implement it.
///
/// # Type Parameters
///
/// * `T` — input type borrowed by each call to [`accept`](Self::accept).
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxStatefulConsumer, StatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// let values = Arc::new(Mutex::new(Vec::new()));
/// let captured_values = Arc::clone(&values);
/// let mut consumer = BoxStatefulConsumer::new(move |value: &i32| {
///     captured_values.lock().expect("mutex should not be poisoned").push(*value);
/// });
/// consumer.accept(&5);
/// assert_eq!(*values.lock().expect("mutex should not be poisoned"), vec![5]);
/// ```
pub trait StatefulConsumer<T> {
    /// Applies the callback to an input while allowing the consumer's state to
    /// change.
    ///
    /// The input is borrowed immutably for the duration of the call. Any
    /// persistent changes belong to the implementation's captured or owned
    /// state.
    ///
    /// # Parameters
    ///
    /// * `value` — input borrowed for this consumer operation.
    fn accept(&mut self, value: &T);
}

crate::macros::impl_closure_trait!(
    StatefulConsumer<T>,
    accept,
    FnMut(value: &T)
);
