// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalMutatingFunction` public type.

use crate::ArcMutatingFunction;
use crate::ArcPredicate;
use crate::MutatingFunction;
use crate::Predicate;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// ArcConditionalMutatingFunction - Arc-based Conditional Mutating Function
// ============================================================================

/// A thread-safe shared wrapper that conditionally mutates an input and returns
/// a result.
///
/// The predicate is evaluated against the input before the wrapped callback is
/// invoked. If it returns `false`, the alternative callback supplied to
/// [`or_else`](Self::or_else) is used instead. The callbacks and predicate use
/// `Arc` ownership, which permits sharing when their input and output types
/// satisfy the required thread-safety bounds; callback invocation itself is
/// not protected by a lock.
///
/// This type is typically created by calling `ArcMutatingFunction::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe Sharing**: Uses `Arc` for shared callback and predicate
///   ownership
/// - **Conditional Execution**: Mutates the input through the selected callback
/// - **Shared Receiver**: Calls through `&self`; callback side effects still
///   depend on the callback implementation
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatingFunction, ArcMutatingFunction};
///
/// let double = ArcMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
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
/// # Type Parameters
///
/// * `T` - The input value type, mutably borrowed by the predicate and
///   callback.
/// * `R` - The result type returned by whichever callback is selected.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcMutatingFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalMutatingFunction<T, R>,
    ArcMutatingFunction,
    MutatingFunction,
    callback_bounds = (Send + Sync + 'static)
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalMutatingFunction<T, R>);
