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
//! Defines the `ArcConditionalStatefulMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcConditionalStatefulMutatingFunction - Arc-based Conditional Stateful Mutating Function
// ============================================================================

/// ArcConditionalStatefulMutatingFunction struct
///
/// A thread-safe conditional function that only executes when a predicate is
/// satisfied. Uses `ArcStatefulMutatingFunction` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcStatefulMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction, ArcStatefulMutatingFunction};
///
/// let double = ArcStatefulMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let identity = ArcStatefulMutatingFunction::<i32, i32>::identity();
/// let mut conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let mut conditional_clone = conditional.clone();
///
/// let mut positive = 5;
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut positive), 10);
/// assert_eq!(conditional_clone.apply(&mut negative), -5);
/// ```
///
pub struct ArcConditionalStatefulMutatingFunction<T, R> {
    pub(super) function: ArcStatefulMutatingFunction<T, R>,
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalStatefulMutatingFunction<T, R>,
    ArcStatefulMutatingFunction,
    StatefulMutatingFunction,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalStatefulMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalStatefulMutatingFunction<T, R>);
