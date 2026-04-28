/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnTransformerOnceOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnTransformerOnceOps - Extension trait for FnOnce transformers
// ============================================================================

/// Extension trait for closures implementing `FnOnce(T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for one-time
/// use closures and function pointers without requiring explicit wrapping in
/// `BoxTransformerOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `TransformerOnce<T, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then`, `compose`, and `when`. This extension trait provides those
/// methods, returning `BoxTransformerOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{TransformerOnce, FnTransformerOnceOps};
///
/// let parse = |s: String| s.parse::<i32>().unwrap_or(0);
/// let double = |x: i32| x * 2;
///
/// let composed = parse.and_then(double);
/// assert_eq!(composed.apply("21".to_string()), 42);
/// ```
///
/// ## Forward composition with and_then
///
/// ```rust
/// use qubit_function::{TransformerOnce, FnTransformerOnceOps};
///
/// let double = |x: i32| x * 2;
/// let parse = |s: String| s.parse::<i32>().unwrap_or(0);
///
/// let composed = parse.and_then(double);
/// assert_eq!(composed.apply("21".to_string()), 42);
/// ```
///
/// ## Conditional transformation with when
///
/// ```rust
/// use qubit_function::{TransformerOnce, FnTransformerOnceOps};
///
/// let double = |x: i32| x * 2;
/// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
///
/// assert_eq!(conditional.apply(5), 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnTransformerOnceOps<T, R>: FnOnce(T) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new transformer that applies this transformer first, then
    /// applies the after transformer to the result. Consumes self and returns
    /// a `BoxTransformerOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `G` - The type of the after transformer (must implement
    ///   TransformerOnce<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` transformer, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxTransformerOnce<R, S>`
    ///   - Any type implementing `TransformerOnce<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxTransformerOnce<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{TransformerOnce, FnTransformerOnceOps,
    ///     BoxTransformerOnce};
    ///
    /// let parse = |s: String| s.parse::<i32>().unwrap_or(0);
    /// let double = BoxTransformerOnce::new(|x: i32| x * 2);
    ///
    /// // double is moved and consumed
    /// let composed = parse.and_then(double);
    /// assert_eq!(composed.apply("21".to_string()), 42);
    /// // double.apply(5); // Would not compile - moved
    /// ```
    fn and_then<S, G>(self, after: G) -> BoxTransformerOnce<T, S>
    where
        Self: 'static,
        S: 'static,
        G: TransformerOnce<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformerOnce::new(move |x: T| {
            let intermediate = self(x);
            after.apply(intermediate)
        })
    }

    /// Creates a conditional transformer
    ///
    /// Returns a transformer that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative transformer for when
    /// the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original predicate, clone it first (if it implements `Clone`). Can be:
    ///   - A closure: `|x: &T| -> bool`
    ///   - A function pointer: `fn(&T) -> bool`
    ///   - A `BoxPredicate<T>`
    ///   - An `RcPredicate<T>`
    ///   - An `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalTransformerOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{TransformerOnce, FnTransformerOnceOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
    ///
    /// assert_eq!(conditional.apply(5), 10);
    /// ```
    ///
    /// ## Preserving predicate with a second closure
    ///
    /// ```rust
    /// use qubit_function::{Predicate, TransformerOnce, FnTransformerOnceOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_positive_for_validation = |x: &i32| *x > 0;
    /// let conditional = double.when(is_positive)
    ///     .or_else(|x: i32| -x);
    ///
    /// assert_eq!(conditional.apply(5), 10);
    ///
    /// // Original predicate still usable
    /// assert!(is_positive_for_validation(&3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalTransformerOnce<T, R>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformerOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnTransformerOnceOps for all FnOnce closures
///
/// Automatically implements `FnTransformerOnceOps<T, R>` for any type that
/// implements `FnOnce(T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnTransformerOnceOps<T, R> for F where F: FnOnce(T) -> R {}
