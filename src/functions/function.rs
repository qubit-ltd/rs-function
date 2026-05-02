/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Function Types
//!
//! Provides Rust implementations of function traits for computing output values
//! from input references. Functions borrow input values (not consuming them)
//! and produce output values.
//!
//! It is similar to the `Fn(&T) -> R` trait in the standard library.
//!
//! This module provides the `Function<T, R>` trait and three
//! implementations:
//!
//! - [`BoxFunction`]: Single ownership, not cloneable
//! - [`ArcFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcFunction`]: Single-threaded shared ownership, cloneable
//!
use std::rc::Rc;
use std::sync::Arc;

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
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

mod box_function;
pub use box_function::BoxFunction;
mod rc_function;
pub use rc_function::RcFunction;
mod arc_function;
pub use arc_function::ArcFunction;
mod box_conditional_function;
pub use box_conditional_function::BoxConditionalFunction;
mod rc_conditional_function;
pub use rc_conditional_function::RcConditionalFunction;
mod arc_conditional_function;
pub use arc_conditional_function::ArcConditionalFunction;
mod fn_function_ops;
pub use fn_function_ops::FnFunctionOps;

// ============================================================================
// Core Trait
// ============================================================================

/// Function trait - computes output from input reference
///
/// Defines the behavior of a function: computing a value of type `R`
/// from a reference to type `T` without consuming the input. This is analogous to
/// `Fn(&T) -> R` in Rust's standard library, similar to Java's `Function<T, R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
///
pub trait Function<T, R> {
    /// Applies the function to the input reference to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, t: &T) -> R;

    /// Converts to BoxFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunction<T, R>`
    fn into_box(self) -> BoxFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxFunction::new(move |t| self.apply(t))
    }

    /// Converts to RcFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcFunction<T, R>`
    fn into_rc(self) -> RcFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcFunction::new(move |t| self.apply(t))
    }

    /// Converts to ArcFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcFunction<T, R>`
    fn into_arc(self) -> ArcFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcFunction::new(move |t| self.apply(t))
    }

    /// Converts function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `transform` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&T) -> R`
    fn into_fn(self) -> impl Fn(&T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Converts to FunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function becomes unavailable after calling this method.
    ///
    /// Converts a reusable function to a one-time function that consumes itself on use.
    /// This enables passing `Function` to functions that require `FunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxFunction, Function, FunctionOnce};
    ///
    /// fn takes_once<F: FunctionOnce<i32, i32>>(func: F, value: &i32) -> i32 {
    ///     func.apply(value)
    /// }
    ///
    /// let func = BoxFunction::new(|x: &i32| x * 2);
    /// let result = takes_once(func.into_once(), &5);
    /// assert_eq!(result, 10);
    /// ```
    fn into_once(self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxFunctionOnce::new(move |t| self.apply(t))
    }

    /// Converts to BoxFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: &i32| x * 2);
    /// let boxed = double.to_box();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(&21), 42);
    /// assert_eq!(boxed.apply(&21), 42);
    /// ```
    fn to_box(&self) -> BoxFunction<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts to RcFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `RcFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcFunction, Function};
    ///
    /// let double = RcFunction::new(|x: &i32| x * 2);
    /// let rc = double.to_rc();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(&21), 42);
    /// assert_eq!(rc.apply(&21), 42);
    /// ```
    fn to_rc(&self) -> RcFunction<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to ArcFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `ArcFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: &i32| x * 2);
    /// let arc = double.to_arc();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(&21), 42);
    /// assert_eq!(arc.apply(&21), 42);
    /// ```
    fn to_arc(&self) -> ArcFunction<T, R>
    where
        Self: Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts function to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures a
    /// clone of `self` and calls its `transform` method. Types can
    /// override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: &i32| x * 2);
    /// let closure = double.to_fn();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(&21), 42);
    /// assert_eq!(closure(&21), 42);
    /// ```
    fn to_fn(&self) -> impl Fn(&T) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to FunctionOnce without consuming self
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
    ///
    /// use qubit_function::{Function, FunctionOnce, RcFunction};
    ///
    /// fn takes_once<F: FunctionOnce<i32, i32>>(func: F, value: &i32) -> i32 {
    ///     func.apply(value)
    /// }
    ///
    /// let func = RcFunction::new(|x: &i32| x * 2);
    /// let result = takes_once(func.to_once(), &5);
    /// assert_eq!(result, 10);
    /// ```
    fn to_once(&self) -> BoxFunctionOnce<T, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
