// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalFunction` public type.

use crate::ArcFunction;
use crate::ArcPredicate;
use crate::Function;
use crate::Predicate;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// ArcConditionalFunction - Arc-based Conditional Function
// ============================================================================

/// An intermediate builder holding an `ArcFunction` and its `ArcPredicate`.
///
/// Created by `ArcFunction::when()`. This builder does not implement
/// `Function` and cannot be invoked directly. Call `or_else()` to provide
/// the fallback and obtain an `ArcFunction`. Calling `apply()` on that
/// returned function evaluates the predicate and invokes the selected branch.
/// Both callbacks and the predicate use shared ownership, so cloning this
/// builder does not consume the original value.
///
/// # Type Parameters
///
/// - `T` is the input type borrowed by the callback and predicate.
/// - `R` is the value returned when the callback is invoked.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Branch Selection**: The completed wrapper invokes one selected branch
/// - **Completion**: Call `or_else()` to obtain an invocable wrapper
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Function, ArcFunction};
///
/// let double = ArcFunction::new(|x: &i32| x * 2);
/// let identity = ArcFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(&5), 10);
/// assert_eq!(conditional_clone.apply(&-5), -5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalFunction<T, R>,
    ArcFunction,
    Function,
    callback_bounds = (Send + Sync + 'static)
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalFunction<T, R>);
