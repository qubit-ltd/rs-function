// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BiFunctionOnce` trait.

/// A consuming callback that computes a value from two borrowed inputs.
///
/// Implementations receive shared references to the input values and consume
/// `self`, matching the ownership behavior of `FnOnce(&T, &U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
/// * `R` - The type of the computed output.
///
/// # Examples
///
/// ```
/// use qubit_function::{BiFunctionOnce, BoxBiFunctionOnce};
///
/// let first = String::from("hello");
/// let second = String::from(" world");
/// let append = BoxBiFunctionOnce::new(|first: &String, second: &String| {
///     format!("{first}{second}")
/// });
/// assert_eq!(append.apply(&first, &second), "hello world");
/// ```
pub trait BiFunctionOnce<T, U, R> {
    /// Computes an output from two borrowed inputs and consumes the callback.
    ///
    /// # Parameters
    ///
    /// * `first` - Shared reference to the first input value.
    /// * `second` - Shared reference to the second input value.
    ///
    /// # Returns
    ///
    /// The value computed from the inputs.
    #[must_use = "the computed callback result should be used"]
    fn apply(self, first: &T, second: &U) -> R;
}
