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
//! Defines the `FnStatefulBiTransformerOps` public type.

use super::{
    BiPredicate,
    BoxConditionalStatefulBiTransformer,
    BoxStatefulBiTransformer,
    StatefulTransformer,
};

// ============================================================================
// FnStatefulBiTransformerOps - Extension trait for FnMut(T, U) -> R bi-transformers
// ============================================================================

/// Extension trait for closures implementing `FnMut(T, U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for bi-transformer
/// closures and function pointers without requiring explicit wrapping in
/// `BoxStatefulBiTransformer`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnMut(T, U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `StatefulBiTransformer<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxStatefulBiTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use std::cell::Cell;
/// use qubit_function::{StatefulBiTransformer, FnStatefulBiTransformerOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let double = |x: i32| x * 2;
///
/// let mut composed = add.and_then(double);
/// assert_eq!(composed.apply(3, 5), 16); // (3 + 5) * 2
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use std::cell::Cell;
/// use qubit_function::{StatefulBiTransformer, FnStatefulBiTransformerOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let multiply = |x: i32, y: i32| x * y;
///
/// let mut conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(5, 3), 8);   // add
/// assert_eq!(conditional.apply(-5, 3), -15); // multiply
/// ```
///
pub trait FnStatefulBiTransformerOps<T, U, R>: FnMut(T, U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-transformer that applies this bi-transformer first,
    /// then applies the after transformer to the result. Consumes self and
    /// returns a `BoxStatefulBiTransformer`.
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
    /// A new `BoxStatefulBiTransformer<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{StatefulBiTransformer, FnStatefulBiTransformerOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = |x: i32| x.to_string();
    ///
    /// // to_string is moved here
    /// let mut composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(20, 22), "42");
    /// // to_string(10); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use qubit_function::{StatefulBiTransformer, FnStatefulBiTransformerOps,
    ///     BoxTransformer};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = |x: i32| x.to_string();
    ///
    /// // Clone to preserve original
    /// let mut composed = add.and_then(to_string.clone());
    /// assert_eq!(composed.apply(20, 22), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string(10), "10");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxStatefulBiTransformer<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: StatefulTransformer<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxStatefulBiTransformer::new(self).and_then(after)
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
    /// Returns `BoxConditionalStatefulBiTransformer<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use qubit_function::{StatefulBiTransformer, FnStatefulBiTransformerOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let mut conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    /// assert_eq!(conditional.apply(-5, 3), -15);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use qubit_function::{BiPredicate, StatefulBiTransformer, FnStatefulBiTransformerOps,
    ///     RcBiPredicate};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let both_positive = RcBiPredicate::new(|x: &i32, y: &i32|
    ///     *x > 0 && *y > 0);
    ///
    /// // Clone to preserve original bi-predicate
    /// let mut conditional = add.when(both_positive.clone())
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    ///
    /// // Original bi-predicate still usable
    /// assert!(both_positive.test(&5, &3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalStatefulBiTransformer<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxStatefulBiTransformer::new(self).when(predicate)
    }

    /// Non-consuming conversion to a function using `&self`.
    ///
    /// Returns a closure that clones `self` and calls the bi-transformer.
    /// This method requires that the bi-transformer implements `Clone`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The closure type (automatically inferred)
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnMut(T, U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use qubit_function::{StatefulBiTransformer, FnStatefulBiTransformerOps};
    ///
    /// let counter = Cell::new(0);
    /// let transformer = move |x: i32, y: i32| {
    ///     let c = counter.get() + 1;
    ///     counter.set(c);
    ///     x + y + c
    /// };
    ///
    /// let mut fn_transformer = FnStatefulBiTransformerOps::to_fn(&transformer);
    /// assert_eq!(fn_transformer(10, 20), 31);
    /// assert_eq!(fn_transformer(10, 20), 32);
    /// ```
    fn to_fn(&self) -> impl FnMut(T, U) -> R
    where
        Self: Clone + 'static,
    {
        let mut cloned = self.clone();
        move |t, u| cloned(t, u)
    }
}

/// Blanket implementation of FnStatefulBiTransformerOps for all closures
///
/// Automatically implements `FnStatefulBiTransformerOps<T, U, R>` for any type that
/// implements `FnMut(T, U) -> R`.
///
impl<T, U, R, F> FnStatefulBiTransformerOps<T, U, R> for F where F: FnMut(T, U) -> R {}
