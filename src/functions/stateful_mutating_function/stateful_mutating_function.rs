// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulMutatingFunction` trait and its blanket implementation.

// =======================================================================
// 1. StatefulMutatingFunction Trait - Unified Interface
// =======================================================================

/// StatefulMutatingFunction trait - Unified stateful mutating function
/// interface
///
/// It is similar to the `FnMut(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all stateful mutating function types.
/// Performs operations that accept a mutable reference, potentially modify
/// both the function's internal state and the input, and return a result.
///
/// # Type Parameters
///
/// * `T` - The type of the mutable input value.
/// * `R` - The type of the result returned after applying the function.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T) -> R`
/// - `BoxStatefulMutatingFunction<T, R>`, `ArcStatefulMutatingFunction<T, R>`,
///   and `RcStatefulMutatingFunction<T, R>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models
/// for operations that need to maintain state while modifying input and
/// returning results. This is useful for scenarios where you need to:
/// - Track statistics or counts during modifications
/// - Accumulate information across multiple calls
/// - Implement stateful validators or transformers
///
/// # Features
///
/// - **Unified Interface**: All stateful mutating function types share the same
///   `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this trait
/// - **Generic Programming**: Write functions that work with any stateful
///   mutating function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       BoxStatefulMutatingFunction};
///
/// fn apply_and_log<F: StatefulMutatingFunction<i32, i32>>(
///     func: &mut F,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     let result = func.apply(&mut val);
///     println!("Modified: {} -> {}, returned: {}", value, val, result);
///     result
/// }
///
/// let mut counter = {
///     let mut count = 0;
///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x += 1;
///         count
///     })
/// };
/// assert_eq!(apply_and_log(&mut counter, 5), 1);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::{BoxStatefulMutatingFunction, StatefulMutatingFunction};
///
/// let mut count = 0;
/// let closure = move |x: &mut i32| {
///     count += 1;
///     *x *= 2;
///     count
/// };
///
/// // Convert to different ownership models
/// let mut box_func = BoxStatefulMutatingFunction::new(closure);
/// ```
pub trait StatefulMutatingFunction<T, R> {
    /// Applies the function to the mutable reference and returns a result
    ///
    /// Executes an operation on the given mutable reference, potentially
    /// modifying both the function's internal state and the input, and
    /// returns a result value.
    ///
    /// # Parameters
    ///
    /// * `t` - A mutable reference to the input value
    ///
    /// # Returns
    ///
    /// The computed result value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// let mut counter = {
    ///     let mut count = 0;
    ///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
    ///         count += 1;
    ///         let old = *x;
    ///         *x += 1;
    ///         (old, count)
    ///     })
    /// };
    ///
    /// let mut value = 5;
    /// let (old_value, call_count) = counter.apply(&mut value);
    /// assert_eq!(old_value, 5);
    /// assert_eq!(call_count, 1);
    /// assert_eq!(value, 6);
    /// ```
    #[must_use = "the computed callback result should be used"]
    fn apply(&mut self, t: &mut T) -> R;
}

impl<T, R, F> StatefulMutatingFunction<T, R> for F
where
    F: FnMut(&mut T) -> R,
{
    /// Forwards the mutable input to the closure and returns its result.
    #[inline(always)]
    fn apply(&mut self, t: &mut T) -> R {
        self(t)
    }
}
