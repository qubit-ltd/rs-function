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
//! Defines the `FnMutatorOnceOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 4. Provide extension methods for closures
// ============================================================================

/// Extension trait providing one-time mutator composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures that
/// implement `FnOnce(&mut T)`, enabling direct method chaining on closures
/// without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxMutatorOnce**: Composition results are `BoxMutatorOnce<T>`
///   for continued chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `FnOnce(&mut T)` closures get these
///   methods automatically
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatorOnce, FnMutatorOnceOps};
///
/// let data1 = vec![1, 2];
/// let data2 = vec![3, 4];
///
/// let chained = (move |x: &mut Vec<i32>| x.extend(data1))
///     .and_then(move |x: &mut Vec<i32>| x.extend(data2));
///
/// let mut target = vec![0];
/// chained.apply(&mut target);
/// assert_eq!(target, vec![0, 1, 2, 3, 4]);
/// ```
///
pub trait FnMutatorOnceOps<T>: FnOnce(&mut T) + Sized {
    /// Chains another mutator in sequence
    ///
    /// Returns a new mutator that first executes the current operation, then
    /// executes the next operation. Consumes the current closure and returns
    /// `BoxMutatorOnce<T>`.
    ///
    /// # Parameters
    ///
    /// * `next` - The mutator to execute after the current operation. **Note: This
    ///   parameter is passed by value and will transfer ownership.** Since
    ///   `BoxMutatorOnce` cannot be cloned, the parameter will be consumed.
    ///   Can be:
    ///   - A closure: `|x: &mut T|`
    ///   - A `BoxMutatorOnce<T>`
    ///   - Any type implementing `MutatorOnce<T>`
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxMutatorOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatorOnce, FnMutatorOnceOps};
    ///
    /// let data1 = vec![1, 2];
    /// let data2 = vec![3, 4];
    ///
    /// // Both closures are moved and consumed
    /// let chained = (move |x: &mut Vec<i32>| x.extend(data1))
    ///     .and_then(move |x: &mut Vec<i32>| x.extend(data2));
    ///
    /// let mut target = vec![0];
    /// chained.apply(&mut target);
    /// assert_eq!(target, vec![0, 1, 2, 3, 4]);
    /// // The original closures are consumed and no longer usable
    /// ```
    fn and_then<C>(self, next: C) -> BoxMutatorOnce<T>
    where
        Self: 'static,
        C: MutatorOnce<T> + 'static,
        T: 'static,
    {
        BoxMutatorOnce::new(move |t| {
            self(t);
            next.apply(t);
        })
    }
}

/// Implements FnMutatorOnceOps for all closure types
impl<T, F> FnMutatorOnceOps<T> for F where F: FnOnce(&mut T) {}
