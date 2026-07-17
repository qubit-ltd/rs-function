// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Mutator Types (Shared Receiver)
//!
//! Provides Java-like `Mutator` interface implementations for performing
//! operations that accept a single mutable input parameter, are invoked through
//! `&self`, and return no result.
//!
//! This module provides a unified `Mutator` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxMutator<T>`**: Box-based single ownership implementation for stored
//!   callbacks and builder patterns
//! - **`ArcMutator<T>`**: Arc-based thread-safe shared ownership implementation
//!   for multi-threaded scenarios
//! - **`RcMutator<T>`**: Rc-based single-threaded shared ownership
//!   implementation with no lock overhead
//!
//! It is similar to the `Fn(&mut T)` trait in the standard library.
//!
//! # Design Philosophy
//!
//! Here, **stateless** describes the `Fn(&mut T)` shared-receiver call shape.
//! Unlike `StatefulMutator` which uses `FnMut(&mut T)` and can maintain
//! internal state through `FnMut`, `Mutator` cannot mutate ordinary captured
//! values through its `Fn` receiver. Implementations can still use interior
//! mutability or produce external side effects.
//!
//! ## Mutator vs StatefulMutator vs Consumer
//!
//! | Type | Input | Modifies Input? | Modifies Self? | Use Cases |
//! |------|-------|----------------|----------------|-----------|
//! | **Consumer** | `&T` | ❌ | ❌ | Observe, log, notify |
//! | **Mutator** | `&mut T` | ✅ | Not through ordinary `Fn` captures | Transform, validate, normalize |
//! | **StatefulMutator** | `&mut T` | ✅ | ✅ | Stateful transform, accumulate |
//!
//! **Key Insight**: Use `Mutator` for shared-receiver transformations,
//! `StatefulMutator` for stateful operations, and `Consumer` for observation.
//!
//! # Comparison Table
//!
//! | Feature          | BoxMutator | ArcMutator | RcMutator |
//! |------------------|------------|------------|-----------|
//! | Ownership        | Single     | Shared     | Shared    |
//! | Cloneable        | ❌         | ✅         | ✅        |
//! | Thread-Safe      | ❌         | ✅         | ❌        |
//! | Interior Mut.    | N/A        | N/A        | N/A       |
//! | `and_then` API   | `self`     | `&self`    | `&self`   |
//! | Lock Overhead    | None       | None       | None      |
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
//! This example requires the `rc` feature.
//!
//! ```rust
//! # #[cfg(feature = "rc")]
//! # {
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
//! # }
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! # {
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
//! assert_eq!(value, 20); // (5 * 2) + 10
//! // first and second are still usable here
//! # }
//! ```
//!
//! ## Working with Closures
//!
//! Compatible closures automatically implement the `Mutator` trait. Chaining
//! starts from a concrete wrapper:
//!
//! ```rust
//! # {
//! use qubit_function::{Mutator, BoxMutator};
//!
//! // Closures can use .apply() directly
//! let mut closure = |x: &mut i32| *x *= 2;
//! let mut value = 5;
//! closure.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! let mut chained = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .and_then(|x: &mut i32| *x += 10);
//! let mut value = 5;
//! chained.apply(&mut value);
//! assert_eq!(value, 20);
//! # }
//! ```
//!
//! ## Wrapper Construction
//!
//! This example requires the `rc` feature.
//!
//! ```rust
//! # #[cfg(feature = "rc")]
//! # {
//! use qubit_function::{ArcMutator, BoxMutator, RcMutator};
//!
//! // Construct each ownership-specific wrapper from a closure.
//! let closure = |x: &mut i32| *x *= 2;
//! let box_mutator = BoxMutator::new(closure);
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let rc_mutator = RcMutator::new(closure);
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let arc_mutator = ArcMutator::new(closure);
//! # }
//! ```
//!
//! ## Conditional Execution
//!
//! All mutator types support conditional execution through the `when` method,
//! which returns a `ConditionalMutator`. You can optionally add an `or_else`
//! branch to create if-then-else logic.
//!
//! ```rust
//! # {
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
//! # }
//! ```
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

// ============================================================================
// 1. Type Aliases
// ============================================================================

/// Type alias for Arc-wrapped stateless mutator function
type ArcMutatorFn<T> = Arc<dyn Fn(&mut T) + Send + Sync>;

/// Type alias for Rc-wrapped stateless mutator function
#[cfg(feature = "rc")]
/// The erased callback representation used by this implementation.
type RcMutatorFn<T> = Rc<dyn Fn(&mut T)>;

mod box_mutator;
pub use box_mutator::BoxMutator;
#[cfg(feature = "rc")]
mod rc_mutator;
#[cfg(feature = "rc")]
pub use rc_mutator::RcMutator;
mod arc_mutator;
pub use arc_mutator::ArcMutator;
mod box_conditional_mutator;
pub use box_conditional_mutator::BoxConditionalMutator;
#[cfg(feature = "rc")]
mod rc_conditional_mutator;
#[cfg(feature = "rc")]
pub use rc_conditional_mutator::RcConditionalMutator;
mod arc_conditional_mutator;
pub use arc_conditional_mutator::ArcConditionalMutator;

// ============================================================================
// 2. Mutator Trait - Unified Mutator Interface
// ============================================================================

/// Mutator trait - Unified shared-receiver mutator interface
///
/// Defines the core behavior of all stateless mutator types. Performs
/// operations that accept a mutable reference and modify the input value
/// without requiring `&mut self`. Interior mutability and external side effects
/// remain possible.
///
/// This trait is automatically implemented by:
/// - All closures implementing `Fn(&mut T)` (stateless)
/// - `BoxMutator<T>`, `ArcMutator<T>`, and `RcMutator<T>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models for
/// **stateless** operations. Unlike `StatefulMutator` which uses `FnMut` and
/// can modify ordinary captured state, `Mutator` uses `Fn`. Implementations may
/// still use interior mutability or external side effects.
///
/// # Features
///
/// - **Shared-receiver calls**: Uses `Fn`, so invocation does not require `&mut
///   self`; interior mutability and external side effects remain possible
/// - **Unified Interface**: All mutator types share the same `mutate` method
///   signature
/// - **Automatic Implementation**: Closures automatically implement this trait
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
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::BoxMutator;
///
/// let closure = |x: &mut i32| *x *= 2;
///
/// // Convert to different ownership models
/// let box_mutator = BoxMutator::new(closure);
/// ```
pub trait Mutator<T> {
    /// Performs the stateless mutation operation
    ///
    /// Executes an operation on the given mutable reference without modifying
    /// ordinary captured state through the `Fn` receiver. Implementations may
    /// still use interior mutability or external side effects.
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
    /// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
    /// let mut value = 5;
    /// mutator.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn apply(&self, value: &mut T);
}

crate::macros::impl_closure_trait!(
    Mutator<T>,
    apply,
    Fn(value: &mut T)
);
