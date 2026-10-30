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
//! `Mutator` uses the `Fn(&mut T)` shared-receiver call shape. It can mutate
//! the input through `&mut T`, but cannot mutate ordinary captured values
//! through `&self`; implementations may still use interior mutability or
//! produce external side effects. `StatefulMutator` instead uses `FnMut` and
//! an `&mut self` receiver.
//!
//! ## Mutator vs StatefulMutator vs Consumer
//!
//! | Type | Receiver | Input | Input mutation | Ordinary captured state | Use Cases |
//! |------|----------|-------|----------------|--------------------------|-----------|
//! | **Consumer** | `&self` | `&T` | No direct mutation | `Fn` cannot mutate it; interior mutability is possible | Observe, log, notify |
//! | **Mutator** | `&self` | `&mut T` | Yes | `Fn` cannot mutate it; interior mutability is possible | Transform, validate, normalize |
//! | **StatefulMutator** | `&mut self` | `&mut T` | Yes | `FnMut` can mutate it | Stateful transform, accumulate |
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
//! // BoxMutator: Single ownership; calls borrow the mutator immutably
//! let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
//! let mut value = 5;
//! mutator.apply(&mut value);
//! assert_eq!(value, 10);
//! mutator.apply(&mut value);
//! assert_eq!(value, 20);
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
#[path = "mutator/mutator.rs"]
mod mutator_trait;
pub use mutator_trait::Mutator;
