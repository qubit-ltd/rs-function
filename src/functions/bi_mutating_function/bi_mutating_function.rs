// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BiMutatingFunction` trait for reusable operations over two
//! mutable inputs.

/// Defines a reusable operation that can mutate two borrowed inputs and
/// produce a result.
///
/// Implementations receive exclusive access to both inputs for each call,
/// allowing them to update the inputs while computing the returned value.
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
/// use qubit_function::BiMutatingFunction;
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
pub trait BiMutatingFunction<T, U, R> {
    /// Applies the operation to both mutable inputs and returns its result.
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
    fn apply(&self, first: &mut T, second: &mut U) -> R;
}

crate::macros::impl_closure_trait!(
    BiMutatingFunction<T, U, R>,
    apply,
    Fn(first: &mut T, second: &mut U) -> R
);
