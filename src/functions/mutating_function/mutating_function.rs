// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the shared interface implemented by mutable callbacks.

/// MutatingFunction trait - Unified mutating function interface
///
/// It is similar to the `Fn(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all mutating function types. Performs
/// operations that accept a mutable reference, potentially modify it, and
/// return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `Fn(&mut T) -> R`
/// - `BoxMutatingFunction<T, R>`, `ArcMutatingFunction<T, R>`, and
///   `RcMutatingFunction<T, R>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models
/// for operations that both modify input and return results. This is useful
/// for scenarios where you need to:
/// - Update state and return information about the update
/// - Perform atomic-like operations (modify and return)
/// - Implement event handlers that modify state and signal continuation
///
/// # Features
///
/// - **Unified Interface**: All mutating function types share the same `apply`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this trait
/// - **Generic Programming**: Write functions that work with any mutating
///   function type
///
/// # Type Parameters
///
/// * `T` - The input type that the function may modify through a mutable
///   reference.
/// * `R` - The value returned after the function processes the input.
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatingFunction, BoxMutatingFunction};
///
/// fn apply_and_log<F: MutatingFunction<i32, i32>>(
///     func: &F,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     let result = func.apply(&mut val);
///     println!("Modified: {} -> {}, returned: {}", value, val, result);
///     result
/// }
///
/// let incrementer = BoxMutatingFunction::new(|x: &mut i32| {
///     *x += 1;
///     *x
/// });
/// assert_eq!(apply_and_log(&incrementer, 5), 6);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::BoxMutatingFunction;
///
/// let closure = |x: &mut i32| {
///     *x *= 2;
///     *x
/// };
///
/// // Convert to different ownership models
/// let box_func = BoxMutatingFunction::new(closure);
/// ```
pub trait MutatingFunction<T, R> {
    /// Applies the function to the mutable reference and returns a result
    ///
    /// Executes an operation on the given mutable reference, potentially
    /// modifying it, and returns a result value.
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
    /// use qubit_function::{MutatingFunction, BoxMutatingFunction};
    ///
    /// let func = BoxMutatingFunction::new(|x: &mut i32| {
    ///     let old = *x;
    ///     *x += 1;
    ///     old
    /// });
    ///
    /// let mut value = 5;
    /// let old_value = func.apply(&mut value);
    /// assert_eq!(old_value, 5);
    /// assert_eq!(value, 6);
    /// ```
    #[must_use = "the computed callback result should be used"]
    fn apply(&self, t: &mut T) -> R;
}

crate::macros::impl_closure_trait!(
    MutatingFunction<T, R>,
    apply,
    Fn(input: &mut T) -> R
);
