// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the [`StatefulFunction`] callback trait.

/// Transforms borrowed values from type `T` to type `R` while retaining state.
///
/// A stateful function receives each input by shared reference, so it does not
/// take ownership of the input. Its mutable receiver allows it to update state
/// between calls, similarly to the standard library's `FnMut(&T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of each borrowed input value.
/// * `R` - The type of each returned output value.
///
/// # Examples
///
/// ```
/// use qubit_function::{BoxStatefulFunction, StatefulFunction};
///
/// let mut call_count = 0;
/// let mut counter = BoxStatefulFunction::new(move |value: &i32| {
///     call_count += 1;
///     value + call_count
/// });
///
/// assert_eq!(counter.apply(&5), 6);
/// assert_eq!(counter.apply(&5), 7);
/// ```
pub trait StatefulFunction<T, R> {
    /// Applies the mapping to the borrowed input and returns its transformed
    /// value.
    ///
    /// # Parameters
    ///
    /// * `t` - The borrowed input value to transform.
    ///
    /// # Returns
    ///
    /// The transformed output value.
    #[must_use = "the computed callback result should be used"]
    fn apply(&mut self, t: &T) -> R;
}

impl<T, R, F> StatefulFunction<T, R> for F
where
    F: FnMut(&T) -> R,
{
    #[inline(always)]
    fn apply(&mut self, t: &T) -> R {
        self(t)
    }
}
