/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # BiPredicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `BiPredicate`
//! interface for testing whether two values satisfy a condition.
//!
//! ## Core Semantics
//!
//! A **BiPredicate** is fundamentally a pure judgment operation that
//! tests whether two values satisfy a specific condition. It should
//! be:
//!
//! - **Read-only**: Does not modify the tested values
//! - **Side-effect free**: Does not change external state (from the
//!   user's perspective)
//! - **Repeatable**: Same inputs should produce the same result
//! - **Deterministic**: Judgment logic should be predictable
//!
//! It is similar to the `Fn(&T, &U) -> bool` trait in the standard library.
//!
//! ## Design Philosophy
//!
//! This module follows the same principles as the `Predicate` module:
//!
//! 1. **Single Trait**: Only one `BiPredicate<T, U>` trait with
//!    `&self`, keeping the API simple and semantically clear
//! 2. **No BiPredicateMut**: All stateful scenarios use interior
//!    mutability (`RefCell`, `Cell`, `Mutex`) instead of `&mut self`
//! 3. **No BiPredicateOnce**: Violates bi-predicate semantics -
//!    judgments should be repeatable
//! 4. **Three Implementations**: `BoxBiPredicate`, `RcBiPredicate`,
//!    and `ArcBiPredicate` cover all ownership scenarios
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended Type | Reason |
//! |----------|------------------|--------|
//! | One-time use | `BoxBiPredicate` | Single ownership, no overhead |
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
//! use qubit_function::{BiPredicate, BoxBiPredicate};
//!
//! let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0)
//!     .and(BoxBiPredicate::new(|x, y| x > y));
//! assert!(pred.test(&10, &5));
//! ```
//!
//! ### Closure Composition with Extension Methods
//!
//! Closures automatically gain `and`, `or`, `not` methods through the
//! `FnBiPredicateOps` extension trait, returning `BoxBiPredicate`:
//!
//! ```rust
//! use qubit_function::{BiPredicate,
//!     FnBiPredicateOps};
//!
//! // Compose closures directly - result is BoxBiPredicate
//! let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
//! let first_larger = |x: &i32, y: &i32| x > y;
//!
//! let combined = is_sum_positive.and(first_larger);
//! assert!(combined.test(&10, &5));
//! assert!(!combined.test(&3, &8));
//!
//! // Use `or` for disjunction
//! let negative_sum = |x: &i32, y: &i32| x + y < 0;
//! let both_large = |x: &i32, y: &i32| *x > 100 && *y > 100;
//! let either = negative_sum.or(both_large);
//! assert!(either.test(&-10, &5));
//! assert!(either.test(&200, &150));
//! ```
//!
//! ### RcBiPredicate - Single-threaded Reuse
//!
//! ```rust
//! use qubit_function::{BiPredicate, RcBiPredicate};
//!
//! let pred = RcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
//! let combined1 = pred.and(RcBiPredicate::new(|x, y| x > y));
//! let combined2 = pred.or(RcBiPredicate::new(|x, y| *x > 100));
//!
//! // Original predicate is still usable
//! assert!(pred.test(&5, &3));
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
//! assert!(handle.join().unwrap());
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
//!
//! ## Author
//!
//! Haixing Hu
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

/// Type alias for bi-predicate function to simplify complex types.
///
/// This type alias represents a function that takes two references and returns a boolean.
/// It is used to reduce type complexity in struct definitions.
type BiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool;

/// Type alias for thread-safe bi-predicate function to simplify complex types.
///
/// This type alias represents a function that takes two references and returns a boolean,
/// with Send + Sync bounds for thread-safe usage. It is used to reduce type complexity
/// in Arc-based struct definitions.
type SendSyncBiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool + Send + Sync;

mod box_bi_predicate;
pub use box_bi_predicate::BoxBiPredicate;
mod rc_bi_predicate;
pub use rc_bi_predicate::RcBiPredicate;
mod arc_bi_predicate;
pub use arc_bi_predicate::ArcBiPredicate;
mod fn_bi_predicate_ops;
pub use fn_bi_predicate_ops::FnBiPredicateOps;

