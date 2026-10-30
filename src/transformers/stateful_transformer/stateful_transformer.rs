// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the stateful transformation trait.

/// Transforms owned input values while allowing the callback to retain state.
///
/// A mutable closure implements this trait and can update captured state
/// between calls.
///
/// # Type Parameters
///
/// * `T` - The input value type, which is consumed by each call.
/// * `R` - The output value type.
///
/// # Examples
///
/// ```
/// use qubit_function::StatefulTransformer;
///
/// let mut double = |value: i32| value * 2;
/// assert_eq!(double.apply(3), 6);
/// ```
pub trait StatefulTransformer<T, R> {
    /// Applies the transformation to an owned input value.
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform; ownership is transferred.
    ///
    /// # Returns
    ///
    /// The transformed output value.
    #[must_use = "the computed callback result should be used"]
    fn apply(&mut self, input: T) -> R;
}

crate::macros::impl_closure_trait!(
    StatefulTransformer<T, R>,
    apply,
    FnMut(input: T) -> R
);
