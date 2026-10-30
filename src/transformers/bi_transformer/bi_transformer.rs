// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BiTransformer` trait.

/// Transforms two owned input values into a result.
///
/// A bi-transformer converts values of types `T` and `U` into a value of type
/// `R`, consuming both inputs. This is analogous to `Fn(T, U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first consumed input value.
/// * `U` - The type of the second consumed input value.
/// * `R` - The type of the output value.
///
/// # Examples
///
/// ```
/// use qubit_function::BiTransformer;
///
/// let add = |first, second| first + second;
/// assert_eq!(add.apply(2, 3), 5);
/// ```
pub trait BiTransformer<T, U, R> {
    /// Transforms two input values to produce an output value.
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value to transform; it is consumed.
    /// * `second` - The second input value to transform; it is consumed.
    ///
    /// # Returns
    ///
    /// The transformed output value of type `R`.
    #[must_use = "the computed callback result should be used"]
    fn apply(&self, first: T, second: U) -> R;
}

impl<T, U, R, F> BiTransformer<T, U, R> for F
where
    F: Fn(T, U) -> R,
{
    #[inline(always)]
    fn apply(&self, first: T, second: U) -> R {
        self(first, second)
    }
}