/// A bi-predicate trait for testing whether two values satisfy a
/// condition.
///
/// This trait represents a **pure judgment operation** - it tests
/// whether two given values meet certain criteria without modifying
/// either the values or the bi-predicate itself (from the user's
/// perspective). This semantic clarity distinguishes bi-predicates
/// from consumers or transformers.
///
/// ## Design Rationale
///
/// This is a **minimal trait** that only defines:
/// - The core `test` method using `&self` (immutable borrow)
/// - Type conversion methods (`into_box`, `into_rc`, `into_arc`)
/// - Closure conversion method (`into_fn`)
///
/// Logical composition methods (`and`, `or`, `not`, `xor`, `nand`,
/// `nor`) are intentionally **not** part of the trait. Instead, they
/// are implemented on concrete types (`BoxBiPredicate`,
/// `RcBiPredicate`, `ArcBiPredicate`), allowing each implementation
/// to maintain its specific ownership characteristics:
///
/// - `BoxBiPredicate`: Methods consume `self` (single ownership)
/// - `RcBiPredicate`: Methods borrow `&self` (shared ownership)
/// - `ArcBiPredicate`: Methods borrow `&self` (thread-safe shared
///   ownership)
///
/// ## Why `&self` Instead of `&mut self`?
///
/// Bi-predicates use `&self` because:
///
/// 1. **Semantic Clarity**: A bi-predicate is a judgment, not a
///    mutation
/// 2. **Flexibility**: Can be used in immutable contexts
/// 3. **Simplicity**: No need for `mut` in user code
/// 4. **Interior Mutability**: State (if needed) can be managed with
///    `RefCell`, `Cell`, or `Mutex`
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
/// let boxed: BoxBiPredicate<i32, i32> = closure.into_box();
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
///
/// ## Author
///
/// Haixing Hu
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
    fn test(&self, first: &T, second: &U) -> bool;

    /// Converts this bi-predicate into a `BoxBiPredicate`.
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` wrapping this bi-predicate.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps the bi-predicate in a
    /// closure that calls `test`, providing automatic conversion
    /// for custom types that only implement the core `test`
    /// method.
    fn into_box(self) -> BoxBiPredicate<T, U>
    where
        Self: Sized + 'static,
    {
        BoxBiPredicate::new(move |first, second| self.test(first, second))
    }

    /// Converts this bi-predicate into an `RcBiPredicate`.
    ///
    /// # Returns
    ///
    /// An `RcBiPredicate` wrapping this bi-predicate.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps the bi-predicate in a
    /// closure that calls `test`, providing automatic conversion
    /// for custom types that only implement the core `test`
    /// method.
    fn into_rc(self) -> RcBiPredicate<T, U>
    where
        Self: Sized + 'static,
    {
        RcBiPredicate::new(move |first, second| self.test(first, second))
    }

    /// Converts this bi-predicate into an `ArcBiPredicate`.
    ///
    /// # Returns
    ///
    /// An `ArcBiPredicate` wrapping this bi-predicate.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps the bi-predicate in a
    /// closure that calls `test`, providing automatic conversion
    /// for custom types that only implement the core `test`
    /// method. Note that this requires `Send + Sync` bounds for
    /// thread-safe sharing.
    fn into_arc(self) -> ArcBiPredicate<T, U>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcBiPredicate::new(move |first, second| self.test(first, second))
    }

    /// Converts this bi-predicate into a closure that can be used
    /// directly with standard library methods.
    ///
    /// This method consumes the bi-predicate and returns a closure
    /// with signature `Fn(&T, &U) -> bool`. Since `Fn` is a subtrait
    /// of `FnMut`, the returned closure can be used in any context
    /// that requires either `Fn(&T, &U) -> bool` or
    /// `FnMut(&T, &U) -> bool`.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&T, &U) -> bool` (also usable as
    /// `FnMut(&T, &U) -> bool`).
    ///
    /// # Default Implementation
    ///
    /// The default implementation returns a closure that calls the
    /// `test` method, providing automatic conversion for custom
    /// types.
    ///
    /// # Examples
    ///
    /// ## Using with Iterator Methods
    ///
    /// ```rust
    /// use qubit_function::{BiPredicate,
    ///     BoxBiPredicate};
    ///
    /// let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
    ///
    /// let pairs = vec![(1, 2), (-1, 3), (5, -6)];
    /// let mut closure = pred.into_fn();
    /// let positives: Vec<_> = pairs.iter()
    ///     .filter(|(x, y)| closure(x, y))
    ///     .collect();
    /// assert_eq!(positives, vec![&(1, 2), &(-1, 3)]);
    /// ```
    fn into_fn(self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + 'static,
    {
        move |first, second| self.test(first, second)
    }

    fn to_box(&self) -> BoxBiPredicate<T, U>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    fn to_rc(&self) -> RcBiPredicate<T, U>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    fn to_arc(&self) -> ArcBiPredicate<T, U>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    fn to_fn(&self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}
