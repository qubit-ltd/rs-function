/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Mutator Types
//!
//! Provides Java-like `Mutator` interface implementations for performing
//! operations that accept a single mutable input parameter and return no result.
//!
//! This module provides a unified `Mutator` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxMutator<T>`**: Box-based single ownership implementation for
//!   one-time use scenarios and builder patterns
//! - **`ArcMutator<T>`**: Arc<Mutex<>>-based thread-safe shared ownership
//!   implementation for multi-threaded scenarios
//! - **`RcMutator<T>`**: Rc<RefCell<>>-based single-threaded shared
//!   ownership implementation with no lock overhead
//!
//! It is similar to the `FnMut(&mut T)` trait in the standard library.
//!
//! # Design Philosophy
//!
//! Unlike `Consumer` which observes values without modifying them (`FnMut(&T)`),
//! `Mutator` is designed to **modify input values** using `FnMut(&mut T)`.
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
//! ## Type Conversions
//!
//! ```rust
//! use qubit_function::Mutator;
//!
//! // Convert closure to concrete type
//! let closure = |x: &mut i32| *x *= 2;
//! let mut box_mutator = closure.into_box();
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let mut rc_mutator = closure.into_rc();
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let mut arc_mutator = closure.into_arc();
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
//!
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
use crate::mutators::{
    macros::{
        impl_box_conditional_mutator,
        impl_box_mutator_methods,
        impl_conditional_mutator_clone,
        impl_conditional_mutator_conversions,
        impl_conditional_mutator_debug_display,
        impl_mutator_clone,
        impl_mutator_common_methods,
        impl_mutator_debug_display,
        impl_shared_conditional_mutator,
        impl_shared_mutator_methods,
    },
    mutator_once::BoxMutatorOnce,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

// ============================================================================
// 1. Type Aliases
// ============================================================================

/// Type alias for Arc-wrapped mutable mutator function
type ArcMutMutatorFn<T> = Arc<Mutex<dyn FnMut(&mut T) + Send>>;

/// Type alias for Rc-wrapped mutable mutator function
type RcMutMutatorFn<T> = Rc<RefCell<dyn FnMut(&mut T)>>;

mod box_stateful_mutator;
pub use box_stateful_mutator::BoxStatefulMutator;
mod rc_stateful_mutator;
pub use rc_stateful_mutator::RcStatefulMutator;
mod arc_stateful_mutator;
pub use arc_stateful_mutator::ArcStatefulMutator;
mod fn_mut_stateful_mutator_ops;
pub use fn_mut_stateful_mutator_ops::FnMutStatefulMutatorOps;
mod box_conditional_stateful_mutator;
pub use box_conditional_stateful_mutator::BoxConditionalStatefulMutator;
mod rc_conditional_stateful_mutator;
pub use rc_conditional_stateful_mutator::RcConditionalStatefulMutator;
mod arc_conditional_stateful_mutator;
pub use arc_conditional_stateful_mutator::ArcConditionalStatefulMutator;

// ============================================================================
// 2. Mutator Trait - Unified Mutator Interface
// ============================================================================

/// Mutator trait - Unified mutator interface
///
/// Defines the core behavior of all mutator types. Performs operations that
/// accept a mutable reference and modify the input value (not just side effects).
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T)`
/// - `BoxMutator<T>`, `ArcMutator<T>`, and `RcMutator<T>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models,
/// allowing generic code to work with any mutator type. Type conversion
/// methods (`into_box`, `into_arc`, `into_rc`) enable flexible ownership
/// transitions based on usage requirements.
///
/// # Features
///
/// - **Unified Interface**: All mutator types share the same `mutate`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions that work with any mutator
///   type
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
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::Mutator;
///
/// let closure = |x: &mut i32| *x *= 2;
///
/// // Convert to different ownership models
/// let box_mutator = closure.into_box();
/// // let rc_mutator = closure.into_rc();  // closure moved
/// // let arc_mutator = closure.into_arc(); // closure moved
/// ```
///
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

    /// Convert this mutator into a `BoxMutator<T>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns a
    /// boxed implementation that forwards calls to the original mutator.
    /// Types that can provide a more efficient conversion may override the
    /// default implementation.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator: the original value will no longer
    /// be available after the call. For cloneable mutators call `.clone()`
    /// before converting if you need to retain the original instance.
    ///
    /// # Returns
    ///
    /// A `BoxMutator<T>` that forwards to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Mutator;
    ///
    /// let closure = |x: &mut i32| *x *= 2;
    /// let mut boxed = closure.into_box();
    /// let mut value = 5;
    /// boxed.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_box(mut self) -> BoxStatefulMutator<T>
    where
        Self: Sized + 'static,
    {
        BoxStatefulMutator::new(move |t| self.apply(t))
    }

    /// Convert this mutator into an `RcMutator<T>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Rc`-backed mutator that forwards calls to the original. Override to
    /// provide a more direct or efficient conversion when available.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator. If you need to keep the original
    /// instance, clone it prior to calling this method.
    ///
    /// # Returns
    ///
    /// An `RcMutator<T>` forwarding to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Mutator;
    ///
    /// let closure = |x: &mut i32| *x *= 2;
    /// let mut rc = closure.into_rc();
    /// let mut value = 5;
    /// rc.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_rc(mut self) -> RcStatefulMutator<T>
    where
        Self: Sized + 'static,
    {
        RcStatefulMutator::new(move |t| self.apply(t))
    }

    /// Convert this mutator into an `ArcMutator<T>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Arc`-wrapped, thread-safe mutator. Types may override the default
    /// implementation to provide a more efficient conversion.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator. Clone the instance first if you
    /// need to retain the original for further use.
    ///
    /// # Returns
    ///
    /// An `ArcMutator<T>` that forwards to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Mutator;
    ///
    /// let closure = |x: &mut i32| *x *= 2;
    /// let mut arc = closure.into_arc();
    /// let mut value = 5;
    /// arc.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_arc(mut self) -> ArcStatefulMutator<T>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulMutator::new(move |t| self.apply(t))
    }

    /// Consume the mutator and return an `FnMut(&mut T)` closure.
    ///
    /// The returned closure forwards calls to the original mutator and is
    /// suitable for use with iterator adapters such as `for_each`.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator. The original instance will not be
    /// available after calling this method.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T)` which forwards to the
    /// original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Mutator, BoxMutator};
    ///
    /// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
    /// let mut values = vec![1, 2, 3, 4, 5];
    /// values.iter_mut().for_each(mutator.into_fn());
    /// assert_eq!(values, vec![2, 4, 6, 8, 10]);
    /// ```
    fn into_fn(mut self) -> impl FnMut(&mut T)
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Create a non-consuming `BoxMutator<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed mutator that calls the cloned instance. Override this
    /// method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxMutator<T>` that forwards to a clone of `self`.
    fn to_box(&self) -> BoxStatefulMutator<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Create a non-consuming `RcMutator<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns an `Rc`-backed mutator that forwards calls to the clone.
    /// Override to provide a more direct or efficient conversion if needed.
    ///
    /// # Returns
    ///
    /// An `RcMutator<T>` that forwards to a clone of `self`.
    fn to_rc(&self) -> RcStatefulMutator<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Create a non-consuming `ArcMutator<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone + Send`) and
    /// returns an `Arc`-wrapped mutator that forwards calls to the clone.
    /// Override when a more efficient conversion is available.
    ///
    /// # Returns
    ///
    /// An `ArcMutator<T>` that forwards to a clone of `self`.
    fn to_arc(&self) -> ArcStatefulMutator<T>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Create a boxed `FnMut(&mut T)` closure that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed closure that invokes the cloned instance. Override to
    /// provide a more efficient conversion when possible.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T)` which forwards to the
    /// original mutator.
    fn to_fn(&self) -> impl FnMut(&mut T)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert this mutator into a `BoxMutatorOnce<T>` (consuming).
    ///
    /// This consuming conversion takes ownership of `self` and returns a
    /// boxed one-time mutator that forwards calls to the original mutator.
    /// The returned mutator can only be used once.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator: the original value will no longer
    /// be available after the call. For cloneable mutators call `.clone()`
    /// before converting if you need to retain the original instance.
    ///
    /// # Returns
    ///
    /// A `BoxMutatorOnce<T>` that forwards to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutator, MutatorOnce, BoxStatefulMutator,
    ///                       BoxMutatorOnce};
    ///
    /// let mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
    /// let once_mutator = mutator.into_once();
    /// let mut value = 5;
    /// once_mutator.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_once(mut self) -> BoxMutatorOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxMutatorOnce::new(move |t| self.apply(t))
    }

    /// Create a non-consuming `BoxMutatorOnce<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed one-time mutator that calls the cloned instance.
    /// Override this method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxMutatorOnce<T>` that forwards to a clone of `self`.
    fn to_once(&self) -> BoxMutatorOnce<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_once()
    }
}
