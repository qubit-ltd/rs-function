// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Mutator Types
//!
//! Provides Java-like `Mutator` interface implementations for performing
//! operations that accept a single mutable input parameter and return no
//! result.
//!
//! This module provides a unified `Mutator` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxMutator<T>`**: Box-based single ownership implementation for
//!   one-time use scenarios and builder patterns
//! - **`ArcMutator<T>`**: Arc<Mutex<>>-based thread-safe shared ownership
//!   implementation for multi-threaded scenarios
//! - **`RcMutator<T>`**: Rc<RefCell<>>-based single-threaded shared ownership
//!   implementation with no lock overhead
//!
//! It is similar to the `FnMut(&mut T)` trait in the standard library.
//!
//! # Design Philosophy
//!
//! Unlike `Consumer` which observes values without modifying them
//! (`FnMut(&T)`), `Mutator` is designed to **modify input values** using
//! `FnMut(&mut T)`.
//!
//! ## Mutator vs Consumer
//!
//! | Type | Input | Modifies Input? | Modifies Self? | Use Cases |
//! |------|-------|----------------|----------------|-----------|
//! | **Consumer** | `&T` | ❌ | ✅ | Observe, log, count, notify |
//! | **Mutator** | `&mut T` | ✅ | ✅ | Modify, transform, update |
//!
//! **Key Insight**: If you need to modify input values, use `Mutator`.
//! If you only need to observe or accumulate state, use `Consumer`.
//!
//! # Comparison Table
//!
//! | Feature          | BoxMutator | ArcMutator | RcMutator |
//! |------------------|------------|------------|-----------|
//! | Ownership        | Single     | Shared     | Shared    |
//! | Cloneable        | ❌         | ✅         | ✅        |
//! | Thread-Safe      | ❌         | ✅         | ❌        |
//! | Interior Mut.    | N/A        | Mutex      | RefCell   |
//! | `and_then` API   | `self`     | `&self`    | `&self`   |
//! | Lock Overhead    | None       | Yes        | None      |
//!
//! # Use Cases
//!
//! ## BoxMutator
//!
//! - One-time operations that don't require sharing
//! - Builder patterns where ownership naturally flows
//! - Simple scenarios with no reuse requirements
//!
//! ## ArcMutator
//!
//! - Multi-threaded shared operations
//! - Concurrent task processing (e.g., thread pools)
//! - Situations requiring the same mutator across threads
//!
//! ## RcMutator
//!
//! - Single-threaded operations with multiple uses
//! - Event handling in single-threaded UI frameworks
//! - Performance-critical single-threaded scenarios
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutator, ArcMutator, RcMutator, Mutator};
//!
//! // BoxMutator: Single ownership, consumes self
//! let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
//! let mut value = 5;
//! mutator.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! // ArcMutator: Shared ownership, cloneable, thread-safe
//! let shared = ArcMutator::new(|x: &mut i32| *x *= 2);
//! let clone = shared.clone();
//! let mut value = 5;
//! let mut m = shared;
//! m.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! // RcMutator: Shared ownership, cloneable, single-threaded
//! let rc = RcMutator::new(|x: &mut i32| *x *= 2);
//! let clone = rc.clone();
//! let mut value = 5;
//! let mut m = rc;
//! m.apply(&mut value);
//! assert_eq!(value, 10);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! use qubit_function::{Mutator, BoxMutator, ArcMutator};
//!
//! // BoxMutator: Consumes self
//! let mut chained = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .and_then(|x: &mut i32| *x += 10);
//! let mut value = 5;
//! chained.apply(&mut value);
//! assert_eq!(value, 20); // (5 * 2) + 10
//!
//! // ArcMutator: Borrows &self, original still usable
//! let first = ArcMutator::new(|x: &mut i32| *x *= 2);
//! let second = ArcMutator::new(|x: &mut i32| *x += 10);
//! let combined = first.clone().and_then(second.clone());
//! let mut value = 5;
//! combined.apply(&mut value);
//! assert_eq!(value, 20);
//! // first and second are still usable here
//! ```
//!
//! ## Working with Closures
//!
//! All closures automatically implement the `Mutator` trait:
//!
//! ```rust
//! use qubit_function::{Mutator, FnMutatorOps};
//!
//! // Closures can use .apply() directly
//! let mut closure = |x: &mut i32| *x *= 2;
//! let mut value = 5;
//! closure.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! // Closures can be chained, returning BoxMutator
//! let mut chained = (|x: &mut i32| *x *= 2)
//!     .and_then(|x: &mut i32| *x += 10);
//! let mut value = 5;
//! chained.apply(&mut value);
//! assert_eq!(value, 20);
//! ```
//!
//! ## Wrapper Construction
//!
//! ```rust
//! use qubit_function::{ArcStatefulMutator, BoxStatefulMutator, RcStatefulMutator};
//!
//! // Construct each ownership-specific wrapper from a closure.
//! let closure = |x: &mut i32| *x *= 2;
//! let box_mutator = BoxStatefulMutator::new(closure);
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let rc_mutator = RcStatefulMutator::new(closure);
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let arc_mutator = ArcStatefulMutator::new(closure);
//! ```
//!
//! ## Conditional Execution
//!
//! All mutator types support conditional execution through the `when` method,
//! which returns a `ConditionalMutator`. You can optionally add an `or_else`
//! branch to create if-then-else logic:
//!
//! ```rust
//! use qubit_function::{Mutator, BoxMutator};
//!
//! // Simple conditional (if-then)
//! let mut conditional = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .when(|x: &i32| *x > 0);
//!
//! let mut positive = 5;
//! conditional.apply(&mut positive);
//! assert_eq!(positive, 10); // Executed
//!
//! let mut negative = -5;
//! conditional.apply(&mut negative);
//! assert_eq!(negative, -5); // Not executed
//!
//! // Conditional with else branch (if-then-else)
//! let mut branched = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .when(|x: &i32| *x > 0)
//!     .or_else(|x: &mut i32| *x -= 1);
//!
//! let mut positive = 5;
//! branched.apply(&mut positive);
//! assert_eq!(positive, 10); // when branch
//!
//! let mut negative = -5;
//! branched.apply(&mut negative);
//! assert_eq!(negative, -6); // or_else branch
//! ```
#[cfg(feature = "rc")]
use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::impl_closure_trait;
#[cfg(feature = "combinators")]
use crate::mutators::macros::{
    impl_box_conditional_mutator,
    impl_conditional_mutator_clone,
    impl_conditional_mutator_debug_display,
    impl_shared_conditional_mutator,
};
use crate::mutators::macros::{
    impl_box_mutator_methods,
    impl_mutator_clone,
    impl_mutator_common_methods,
    impl_mutator_debug_display,
    impl_shared_mutator_methods,
};
#[cfg(all(feature = "rc", feature = "combinators"))]
use crate::predicates::predicate::RcPredicate;
#[cfg(feature = "combinators")]
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
};

