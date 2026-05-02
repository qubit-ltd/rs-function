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
//! Defines the `RcConditionalMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcConditionalMutatingFunction - Rc-based Conditional Mutating Function
// ============================================================================

/// RcConditionalMutatingFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcMutatingFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalFunction`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatingFunction, RcMutatingFunction};
///
/// let double = RcMutatingFunction::new(|x: &mut i32| *x * 2);
/// let identity = RcMutatingFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// let mut positive = 5;
/// assert_eq!(conditional.apply(&mut positive), 10);
/// let mut negative = -5;
/// assert_eq!(conditional_clone.apply(&mut negative), -5);
/// ```
///
pub struct RcConditionalMutatingFunction<T, R> {
    pub(super) function: RcMutatingFunction<T, R>,
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalMutatingFunction<T, R>,
    RcMutatingFunction,
    MutatingFunction,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalMutatingFunction<T, R>);
