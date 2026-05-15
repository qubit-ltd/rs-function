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
//! Defines the `ArcConditionalMutatingFunction` public type.

use super::{
    ArcMutatingFunction,
    ArcPredicate,
    MutatingFunction,
    Predicate,
    impl_conditional_function_clone,
    impl_conditional_function_debug_display,
    impl_shared_conditional_function,
};

// ============================================================================
// ArcConditionalMutatingFunction - Arc-based Conditional Mutating Function
// ============================================================================

/// ArcConditionalMutatingFunction struct
///
/// A thread-safe conditional function that only executes when a predicate is
/// satisfied. Uses `ArcMutatingFunction` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcMutatingFunction::when()` and is
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
/// use qubit_function::{MutatingFunction, ArcMutatingFunction};
///
/// let double = ArcMutatingFunction::new(|x: &mut i32| *x * 2);
/// let identity = ArcMutatingFunction::<i32, i32>::identity();
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
pub struct ArcConditionalMutatingFunction<T, R> {
    pub(super) function: ArcMutatingFunction<T, R>,
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalMutatingFunction<T, R>,
    ArcMutatingFunction,
    MutatingFunction,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalMutatingFunction<T, R>);
