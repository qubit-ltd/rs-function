/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnStatefulTransformerOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnStatefulTransformerOps - Extension trait for closure transformers
// ============================================================================

/// Extension trait for closures implementing `FnMut(T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for
/// closures without requiring explicit wrapping in `BoxStatefulTransformer`,
/// `RcStatefulTransformer`, or `ArcStatefulTransformer`.
///
/// This trait is automatically implemented for all closures that
/// implement `FnMut(T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `StatefulTransformer<T, R>` through blanket
/// implementation, they don't have access to instance methods like
/// `and_then`, `compose`, and `when`. This extension trait provides
/// those methods, returning `BoxStatefulTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{StatefulTransformer, FnStatefulTransformerOps, FnTransformerOps, Transformer};
///
/// let mut counter1 = 0;
/// let transformer1 = move |x: i32| {
///     counter1 += 1;
///     x + counter1
/// };
///
/// let mut counter2 = 0;
/// let transformer2 = move |x: i32| {
///     counter2 += 1;
///     x * counter2
/// };
///
/// let mut composed = FnStatefulTransformerOps::and_then(transformer1, transformer2);
/// assert_eq!(composed.apply(10), 11);  // (10 + 1) * 1
/// ```
///
/// ## Reverse composition with compose
///
/// ```rust
/// use qubit_function::{StatefulTransformer, FnStatefulTransformerOps, FnTransformerOps, Transformer};
///
/// let transformer = |x: i32| x * 2;
///
/// let mut composed = transformer.compose(|x: i32| x + 1);
/// assert_eq!(composed.apply(10), 22); // (10 + 1) * 2
/// ```
///
/// ## Conditional mapping with when
///
/// ```rust
/// use qubit_function::{StatefulTransformer, FnStatefulTransformerOps};
///
/// let mut transformer = (|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnStatefulTransformerOps<T, R>: FnMut(T) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new transformer that applies this transformer first, then applies
    /// the after transformer to the result. Consumes self and returns a
    /// `BoxStatefulTransformer`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `F` - The type of the after transformer (must implement StatefulTransformer<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A `BoxStatefulTransformer<R, S>`
    ///   - An `RcStatefulTransformer<R, S>`
    ///   - An `ArcStatefulTransformer<R, S>`
    ///   - Any type implementing `StatefulTransformer<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulTransformer<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, FnStatefulTransformerOps, BoxStatefulTransformer};
    ///
    /// let mut counter1 = 0;
    /// let transformer1 = move |x: i32| {
    ///     counter1 += 1;
    ///     x + counter1
    /// };
    ///
    /// let mut counter2 = 0;
    /// let transformer2 = BoxStatefulTransformer::new(move |x: i32| {
    ///     counter2 += 1;
    ///     x * counter2
    /// });
    ///
    /// let mut composed = transformer1.and_then(transformer2);
    /// assert_eq!(composed.apply(10), 11);
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxStatefulTransformer<T, S>
    where
        Self: 'static,
        S: 'static,
        F: StatefulTransformer<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulTransformer::new(self).and_then(after)
    }

    /// Creates a conditional transformer
    ///
    /// Returns a transformer that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative transformer for
    /// when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. Can be:
    ///   - A closure: `|x: &T| -> bool`
    ///   - A function pointer: `fn(&T) -> bool`
    ///   - A `BoxPredicate<T>`
    ///   - An `RcPredicate<T>`
    ///   - An `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalStatefulTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, FnStatefulTransformerOps};
    ///
    /// let mut transformer = (|x: i32| x * 2)
    ///     .when(|x: &i32| *x > 0)
    ///     .or_else(|x: i32| -x);
    ///
    /// assert_eq!(transformer.apply(5), 10);
    /// assert_eq!(transformer.apply(-5), 5);
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalStatefulTransformer<T, R>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulTransformer::new(self).when(predicate)
    }
}

/// Blanket implementation of FnStatefulTransformerOps for all closures
///
/// Automatically implements `FnStatefulTransformerOps<T, R>` for any type that
/// implements `FnMut(T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnStatefulTransformerOps<T, R> for F where F: FnMut(T) -> R {}
