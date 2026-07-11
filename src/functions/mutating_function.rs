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
//! ```rust
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
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "combinators")]
use crate::functions::macros::impl_fn_ops_trait;
use crate::functions::{
    function::Function,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_debug_display,
        impl_function_identity_method,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
};
use crate::macros::impl_closure_trait;
#[cfg(feature = "rc")]
use crate::predicates::predicate::RcPredicate;
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
};

mod box_mutating_function;
pub use box_mutating_function::BoxMutatingFunction;
#[cfg(feature = "rc")]
mod rc_mutating_function;
#[cfg(feature = "rc")]
pub use rc_mutating_function::RcMutatingFunction;
mod arc_mutating_function;
pub use arc_mutating_function::ArcMutatingFunction;
mod box_conditional_mutating_function;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_mutating_function::BoxConditionalMutatingFunction;
#[cfg(feature = "combinators")]
pub use box_conditional_mutating_function::BoxConditionalMutatingFunction;
#[cfg(feature = "rc")]
mod rc_conditional_mutating_function;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_mutating_function::RcConditionalMutatingFunction;
#[cfg(all(feature = "rc", feature = "combinators"))]
pub use rc_conditional_mutating_function::RcConditionalMutatingFunction;
mod arc_conditional_mutating_function;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_mutating_function::ArcConditionalMutatingFunction;
#[cfg(feature = "combinators")]
pub use arc_conditional_mutating_function::ArcConditionalMutatingFunction;
#[cfg(feature = "combinators")]
mod fn_mutating_function_ops;
#[cfg(feature = "combinators")]
pub use fn_mutating_function_ops::FnMutatingFunctionOps;

// =======================================================================
// 1. MutatingFunction Trait - Unified Interface
// =======================================================================

/// MutatingFunction trait - Unified mutating function interface
///
/// It is similar to the `Fn(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all mutating function types. Performs
/// operations that accept a mutable reference, potentially modify it, and
/// return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `Fn(&mut T) -> R`
/// - `BoxMutatingFunction<T, R>`, `ArcMutatingFunction<T, R>`, and
///   `RcMutatingFunction<T, R>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models
/// for operations that both modify input and return results. This is useful
/// for scenarios where you need to:
/// - Update state and return information about the update
/// - Perform atomic-like operations (modify and return)
/// - Implement event handlers that modify state and signal continuation
///
/// # Features
///
/// - **Unified Interface**: All mutating function types share the same `apply`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this trait
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions that work with any mutating
///   function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatingFunction, BoxMutatingFunction};
///
/// fn apply_and_log<F: MutatingFunction<i32, i32>>(
///     func: &F,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     let result = func.apply(&mut val);
///     println!("Modified: {} -> {}, returned: {}", value, val, result);
///     result
/// }
///
/// let incrementer = BoxMutatingFunction::new(|x: &mut i32| {
///     *x += 1;
///     *x
/// });
/// assert_eq!(apply_and_log(&incrementer, 5), 6);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::BoxMutatingFunction;
///
/// let closure = |x: &mut i32| {
///     *x *= 2;
///     *x
/// };
///
/// // Convert to different ownership models
/// let box_func = BoxMutatingFunction::new(closure);
/// ```
pub trait MutatingFunction<T, R> {
    /// Applies the function to the mutable reference and returns a result
    ///
    /// Executes an operation on the given mutable reference, potentially
    /// modifying it, and returns a result value.
    ///
    /// # Parameters
    ///
    /// * `t` - A mutable reference to the input value
    ///
    /// # Returns
    ///
    /// The computed result value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunction, BoxMutatingFunction};
    ///
    /// let func = BoxMutatingFunction::new(|x: &mut i32| {
    ///     let old = *x;
    ///     *x += 1;
    ///     old
    /// });
    ///
    /// let mut value = 5;
    /// let old_value = func.apply(&mut value);
    /// assert_eq!(old_value, 5);
    /// assert_eq!(value, 6);
    /// ```
    fn apply(&self, t: &mut T) -> R;
}
