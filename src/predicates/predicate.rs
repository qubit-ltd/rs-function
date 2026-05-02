/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Predicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `Predicate` interface
//! for condition testing and logical composition.
//!
//! ## Core Semantics
//!
//! A **Predicate** is fundamentally a pure judgment operation that tests
//! whether a value satisfies a specific condition. It should be:
//!
//! - **Read-only**: Does not modify the tested value
//! - **Side-effect free**: Does not change external state (from the user's
//!   perspective)
//! - **Repeatable**: Same input should produce the same result
//! - **Deterministic**: Judgment logic should be predictable
//!
//! It is similar to the `Fn(&T) -> bool` trait in the standard library.
//!
//! ## Design Philosophy
//!
//! This module follows these principles:
//!
//! 1. **Single Trait**: Only one `Predicate<T>` trait with `&self`, keeping
//!    the API simple and semantically clear
//! 2. **No PredicateMut**: All stateful scenarios use interior mutability
//!    (`RefCell`, `Cell`, `Mutex`) instead of `&mut self`
//! 3. **No PredicateOnce**: Violates predicate semantics - judgments should
//!    be repeatable
//! 4. **Three Implementations**: `BoxPredicate`, `RcPredicate`, and
//!    `ArcPredicate` cover all ownership scenarios
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended Type | Reason |
//! |----------|------------------|--------|
//! | One-time use | `BoxPredicate` | Single ownership, no overhead |
//! | Multi-threaded | `ArcPredicate` | Thread-safe, clonable |
//! | Single-threaded reuse | `RcPredicate` | Better performance |
//! | Stateful predicate | Any type + `RefCell`/`Cell`/`Mutex` | Interior mutability |
//!
//! ## Examples
//!
//! ### Basic Usage with Closures
//!
//! ```rust
//! use qubit_function::Predicate;
//!
//! let is_positive = |x: &i32| *x > 0;
//! assert!(is_positive.test(&5));
//! assert!(!is_positive.test(&-3));
//! ```
//!
//! ### BoxPredicate - Single Ownership
//!
//! ```rust
//! use qubit_function::{Predicate, BoxPredicate};
//!
//! let pred = BoxPredicate::new(|x: &i32| *x > 0)
//!     .and(BoxPredicate::new(|x| x % 2 == 0));
//! assert!(pred.test(&4));
//! ```
//!
//! ### Closure Composition with Extension Methods
//!
//! Closures automatically gain `and`, `or`, `not` methods through the
//! `FnPredicateOps` extension trait, returning `BoxPredicate`:
//!
//! ```rust
//! use qubit_function::{Predicate, FnPredicateOps, BoxPredicate};
//!
//! // Compose closures directly - result is BoxPredicate
//! let is_positive = |x: &i32| *x > 0;
//! let is_even = |x: &i32| x % 2 == 0;
//!
//! let positive_and_even = is_positive.and(is_even);
//! assert!(positive_and_even.test(&4));
//! assert!(!positive_and_even.test(&3));
//!
//! // Can chain multiple operations
//! let pred = (|x: &i32| *x > 0)
//!     .and(|x: &i32| x % 2 == 0)
//!     .and(BoxPredicate::new(|x: &i32| *x < 100));
//! assert!(pred.test(&42));
//!
//! // Use `or` for disjunction
//! let negative_or_large = (|x: &i32| *x < 0)
//!     .or(|x: &i32| *x > 100);
//! assert!(negative_or_large.test(&-5));
//! assert!(negative_or_large.test(&200));
//!
//! // Use `not` for negation
//! let not_zero = (|x: &i32| *x == 0).not();
//! assert!(not_zero.test(&5));
//! assert!(!not_zero.test(&0));
//! ```
//!
//! ### Complex Predicate Composition
//!
//! Build complex predicates by mixing closures and predicate types:
//!
//! ```rust
//! use qubit_function::{Predicate, BoxPredicate, FnPredicateOps};
//!
//! // Start with a closure, compose with BoxPredicate
//! let in_range = (|x: &i32| *x >= 0)
//!     .and(BoxPredicate::new(|x| *x <= 100));
//!
//! // Use in filtering
//! let numbers = vec![-10, 5, 50, 150, 75];
//! let filtered: Vec<_> = numbers.iter()
//!     .copied()
//!     .filter(in_range.into_fn())
//!     .collect();
//! assert_eq!(filtered, vec![5, 50, 75]);
//! ```
//!
//! ### RcPredicate - Single-threaded Reuse
//!
//! ```rust
//! use qubit_function::{Predicate, RcPredicate};
//!
//! let pred = RcPredicate::new(|x: &i32| *x > 0);
//! let combined1 = pred.and(RcPredicate::new(|x| x % 2 == 0));
//! let combined2 = pred.or(RcPredicate::new(|x| *x > 100));
//!
//! // Original predicate is still usable
//! assert!(pred.test(&5));
//! ```
//!
//! ### ArcPredicate - Thread-safe Sharing
//!
//! ```rust
//! use qubit_function::{Predicate, ArcPredicate};
//! use std::thread;
//!
//! let pred = ArcPredicate::new(|x: &i32| *x > 0);
//! let pred_clone = pred.clone();
//!
//! let handle = thread::spawn(move || {
//!     pred_clone.test(&10)
//! });
//!
//! assert!(handle.join().unwrap());
//! assert!(pred.test(&5));  // Original still usable
//! ```
//!
//! ### Stateful Predicates with Interior Mutability
//!
//! ```rust
//! use qubit_function::{Predicate, BoxPredicate};
//! use std::cell::Cell;
//!
//! let count = Cell::new(0);
//! let pred = BoxPredicate::new(move |x: &i32| {
//!     count.set(count.get() + 1);
//!     *x > 0
//! });
//!
//! // No need for `mut` - interior mutability handles state
//! assert!(pred.test(&5));
//! assert!(!pred.test(&-3));
//! ```
//!
use std::rc::Rc;
use std::sync::Arc;

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
    impl_box_predicate_methods,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
    impl_shared_predicate_methods,
};

