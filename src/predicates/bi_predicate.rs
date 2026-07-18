// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiPredicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `BiPredicate`
//! interface for testing whether two values satisfy a condition.
//!
//! ## Core Semantics
//!
//! A **BiPredicate** tests whether two values satisfy a condition through
//! shared references. Implementations may use interior mutability or observe
//! external state; purity and determinism are application-level conventions.
//!
//! - **Read-only**: Does not modify the tested values
//!
//! It is similar to the `Fn(&T, &U) -> bool` trait in the standard library.
//!
//! ## Design Philosophy
//!
//! This module follows the same principles as the `Predicate` module:
//!
//! 1. **Single Trait**: Only one `BiPredicate<T, U>` trait with `&self`,
//!    keeping the API simple and semantically clear
//! 2. **No BiPredicateMut**: All stateful scenarios use interior mutability
//!    (`RefCell`, `Cell`, `Mutex`) instead of `&mut self`
//! 3. **No BiPredicateOnce**: Violates bi-predicate semantics - judgments
//!    should be repeatable
//! 4. **Three Implementations**: `BoxBiPredicate`, `RcBiPredicate`, and
//!    `ArcBiPredicate` cover all ownership scenarios
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended Type | Reason |
//! |----------|------------------|--------|
//! | Single ownership | `BoxBiPredicate` | Type erasure with heap allocation and dynamic dispatch |
//! | Multi-threaded | `ArcBiPredicate` | Thread-safe, clonable |
//! | Single-threaded reuse | `RcBiPredicate` | Better performance |
//! | Stateful predicate | Any type + `RefCell`/`Cell`/`Mutex` | Interior mutability |
//!
//! ## Examples
//!
//! ### Basic Usage with Closures
//!
//! ```rust
//! use qubit_function::BiPredicate;
//!
//! let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
//! assert!(is_sum_positive.test(&5, &3));
//! assert!(!is_sum_positive.test(&-3, &-7));
//! ```
//!
//! ### BoxBiPredicate - Single Ownership
//!
//! ```rust
//! # {
//! use qubit_function::{BiPredicate, BoxBiPredicate};
//!
//! let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0)
//!     .and(BoxBiPredicate::new(|x: &i32, y: &i32| x > y));
//! assert!(pred.test(&10, &5));
//! # }
//! ```
//!
//! ### Explicit Closure Composition
//!
//! Closures implement `BiPredicate`, but chaining begins with a concrete
//! wrapper so the ownership model is explicit:
//!
//! ```rust
//! # {
//! use qubit_function::{BiPredicate, BoxBiPredicate};
//!
//! let combined = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0)
//!     .and(|x: &i32, y: &i32| x > y);
//! assert!(combined.test(&10, &5));
//! assert!(!combined.test(&3, &8));
//!
//! let either = BoxBiPredicate::new(|x: &i32, y: &i32| x + y < 0)
//!     .or(|x: &i32, y: &i32| *x > 100 && *y > 100);
//! assert!(either.test(&-10, &5));
//! assert!(either.test(&200, &150));
//! # }
//! ```
//!
//! ### RcBiPredicate - Single-threaded Reuse
//!
//! This example requires the `rc` feature.
//!
//! ```rust
//! # #[cfg(feature = "rc")]
//! # {
//! use qubit_function::{BiPredicate, RcBiPredicate};
//!
//! let pred = RcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
//! let combined1 = pred.and(RcBiPredicate::new(|x: &i32, y: &i32| x > y));
//! let combined2 = pred.or(RcBiPredicate::new(|x: &i32, y: &i32| *x > 100));
//!
//! // Original predicate is still usable
//! assert!(pred.test(&5, &3));
//! # }
//! ```
//!
//! ### ArcBiPredicate - Thread-safe Sharing
//!
//! ```rust
//! use qubit_function::{BiPredicate, ArcBiPredicate};
//! use std::thread;
//!
//! let pred = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
//! let pred_clone = pred.clone();
//!
//! let handle = thread::spawn(move || {
//!     pred_clone.test(&10, &5)
//! });
//!
//! assert!(handle.join().expect("thread should not panic"));
//! assert!(pred.test(&3, &7));  // Original still usable
//! ```
//!
//! ### Stateful BiPredicates with Interior Mutability
//!
//! ```rust
//! use qubit_function::{BiPredicate, BoxBiPredicate};
//! use std::cell::Cell;
//!
//! let count = Cell::new(0);
//! let pred = BoxBiPredicate::new(move |x: &i32, y: &i32| {
//!     count.set(count.get() + 1);
//!     x + y > 0
//! });
//!
//! // No need for `mut` - interior mutability handles state
//! assert!(pred.test(&5, &3));
//! assert!(!pred.test(&-8, &-3));
//! ```

