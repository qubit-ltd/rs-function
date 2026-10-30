// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulBiTransformer` trait.

/// StatefulBiTransformer trait - transforms two values to produce a result
///
/// Defines the behavior of a bi-transformation: converting two values of types
/// `T` and `U` to a value of type `R` by consuming the inputs. This is
/// analogous to `FnMut(T, U) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (consumed)
/// * `U` - The type of the second input value (consumed)
/// * `R` - The type of the output value
///
/// # Examples
///
/// A mutable closure implements `StatefulBiTransformer` and can retain state
/// between calls:
///
/// ```
/// use qubit_function::StatefulBiTransformer;
///
/// let mut total = 0;
/// let mut add_and_accumulate = |first, second| {
///     total += first + second;
///     total
/// };
///
/// assert_eq!(add_and_accumulate.apply(2, 3), 5);
/// assert_eq!(add_and_accumulate.apply(1, 4), 10);
/// ```
pub trait StatefulBiTransformer<T, U, R> {
    /// Transforms two input values to produce an output value
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value to transform (consumed)
    /// * `second` - The second input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    #[must_use = "the computed callback result should be used"]
    fn apply(&mut self, first: T, second: U) -> R;
}

crate::macros::impl_closure_trait!(
    StatefulBiTransformer<T, U, R>,
    apply,
    FnMut(first: T, second: U) -> R
);
