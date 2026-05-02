/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # StatefulFunction Types
//!
//! Provides Rust implementations of stateful function traits for stateful value
//! transformation. StatefulFunctions consume input values (taking ownership) and
//! produce output values while allowing internal state modification.
//!
//! It is similar to the `FnMut(&T) -> R` trait in the standard library.
//!
//! This module provides the `StatefulFunction<T, R>` trait and three implementations:
//!
//! - [`BoxStatefulFunction`]: Single ownership, not cloneable
//! - [`ArcStatefulFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulFunction`]: Single-threaded shared ownership, cloneable
//!
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::functions::{
    function_once::BoxFunctionOnce,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_fn_ops_trait,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
        impl_function_identity_method,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
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

mod box_stateful_function;
pub use box_stateful_function::BoxStatefulFunction;
mod rc_stateful_function;
pub use rc_stateful_function::RcStatefulFunction;
mod arc_stateful_function;
pub use arc_stateful_function::ArcStatefulFunction;
mod box_conditional_stateful_function;
pub use box_conditional_stateful_function::BoxConditionalStatefulFunction;
mod rc_conditional_stateful_function;
pub use rc_conditional_stateful_function::RcConditionalStatefulFunction;
mod arc_conditional_stateful_function;
pub use arc_conditional_stateful_function::ArcConditionalStatefulFunction;
mod fn_stateful_function_ops;
pub use fn_stateful_function_ops::FnStatefulFunctionOps;

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulFunction trait - transforms values from type T to type R with state
///
/// Defines the behavior of a stateful transformation: converting a value
/// of type `T` to a value of type `R` by consuming the input while
/// allowing modification of internal state. This is analogous to
/// `FnMut(&T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
pub trait StatefulFunction<T, R> {
    /// Applies the mapping to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&mut self, t: &T) -> R;

    /// Converts to BoxStatefulFunction
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxStatefulFunction<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `BoxStatefulFunction` by
    /// creating a new closure that calls `self.apply()`. This is a lightweight
    /// adapter, but it is not strictly zero-cost.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulFunction, BoxStatefulFunction};
    ///
    /// struct CustomStatefulFunction {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulFunction<i32, i32> for CustomStatefulFunction {
    ///     fn apply(&mut self, input: &i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let function = CustomStatefulFunction { multiplier: 0 };
    /// let mut boxed = function.into_box();
    /// assert_eq!(boxed.apply(&10), 10);  // 10 * 1
    /// assert_eq!(boxed.apply(&10), 20);  // 10 * 2
    /// ```
    fn into_box(mut self) -> BoxStatefulFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxStatefulFunction::new(move |t| self.apply(t))
    }

    /// Converts to RcStatefulFunction
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `RcStatefulFunction<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation first converts to `BoxStatefulFunction` using
    /// `into_box()`, then wraps it in `RcStatefulFunction`. Specific implementations
    /// may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulFunction, RcStatefulFunction};
    ///
    /// struct CustomStatefulFunction {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulFunction<i32, i32> for CustomStatefulFunction {
    ///     fn apply(&mut self, input: &i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let function = CustomStatefulFunction { multiplier: 0 };
    /// let mut rc_function = function.into_rc();
    /// assert_eq!(rc_function.apply(&10), 10);  // 10 * 1
    /// assert_eq!(rc_function.apply(&10), 20);  // 10 * 2
    /// ```
    fn into_rc(mut self) -> RcStatefulFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcStatefulFunction::new(move |t| self.apply(t))
    }

    /// Converts to ArcStatefulFunction
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `ArcStatefulFunction<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `ArcStatefulFunction` by creating
    /// a new closure that calls `self.apply()`. Note that this requires `self`
    /// to implement `Send` due to Arc's thread-safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulFunction, ArcStatefulFunction};
    ///
    /// struct CustomStatefulFunction {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulFunction<i32, i32> for CustomStatefulFunction {
    ///     fn apply(&mut self, input: &i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let function = CustomStatefulFunction { multiplier: 0 };
    /// let mut arc_function = function.into_arc();
    /// assert_eq!(arc_function.apply(&10), 10);  // 10 * 1
    /// assert_eq!(arc_function.apply(&10), 20);  // 10 * 2
    /// ```
    fn into_arc(mut self) -> ArcStatefulFunction<T, R>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulFunction::new(move |t| self.apply(t))
    }

    /// Converts to a closure implementing `FnMut(&T) -> R`
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns an implementation of `FnMut(&T) -> R`
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new closure that calls `self.apply()`.
    /// Specific implementations may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulFunction, BoxStatefulFunction};
    ///
    /// let function = BoxStatefulFunction::new(|x: &i32| x * 2);
    /// let mut closure = function.into_fn();
    /// assert_eq!(closure(&10), 20);
    /// assert_eq!(closure(&15), 30);
    /// ```
    fn into_fn(mut self) -> impl FnMut(&T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Converts to a mutable closure (`FnMut`) with an explicit method name.
    ///
    /// This is a naming alias of [`StatefulFunction::into_fn`] to make the
    /// mutability of the returned closure explicit.
    fn into_mut_fn(self) -> impl FnMut(&T) -> R
    where
        Self: Sized + 'static,
    {
        self.into_fn()
    }

    /// Convert to StatefulFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function will be unavailable
    /// after calling this method.
    ///
    /// Converts a reusable stateful function to a one-time function that
    /// consumes itself on use. This enables passing `StatefulFunction` to
    /// functions that require `StatefulFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FunctionOnce, StatefulFunction,
    ///                       RcStatefulFunction};
    ///
    /// fn takes_once<F: FunctionOnce<i32, i32>>(func: F, value: &i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = RcStatefulFunction::new(|x: &i32| x * 2);
    /// takes_once(func.into_once(), &5);
    /// ```
    fn into_once(mut self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxFunctionOnce::new(move |t| self.apply(t))
    }

    /// Non-consuming conversion to `BoxStatefulFunction`.
    ///
    /// Default implementation requires `Self: Clone` and wraps a cloned
    /// instance in a `RefCell` so the returned stateful function can mutate state
    /// across calls.
    fn to_box(&self) -> BoxStatefulFunction<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcStatefulFunction`.
    ///
    /// Default implementation clones `self` into an `Rc<RefCell<_>>` so the
    /// resulting stateful function can be shared within a single thread.
    fn to_rc(&self) -> RcStatefulFunction<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcStatefulFunction` (thread-safe).
    ///
    /// Default implementation requires `Self: Clone + Send + Sync` and wraps
    /// the cloned instance in `Arc<Mutex<_>>` so it can be used across
    /// threads.
    fn to_arc(&self) -> ArcStatefulFunction<T, R>
    where
        Self: Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a closure (`FnMut(&T) -> R`).
    ///
    /// Default implementation clones `self` into a `RefCell` and returns a
    /// closure that calls `apply` on the interior mutable value.
    fn to_fn(&self) -> impl FnMut(&T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Non-consuming conversion to a mutable closure (`FnMut`) with an explicit
    /// method name.
    ///
    /// This is a naming alias of [`StatefulFunction::to_fn`] and preserves the
    /// same clone-based behavior.
    fn to_mut_fn(&self) -> impl FnMut(&T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.to_fn()
    }

    /// Convert to StatefulFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current function and converts the clone to a one-time function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FunctionOnce, StatefulFunction,
    ///                       RcStatefulFunction};
    ///
    /// fn takes_once<F: FunctionOnce<i32, i32>>(func: F, value: &i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = RcStatefulFunction::new(|x: &i32| x * 2);
    /// takes_once(func.to_once(), &5);
    /// ```
    fn to_once(&self) -> BoxFunctionOnce<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
