/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
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
//! - **`BoxMutatingFunction<T, R>`**: Box-based single ownership
//!   implementation
//! - **`ArcMutatingFunction<T, R>`**: Arc-based thread-safe shared ownership
//!   implementation
//! - **`RcMutatingFunction<T, R>`**: Rc-based single-threaded shared
//!   ownership implementation
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
//!
use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    function::Function,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_fn_ops_trait,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_debug_display,
        impl_function_identity_method,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
    mutating_function_once::BoxMutatingFunctionOnce,
};
use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

mod box_mutating_function;
pub use box_mutating_function::BoxMutatingFunction;
mod rc_mutating_function;
pub use rc_mutating_function::RcMutatingFunction;
mod arc_mutating_function;
pub use arc_mutating_function::ArcMutatingFunction;
mod box_conditional_mutating_function;
pub use box_conditional_mutating_function::BoxConditionalMutatingFunction;
mod rc_conditional_mutating_function;
pub use rc_conditional_mutating_function::RcConditionalMutatingFunction;
mod arc_conditional_mutating_function;
pub use arc_conditional_mutating_function::ArcConditionalMutatingFunction;
mod fn_mutating_function_ops;
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
/// - **Unified Interface**: All mutating function types share the same
///   `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait
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
/// use qubit_function::MutatingFunction;
///
/// let closure = |x: &mut i32| {
///     *x *= 2;
///     *x
/// };
///
/// // Convert to different ownership models
/// let box_func = closure.into_box();
/// // let rc_func = closure.into_rc();  // closure moved
/// // let arc_func = closure.into_arc(); // closure moved
/// ```
///
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

    /// Convert this mutating function into a `BoxMutatingFunction<T, R>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns a
    /// boxed implementation that forwards calls to the original function.
    /// Types that can provide a more efficient conversion may override the
    /// default implementation.
    ///
    /// # Consumption
    ///
    /// This method consumes the function: the original value will no longer
    /// be available after the call. For cloneable functions call `.clone()`
    /// before converting if you need to retain the original instance.
    ///
    /// # Returns
    ///
    /// A `BoxMutatingFunction<T, R>` that forwards to the original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::MutatingFunction;
    ///
    /// let closure = |x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// };
    /// let mut boxed = closure.into_box();
    /// let mut value = 5;
    /// assert_eq!(boxed.apply(&mut value), 10);
    /// ```
    fn into_box(self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this mutating function into an `RcMutatingFunction<T, R>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Rc`-backed function that forwards calls to the original. Override to
    /// provide a more direct or efficient conversion when available.
    ///
    /// # Consumption
    ///
    /// This method consumes the function. If you need to keep the original
    /// instance, clone it prior to calling this method.
    ///
    /// # Returns
    ///
    /// An `RcMutatingFunction<T, R>` forwarding to the original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::MutatingFunction;
    ///
    /// let closure = |x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// };
    /// let mut rc = closure.into_rc();
    /// let mut value = 5;
    /// assert_eq!(rc.apply(&mut value), 10);
    /// ```
    fn into_rc(self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this mutating function into an `ArcMutatingFunction<T, R>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Arc`-wrapped, thread-safe function. Types may override the default
    /// implementation to provide a more efficient conversion.
    ///
    /// # Consumption
    ///
    /// This method consumes the function. Clone the instance first if you
    /// need to retain the original for further use.
    ///
    /// # Returns
    ///
    /// An `ArcMutatingFunction<T, R>` that forwards to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::MutatingFunction;
    ///
    /// let closure = |x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// };
    /// let mut arc = closure.into_arc();
    /// let mut value = 5;
    /// assert_eq!(arc.apply(&mut value), 10);
    /// ```
    fn into_arc(self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcMutatingFunction::new(move |t| self.apply(t))
    }

    /// Consume the function and return an `Fn(&mut T) -> R` closure.
    ///
    /// The returned closure forwards calls to the original function and is
    /// suitable for use with iterator adapters or other contexts expecting
    /// closures.
    ///
    /// # Consumption
    ///
    /// This method consumes the function. The original instance will not be
    /// available after calling this method.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&mut T) -> R` which forwards to the
    /// original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunction, BoxMutatingFunction};
    ///
    /// let func = BoxMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let closure = func.into_fn();
    /// let mut value = 5;
    /// assert_eq!(closure(&mut value), 10);
    /// ```
    fn into_fn(self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Convert to MutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function will be unavailable
    /// after calling this method.
    ///
    /// Converts a reusable mutating function to a one-time function that
    /// consumes itself on use. This enables passing `MutatingFunction` to
    /// functions that require `MutatingFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxMutatingFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunctionOnce, MutatingFunction,
    ///                       ArcMutatingFunction, BoxMutatingFunction};
    ///
    /// fn takes_once<F: MutatingFunctionOnce<i32, i32>>(func: F, value: &mut i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = BoxMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let mut value = 5;
    /// takes_once(func.into_once(), &mut value);
    /// ```
    fn into_once(self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxMutatingFunctionOnce::new(move |t| self.apply(t))
    }

    /// Create a non-consuming `BoxMutatingFunction<T, R>` that forwards to
    /// `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed function that calls the cloned instance. Override this
    /// method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxMutatingFunction<T, R>` that forwards to a clone of `self`.
    fn to_box(&self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Create a non-consuming `RcMutatingFunction<T, R>` that forwards to
    /// `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns an `Rc`-backed function that forwards calls to the clone.
    /// Override to provide a more direct or efficient conversion if needed.
    ///
    /// # Returns
    ///
    /// An `RcMutatingFunction<T, R>` that forwards to a clone of `self`.
    fn to_rc(&self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Create a non-consuming `ArcMutatingFunction<T, R>` that forwards to
    /// `self`.
    ///
    /// The default implementation clones `self` (requires
    /// `Clone + Send + Sync`) and returns an `Arc`-wrapped function that
    /// forwards calls to the clone. Override when a more efficient conversion
    /// is available.
    ///
    /// # Returns
    ///
    /// An `ArcMutatingFunction<T, R>` that forwards to a clone of `self`.
    fn to_arc(&self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Create a boxed `Fn(&mut T) -> R` closure that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed closure that invokes the cloned instance. Override to
    /// provide a more efficient conversion when possible.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&mut T) -> R` which forwards to the
    /// original function.
    fn to_fn(&self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to MutatingFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current function and converts the clone to a one-time function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxMutatingFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunctionOnce, MutatingFunction,
    ///                       ArcMutatingFunction};
    ///
    /// fn takes_once<F: MutatingFunctionOnce<i32, i32>>(func: F, value: &mut i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = ArcMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let mut value = 5;
    /// takes_once(func.to_once(), &mut value);
    /// ```
    fn to_once(&self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