mod box_predicate;
pub use box_predicate::BoxPredicate;
mod rc_predicate;
pub use rc_predicate::RcPredicate;
mod arc_predicate;
pub use arc_predicate::ArcPredicate;
mod fn_predicate_ops;
pub use fn_predicate_ops::FnPredicateOps;

/// A predicate trait for testing whether a value satisfies a condition.
///
/// This trait represents a **pure judgment operation** - it tests whether
/// a given value meets certain criteria without modifying either the value
/// or the predicate itself (from the user's perspective). This semantic
/// clarity distinguishes predicates from consumers or transformers.
///
/// ## Design Rationale
///
/// This is a **minimal trait** that only defines:
/// - The core `test` method using `&self` (immutable borrow)
/// - Type conversion methods (`into_box`, `into_rc`, `into_arc`)
/// - Closure conversion method (`into_fn`)
///
/// Logical composition methods (`and`, `or`, `not`) are intentionally
/// **not** part of the trait. Instead, they are implemented on concrete
/// types (`BoxPredicate`, `RcPredicate`, `ArcPredicate`), allowing each
/// implementation to maintain its specific ownership characteristics:
///
/// - `BoxPredicate`: Methods consume `self` (single ownership)
/// - `RcPredicate`: Methods borrow `&self` (shared ownership)
/// - `ArcPredicate`: Methods borrow `&self` (thread-safe shared ownership)
///
/// ## Why `&self` Instead of `&mut self`?
///
/// Predicates use `&self` because:
///
/// 1. **Semantic Clarity**: A predicate is a judgment, not a mutation
/// 2. **Flexibility**: Can be used in immutable contexts
/// 3. **Simplicity**: No need for `mut` in user code
/// 4. **Interior Mutability**: State (if needed) can be managed with
///    `RefCell`, `Cell`, or `Mutex`
///
/// ## Automatic Implementation for Closures
///
/// Any closure matching `Fn(&T) -> bool` automatically implements this
/// trait, providing seamless integration with Rust's closure system.
///
/// ## Examples
///
/// ### Basic Usage
///
/// ```rust
/// use qubit_function::Predicate;
///
/// let is_positive = |x: &i32| *x > 0;
/// assert!(is_positive.test(&5));
/// assert!(!is_positive.test(&-3));
/// ```
///
/// ### Type Conversion
///
/// ```rust
/// use qubit_function::{Predicate, BoxPredicate};
///
/// let closure = |x: &i32| *x > 0;
/// let boxed: BoxPredicate<i32> = closure.into_box();
/// assert!(boxed.test(&5));
/// ```
///
/// ### Stateful Predicate with Interior Mutability
///
/// ```rust
/// use qubit_function::{Predicate, BoxPredicate};
/// use std::cell::Cell;
///
/// let count = Cell::new(0);
/// let counting_pred = BoxPredicate::new(move |x: &i32| {
///     count.set(count.get() + 1);
///     *x > 0
/// });
///
/// // Note: No `mut` needed - interior mutability handles state
/// assert!(counting_pred.test(&5));
/// assert!(!counting_pred.test(&-3));
/// ```
///
pub trait Predicate<T> {
    /// Tests whether the given value satisfies this predicate.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to test.
    ///
    /// # Returns
    ///
    /// `true` if the value satisfies this predicate, `false` otherwise.
    fn test(&self, value: &T) -> bool;

