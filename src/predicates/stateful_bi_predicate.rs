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
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::macros::{
    constants::{
        ALWAYS_FALSE_NAME,
        ALWAYS_TRUE_NAME,
    },
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
};

mod arc_stateful_bi_predicate;
pub use arc_stateful_bi_predicate::ArcStatefulBiPredicate;
mod box_stateful_bi_predicate;
pub use box_stateful_bi_predicate::BoxStatefulBiPredicate;
mod fn_stateful_bi_predicate_ops;
pub use fn_stateful_bi_predicate_ops::FnStatefulBiPredicateOps;
mod rc_stateful_bi_predicate;
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

    /// Converts this bi-predicate into a `BoxStatefulBiPredicate`.
    ///
    /// This consumes `self` and wraps it in a single-owner stateful
    /// bi-predicate. The returned wrapper forwards each call to this
    /// predicate's [`test`](StatefulBiPredicate::test) method.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` wrapping this predicate.
    fn into_box(mut self) -> BoxStatefulBiPredicate<T, U>
    where
        Self: Sized + 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self.test(first, second)
        })
    }

    /// Converts this bi-predicate into an `RcStatefulBiPredicate`.
    ///
    /// This consumes `self` and wraps it in a single-threaded shared
    /// stateful bi-predicate using `Rc<RefCell<_>>`.
    ///
    /// # Returns
    ///
    /// An `RcStatefulBiPredicate` wrapping this predicate.
    fn into_rc(mut self) -> RcStatefulBiPredicate<T, U>
    where
        Self: Sized + 'static,
    {
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            self.test(first, second)
        })
    }

    /// Converts this bi-predicate into an `ArcStatefulBiPredicate`.
    ///
    /// This consumes `self` and wraps it in a thread-safe shared stateful
    /// bi-predicate using `Arc<Mutex<_>>`. The wrapped predicate must be
    /// `Send` so it can be stored behind the thread-safe wrapper.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulBiPredicate` wrapping this predicate.
    fn into_arc(mut self) -> ArcStatefulBiPredicate<T, U>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            self.test(first, second)
        })
    }

    /// Converts this bi-predicate into a closure implementing
    /// `FnMut(&T, &U) -> bool`.
    ///
    /// This consumes `self` and returns a mutable closure that forwards each
    /// call to [`test`](StatefulBiPredicate::test).
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&T, &U) -> bool`.
    fn into_fn(mut self) -> impl FnMut(&T, &U) -> bool
    where
        Self: Sized + 'static,
    {
        move |first: &T, second: &U| self.test(first, second)
    }

    /// Converts a clone of this bi-predicate into a `BoxStatefulBiPredicate`.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate owns independent state unless its clone implementation shares
    /// state internally.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` wrapping a clone of this predicate.
    fn to_box(&self) -> BoxStatefulBiPredicate<T, U>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts a clone of this bi-predicate into an `RcStatefulBiPredicate`.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate owns independent state unless its clone implementation shares
    /// state internally.
    ///
    /// # Returns
    ///
    /// An `RcStatefulBiPredicate` wrapping a clone of this predicate.
    fn to_rc(&self) -> RcStatefulBiPredicate<T, U>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts a clone of this bi-predicate into an `ArcStatefulBiPredicate`.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate must be `Send` so it can be stored behind the thread-safe
    /// wrapper.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulBiPredicate` wrapping a clone of this predicate.
    fn to_arc(&self) -> ArcStatefulBiPredicate<T, U>
    where
        Self: Clone + Sized + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts a clone of this bi-predicate into a mutable closure.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate owns independent state unless its clone implementation shares
    /// state internally.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&T, &U) -> bool`.
    fn to_fn(&self) -> impl FnMut(&T, &U) -> bool
    where
        Self: Clone + Sized + 'static,
    {
        let mut predicate = self.clone();
        move |first: &T, second: &U| predicate.test(first, second)
    }
}
