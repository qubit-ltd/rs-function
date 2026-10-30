// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # MutatingFunction Types
//!
//! Provides Java-like `MutatingFunction` interface implementations for
//! performing operations that accept a mutable reference and return a result.
//!
//! It is similar to the `Fn(&mut T) -> R` trait in the standard library.
//!
//! This module provides a unified `MutatingFunction` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxMutatingFunction<T, R>`**: Box-based single ownership implementation
//! - **`ArcMutatingFunction<T, R>`**: Arc-based thread-safe shared ownership
//!   implementation
//! - **`RcMutatingFunction<T, R>`**: Rc-based single-threaded shared ownership
//!   implementation
//!
//! # Design Philosophy
//!
//! `MutatingFunction` uses the shared-receiver `Fn(&mut T) -> R` call shape:
//! invocation does not require `&mut self`, while interior mutability and
//! external side effects remain possible.
//!
//! `MutatingFunction` bridges the gap between `Function` and `Mutator`:
//!
//! - **Function**: `Fn(&T) -> R` - reads input, returns result
//! - **Mutator**: `Fn(&mut T)` - modifies input, no return
//! - **MutatingFunction**: `Fn(&mut T) -> R` - modifies input AND returns
//!   result
//!
//! ## Comparison with Related Types
//!
//! | Type | Input | Modifies? | Returns? | Use Cases |
//! |------|-------|-----------|----------|-----------|
//! | **Function** | `&T` | ❌ | ✅ | Read-only transform |
//! | **Mutator** | `&mut T` | ✅ | ❌ | In-place modification |
//! | **MutatingFunction** | `&mut T` | ✅ | ✅ | Modify + return info |
//! | **Transformer** | `T` | N/A | ✅ | Consume + transform |
//!
//! **Key Insight**: Use `MutatingFunction` when you need to both modify the
//! input and return information about the modification or the previous state.
//!
//! # Comparison Table
//!
//! | Feature          | Box | Arc | Rc |
//! |------------------|-----|-----|----|
//! | Ownership        | Single | Shared | Shared |
//! | Cloneable        | ❌ | ✅ | ✅ |
//! | Thread-Safe      | ❌ | ✅ | ❌ |
//! | Interior Mut.    | N/A | N/A | N/A |
//! | `and_then` API   | `self` | `&self` | `&self` |
//! | Lock Overhead    | None | None | None |
//!
//! # Use Cases
//!
//! ## Common Scenarios
//!
//! - **Atomic operations**: Increment counter and return new value
//! - **Cache updates**: Update cache and return old value
//! - **Validation**: Validate and fix data, return validation result
//! - **Event handlers**: Process event and return whether to continue
//! - **State machines**: Transition state and return transition info
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunction, MutatingFunction};
//!
//! // Increment counter and return new value
//! let incrementer = BoxMutatingFunction::new(|x: &mut i32| {
//!     *x += 1;
//!     *x
//! });
//!
//! let mut value = 5;
//! let result = incrementer.apply(&mut value);
//! assert_eq!(value, 6);
//! assert_eq!(result, 6);
//! ```
//!
//! ## Method Chaining
//!
//!
//! ```rust
//! # {
//! use qubit_function::{BoxMutatingFunction, MutatingFunction};
//!
//! let chained = BoxMutatingFunction::new(|x: &mut i32| {
//!     *x *= 2;
//!     *x
//! })
//! .and_then(|x: &i32| x + 10);
//!
//! let mut value = 5;
//! let result = chained.apply(&mut value);
//! assert_eq!(value, 10); // (5 * 2), value is still mutated by the first function
//! assert_eq!(result, 20);
//! # }
//! ```
//!
//! ## Cache Update Pattern
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunction, MutatingFunction};
//! use std::collections::HashMap;
//!
//! let updater = BoxMutatingFunction::new(
//!     |cache: &mut HashMap<String, i32>| {
//!         cache.insert("key".to_string(), 42)
//!     }
//! );
//!
//! let mut cache = HashMap::new();
//! cache.insert("key".to_string(), 10);
//! let old_value = updater.apply(&mut cache);
//! assert_eq!(old_value, Some(10));
//! assert_eq!(cache.get("key"), Some(&42));
//! ```

mod box_mutating_function;
pub use box_mutating_function::BoxMutatingFunction;
#[cfg(feature = "rc")]
mod rc_mutating_function;
#[cfg(feature = "rc")]
pub use rc_mutating_function::RcMutatingFunction;
mod arc_mutating_function;
pub use arc_mutating_function::ArcMutatingFunction;
mod box_conditional_mutating_function;
pub use box_conditional_mutating_function::BoxConditionalMutatingFunction;
#[cfg(feature = "rc")]
mod rc_conditional_mutating_function;
#[cfg(feature = "rc")]
pub use rc_conditional_mutating_function::RcConditionalMutatingFunction;
mod arc_conditional_mutating_function;
pub use arc_conditional_mutating_function::ArcConditionalMutatingFunction;
#[path = "mutating_function/mutating_function.rs"]
mod mutating_function_trait;
pub use mutating_function_trait::MutatingFunction;
