/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
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
//!
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
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

mod arc_stateful_predicate;
pub use arc_stateful_predicate::ArcStatefulPredicate;
mod box_stateful_predicate;
pub use box_stateful_predicate::BoxStatefulPredicate;
mod fn_stateful_predicate_ops;
pub use fn_stateful_predicate_ops::FnStatefulPredicateOps;
mod rc_stateful_predicate;
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

    /// Converts this predicate into a `BoxStatefulPredicate`.
    ///
    /// This consumes `self` and wraps it in a single-owner stateful
    /// predicate. The returned wrapper forwards each call to this
    /// predicate's [`test`](StatefulPredicate::test) method.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` wrapping this predicate.
    fn into_box(mut self) -> BoxStatefulPredicate<T>
    where
        Self: Sized + 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into an `RcStatefulPredicate`.
    ///
    /// This consumes `self` and wraps it in a single-threaded shared
    /// stateful predicate using `Rc<RefCell<_>>`.
    ///
    /// # Returns
    ///
    /// An `RcStatefulPredicate` wrapping this predicate.
    fn into_rc(mut self) -> RcStatefulPredicate<T>
    where
        Self: Sized + 'static,
    {
        RcStatefulPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into an `ArcStatefulPredicate`.
    ///
    /// This consumes `self` and wraps it in a thread-safe shared stateful
    /// predicate using `Arc<Mutex<_>>`. The wrapped predicate must be
    /// `Send` so it can be stored behind the thread-safe wrapper.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulPredicate` wrapping this predicate.
    fn into_arc(mut self) -> ArcStatefulPredicate<T>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into a closure implementing `FnMut(&T) -> bool`.
    ///
    /// This consumes `self` and returns a mutable closure that forwards each
    /// call to [`test`](StatefulPredicate::test).
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&T) -> bool`.
    fn into_fn(mut self) -> impl FnMut(&T) -> bool
    where
        Self: Sized + 'static,
    {
        move |value: &T| self.test(value)
    }

    /// Converts a clone of this predicate into a `BoxStatefulPredicate`.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate owns independent state unless its clone implementation shares
    /// state internally.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` wrapping a clone of this predicate.
    fn to_box(&self) -> BoxStatefulPredicate<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts a clone of this predicate into an `RcStatefulPredicate`.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate owns independent state unless its clone implementation shares
    /// state internally.
    ///
    /// # Returns
    ///
    /// An `RcStatefulPredicate` wrapping a clone of this predicate.
    fn to_rc(&self) -> RcStatefulPredicate<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts a clone of this predicate into an `ArcStatefulPredicate`.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate must be `Send` so it can be stored behind the thread-safe
    /// wrapper.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulPredicate` wrapping a clone of this predicate.
    fn to_arc(&self) -> ArcStatefulPredicate<T>
    where
        Self: Clone + Sized + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts a clone of this predicate into a mutable closure.
    ///
    /// The original predicate remains available after this call. The cloned
    /// predicate owns independent state unless its clone implementation shares
    /// state internally.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&T) -> bool`.
    fn to_fn(&self) -> impl FnMut(&T) -> bool
    where
        Self: Clone + Sized + 'static,
    {
        let mut predicate = self.clone();
        move |value: &T| predicate.test(value)
    }
}
