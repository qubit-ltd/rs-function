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
//! Defines the `FnBiFunctionOnceOps` public type.

use super::{
    BiPredicate,
    BoxBiFunctionOnce,
    BoxConditionalBiFunctionOnce,
};

// ============================================================================
// FnBiFunctionOnceOps - Extension trait for FnOnce(&T, &U) -> R bi-functions
// ============================================================================

/// Extension trait for closures implementing `FnOnce(&T, &U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for one-time use
/// bi-function closures and function pointers without requiring explicit
/// wrapping in `BoxBiFunctionOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(&T, &U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiFunctionOnce<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiFunctionOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps, FunctionOnce};
///
/// let add = |x: &i32, y: &i32| *x + *y;
/// let double = |x: &i32| x * 2;
///
/// let composed = add.and_then(double);
/// assert_eq!(composed.apply(&3, &5), 16); // (3 + 5) * 2
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps};
///
/// let add = |x: &i32, y: &i32| *x + *y;
/// let multiply = |x: &i32, y: &i32| *x * *y;
///
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// assert_eq!(conditional.apply(&5, &3), 8); // when branch executed
/// ```
///
pub trait FnBiFunctionOnceOps<T, U, R>: FnOnce(&T, &U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-function that applies this bi-function first,
    /// then applies the after function to the result. Consumes self and
    /// returns a `BoxBiFunctionOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement FunctionOnce<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` bi-function, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxFunctionOnce<R, S>`
    ///   - Any type implementing `FunctionOnce<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiFunctionOnce<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps,
    ///     BoxFunctionOnce};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let to_string = BoxFunctionOnce::new(|x: &i32| x.to_string());
    ///
    /// // to_string is moved and consumed
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(&20, &22), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiFunctionOnce<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::functions::function_once::FunctionOnce<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunctionOnce::new(move |t: &T, u: &U| after.apply(&self(t, u)))
    }

    /// Creates a conditional bi-function
    ///
    /// Returns a bi-function that only executes when a bi-predicate is
    /// satisfied. You must call `or_else()` to provide an alternative
    /// bi-function for when the condition is not satisfied.
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
    /// Returns `BoxConditionalBiFunctionOnce<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let multiply = |x: &i32, y: &i32| *x * *y;
    /// let conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(multiply);
    ///
    /// assert_eq!(conditional.apply(&5, &3), 8);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps,
    ///     RcBiPredicate};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let both_positive = RcBiPredicate::new(|x: &i32, y: &i32|
    ///     *x > 0 && *y > 0);
    ///
    /// // Clone to preserve original bi-predicate
    /// let conditional = add.when(both_positive.clone())
    ///     .or_else(|x: &i32, y: &i32| *x * *y);
    ///
    /// assert_eq!(conditional.apply(&5, &3), 8);
    ///
    /// // Original bi-predicate still usable
    /// use qubit_function::BiPredicate;
    /// assert!(both_positive.test(&5, &3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiFunctionOnce<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunctionOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiFunctionOnceOps for all closures
///
/// Automatically implements `FnBiFunctionOnceOps<T, U, R>` for any type that
/// implements `FnOnce(&T, &U) -> R`.
///
impl<T, U, R, F> FnBiFunctionOnceOps<T, U, R> for F
where
    F: FnOnce(&T, &U) -> R,
{
    // empty
}
