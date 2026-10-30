// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BiMutatingFunctionOnce` trait for consuming operations over
//! two mutable inputs.

/// Defines an operation that consumes itself, mutates two borrowed inputs, and
/// produces a result.
///
/// Implementations receive exclusive access to both inputs for one call, and
/// the operation cannot be reused after that call.
///
/// # Type Parameters
///
/// * `T` - Type of the first mutable input.
/// * `U` - Type of the second mutable input.
/// * `R` - Type of the computed result.
///
/// # Examples
///
/// ```
/// use qubit_function::BiMutatingFunctionOnce;
///
/// let add_second = |first: &mut i32, second: &mut i32| {
///     *first += *second;
///     *first
/// };
/// let mut first = 2;
/// let mut second = 3;
///
/// assert_eq!(add_second.apply(&mut first, &mut second), 5);
/// assert_eq!(first, 5);
/// assert_eq!(second, 3);
/// ```
pub trait BiMutatingFunctionOnce<T, U, R> {
    /// Computes a result from two mutable inputs and consumes the operation.
    ///
    /// # Parameters
    ///
    /// * `first` - First input, which the implementation may modify.
    /// * `second` - Second input, which the implementation may modify.
    ///
    /// # Returns
    ///
    /// The value computed from the inputs after any mutations.
    #[must_use = "the computed callback result should be used"]
    fn apply(self, first: &mut T, second: &mut U) -> R;
}
