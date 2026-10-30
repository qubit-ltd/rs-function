// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

/// Computes an output from two borrowed input values.
///
/// The inputs remain owned by the caller; implementations derive and return a
/// value of type `R` without consuming either input. This is analogous to
/// `Fn(&T, &U) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` — Type of the first borrowed input.
/// * `U` — Type of the second borrowed input.
/// * `R` — Type of the computed output.
///
/// # Examples
///
/// ```
/// use qubit_function::{BiFunction, BoxBiFunction};
///
/// let add = BoxBiFunction::new(|first: &i32, second: &i32| first + second);
/// assert_eq!(add.apply(&2, &3), 5);
/// ```
pub trait BiFunction<T, U, R> {
    /// Computes an output from references to the two input values.
    ///
    /// # Parameters
    ///
    /// * `first` — Reference to the first input.
    /// * `second` — Reference to the second input.
    ///
    /// # Returns
    ///
    /// The output derived from both inputs.
    #[must_use = "the computed callback result should be used"]
    fn apply(&self, first: &T, second: &U) -> R;
}

crate::macros::impl_closure_trait!(
    BiFunction<T, U, R>,
    apply,
    Fn(first: &T, second: &U) -> R
);
