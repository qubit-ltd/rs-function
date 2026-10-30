// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! The consuming transformation trait used by one-time transformer types.

/// TransformerOnce trait - consuming transformation that takes ownership
///
/// Defines the behavior of a consuming transformer: converting a value of
/// type `T` to a value of type `R` by taking ownership of both self and the
/// input. This trait is analogous to `FnOnce(T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
/// # Examples
///
/// ```
/// use qubit_function::TransformerOnce;
///
/// let double_once = |value: i32| value * 2;
/// assert_eq!(double_once.apply(21), 42);
/// ```
pub trait TransformerOnce<T, R> {
    /// Transforms the input value, consuming both self and input
    ///
    /// # Parameters
    ///
    /// * `input` - The input value (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    #[must_use = "the computed callback result should be used"]
    fn apply(self, input: T) -> R;
}
