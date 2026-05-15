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
//! Defines the `FnMutatorOps` public type.

use super::{
    BoxMutator,
    Mutator,
};

// ============================================================================
// 7. Provide extension methods for closures
// ============================================================================

/// Extension trait providing mutator composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures that
/// implement `Fn(&mut T)`, enabling direct method chaining on closures
/// without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxMutator**: Composition results are `BoxMutator<T>` for
///   continued chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `Fn(&mut T)` closures get these
///   methods automatically
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Mutator, FnMutatorOps};
///
/// let chained = (|x: &mut i32| *x *= 2)
///     .and_then(|x: &mut i32| *x += 10);
/// let mut value = 5;
/// chained.apply(&mut value);
/// assert_eq!(value, 20); // (5 * 2) + 10
/// ```
///
pub trait FnMutatorOps<T>: Fn(&mut T) + Sized {
    /// Chains another mutator in sequence
    ///
    /// Returns a new mutator that first executes the current operation, then
    /// executes the next operation. Consumes the current closure and returns
    /// `BoxMutator<T>`.
    ///
    /// # Parameters
    ///
    /// * `next` - The mutator to execute after the current operation. **Note:
    ///   This parameter is passed by value and will transfer ownership.** If you
    ///   need to preserve the original mutator, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: &mut T|`
    ///   - A `BoxMutator<T>`
    ///   - An `ArcMutator<T>`
    ///   - An `RcMutator<T>`
    ///   - Any type implementing `Mutator<T>`
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxMutator<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Mutator, FnMutatorOps};
    ///
    /// let chained = (|x: &mut i32| *x *= 2)
    ///     .and_then(|x: &mut i32| *x += 10);
    ///
    /// let mut value = 5;
    /// chained.apply(&mut value);
    /// assert_eq!(value, 20);
    /// ```
    fn and_then<C>(self, next: C) -> BoxMutator<T>
    where
        Self: 'static,
        C: Mutator<T> + 'static,
        T: 'static,
    {
        let first = self;
        let second = next.into_fn();
        BoxMutator::new(move |t| {
            (first)(t);
            (second)(t);
        })
    }
}

/// Implements FnMutatorOps for all closure types
impl<T, F> FnMutatorOps<T> for F where F: Fn(&mut T) {}
