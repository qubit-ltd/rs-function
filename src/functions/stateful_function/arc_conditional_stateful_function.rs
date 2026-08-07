// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalStatefulFunction` public type.

use crate::ArcPredicate;
use crate::ArcStatefulFunction;
use crate::Predicate;
use crate::StatefulFunction;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// ArcConditionalStatefulFunction - Arc-based Conditional StatefulFunction
// ============================================================================

/// ArcConditionalStatefulFunction struct
///
/// A thread-safe conditional function that only executes when a predicate
/// is satisfied. Uses `ArcStatefulFunction` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulFunction::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send`, safe for concurrent use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulFunction, ArcStatefulFunction};
///
/// let mut function = ArcStatefulFunction::new(|x: &i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: &i32| -x);
///
/// let mut function_clone = function.clone();
///
/// assert_eq!(function.apply(&5), 10);
/// assert_eq!(function_clone.apply(&-5), 5);
/// ```
///
/// # Locking and reentrancy
///
/// When the wrapped stateful callback executes, the underlying
/// `parking_lot::Mutex` remains locked until that callback returns.
/// Synchronous re-entry through the same shared state deadlocks. The mutex is
/// not poisoned after a panic, and mutations completed before a panic are not
/// rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalStatefulFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcStatefulFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalStatefulFunction<T, R>,
    ArcStatefulFunction,
    StatefulFunction,
    callback_bounds = (Send + 'static)
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalStatefulFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalStatefulFunction<T, R>);
