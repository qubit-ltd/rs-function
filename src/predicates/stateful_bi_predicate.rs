// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulBiPredicate Abstraction
//!
//! Provides bi-predicate wrappers for closures that implement
//! `FnMut(&T, &U) -> bool`. A stateful bi-predicate can update its own
//! internal state while testing two borrowed input values.
//!
//! Use [`BiPredicate`](crate::BiPredicate) for immutable
//! `Fn(&T, &U) -> bool` predicates and `StatefulBiPredicate` when the
//! predicate needs native `FnMut` semantics, such as counters, rolling
//! windows, sampling, or stateful filters over pairs of values.

use crate::predicates::macros::constants::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
};

mod arc_stateful_bi_predicate;
pub use arc_stateful_bi_predicate::ArcStatefulBiPredicate;
mod box_stateful_bi_predicate;
pub use box_stateful_bi_predicate::BoxStatefulBiPredicate;
#[cfg(feature = "rc")]
mod rc_stateful_bi_predicate;
#[cfg(feature = "rc")]
pub use rc_stateful_bi_predicate::RcStatefulBiPredicate;

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
    fn test(&mut self, first: &T, second: &U) -> bool;
}

crate::macros::impl_closure_trait!(
    StatefulBiPredicate<T, U>,
    test,
    FnMut(first: &T, second: &U) -> bool
);
