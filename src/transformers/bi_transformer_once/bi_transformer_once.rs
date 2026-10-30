// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiTransformerOnce Trait
//!
//! Defines a consuming bi-transformer that converts two owned inputs into an
//! output. The trait is analogous to `FnOnce(T, U) -> R`.

/// Consumes a transformer and two inputs to produce an output value.
///
/// This trait is analogous to `FnOnce(T, U) -> R`: applying it consumes the
/// transformer and both inputs.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value, which is consumed.
/// * `U` - The type of the second input value, which is consumed.
/// * `R` - The type of the output value.
///
/// # Examples
///
/// ```
/// use qubit_function::BiTransformerOnce;
///
/// let add_once = |first, second| first + second;
/// assert_eq!(add_once.apply(2, 3), 5);
/// ```
pub trait BiTransformerOnce<T, U, R> {
    /// Transforms two input values, consuming self and both inputs.
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value, which is consumed.
    /// * `second` - The second input value, which is consumed.
    ///
    /// # Returns
    ///
    /// The transformed output value.
    #[must_use = "the computed callback result should be used"]
    fn apply(self, first: T, second: U) -> R;
}
