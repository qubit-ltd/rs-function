// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Stateful Mutator Types
//!
//! Provides `StatefulMutator` implementations for operations that accept a
//! mutable input, may update captured state, and return no result. This module
//! is available with the `stateful` feature.
//!
//! This module provides a unified `StatefulMutator` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxStatefulMutator<T>`**: Box-based single ownership implementation for
//!   stored callbacks and builder patterns
//! - **`ArcStatefulMutator<T>`**: `Arc<Mutex<_>>`-based thread-safe shared
//!   ownership implementation for multi-threaded scenarios
//! - **`RcStatefulMutator<T>`**: `Rc<RefCell<_>>`-based single-threaded shared
//!   ownership implementation with no lock overhead
//!
//! It is similar to the `FnMut(&mut T)` trait in the standard library.
//!
//! # Design Philosophy
//!
//! `StatefulMutator` is called through `&mut self` and accepts `&mut T`, so its
//! `FnMut(&mut T)` callback can update both the input and ordinary captured
//! state. `Mutator` is called through `&self`; its `Fn(&mut T)` callback can
//! modify the input and can use interior mutability, but cannot mutate ordinary
//! captured values through the shared receiver.
//!
//! | Type | Receiver | Callback | Input access | Ordinary captured state |
//! |------|----------|----------|--------------|--------------------------|
//! | **Mutator** | `&self` | `Fn(&mut T)` | Mutable | Not mutable through the receiver; interior mutability is possible |
//! | **StatefulMutator** | `&mut self` | `FnMut(&mut T)` | Mutable | Mutable |
//!
//! # Comparison Table
//!
//! | Feature | BoxStatefulMutator | ArcStatefulMutator | RcStatefulMutator |
//! |---------|--------------------|--------------------|-------------------|
//! | Ownership | Single | Shared | Shared |
//! | Cloneable | ❌ | ✅ | ✅ |
//! | Thread-safe | ❌ | ✅ | ❌ |
//! | Interior mutability | None | Mutex | RefCell |
//! | `and_then` receiver | `self` | `&self` | `&self` |
//! | Lock overhead | None | Yes | None |
//!
//! # Use Cases
//!
//! ## BoxStatefulMutator
//!
//! - Repeatable operations with single ownership
//! - Builder patterns where ownership naturally flows
//! - Scenarios where the mutator is reused without sharing
//!
//! ## ArcStatefulMutator
//!
//! - Multi-threaded shared operations
//! - Concurrent task processing (e.g., thread pools)
//! - Situations requiring the same mutator across threads
//!
//! ## RcStatefulMutator
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
//! use qubit_function::{BoxStatefulMutator, StatefulMutator};
//!
//! let mut call_count = 0;
//! let mut mutator = BoxStatefulMutator::new(move |x: &mut i32| {
//!     call_count += 1;
//!     *x += call_count;
//! });
//! let mut value = 10;
//! mutator.apply(&mut value);
//! assert_eq!(value, 11);
//! mutator.apply(&mut value);
//! assert_eq!(value, 13);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! # {
//! use qubit_function::{BoxStatefulMutator, StatefulMutator};
//!
//! let mut call_count = 0;
//! let mut chained = BoxStatefulMutator::new(move |x: &mut i32| {
//!     call_count += 1;
//!     *x += call_count;
//! }).and_then(|x: &mut i32| *x *= 2);
//! let mut value = 10;
//! chained.apply(&mut value);
//! assert_eq!(value, 22);
//! # }
//! ```
//!
//! ## Working with Closures
//!
//! `FnMut(&mut T)` closures automatically implement `StatefulMutator`.
//! Chaining starts from a concrete wrapper:
//!
//! ```rust
//! # {
//! use qubit_function::{BoxStatefulMutator, StatefulMutator};
//!
//! let mut call_count = 0;
//! let mut chained = BoxStatefulMutator::new(move |x: &mut i32| {
//!     call_count += 1;
//!     *x += call_count;
//! }).and_then(|x: &mut i32| *x *= 2);
//! let mut value = 10;
//! chained.apply(&mut value);
//! assert_eq!(value, 22);
//! # }
//! ```
//!
//! ## Wrapper Construction
//!
//! This example additionally requires the `rc` feature.
//!
//! ```rust
//! # #[cfg(feature = "rc")]
//! # {
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
//! # }
//! ```
//!
//! ## Conditional Execution
//!
//! Stateful mutator wrappers support conditional execution through `when` and
//! an optional `or_else` branch:
//!
//! ```rust
//! # {
//! use qubit_function::{BoxStatefulMutator, StatefulMutator};
//!
//! let mut call_count = 0;
//! let mut conditional = BoxStatefulMutator::new(move |x: &mut i32| {
//!     call_count += 1;
//!     *x += call_count;
//! })
//!     .when(|x: &i32| *x > 0);
//!
//! let mut positive = 5;
//! conditional.apply(&mut positive);
//! assert_eq!(positive, 6);
//!
//! let mut negative = -5;
//! conditional.apply(&mut negative);
//! assert_eq!(negative, -5);
//!
//! let mut branch_calls = 0;
//! let mut branched = BoxStatefulMutator::new(move |x: &mut i32| {
//!     branch_calls += 1;
//!     *x += branch_calls;
//! })
//!     .when(|x: &i32| *x > 0)
//!     .or_else(|x: &mut i32| *x -= 1);
//!
//! let mut positive = 5;
//! branched.apply(&mut positive);
//! assert_eq!(positive, 6);
//!
//! let mut negative = -5;
//! branched.apply(&mut negative);
//! assert_eq!(negative, -6);
//! # }
//! ```
mod box_stateful_mutator;
pub use box_stateful_mutator::BoxStatefulMutator;
#[cfg(feature = "rc")]
mod rc_stateful_mutator;
#[cfg(feature = "rc")]
pub use rc_stateful_mutator::RcStatefulMutator;
mod arc_stateful_mutator;
pub use arc_stateful_mutator::ArcStatefulMutator;
mod box_conditional_stateful_mutator;
pub use box_conditional_stateful_mutator::BoxConditionalStatefulMutator;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_mutator;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_mutator::RcConditionalStatefulMutator;
mod arc_conditional_stateful_mutator;
pub use arc_conditional_stateful_mutator::ArcConditionalStatefulMutator;
#[path = "stateful_mutator/stateful_mutator.rs"]
mod stateful_mutator_trait;
pub use stateful_mutator_trait::StatefulMutator;
