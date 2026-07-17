// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Predicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `Predicate` interface
//! for condition testing and logical composition.
//!
//! ## Core Semantics
//!
//! A **Predicate** is a judgment operation that tests whether a value satisfies
//! a condition through a shared reference. The API guarantees that it cannot
//! mutate the input through that reference, but implementations may use
//! interior mutability or observe external state. Purity and determinism are
//! application-level conventions, not type-system guarantees.
//!
//! - **Read-only**: Does not modify the tested value
//!
//! It is similar to the `Fn(&T) -> bool` trait in the standard library.
//!
//! ## Design Philosophy
//!
//! This module follows these principles:
//!
//! 1. **Single Trait**: Only one `Predicate<T>` trait with `&self`, keeping the
//!    API simple and semantically clear
//! 2. **Dedicated Stateful API**: Use `StatefulPredicate` for `FnMut(&T) ->
//!    bool` closures that need `&mut self`
//! 3. **No PredicateOnce**: Violates predicate semantics - judgments should be
//!    repeatable
//! 4. **Three Implementations**: `BoxPredicate`, `RcPredicate`, and
//!    `ArcPredicate` cover all ownership scenarios
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended Type | Reason |
//! |----------|------------------|--------|
//! | Single ownership | `BoxPredicate` | Type erasure with heap allocation and dynamic dispatch |
//! | Multi-threaded | `ArcPredicate` | Thread-safe, clonable |
//! | Single-threaded reuse | `RcPredicate` | Better performance |
//! | Stateful closure | `BoxStatefulPredicate`, `RcStatefulPredicate`, or `ArcStatefulPredicate` | Native `FnMut(&T) -> bool` support |
//! | Immutable API with state | Any `Predicate` type + `RefCell`/`Cell`/`Mutex` | Preserve `Predicate`'s `&self` API |
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
//! # {
//! use qubit_function::{Predicate, BoxPredicate};
//!
//! let pred = BoxPredicate::new(|x: &i32| *x > 0)
//!     .and(BoxPredicate::new(|x: &i32| x % 2 == 0));
//! assert!(pred.test(&4));
//! # }
//! ```
//!
//! ### Explicit Closure Composition
//!
//! Closures implement `Predicate`, but chaining is intentionally available
//! only on concrete wrappers. Construct a wrapper first to choose the ownership
//! model explicitly:
//!
//! ```rust
//! # {
//! use qubit_function::{Predicate, BoxPredicate};
//! use std::ops::Not;
//!
//! let positive_and_even = BoxPredicate::new(|x: &i32| *x > 0)
//!     .and(|x: &i32| x % 2 == 0);
//! assert!(positive_and_even.test(&4));
//! assert!(!positive_and_even.test(&3));
//!
//! let pred = BoxPredicate::new(|x: &i32| *x > 0)
//!     .and(|x: &i32| x % 2 == 0)
//!     .and(BoxPredicate::new(|x: &i32| *x < 100));
//! assert!(pred.test(&42));
//!
//! let negative_or_large = BoxPredicate::new(|x: &i32| *x < 0)
//!     .or(|x: &i32| *x > 100);
//! assert!(negative_or_large.test(&-5));
//! assert!(negative_or_large.test(&200));
//!
//! let not_zero = BoxPredicate::new(|x: &i32| *x == 0).not();
//! assert!(not_zero.test(&5));
//! assert!(!not_zero.test(&0));
//! # }
//! ```
//!
//! ### Complex Predicate Composition
//!
//! Build complex predicates by mixing closures and predicate types:
//!
//! ```rust
//! # {
//! use qubit_function::{Predicate, BoxPredicate};
//!
//! let in_range = BoxPredicate::new(|x: &i32| *x >= 0)
//!     .and(BoxPredicate::new(|x: &i32| *x <= 100));
//!
//! // Use in filtering
//! let numbers = vec![-10, 5, 50, 150, 75];
//! let filtered: Vec<_> = numbers.iter()
//!     .copied()
//!     .filter(|value| in_range.test(value))
//!     .collect();
//! assert_eq!(filtered, vec![5, 50, 75]);
//! # }
//! ```
//!
//! ### RcPredicate - Single-threaded Reuse
//!
//! This example requires the `rc` feature.
//!
//! ```rust
//! # #[cfg(feature = "rc")]
//! # {
//! use qubit_function::{Predicate, RcPredicate};
//!
//! let pred = RcPredicate::new(|x: &i32| *x > 0);
//! let combined1 = pred.and(RcPredicate::new(|x: &i32| x % 2 == 0));
//! let combined2 = pred.or(RcPredicate::new(|x: &i32| *x > 100));
//!
//! // Original predicate is still usable
//! assert!(pred.test(&5));
//! # }
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
//! assert!(handle.join().expect("thread should not panic"));
//! assert!(pred.test(&5));  // Original still usable
//! ```
//!
//! ### Stateful Predicate Closures
//!
//! This example requires the `stateful` feature.
//!
//! ```rust
//! # #[cfg(feature = "stateful")]
//! # {
//! use qubit_function::{StatefulPredicate, BoxStatefulPredicate};
//!
//! let mut count = 0;
//! let mut pred = BoxStatefulPredicate::new(move |x: &i32| {
//!     count += 1;
//!     *x > 0
//! });
//!
//! assert!(pred.test(&5));
//! assert!(!pred.test(&-3));
//! # }
//! ```

use crate::predicates::macros::constants::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
};

mod box_predicate;
pub use box_predicate::BoxPredicate;
#[cfg(feature = "rc")]
mod rc_predicate;
#[cfg(feature = "rc")]
pub use rc_predicate::RcPredicate;
mod arc_predicate;
pub use arc_predicate::ArcPredicate;

/// A predicate trait for testing whether a value satisfies a condition.
///
/// This trait tests whether a value meets a condition through `&self` and a
/// shared input reference. It does not guarantee purity, determinism, or the
/// absence of interior mutability and external side effects.
///
/// ## Design Rationale
///
/// This is a **minimal trait** that only defines:
/// - The core `test` method using `&self` (immutable borrow)
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
/// 4. **Explicit Stateful Alternative**: Use `StatefulPredicate` when the
///    predicate closure needs native `FnMut(&T) -> bool` semantics
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
/// ### Explicit Type Erasure
///
/// ```rust
/// use qubit_function::{Predicate, BoxPredicate};
///
/// let closure = |x: &i32| *x > 0;
/// let boxed = BoxPredicate::new(closure);
/// assert!(boxed.test(&5));
/// ```
///
/// ### Stateful Predicate Alternative
///
/// This example requires the `stateful` feature.
///
/// ```rust
/// # #[cfg(feature = "stateful")]
/// # {
/// use qubit_function::{StatefulPredicate, BoxStatefulPredicate};
///
/// let mut count = 0;
/// let mut counting_pred = BoxStatefulPredicate::new(move |x: &i32| {
///     count += 1;
///     *x > 0
/// });
///
/// assert!(counting_pred.test(&5));
/// assert!(!counting_pred.test(&-3));
/// # }
/// ```
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
    #[must_use = "the predicate result should be used"]
    fn test(&self, value: &T) -> bool;
}

impl<T, F> Predicate<T> for F
where
    F: Fn(&T) -> bool,
{
    #[inline]
    fn test(&self, value: &T) -> bool {
        self(value)
    }
}
