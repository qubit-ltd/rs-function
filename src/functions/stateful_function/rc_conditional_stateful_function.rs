/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `RcConditionalStatefulFunction` public type.

use super::{
    Predicate,
    RcPredicate,
    RcStatefulFunction,
    StatefulFunction,
    impl_conditional_function_clone,
    impl_conditional_function_debug_display,
    impl_shared_conditional_function,
};

// ============================================================================
// RcConditionalStatefulFunction - Rc-based Conditional StatefulFunction
// ============================================================================

/// RcConditionalStatefulFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcStatefulFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulFunction`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulFunction, RcStatefulFunction};
///
/// let mut function = RcStatefulFunction::new(|x: &i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: &i32| -x);
///
/// let mut function_clone = function.clone();
///
/// assert_eq!(function.apply(&5), 10);
/// assert_eq!(function_clone.apply(&-5), 5);
/// ```
///
pub struct RcConditionalStatefulFunction<T, R> {
    pub(super) function: RcStatefulFunction<T, R>,
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalStatefulFunction<T, R>,
    RcStatefulFunction,
    StatefulFunction,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalStatefulFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalStatefulFunction<T, R>);
