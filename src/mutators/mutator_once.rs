// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # MutatorOnce Types
//!
//! Provides Java-style one-time `Mutator` interface implementations for
//! performing operations that consume self and modify the input value.
//!
//! It is similar to the `FnOnce(&mut T)` trait in the standard library.
//!
//! This module provides a unified `MutatorOnce` trait and a Box-based single
//! ownership implementation:
//!
//! - **`BoxMutatorOnce<T>`**: Box-based single ownership implementation for
//!   one-time use scenarios
//!
//! # Design Philosophy
//!
//! The key difference between `MutatorOnce` and `Mutator`:
//!
//! - **Mutator**: `&self`, can be called multiple times, uses `Fn(&mut T)`
//! - **MutatorOnce**: `self`, can only be called once, uses `FnOnce(&mut T)`
//!
//! ## MutatorOnce vs Mutator
//!
//! | Feature | Mutator | MutatorOnce |
//! |---------|---------|-------------|
//! | **Self Parameter** | `&self` | `self` |
//! | **Call Count** | Multiple | Once |
//! | **Closure Type** | `Fn(&mut T)` | `FnOnce(&mut T)` |
//! | **Use Cases** | Repeatable modifications | One-time resource transfers, init callbacks |
//!
//! # Why MutatorOnce?
//!
//! Core value of MutatorOnce:
//!
//! 1. **Store FnOnce closures**: Allows moving captured variables
//! 2. **Delayed execution**: Store in data structures, execute later
//! 3. **Resource transfer**: Suitable for scenarios requiring ownership
//!    transfer
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
//! ## BoxMutatorOnce
//!
//! - Post-initialization callbacks (moving data)
//! - Resource transfer (moving Vec, String, etc.)
//! - One-time complex operations (requiring moved capture variables)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutatorOnce, MutatorOnce};
//!
//! let data = vec![1, 2, 3];
//! let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data); // Move data
//! });
//!
//! let mut target = vec![0];
//! mutator.apply(&mut target);
//! assert_eq!(target, vec![0, 1, 2, 3]);
//! ```
//!
//! ## Method Chaining
//!
//!
//! ```rust
//! # {
//! use qubit_function::{BoxMutatorOnce, MutatorOnce};
//!
//! let data1 = vec![1, 2];
//! let data2 = vec![3, 4];
//!
//! let chained = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data1);
//! })
//! .and_then(move |x: &mut Vec<i32>| {
//!     x.extend(data2);
//! });
//!
//! let mut target = vec![0];
//! chained.apply(&mut target);
//! assert_eq!(target, vec![0, 1, 2, 3, 4]);
//! # }
//! ```
//!
//! ## Initialization Callback
//!
//! ```rust
//! use qubit_function::{BoxMutatorOnce, MutatorOnce};
//!
//! struct Initializer {
//!     on_complete: Option<BoxMutatorOnce<Vec<i32>>>,
//! }
//!
//! impl Initializer {
//!     fn new<F>(callback: F) -> Self
//!     where
//!         F: FnOnce(&mut Vec<i32>) + 'static
//!     {
//!         Self {
//!             on_complete: Some(BoxMutatorOnce::new(callback))
//!         }
//!     }
//!
//!     fn run(mut self, data: &mut Vec<i32>) {
//!         // Execute initialization logic
//!         data.push(42);
//!
//!         // Call callback
//!         if let Some(callback) = self.on_complete.take() {
//!             callback.apply(data);
//!         }
//!     }
//! }
//!
//! let data_to_add = vec![1, 2, 3];
//! let init = Initializer::new(move |x| {
//!     x.extend(data_to_add); // Move data_to_add
//! });
//!
//! let mut result = Vec::new();
//! init.run(&mut result);
//! assert_eq!(result, vec![42, 1, 2, 3]);
//! ```

mod box_mutator_once;
pub use box_mutator_once::BoxMutatorOnce;
mod box_conditional_mutator_once;
pub use box_conditional_mutator_once::BoxConditionalMutatorOnce;

#[path = "mutator_once/mutator_once.rs"]
mod mutator_once_trait;
pub use mutator_once_trait::MutatorOnce;