    /// Converts this predicate into a `BoxPredicate`.
    ///
    /// The default implementation wraps the predicate in a closure that
    /// calls the `test` method. Concrete types may override this with
    /// more efficient implementations.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` wrapping this predicate.
    fn into_box(self) -> BoxPredicate<T>
    where
        Self: Sized + 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into an `RcPredicate`.
    ///
    /// The default implementation wraps the predicate in a closure that
    /// calls the `test` method. Concrete types may override this with
    /// more efficient implementations.
    ///
    /// # Returns
    ///
    /// An `RcPredicate` wrapping this predicate.
    fn into_rc(self) -> RcPredicate<T>
    where
        Self: Sized + 'static,
    {
        RcPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into an `ArcPredicate`.
    ///
    /// The default implementation wraps the predicate in a closure that
    /// calls the `test` method. Concrete types may override this with
    /// more efficient implementations.
    ///
    /// # Returns
    ///
    /// An `ArcPredicate` wrapping this predicate.
    fn into_arc(self) -> ArcPredicate<T>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into a closure that can be used directly
    /// with standard library methods.
    ///
    /// This method consumes the predicate and returns a closure with
    /// signature `Fn(&T) -> bool`. Since `Fn` is a subtrait of `FnMut`,
    /// the returned closure can be used in any context that requires
    /// either `Fn(&T) -> bool` or `FnMut(&T) -> bool`, making it
    /// compatible with methods like `Iterator::filter`,
    /// `Iterator::filter_map`, `Vec::retain`, and similar standard
    /// library APIs.
    ///
    /// The default implementation returns a closure that calls the
    /// `test` method. Concrete types may override this with more
    /// efficient implementations.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&T) -> bool` (also usable as
    /// `FnMut(&T) -> bool`).
    ///
    /// # Examples
    ///
    /// ## Using with `Iterator::filter` (requires `FnMut`)
    ///
    /// ```rust
    /// use qubit_function::{Predicate, BoxPredicate};
    ///
    /// let pred = BoxPredicate::new(|x: &i32| *x > 0);
    ///
    /// let numbers = vec![-2, -1, 0, 1, 2, 3];
    /// let positives: Vec<_> = numbers.iter()
    ///     .copied()
    ///     .filter(pred.into_fn())
    ///     .collect();
    /// assert_eq!(positives, vec![1, 2, 3]);
    /// ```
    ///
    /// ## Using with `Vec::retain` (requires `FnMut`)
    ///
    /// ```rust
    /// use qubit_function::{Predicate, BoxPredicate};
    ///
    /// let pred = BoxPredicate::new(|x: &i32| *x % 2 == 0);
    /// let mut numbers = vec![1, 2, 3, 4, 5, 6];
    /// numbers.retain(pred.into_fn());
    /// assert_eq!(numbers, vec![2, 4, 6]);
    /// ```
    fn into_fn(self) -> impl Fn(&T) -> bool
    where
        Self: Sized + 'static,
    {
        move |value: &T| self.test(value)
    }

    /// Converts a reference to this predicate into a `BoxPredicate`.
    ///
    /// This method clones the predicate and then converts it to a
    /// `BoxPredicate`. The original predicate remains usable after this call.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` wrapping a clone of this predicate.
    fn to_box(&self) -> BoxPredicate<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts a reference to this predicate into an `RcPredicate`.
    ///
    /// This method clones the predicate and then converts it to an
    /// `RcPredicate`. The original predicate remains usable after this call.
    ///
    /// # Returns
    ///
    /// An `RcPredicate` wrapping a clone of this predicate.
    fn to_rc(&self) -> RcPredicate<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts a reference to this predicate into an `ArcPredicate`.
    ///
    /// This method clones the predicate and then converts it to an
    /// `ArcPredicate`. The original predicate remains usable after this call.
    ///
    /// # Returns
    ///
    /// An `ArcPredicate` wrapping a clone of this predicate.
    fn to_arc(&self) -> ArcPredicate<T>
    where
        Self: Clone + Sized + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts a reference to this predicate into a closure that can be
    /// used directly with standard library methods.
    ///
    /// This method clones the predicate and then converts it to a closure.
    /// The original predicate remains usable after this call.
    ///
    /// The returned closure has signature `Fn(&T) -> bool`. Since `Fn` is a
    /// subtrait of `FnMut`, it can be used in any context that requires
    /// either `Fn(&T) -> bool` or `FnMut(&T) -> bool`, making it compatible
    /// with methods like `Iterator::filter`, `Iterator::filter_map`,
    /// `Vec::retain`, and similar standard library APIs.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&T) -> bool` (also usable as
    /// `FnMut(&T) -> bool`).
    fn to_fn(&self) -> impl Fn(&T) -> bool
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }
}
