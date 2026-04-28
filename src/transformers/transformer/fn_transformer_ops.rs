/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnTransformerOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnTransformerOps - Extension trait for closure transformers
// ============================================================================

/// Extension trait for closures implementing `Fn(T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for closures
/// and function pointers without requiring explicit wrapping in
/// `BoxTransformer`, `RcTransformer`, or `ArcTransformer`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `Transformer<T, R>` through blanket
/// implementation, they don't have access to instance methods like `and_then`,
/// `compose`, and `when`. This extension trait provides those methods,
/// returning `BoxTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{Transformer, FnTransformerOps};
///
/// let double = |x: i32| x * 2;
/// let to_string = |x: i32| x.to_string();
///
/// let composed = double.and_then(to_string);
/// assert_eq!(composed.apply(21), "42");
/// ```
///
/// ## Reverse composition with compose
///
/// ```rust
/// use qubit_function::{Transformer, FnTransformerOps};
///
/// let double = |x: i32| x * 2;
/// let add_one = |x: i32| x + 1;
///
/// let composed = double.compose(add_one);
/// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
/// ```
///
/// ## Conditional transformation with when
///
/// ```rust
/// use qubit_function::{Transformer, FnTransformerOps};
///
/// let double = |x: i32| x * 2;
/// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnTransformerOps<T, R>: Fn(T) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new transformer that applies this transformer first, then
    /// applies the after transformer to the result. Consumes self and returns
    /// a `BoxTransformer`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `F` - The type of the after transformer (must implement Transformer<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original transformer, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxTransformer<R, S>`
    ///   - An `RcTransformer<R, S>`
    ///   - An `ArcTransformer<R, S>`
    ///   - Any type implementing `Transformer<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxTransformer<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{Transformer, FnTransformerOps, BoxTransformer};
    ///
    /// let double = |x: i32| x * 2;
    /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved here
    /// let composed = double.and_then(to_string);
    /// assert_eq!(composed.apply(21), "42");
    /// // to_string.apply(5); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with separate closures
    ///
    /// ```rust
    /// use qubit_function::{Transformer, FnTransformerOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let to_string = |x: i32| x.to_string();
    /// let to_string_for_validation = |x: i32| x.to_string();
    ///
    /// let composed = double.and_then(to_string);
    /// assert_eq!(composed.apply(21), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string_for_validation(5), "5");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxTransformer<T, S>
    where
        Self: 'static,
        S: 'static,
        F: Transformer<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |x: T| after.apply(self(x)))
    }

    /// Reverse composition - applies before first, then self
    ///
    /// Creates a new transformer that applies the before transformer first,
    /// then applies this transformer to the result. Consumes self and returns
    /// a `BoxTransformer`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The input type of the before transformer
    /// * `F` - The type of the before transformer (must implement Transformer<S, T>)
    ///
    /// # Parameters
    ///
    /// * `before` - The transformer to apply before self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original transformer, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: S| -> T`
    ///   - A function pointer: `fn(S) -> T`
    ///   - A `BoxTransformer<S, T>`
    ///   - An `RcTransformer<S, T>`
    ///   - An `ArcTransformer<S, T>`
    ///   - Any type implementing `Transformer<S, T>`
    ///
    /// # Returns
    ///
    /// A new `BoxTransformer<S, R>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{Transformer, FnTransformerOps, BoxTransformer};
    ///
    /// let double = |x: i32| x * 2;
    /// let add_one = BoxTransformer::new(|x: i32| x + 1);
    ///
    /// // add_one is moved here
    /// let composed = double.compose(add_one);
    /// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    /// // add_one.apply(3); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with separate closures
    ///
    /// ```rust
    /// use qubit_function::{Transformer, FnTransformerOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let add_one = |x: i32| x + 1;
    /// let add_one_for_validation = |x: i32| x + 1;
    ///
    /// let composed = double.compose(add_one);
    /// assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    ///
    /// // Original still usable
    /// assert_eq!(add_one_for_validation(3), 4);
    /// ```
    fn compose<S, F>(self, before: F) -> BoxTransformer<S, R>
    where
        Self: 'static,
        S: 'static,
        F: Transformer<S, T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(move |x: S| self(before.apply(x)))
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
    /// Returns `BoxConditionalTransformer<T, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{Transformer, FnTransformerOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
    ///
    /// assert_eq!(conditional.apply(5), 10);
    /// assert_eq!(conditional.apply(-5), 5);
    /// ```
    ///
    /// ## Preserving original with separate predicates
    ///
    /// ```rust
    /// use qubit_function::{Transformer, FnTransformerOps};
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
    fn when<P>(self, predicate: P) -> BoxConditionalTransformer<T, R>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformer::new(self).when(predicate)
    }
}

/// Blanket implementation of FnTransformerOps for all closures
///
/// Automatically implements `FnTransformerOps<T, R>` for any type that
/// implements `Fn(T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnTransformerOps<T, R> for F where F: Fn(T) -> R {}
