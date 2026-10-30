// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # MutatingFunctionOnce Types
//!
//! Provides Java-like one-time `MutatingFunction` interface implementations
//! for performing operations that consume self, accept a mutable reference,
//! and return a result.
//!
//! It is similar to the `FnOnce(&mut T) -> R` trait in the standard library.
//!
//! This module provides a unified `MutatingFunctionOnce` trait and a
//! Box-based single ownership implementation:
//!
//! - **`BoxMutatingFunctionOnce<T, R>`**: Box-based single ownership
//!   implementation for one-time use scenarios
//!
//! # Design Philosophy
//!
//! The key difference between `MutatingFunctionOnce` and
//! `MutatingFunction`:
//!
//! - **MutatingFunction**: `&self`, can be called multiple times, uses `Fn(&mut
//!   T) -> R`
//! - **MutatingFunctionOnce**: `self`, can only be called once, uses
//!   `FnOnce(&mut T) -> R`
//!
//! ## MutatingFunctionOnce vs MutatingFunction
//!
//! | Feature | MutatingFunction | MutatingFunctionOnce |
//! |---------|------------------|----------------------|
//! | **Self Parameter** | `&self` | `self` |
//! | **Call Count** | Multiple | Once |
//! | **Closure Type** | `Fn(&mut T) -> R` | `FnOnce(&mut T) -> R` |
//! | **Use Cases** | Repeatable operations | One-time resource
//! transfers |
//!
//! # Why MutatingFunctionOnce?
//!
//! Core value of MutatingFunctionOnce:
//!
//! 1. **Store FnOnce closures**: Allows moving captured variables
//! 2. **Delayed execution**: Store in data structures, execute later
//! 3. **Resource transfer**: Suitable for scenarios requiring ownership
//!    transfer
//! 4. **Return results**: Unlike MutatorOnce, returns information about the
//!    operation
//!
//! # Why Only Box Variant?
//!
//! - **Arc/Rc conflicts with FnOnce semantics**: FnOnce can only be called
//!   once, while shared ownership implies multiple references
//! - **Box is perfect match**: Single ownership aligns perfectly with one-time
//!   call semantics
//!
//! # Use Cases
//!
//! ## BoxMutatingFunctionOnce
//!
//! - Post-initialization callbacks (moving data, returning status)
//! - Resource transfer with result (moving Vec, returning old value)
//! - One-time complex operations (requiring moved capture variables)
//! - Validation with fixes (fix data once, return validation result)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! let data = vec![1, 2, 3];
//! let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
//!     let old_len = x.len();
//!     x.extend(data); // Move data
//!     old_len
//! });
//!
//! let mut target = vec![0];
//! let old_len = func.apply(&mut target);
//! assert_eq!(old_len, 1);
//! assert_eq!(target, vec![0, 1, 2, 3]);
//! ```
//!
//! ## Method Chaining
//!
//!
//! ```rust
//! # {
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! let data1 = vec![1, 2];
//!
//! let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data1);
//!     x.len()
//! })
//! .and_then(|len: &usize| len + 2);
//!
//! let mut target = vec![0];
//! let final_len = chained.apply(&mut target);
//! assert_eq!(final_len, 5);
//! assert_eq!(target, vec![0, 1, 2]);
//! # }
//! ```
//!
//! ## Validation Pattern
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! struct Data {
//!     value: i32,
//! }
//!
//! let validator = BoxMutatingFunctionOnce::new(|data: &mut Data| {
//!     if data.value < 0 {
//!         data.value = 0;
//!         Err("Fixed negative value")
//!     } else {
//!         Ok("Valid")
//!     }
//! });
//!
//! let mut data = Data { value: -5 };
//! let result = validator.apply(&mut data);
//! assert_eq!(data.value, 0);
//! assert!(result.is_err());
//! ```

mod box_mutating_function_once;
pub use box_mutating_function_once::BoxMutatingFunctionOnce;
mod box_conditional_mutating_function_once;
pub use box_conditional_mutating_function_once::BoxConditionalMutatingFunctionOnce;
#[path = "mutating_function_once/mutating_function_once.rs"]
mod mutating_function_once_trait;
pub use mutating_function_once_trait::MutatingFunctionOnce;
