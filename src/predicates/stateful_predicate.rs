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
use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::predicates::macros::{
    constants::{
        ALWAYS_FALSE_NAME,
        ALWAYS_TRUE_NAME,
    },
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
};

mod arc_stateful_predicate;
pub use arc_stateful_predicate::ArcStatefulPredicate;
mod box_stateful_predicate;
pub use box_stateful_predicate::BoxStatefulPredicate;
#[cfg(feature = "combinators")]
mod fn_stateful_predicate_ops;
#[cfg(feature = "combinators")]
pub use fn_stateful_predicate_ops::FnStatefulPredicateOps;
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
    fn test(&mut self, value: &T) -> bool;
}
