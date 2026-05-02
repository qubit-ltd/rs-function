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
//! Defines the `FnBiTransformerOnceOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnBiTransformerOnceOps - Extension trait for FnOnce(T, U) -> R bi-transformers
// ============================================================================

/// Extension trait for closures implementing `FnOnce(T, U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for one-time use
/// bi-transformer closures and function pointers without requiring explicit
/// wrapping in `BoxBiTransformerOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(T, U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiTransformerOnce<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiTransformerOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiTransformerOnce, FnBiTransformerOnceOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let double = |x: i32| x * 2;
///
/// let composed = add.and_then(double);
/// assert_eq!(composed.apply(3, 5), 16); // (3 + 5) * 2
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use qubit_function::{BiTransformerOnce, FnBiTransformerOnceOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let multiply = |x: i32, y: i32| x * y;
///
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// assert_eq!(conditional.apply(5, 3), 8); // add
/// ```
///
pub trait FnBiTransformerOnceOps<T, U, R>: FnOnce(T, U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-transformer that applies this bi-transformer first,
    /// then applies the after transformer to the result. Consumes self and
    /// returns a `BoxBiTransformerOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `F` - The type of the after transformer (must implement TransformerOnce<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` bi-transformer, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxTransformerOnce<R, S>`
    ///   - Any type implementing `TransformerOnce<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiTransformerOnce<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiTransformerOnce, FnBiTransformerOnceOps,
    ///     BoxTransformerOnce};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = BoxTransformerOnce::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved and consumed
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(20, 22), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiTransformerOnce<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::transformers::transformer_once::TransformerOnce<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformerOnce::new(move |t: T, u: U| after.apply(self(t, u)))
    }

    /// Creates a conditional bi-transformer
    ///
    /// Returns a bi-transformer that only executes when a bi-predicate is
    /// satisfied. You must call `or_else()` to provide an alternative
    /// bi-transformer for when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original bi-predicate, clone it first (if it implements `Clone`).
    ///   Can be:
    ///   - A closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalBiTransformerOnce<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiTransformerOnce, FnBiTransformerOnceOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let multiply = |x: i32, y: i32| x * y;
    /// let conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(multiply);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    /// ```
    ///
    /// ## Preserving original with separate bi-predicates
    ///
    /// ```rust
    /// use qubit_function::{BiTransformerOnce, FnBiTransformerOnceOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let both_positive = |x: &i32, y: &i32| *x > 0 && *y > 0;
    /// let both_positive_for_validation = |x: &i32, y: &i32| *x > 0 && *y > 0;
    ///
    /// let conditional = add.when(both_positive)
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    ///
    /// // Original bi-predicate still usable
    /// assert!(both_positive_for_validation(&5, &3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiTransformerOnce<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformerOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiTransformerOnceOps for all closures
///
/// Automatically implements `FnBiTransformerOnceOps<T, U, R>` for any type that
/// implements `FnOnce(T, U) -> R`.
///
impl<T, U, R, F> FnBiTransformerOnceOps<T, U, R> for F where F: FnOnce(T, U) -> R {}
