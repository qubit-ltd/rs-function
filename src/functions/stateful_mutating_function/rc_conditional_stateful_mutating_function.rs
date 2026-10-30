// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalStatefulMutatingFunction` public type.

use crate::Predicate;
use crate::RcPredicate;
use crate::RcStatefulMutatingFunction;
use crate::StatefulMutatingFunction;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// RcConditionalStatefulMutatingFunction - Rc-based Conditional Stateful
// Mutating Function
// ============================================================================

/// RcConditionalStatefulMutatingFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcStatefulMutatingFunction` and `RcPredicate`
/// for shared ownership within a single thread.
///
/// This type is typically created by calling
/// `RcStatefulMutatingFunction::when()` and is designed to work with the
/// `or_else()` method to create if-then-else logic.
///
/// The predicate borrows the input through `&T`. After it returns, the
/// selected callback receives `&mut T`. The value returned by `when` is an
/// intermediate builder; call `or_else` to obtain a callable wrapper.
///
/// # Type Parameters
///
/// * `T` - The input type borrowed by the predicate and mutably borrowed by the
///   selected callback.
/// * `R` - The result type returned by the wrapped callback.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Callback sharing**: Uses `Rc` and checks mutable callback borrows with
///   `RefCell`; it does not use a mutex.
///
/// # Examples
///
/// ```rust
/// use qubit_function::RcStatefulMutatingFunction;
/// use qubit_function::StatefulMutatingFunction;
///
/// let double = RcStatefulMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let identity = RcStatefulMutatingFunction::<i32, i32>::identity();
/// let mut conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let mut conditional_clone = conditional.clone();
///
/// let mut positive = 5;
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut positive), 10);
/// assert_eq!(conditional_clone.apply(&mut negative), -5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalStatefulMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcStatefulMutatingFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalStatefulMutatingFunction<T, R>,
    RcStatefulMutatingFunction,
    StatefulMutatingFunction,
    callback_bounds = ('static)
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalStatefulMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalStatefulMutatingFunction<T, R>);
