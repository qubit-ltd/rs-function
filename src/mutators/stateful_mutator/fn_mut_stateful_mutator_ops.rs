// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `FnMutStatefulMutatorOps` public type.

use super::{
    BoxStatefulMutator,
    StatefulMutator,
};

// ============================================================================
// 6. Provide extension methods for closures
// ============================================================================

/// Extension trait providing stateful mutator composition for closures.
///
/// Provides `and_then` and other composition methods for all closures that
/// implement `FnMut(&mut T)`, enabling direct method chaining on closures
/// without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxStatefulMutator**: Composition results are
///   `BoxStatefulMutator<T>` for continued chaining
/// - **Typed Composition**: Returns a new closure that captures both operations
/// - **Automatic Implementation**: All `FnMut(&mut T)` closures get these
///   methods automatically
///
/// # Examples
///
/// ```rust
/// use qubit_function::{FnMutStatefulMutatorOps, StatefulMutator};
///
/// let mut calls = 0;
/// let mut chained = (move |x: &mut i32| {
///     calls += 1;
///     *x += calls;
/// }).and_then(|x: &mut i32| *x *= 2);
/// let mut value = 10;
/// chained.apply(&mut value);
/// assert_eq!(value, 22);
/// ```
pub trait FnMutStatefulMutatorOps<T>: FnMut(&mut T) + Sized {
    /// Chains another mutator in sequence
    ///
    /// Returns a new mutator that first executes the current operation, then
    /// executes the next operation. Consumes the current closure and returns
    /// `BoxStatefulMutator<T>`.
    ///
    /// # Parameters
    ///
    /// * `next` - The mutator to execute after the current operation. **Note:
    ///   This parameter is passed by value and will transfer ownership.** If
    ///   you need to preserve the original mutator, clone it first (if it
    ///   implements `Clone`). Can be:
    ///   - A closure: `|x: &mut T|`
    ///   - A `BoxStatefulMutator<T>`
    ///   - An `ArcStatefulMutator<T>`
    ///   - An `RcStatefulMutator<T>`
    ///   - Any type implementing `StatefulMutator<T>`
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxStatefulMutator<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnMutStatefulMutatorOps, StatefulMutator};
    ///
    /// let mut calls = 0;
    /// let mut chained = (move |x: &mut i32| {
    ///     calls += 1;
    ///     *x += calls;
    /// })
    /// .and_then(|x: &mut i32| *x *= 2)
    /// .and_then(|x: &mut i32| println!("Result: {x}"));
    ///
    /// let mut value = 10;
    /// chained.apply(&mut value); // Prints: Result: 22
    /// assert_eq!(value, 22);
    /// ```
    fn and_then<C>(self, next: C) -> BoxStatefulMutator<T>
    where
        Self: 'static,
        C: StatefulMutator<T> + 'static,
        T: 'static,
    {
        let mut first = self;
        let mut second = next;
        BoxStatefulMutator::new(move |t: &mut T| {
            (first)(t);
            second.apply(t);
        })
    }
}

/// Implements `FnMutStatefulMutatorOps` for all `FnMut` closure types.
impl<T, F> FnMutStatefulMutatorOps<T> for F where F: FnMut(&mut T) {}
