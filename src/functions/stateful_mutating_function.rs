/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # StatefulMutatingFunction Types
//!
//! Provides Java-like `StatefulMutatingFunction` interface implementations
//! for performing operations that accept a mutable reference, potentially
//! modify internal state, and return a result.
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
//! - **MutatingFunction**: `Fn(&mut T) -> R` - stateless, immutable self
//! - **StatefulMutatingFunction**: `FnMut(&mut T) -> R` - stateful, mutable
//!   self
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
//! use qubit_function::{BoxStatefulMutatingFunction,
//!                       StatefulMutatingFunction};
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
//! use qubit_function::{BoxStatefulMutatingFunction,
//!                       StatefulMutatingFunction};
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
//!
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

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
    impl_rc_conversions,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

mod box_stateful_mutating_function;
pub use box_stateful_mutating_function::BoxStatefulMutatingFunction;
mod rc_stateful_mutating_function;
pub use rc_stateful_mutating_function::RcStatefulMutatingFunction;
mod arc_stateful_mutating_function;
pub use arc_stateful_mutating_function::ArcStatefulMutatingFunction;
mod box_conditional_stateful_mutating_function;
pub use box_conditional_stateful_mutating_function::BoxConditionalStatefulMutatingFunction;
mod rc_conditional_stateful_mutating_function;
pub use rc_conditional_stateful_mutating_function::RcConditionalStatefulMutatingFunction;
mod arc_conditional_stateful_mutating_function;
pub use arc_conditional_stateful_mutating_function::ArcConditionalStatefulMutatingFunction;
mod fn_stateful_mutating_function_ops;
pub use fn_stateful_mutating_function_ops::FnStatefulMutatingFunctionOps;

// =======================================================================
// 1. StatefulMutatingFunction Trait - Unified Interface
// =======================================================================

/// StatefulMutatingFunction trait - Unified stateful mutating function
/// interface
///
/// It is similar to the `FnMut(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all stateful mutating function types.
/// Performs operations that accept a mutable reference, potentially modify
/// both the function's internal state and the input, and return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T) -> R`
/// - `BoxStatefulMutatingFunction<T, R>`,
///   `ArcStatefulMutatingFunction<T, R>`, and
///   `RcStatefulMutatingFunction<T, R>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models
/// for operations that need to maintain state while modifying input and
/// returning results. This is useful for scenarios where you need to:
/// - Track statistics or counts during modifications
/// - Accumulate information across multiple calls
/// - Implement stateful validators or transformers
///
/// # Features
///
/// - **Unified Interface**: All stateful mutating function types share the
///   same `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions that work with any stateful
///   mutating function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       BoxStatefulMutatingFunction};
///
/// fn apply_and_log<F: StatefulMutatingFunction<i32, i32>>(
///     func: &mut F,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     let result = func.apply(&mut val);
///     println!("Modified: {} -> {}, returned: {}", value, val, result);
///     result
/// }
///
/// let mut counter = {
///     let mut count = 0;
///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x += 1;
///         count
///     })
/// };
/// assert_eq!(apply_and_log(&mut counter, 5), 1);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::StatefulMutatingFunction;
///
/// let mut count = 0;
/// let closure = move |x: &mut i32| {
///     count += 1;
///     *x *= 2;
///     count
/// };
///
/// // Convert to different ownership models
/// let mut box_func = closure.into_box();
/// // let mut rc_func = closure.into_rc();  // closure moved
/// // let mut arc_func = closure.into_arc(); // closure moved
/// ```
///
pub trait StatefulMutatingFunction<T, R> {
    /// Applies the function to the mutable reference and returns a result
    ///
    /// Executes an operation on the given mutable reference, potentially
    /// modifying both the function's internal state and the input, and
    /// returns a result value.
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
    /// use qubit_function::{StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// let mut counter = {
    ///     let mut count = 0;
    ///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
    ///         count += 1;
    ///         let old = *x;
    ///         *x += 1;
    ///         (old, count)
    ///     })
    /// };
    ///
    /// let mut value = 5;
    /// let (old_value, call_count) = counter.apply(&mut value);
    /// assert_eq!(old_value, 5);
    /// assert_eq!(call_count, 1);
    /// assert_eq!(value, 6);
    /// ```
    fn apply(&mut self, t: &mut T) -> R;

