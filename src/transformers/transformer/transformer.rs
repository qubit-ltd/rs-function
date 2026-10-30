// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `Transformer` trait.

/// Transforms an owned input value into an output value.
///
/// Implementations consume `T` and return `R`, which makes this trait
/// analogous to `Fn(T) -> R` while allowing wrapper types to provide
/// transformation-specific APIs.
///
/// # Type Parameters
///
/// * `T` - The input value consumed by each transformation.
/// * `R` - The output value produced by each transformation.
///
/// # Examples
///
/// ```
/// use qubit_function::Transformer;
///
/// let increment = |value: i32| value + 1;
/// assert_eq!(increment.apply(41), 42);
/// ```
pub trait Transformer<T, R> {
    /// Applies this transformation to an owned input and returns its result.
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to consume.
    ///
    /// # Returns
    ///
    /// The output produced by the transformation.
    #[must_use = "the computed callback result should be used"]
    fn apply(&self, input: T) -> R;
}

crate::macros::impl_closure_trait!(Transformer<T, R>, apply, Fn(input: T) -> R);
