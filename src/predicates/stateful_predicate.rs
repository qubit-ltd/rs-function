// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulPredicate Abstraction
//!
//! Provides predicate wrappers for closures that implement
//! `FnMut(&T) -> bool`. A stateful predicate can update its own internal
//! state while testing borrowed input values.
//!
//! Use [`Predicate`](crate::Predicate) for immutable `Fn(&T) -> bool`
//! predicates and `StatefulPredicate` when the predicate needs native
//! `FnMut` semantics, such as counters, rolling windows, sampling, or
//! stateful filters.

use crate::predicates::macros::constants::ALWAYS_FALSE_NAME;
use crate::predicates::macros::constants::ALWAYS_TRUE_NAME;

mod arc_stateful_predicate;
pub use arc_stateful_predicate::ArcStatefulPredicate;
mod box_stateful_predicate;
pub use box_stateful_predicate::BoxStatefulPredicate;
#[cfg(feature = "rc")]
mod rc_stateful_predicate;
#[cfg(feature = "rc")]
pub use rc_stateful_predicate::RcStatefulPredicate;

/// A stateful predicate trait for testing values with mutable internal state.
///
/// This trait represents closures and wrapper types equivalent to
/// `FnMut(&T) -> bool`: each call borrows the predicate mutably so the
/// predicate can update counters, caches, rolling state, or other internal
/// data while leaving the tested value borrowed immutably.
///
/// # Type Parameters
///
/// * `T` - The type of the value being tested.
///
/// # Examples
///
/// ```
/// use qubit_function::StatefulPredicate;
///
/// let mut calls = 0;
/// let mut is_positive = |value: &i32| {
///     calls += 1;
///     *value > 0
/// };
///
/// assert!(is_positive.test(&5));
/// assert!(!is_positive.test(&-3));
/// assert_eq!(calls, 2);
/// ```
pub trait StatefulPredicate<T> {
    /// Tests whether the given value satisfies this predicate.
    ///
    /// The method takes `&mut self` because evaluating the predicate may
    /// update internal state. The tested value itself is only borrowed
    /// immutably and is not modified by this API.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to test.
    ///
    /// # Returns
    ///
    /// `true` if the value satisfies this predicate, `false` otherwise.
    #[must_use = "the predicate result should be used"]
    fn test(&mut self, value: &T) -> bool;
}

/// Implements `StatefulPredicate<T>` for `FnMut(&T) -> bool` closures.
///
/// This blanket implementation lets mutable closures be used directly
/// wherever a `StatefulPredicate` is expected.
impl<F, T> StatefulPredicate<T> for F
where
    F: FnMut(&T) -> bool,
{
    fn test(&mut self, value: &T) -> bool {
        self(value)
    }
}