use crate::predicates::macros::constants::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
};

/// Type alias for bi-predicate function to simplify complex types.
///
/// This type alias represents a function that takes two references and returns
/// a boolean. It is used to reduce type complexity in struct definitions.
type BiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool;

/// Type alias for thread-safe bi-predicate function to simplify complex types.
///
/// This type alias represents a function that takes two references and returns
/// a boolean, with Send + Sync bounds for thread-safe usage. It is used to
/// reduce type complexity in Arc-based struct definitions.
type SendSyncBiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool + Send + Sync;

mod box_bi_predicate;
pub use box_bi_predicate::BoxBiPredicate;
#[cfg(feature = "rc")]
mod rc_bi_predicate;
#[cfg(feature = "rc")]
pub use rc_bi_predicate::RcBiPredicate;
mod arc_bi_predicate;
pub use arc_bi_predicate::ArcBiPredicate;

/// A bi-predicate trait for testing whether two values satisfy a
/// condition.
///
/// This trait tests whether two values meet a condition through `&self` and
/// shared input references. It does not guarantee purity, determinism, or the
/// absence of interior mutability and external side effects.
///
/// ## Design Rationale
///
/// This is a **minimal trait** that only defines:
/// - The core `test` method using `&self` (immutable borrow)
///
/// Logical composition methods (`and`, `or`, `not`, `xor`, `nand`,
/// `nor`) are intentionally **not** part of the trait. Instead, they
/// are implemented on concrete types (`BoxBiPredicate`,
/// `RcBiPredicate`, `ArcBiPredicate`), allowing each implementation
/// to maintain its specific ownership characteristics:
///
/// - `BoxBiPredicate`: Methods consume `self` (single ownership)
/// - `RcBiPredicate`: Methods borrow `&self` (shared ownership)
/// - `ArcBiPredicate`: Methods borrow `&self` (thread-safe shared ownership)
///
/// ## Why `&self` Instead of `&mut self`?
///
/// Bi-predicates use `&self` because:
///
/// 1. **Semantic Clarity**: A bi-predicate is a judgment, not a mutation
/// 2. **Flexibility**: Can be used in immutable contexts
/// 3. **Simplicity**: No need for `mut` in user code
/// 4. **Interior Mutability**: State (if needed) can be managed with `RefCell`,
///    `Cell`, or `Mutex`
///
/// ## Automatic Implementation for Closures
///
/// Any closure matching `Fn(&T, &U) -> bool` automatically implements
/// this trait, providing seamless integration with Rust's closure
/// system.
///
/// ## Examples
///
/// ### Basic Usage
///
/// ```rust
/// use qubit_function::BiPredicate;
///
/// let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
/// assert!(is_sum_positive.test(&5, &3));
/// assert!(!is_sum_positive.test(&-5, &-3));
/// ```
///
/// ### Type Conversion
///
/// ```rust
/// use qubit_function::{BiPredicate,
///     BoxBiPredicate};
///
/// let closure = |x: &i32, y: &i32| x + y > 0;
/// let boxed = BoxBiPredicate::new(closure);
/// assert!(boxed.test(&5, &3));
/// ```
///
/// ### Stateful BiPredicate with Interior Mutability
///
/// ```rust
/// use qubit_function::{BiPredicate,
///     BoxBiPredicate};
/// use std::cell::Cell;
///
/// let count = Cell::new(0);
/// let counting_pred = BoxBiPredicate::new(move |x: &i32, y: &i32| {
///     count.set(count.get() + 1);
///     x + y > 0
/// });
///
/// // Note: No `mut` needed - interior mutability handles state
/// assert!(counting_pred.test(&5, &3));
/// assert!(!counting_pred.test(&-5, &-3));
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
    /// `true` if the values satisfy this bi-predicate, `false`
    /// otherwise.
    #[must_use = "the predicate result should be used"]
    fn test(&self, first: &T, second: &U) -> bool;
}

impl<T, U, F> BiPredicate<T, U> for F
where
    F: Fn(&T, &U) -> bool,
{
    #[inline(always)]
    fn test(&self, first: &T, second: &U) -> bool {
        self(first, second)
    }
}
