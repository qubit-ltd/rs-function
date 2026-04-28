/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnConsumerOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 6. Provide extension methods for closures
// ============================================================================

/// Extension trait providing non-mutating consumer composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures
/// implementing `Fn(&T)`, allowing closures to directly chain methods without
/// explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxConsumer**: Combined results can continue chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Auto-implementation**: All `Fn(&T)` closures automatically get these
///   methods
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, FnConsumerOps};
///
/// let chained = (|x: &i32| {
///     println!("First: {}", x);
/// }).and_then(|x: &i32| {
///     println!("Second: {}", x);
/// });
/// chained.accept(&5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnConsumerOps<T>: Fn(&T) + Sized {
    /// Sequentially chain another non-mutating consumer
    ///
    /// Returns a new consumer that executes the current operation first, then the
    /// next operation. Consumes the current closure and returns
    /// `BoxConsumer<T>`.
    ///
    /// # Type Parameters
    ///
    /// * `C` - Type of the next consumer
    ///
    /// # Parameters
    ///
    /// * `next` - Consumer to execute after the current operation
    ///
    /// # Returns
    ///
    /// Returns a combined `BoxConsumer<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, FnConsumerOps};
    ///
    /// let chained = (|x: &i32| {
    ///     println!("First: {}", x);
    /// }).and_then(|x: &i32| {
    ///     println!("Second: {}", x);
    /// }).and_then(|x: &i32| println!("Third: {}", x));
    ///
    /// chained.accept(&5);
    /// ```
    fn and_then<C>(self, next: C) -> BoxConsumer<T>
    where
        Self: 'static,
        C: Consumer<T> + 'static,
        T: 'static,
    {
        let first = self;
        let second = next;
        BoxConsumer::new(move |t| {
            first(t);
            second.accept(t);
        })
    }
}

/// Implement FnConsumerOps for all closure types
impl<T, F> FnConsumerOps<T> for F where F: Fn(&T) {}