    /// Convert this function into a `BoxStatefulMutatingFunction<T, R>`.
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
    /// A `BoxStatefulMutatingFunction<T, R>` that forwards to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut count = 0;
    /// let closure = move |x: &mut i32| {
    ///     count += 1;
    ///     *x *= 2;
    ///     count
    /// };
    /// let mut boxed = closure.into_box();
    /// let mut value = 5;
    /// assert_eq!(boxed.apply(&mut value), 1);
    /// ```
    fn into_box(mut self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxStatefulMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this function into an `RcStatefulMutatingFunction<T, R>`.
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
    /// An `RcStatefulMutatingFunction<T, R>` forwarding to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut count = 0;
    /// let closure = move |x: &mut i32| {
    ///     count += 1;
    ///     *x *= 2;
    ///     count
    /// };
    /// let mut rc = closure.into_rc();
    /// let mut value = 5;
    /// assert_eq!(rc.apply(&mut value), 1);
    /// ```
    fn into_rc(mut self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcStatefulMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this function into an `ArcStatefulMutatingFunction<T, R>`.
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
    /// An `ArcStatefulMutatingFunction<T, R>` that forwards to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut count = 0;
    /// let closure = move |x: &mut i32| {
    ///     count += 1;
    ///     *x *= 2;
    ///     count
    /// };
    /// let mut arc = closure.into_arc();
    /// let mut value = 5;
    /// assert_eq!(arc.apply(&mut value), 1);
    /// ```
    fn into_arc(mut self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulMutatingFunction::new(move |t| self.apply(t))
    }

    /// Consume the function and return an `FnMut(&mut T) -> R` closure.
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
    /// A closure implementing `FnMut(&mut T) -> R` which forwards to the
    /// original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// let func = {
    ///     let mut sum = 0;
    ///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
    ///         *x *= 2;
    ///         sum += *x;
    ///         sum
    ///     })
    /// };
    /// let mut closure = func.into_fn();
    /// let mut value = 5;
    /// assert_eq!(closure(&mut value), 10);
    /// ```
    fn into_fn(mut self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Consume the function and return an explicit `FnMut` closure.
    ///
    /// This is a naming alias of [`StatefulMutatingFunction::into_fn`] to make
    /// the mutability of the returned closure explicit.
    fn into_mut_fn(self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + 'static,
    {
        self.into_fn()
    }

    /// Create a non-consuming `BoxStatefulMutatingFunction<T, R>` that
    /// forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed function that calls the cloned instance. Override this
    /// method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulMutatingFunction<T, R>` that forwards to a clone of
    /// `self`.
    fn to_box(&self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Create a non-consuming `RcStatefulMutatingFunction<T, R>` that
    /// forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns an `Rc`-backed function that forwards calls to the clone.
    /// Override to provide a more direct or efficient conversion if needed.
    ///
    /// # Returns
    ///
    /// An `RcStatefulMutatingFunction<T, R>` that forwards to a clone of
    /// `self`.
    fn to_rc(&self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Create a non-consuming `ArcStatefulMutatingFunction<T, R>` that
    /// forwards to `self`.
    ///
    /// The default implementation clones `self` (requires
    /// `Clone + Send`) and returns an `Arc`-wrapped function that forwards
    /// calls to the clone. Override when a more efficient conversion is
    /// available.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulMutatingFunction<T, R>` that forwards to a clone of
    /// `self`.
    fn to_arc(&self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Create a boxed `FnMut(&mut T) -> R` closure that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed closure that invokes the cloned instance. Override to
    /// provide a more efficient conversion when possible.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> R` which forwards to the
    /// original function.
    fn to_fn(&self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Create a non-consuming explicit `FnMut` closure from `self`.
    ///
    /// This is a naming alias of [`StatefulMutatingFunction::to_fn`] and
    /// preserves the same clone-based behavior.
    fn to_mut_fn(&self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.to_fn()
    }

    /// Convert to StatefulMutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function will be unavailable
    /// after calling this method.
    ///
    /// Converts a reusable stateful mutating function to a one-time function
    /// that consumes itself on use. This enables passing `StatefulMutatingFunction`
    /// to functions that require `StatefulMutatingFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxMutatingFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunctionOnce,
    ///                       StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// fn takes_once<F: MutatingFunctionOnce<i32, i32>>(func: F, value: &mut i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = BoxStatefulMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let mut value = 5;
    /// takes_once(func.into_once(), &mut value);
    /// ```
    fn into_once(mut self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxMutatingFunctionOnce::new(move |t| self.apply(t))
    }

    /// Convert to StatefulMutatingFunctionOnce without consuming self
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
    /// use qubit_function::{MutatingFunctionOnce,
    ///                       StatefulMutatingFunction,
    ///                       RcStatefulMutatingFunction};
    ///
    /// fn takes_once<F: MutatingFunctionOnce<i32, i32>>(func: F, value: &mut i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = RcStatefulMutatingFunction::new(|x: &mut i32| {
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

// =======================================================================
// 2. Type Aliases
// =======================================================================

/// Type alias for Arc-wrapped stateful mutating function
type ArcStatefulMutatingFunctionFn<T, R> = Arc<Mutex<dyn FnMut(&mut T) -> R + Send + 'static>>;

/// Type alias for Rc-wrapped stateful mutating function
type RcStatefulMutatingFunctionFn<T, R> = Rc<RefCell<dyn FnMut(&mut T) -> R>>;
