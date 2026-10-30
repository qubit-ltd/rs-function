// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulMutatingFunction Types
//!
//! Provides Java-like `StatefulMutatingFunction` interface implementations
//! for performing operations that accept a mutable reference, potentially
//! modify internal state, and return a result. This module is available with
//! the `stateful` feature.
//!
//! It is similar to the `FnMut(&mut T) -> R` trait in the standard library.
//!
//! This module provides a unified `StatefulMutatingFunction` trait and three
//! concrete implementations based on different ownership models:
//!
//! - **`BoxStatefulMutatingFunction<T, R>`**: Box-based single ownership
//!   implementation
//! - **`ArcStatefulMutatingFunction<T, R>`**: Arc<Mutex<>>-based thread-safe
//!   shared ownership implementation
//! - **`RcStatefulMutatingFunction<T, R>`**: Rc<RefCell<>>-based
//!   single-threaded shared ownership implementation
//!
//! # Design Philosophy
//!
//! `StatefulMutatingFunction` extends `MutatingFunction` with the ability to
//! maintain internal state:
//!
//! - **MutatingFunction**: `Fn(&mut T) -> R`, called through `&self`; it
//!   can mutate the input but not ordinary captured values. Interior
//!   mutability and external side effects remain possible.
//! - **StatefulMutatingFunction**: `FnMut(&mut T) -> R`, called through
//!   `&mut self`; it can mutate both the input and ordinary captured state.
//!
//! ## Comparison with Related Types
//!
//! | Type | Self | Input | Modifies Self? | Modifies Input? | Returns? |
//! |------|------|-------|----------------|-----------------|----------|
//! | **StatefulFunction** | `&mut self` | `&T` | ✅ | ❌ | ✅ |
//! | **StatefulMutator** | `&mut self` | `&mut T` | ✅ | ✅ | ❌ |
//! | **StatefulMutatingFunction** | `&mut self` | `&mut T` | ✅ | ✅ | ✅ |
//!
//! **Key Insight**: Use `StatefulMutatingFunction` when you need to:
//! - Maintain internal state (counters, accumulators, etc.)
//! - Modify the input value
//! - Return information about the operation
//!
//! # Comparison Table
//!
//! | Feature          | Box | Arc | Rc |
//! |------------------|-----|-----|----|
//! | Ownership        | Single | Shared | Shared |
//! | Cloneable        | ❌ | ✅ | ✅ |
//! | Thread-Safe      | ❌ | ✅ | ❌ |
//! | Interior Mut.    | N/A | Mutex | RefCell |
//! | `and_then` API   | `self` | `&self` | `&self` |
//! | Lock Overhead    | None | Yes | None |
//!
//! # Use Cases
//!
//! ## Common Scenarios
//!
//! - **Stateful counters**: Increment and track modification count
//! - **Accumulators**: Collect statistics while modifying data
//! - **Rate limiters**: Track calls and conditionally modify
//! - **Validators**: Accumulate errors while fixing data
//! - **Stateful transformers**: Apply transformations based on history
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::BoxStatefulMutatingFunction;
//! use qubit_function::StatefulMutatingFunction;
//!
//! // Counter that increments value and tracks calls
//! let mut counter = {
//!     let mut call_count = 0;
//!     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
//!         call_count += 1;
//!         *x += 1;
//!         call_count
//!     })
//! };
//!
//! let mut value = 5;
//! assert_eq!(counter.apply(&mut value), 1);
//! assert_eq!(value, 6);
//! assert_eq!(counter.apply(&mut value), 2);
//! assert_eq!(value, 7);
//! ```
//!
//! ## Accumulator Pattern
//!
//! ```rust
//! use qubit_function::BoxStatefulMutatingFunction;
//! use qubit_function::StatefulMutatingFunction;
//!
//! // Accumulate sum while doubling values
//! let mut accumulator = {
//!     let mut sum = 0;
//!     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
//!         *x *= 2;
//!         sum += *x;
//!         sum
//!     })
//! };
//!
//! let mut value = 5;
//! assert_eq!(accumulator.apply(&mut value), 10);
//! assert_eq!(value, 10);
//!
//! let mut value2 = 3;
//! assert_eq!(accumulator.apply(&mut value2), 16); // 10 + 6
//! assert_eq!(value2, 6);
//! ```
#[path = "stateful_mutating_function/stateful_mutating_function.rs"]
mod stateful_mutating_function_trait;
pub use stateful_mutating_function_trait::StatefulMutatingFunction;
mod box_stateful_mutating_function;
pub use box_stateful_mutating_function::BoxStatefulMutatingFunction;
#[cfg(feature = "rc")]
mod rc_stateful_mutating_function;
#[cfg(feature = "rc")]
pub use rc_stateful_mutating_function::RcStatefulMutatingFunction;
mod arc_stateful_mutating_function;
pub use arc_stateful_mutating_function::ArcStatefulMutatingFunction;
mod box_conditional_stateful_mutating_function;
pub use box_conditional_stateful_mutating_function::BoxConditionalStatefulMutatingFunction;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_mutating_function;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_mutating_function::RcConditionalStatefulMutatingFunction;
mod arc_conditional_stateful_mutating_function;
pub use arc_conditional_stateful_mutating_function::ArcConditionalStatefulMutatingFunction;
