// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `MutatingFunctionOnce` trait.

/// MutatingFunctionOnce trait - One-time mutating function interface
///
/// It is similar to the `FnOnce(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all one-time mutating function types.
/// Performs operations that consume self, accept a mutable reference,
/// potentially modify it, and return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnOnce(&mut T) -> R`
/// - `BoxMutatingFunctionOnce<T, R>`
///
/// # Design Rationale
///
/// This trait provides a unified abstraction for one-time mutating function
/// operations. The key difference from `MutatingFunction`:
/// - `MutatingFunction` uses `&self`, can be called multiple times
/// - `MutatingFunctionOnce` uses `self`, can only be called once
///
/// # Type Parameters
///
/// * `T` - The type of the value the operation may mutate.
/// * `R` - The result type produced by the operation.
///
/// # Features
///
/// - **Unified Interface**: All one-time mutating functions share the same
///   `apply` method signature
/// - **Automatic Implementation**: Closures implement this trait directly,
///   without allocating an adapter
/// - **Generic Programming**: Write functions that work with any one-time
///   mutating function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// fn apply<F: MutatingFunctionOnce<Vec<i32>, usize>>(
///     func: F,
///     initial: Vec<i32>
/// ) -> (Vec<i32>, usize) {
///     let mut val = initial;
///     let result = func.apply(&mut val);
///     (val, result)
/// }
///
/// let data = vec![1, 2, 3];
/// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data);
///     old_len
/// });
/// let (vec, old_len) = apply(func, vec![0]);
/// assert_eq!(vec, vec![0, 1, 2, 3]);
/// assert_eq!(old_len, 1);
/// ```
///
/// ## Wrapper Construction
///
/// ```rust
/// use qubit_function::BoxMutatingFunctionOnce;
///
/// let data = vec![1, 2, 3];
/// let closure = move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data);
///     old_len
/// };
/// let box_func = BoxMutatingFunctionOnce::new(closure);
/// ```
pub trait MutatingFunctionOnce<T, R> {
    /// Performs the one-time mutating function operation
    ///
    /// Consumes self and executes an operation on the given mutable
    /// reference, potentially modifying it, and returns a result. The
    /// operation can only be called once.
    ///
    /// # Parameters
    ///
    /// * `t` - The value the operation may inspect and modify.
    ///
    /// # Returns
    ///
    /// The result produced by the one-time operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunctionOnce,
    ///                       BoxMutatingFunctionOnce};
    ///
    /// let data = vec![1, 2, 3];
    /// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
    ///     let old_len = x.len();
    ///     x.extend(data);
    ///     old_len
    /// });
    ///
    /// let mut target = vec![0];
    /// let old_len = func.apply(&mut target);
    /// assert_eq!(old_len, 1);
    /// assert_eq!(target, vec![0, 1, 2, 3]);
    /// ```
    #[must_use = "the computed callback result should be used"]
    fn apply(self, t: &mut T) -> R;
}
