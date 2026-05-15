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
//! Defines the `FnBiConsumerOps` public type.

use super::{
    BiConsumer,
    BoxBiConsumer,
};

// =======================================================================
// 6. Provide extension methods for closures
// =======================================================================

/// Extension trait providing non-mutating bi-consumer composition methods for
/// closures
///
/// Provides `and_then` and other composition methods for all closures
/// implementing `Fn(&T, &U)`, enabling direct method chaining on closures
/// without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxBiConsumer**: Composition results can be
///   further chained
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `Fn(&T, &U)` closures get these
///   methods automatically
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, FnBiConsumerOps};
///
/// let chained = (|x: &i32, y: &i32| {
///     println!("First: {}, {}", x, y);
/// }).and_then(|x: &i32, y: &i32| {
///     println!("Second: sum = {}", x + y);
/// });
/// chained.accept(&5, &3);
/// ```
///
pub trait FnBiConsumerOps<T, U>: Fn(&T, &U) + Sized {
    /// Chains another non-mutating bi-consumer in sequence
    ///
    /// Returns a new consumer executing the current operation first, then
    /// the next operation. Consumes the current closure and returns
    /// `BoxBiConsumer<T, U>`.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the next consumer
    ///
    /// # Parameters
    ///
    /// * `next` - The consumer to execute after the current operation
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxBiConsumer<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, FnBiConsumerOps};
    ///
    /// let chained = (|x: &i32, y: &i32| {
    ///     println!("First: {}, {}", x, y);
    /// }).and_then(|x: &i32, y: &i32| {
    ///     println!("Second: sum = {}", x + y);
    /// }).and_then(|x: &i32, y: &i32| {
    ///     println!("Third: product = {}", x * y);
    /// });
    ///
    /// chained.accept(&5, &3);
    /// ```
    fn and_then<C>(self, next: C) -> BoxBiConsumer<T, U>
    where
        Self: 'static,
        C: BiConsumer<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let first = self;
        let second = next;
        BoxBiConsumer::new(move |t, u| {
            first(t, u);
            second.accept(t, u);
        })
    }
}

/// Implements FnBiConsumerOps for all closure types
impl<T, U, F> FnBiConsumerOps<T, U> for F where F: Fn(&T, &U) {}
