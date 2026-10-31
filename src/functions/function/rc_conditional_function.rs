// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalFunction` public type.

use crate::Function;
use crate::Predicate;
use crate::RcFunction;
use crate::RcPredicate;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// RcConditionalFunction - Rc-based Conditional Function
// ============================================================================

/// An intermediate builder holding an `RcFunction` and its `RcPredicate`.
///
/// Created by `RcFunction::when()`. This builder does not implement
/// `Function` and cannot be invoked directly. Call `or_else()` to provide
/// the fallback and obtain an `RcFunction`. Calling `apply()` on that
/// returned function evaluates the predicate and invokes the selected branch.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Branch Selection**: The completed wrapper invokes one selected branch
/// - **Ownership Cost**: `Rc` uses non-atomic reference counting; `Arc` uses
///   atomic reference counting, and neither wrapper locks callback invocation
///
/// # Type Parameters
///
/// - `T`: The input type borrowed by the predicate and wrapped function.
/// - `R`: The output type produced by the wrapped function.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Function, RcFunction};
///
/// let double = RcFunction::new(|x: &i32| x * 2);
/// let identity = RcFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(&5), 10);
/// assert_eq!(conditional_clone.apply(&-5), -5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalFunction<T, R>,
    RcFunction,
    Function,
    callback_bounds = ('static)
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalFunction<T, R>);