// ============================================================================
// 1. Type Aliases
// ============================================================================

/// Type alias for Arc-wrapped mutable mutator function
type ArcMutMutatorFn<T> = Arc<Mutex<dyn FnMut(&mut T) + Send>>;

/// Type alias for Rc-wrapped mutable mutator function
#[cfg(feature = "rc")]
type RcMutMutatorFn<T> = Rc<RefCell<dyn FnMut(&mut T)>>;

mod box_stateful_mutator;
pub use box_stateful_mutator::BoxStatefulMutator;
#[cfg(feature = "rc")]
mod rc_stateful_mutator;
#[cfg(feature = "rc")]
pub use rc_stateful_mutator::RcStatefulMutator;
mod arc_stateful_mutator;
pub use arc_stateful_mutator::ArcStatefulMutator;
#[cfg(feature = "combinators")]
mod fn_mut_stateful_mutator_ops;
#[cfg(feature = "combinators")]
pub use fn_mut_stateful_mutator_ops::FnMutStatefulMutatorOps;
#[cfg(feature = "combinators")]
mod box_conditional_stateful_mutator;
#[cfg(feature = "combinators")]
pub use box_conditional_stateful_mutator::BoxConditionalStatefulMutator;
#[cfg(all(feature = "rc", feature = "combinators"))]
mod rc_conditional_stateful_mutator;
#[cfg(all(feature = "rc", feature = "combinators"))]
pub use rc_conditional_stateful_mutator::RcConditionalStatefulMutator;
#[cfg(feature = "combinators")]
mod arc_conditional_stateful_mutator;
#[cfg(feature = "combinators")]
pub use arc_conditional_stateful_mutator::ArcConditionalStatefulMutator;

// ============================================================================
// 2. Mutator Trait - Unified Mutator Interface
// ============================================================================

/// Mutator trait - Unified mutator interface
///
/// Defines the core behavior of all mutator types. Performs operations that
/// accept a mutable reference and modify the input value (not just side
/// effects).
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T)`
/// - `BoxMutator<T>`, `ArcMutator<T>`, and `RcMutator<T>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models,
/// allowing generic code to work with any stateful mutator type.
///
/// # Features
///
/// - **Unified Interface**: All mutator types share the same `mutate` method
///   signature
/// - **Automatic Implementation**: Closures implement this trait directly,
///   without allocating an adapter
/// - **Generic Programming**: Write functions that work with any mutator type
///
/// # Examples
///
/// ## Generic Mutator Function
///
/// ```rust
/// use qubit_function::{Mutator, BoxMutator, ArcMutator};
///
/// fn apply_mutator<M: Mutator<i32>>(
///     mutator: &mut M,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     mutator.apply(&mut val);
///     val
/// }
///
/// // Works with any mutator type
/// let mut box_mut = BoxMutator::new(|x: &mut i32| *x *= 2);
/// assert_eq!(apply_mutator(&mut box_mut, 5), 10);
///
/// let mut arc_mut = ArcMutator::new(|x: &mut i32| *x *= 2);
/// assert_eq!(apply_mutator(&mut arc_mut, 5), 10);
///
/// let mut closure = |x: &mut i32| *x *= 2;
/// assert_eq!(apply_mutator(&mut closure, 5), 10);
/// ```
///
/// ## Wrapper Construction
///
/// ```rust
/// use qubit_function::BoxStatefulMutator;
///
/// let closure = |x: &mut i32| *x *= 2;
///
/// // Construct a concrete ownership wrapper.
/// let box_mutator = BoxStatefulMutator::new(closure);
/// ```
pub trait StatefulMutator<T> {
    /// Performs the mutation operation
    ///
    /// Executes an operation on the given mutable reference. The operation
    /// typically modifies the input value or produces side effects.
    ///
    /// # Parameters
    ///
    /// * `value` - A mutable reference to the value to be mutated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Mutator, BoxMutator};
    ///
    /// let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
    /// let mut value = 5;
    /// mutator.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn apply(&mut self, value: &mut T);
}
