/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnBiTransformerOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnBiTransformerOps - Extension trait for Fn(T, U) -> R bi-transformers
// ============================================================================

/// Extension trait for closures implementing `Fn(T, U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for bi-transformer
/// closures and function pointers without requiring explicit wrapping in
/// `BoxBiTransformer`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(T, U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiTransformer<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiTransformer` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiTransformer, FnBiTransformerOps};
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
/// use qubit_function::{BiTransformer, FnBiTransformerOps};
///
/// let add = |x: i32, y: i32| x + y;
/// let multiply = |x: i32, y: i32| x * y;
///
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(5, 3), 8);   // add
/// assert_eq!(conditional.apply(-5, 3), -15); // multiply
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiTransformerOps<T, U, R>: Fn(T, U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-transformer that applies this bi-transformer first,
    /// then applies the after transformer to the result. Consumes self and
    /// returns a `BoxBiTransformer`.
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
    /// A new `BoxBiTransformer<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps,
    ///     BoxTransformer};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved here
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(20, 22), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with separate transformers
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let to_string = |x: i32| x.to_string();
    /// let to_string_for_validation = |x: i32| x.to_string();
    ///
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(20, 22), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string_for_validation(10), "10");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiTransformer<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::transformers::transformer::Transformer<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformer::new(move |t: T, u: U| after.apply(self(t, u)))
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
    /// Returns `BoxConditionalBiTransformer<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    /// assert_eq!(conditional.apply(-5, 3), -15);
    /// ```
    ///
    /// ## Preserving original with separate bi-predicates
    ///
    /// ```rust
    /// use qubit_function::{BiTransformer, FnBiTransformerOps};
    ///
    /// let add = |x: i32, y: i32| x + y;
    /// let both_positive = |x: &i32, y: &i32| *x > 0 && *y > 0;
    /// let both_positive_for_validation = |x: &i32, y: &i32|
    ///     *x > 0 && *y > 0;
    ///
    /// let conditional = add.when(both_positive)
    ///     .or_else(|x: i32, y: i32| x * y);
    ///
    /// assert_eq!(conditional.apply(5, 3), 8);
    ///
    /// // Original bi-predicate still usable
    /// assert!(both_positive_for_validation(&5, &3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiTransformer<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiTransformer::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiTransformerOps for all closures
///
/// Automatically implements `FnBiTransformerOps<T, U, R>` for any type that
/// implements `Fn(T, U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiTransformerOps<T, U, R> for F where F: Fn(T, U) -> R {}
