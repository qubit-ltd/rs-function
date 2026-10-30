// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the public bi-predicate trait and its closure implementation.

/// Tests whether two values satisfy a condition through shared references.
///
/// Implementations need not be pure or deterministic and may observe
/// interior or external state. Logical composition is provided by the
/// concrete wrapper types, which retain their respective ownership models.
///
/// # Type Parameters
///
/// * `T` - Type of the first tested value.
/// * `U` - Type of the second tested value.
///
/// # Examples
///
/// ```
/// use qubit_function::BiPredicate;
///
/// let positive_sum = |x: &i32, y: &i32| x + y > 0;
/// assert!(positive_sum.test(&5, &3));
/// assert!(!positive_sum.test(&-5, &-3));
/// ```
pub trait BiPredicate<T, U> {
    /// Tests whether the given values satisfy this bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `first` - The first value to test.
    /// * `second` - The second value to test.
    ///
    /// # Returns
    ///
    /// `true` if the values satisfy this bi-predicate, and `false` otherwise.
    #[must_use = "the predicate result should be used"]
    fn test(&self, first: &T, second: &U) -> bool;
}

impl<T, U, F> BiPredicate<T, U> for F
where
    F: Fn(&T, &U) -> bool,
{
    /// Calls the closure with both values and returns its decision.
    #[inline(always)]
    fn test(&self, first: &T, second: &U) -> bool {
        self(first, second)
    }
}
