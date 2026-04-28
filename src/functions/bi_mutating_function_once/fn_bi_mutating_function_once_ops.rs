/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnBiMutatingFunctionOnceOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnBiMutatingFunctionOnceOps - Extension trait for FnOnce(&mut T, &mut U) -> R bi-functions
// ============================================================================

/// Extension trait for closures implementing `FnOnce(&mut T, &mut U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for one-time use
/// bi-mutating-function closures and function pointers without requiring explicit
/// wrapping in `BoxBiMutatingFunctionOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(&mut T, &mut U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiMutatingFunctionOnce<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiMutatingFunctionOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps, FunctionOnce};
///
/// let swap_and_sum = |x: &mut i32, y: &mut i32| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// };
/// let double = |x: &i32| x * 2;
///
/// let composed = swap_and_sum.and_then(double);
/// let mut a = 3;
/// let mut b = 5;
/// assert_eq!(composed.apply(&mut a, &mut b), 16); // (5 + 3) * 2 = 16
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use qubit_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps};
///
/// let swap_and_sum = |x: &mut i32, y: &mut i32| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// };
/// let multiply = |x: &mut i32, y: &mut i32| {
///     *x *= *y;
///     *x
/// };
///
/// let conditional = swap_and_sum.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// let mut a = 5;
/// let mut b = 3;
/// assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed
///
/// let conditional2 = swap_and_sum.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// let mut a = -5;
/// let mut b = 3;
/// assert_eq!(conditional2.apply(&mut a, &mut b), -15); // multiply executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiMutatingFunctionOnceOps<T, U, R>: FnOnce(&mut T, &mut U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-mutating-function that applies this bi-mutating-function first,
    /// then applies the after function to the result. Consumes self and
    /// returns a `BoxBiMutatingFunctionOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement Function<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` bi-mutating-function, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxFunction<R, S>`
    ///   - An `RcFunction<R, S>`
    ///   - An `ArcFunction<R, S>`
    ///   - Any type implementing `Function<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiMutatingFunctionOnce<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps,
    ///     BoxFunction};
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let to_string = BoxFunction::new(|x: &i32| x.to_string());
    ///
    /// // to_string is moved and consumed
    /// let composed = swap_and_sum.and_then(to_string);
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(composed.apply(&mut a, &mut b), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiMutatingFunctionOnce<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::functions::function::Function<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunctionOnce::new(move |t: &mut T, u: &mut U| after.apply(&self(t, u)))
    }

    /// Creates a conditional bi-mutating-function
    ///
    /// Returns a bi-mutating-function that only executes when a bi-predicate is
    /// satisfied. You must call `or_else()` to provide an alternative
    /// bi-mutating-function for when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original bi-predicate, clone it first (if it implements `Clone`).
    ///   Can be:
    ///   - A closure: `|x: &mut T, y: &mut U| -> bool`
    ///   - A function pointer: `fn(&mut T, &mut U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalBiMutatingFunctionOnce<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps};
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let multiply = |x: &mut i32, y: &mut i32| {
    ///     *x *= *y;
    ///     *x
    /// };
    /// let conditional = swap_and_sum.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(multiply);
    ///
    /// let mut a = 5;
    /// let mut b = 3;
    /// assert_eq!(conditional.apply(&mut a, &mut b), 8);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use qubit_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps,
    ///     RcBiPredicate};
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let both_positive = RcBiPredicate::new(|x: &i32, y: &i32|
    ///     *x > 0 && *y > 0);
    ///
    /// // Clone to preserve original bi-predicate
    /// let conditional = swap_and_sum.when(both_positive.clone())
    ///     .or_else(|x: &mut i32, y: &mut i32| *x * *y);
    ///
    /// let mut a = 5;
    /// let mut b = 3;
    /// assert_eq!(conditional.apply(&mut a, &mut b), 8);
    ///
    /// // Original bi-predicate still usable
    /// use qubit_function::BiPredicate;
    /// let test_a = 5;
    /// let test_b = 3;
    /// assert!(both_positive.test(&test_a, &test_b));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiMutatingFunctionOnce<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunctionOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiMutatingFunctionOnceOps for all closures
///
/// Automatically implements `FnBiMutatingFunctionOnceOps<T, U, R>` for any type that
/// implements `FnOnce(&mut T, &mut U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiMutatingFunctionOnceOps<T, U, R> for F
where
    F: FnOnce(&mut T, &mut U) -> R,
{
    // empty
}
