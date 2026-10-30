// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulBiPredicate` trait.

/// A stateful bi-predicate trait for testing pairs with mutable internal state.
///
/// This trait represents closures and wrapper types equivalent to
/// `FnMut(&T, &U) -> bool`: each call borrows the predicate mutably so the
/// predicate can update counters, caches, rolling state, or other internal
/// data while leaving both tested values borrowed immutably.
///
/// # Type Parameters
///
/// * `T` - The type of the first value being tested.
/// * `U` - The type of the second value being tested.
///
/// # Examples
///
/// ```
/// use qubit_function::StatefulBiPredicate;
///
/// let mut calls = 0;
/// let mut equals_sum = |first: &i32, second: &i32| {
///     calls += 1;
///     *first + *second == 10
/// };
///
/// assert!(equals_sum.test(&4, &6));
/// assert_eq!(calls, 1);
/// ```
pub trait StatefulBiPredicate<T, U> {
    /// Tests whether the given values satisfy this bi-predicate.
    ///
    /// The method takes `&mut self` because evaluating the predicate may
    /// update internal state. The tested values themselves are only borrowed
    /// immutably and are not modified by this API.
    ///
    /// # Parameters
    ///
    /// * `first` - The first value to test.
    /// * `second` - The second value to test.
    ///
    /// # Returns
    ///
    /// `true` if the values satisfy this bi-predicate, `false` otherwise.
    #[must_use = "the predicate result should be used"]
    fn test(&mut self, first: &T, second: &U) -> bool;
}

crate::macros::impl_closure_trait!(
    StatefulBiPredicate<T, U>,
    test,
    FnMut(first: &T, second: &U) -> bool
);
