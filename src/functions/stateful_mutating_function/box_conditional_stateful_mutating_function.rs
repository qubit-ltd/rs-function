// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalStatefulMutatingFunction` public type.

use crate::BoxPredicate;
use crate::BoxStatefulMutatingFunction;
use crate::Predicate;
use crate::StatefulMutatingFunction;
use crate::functions::macros::impl_box_conditional_function;
use crate::functions::macros::impl_conditional_function_debug_display;

// ============================================================================
// BoxConditionalStatefulMutatingFunction - Box-based Conditional Stateful
// Mutating Function
// ============================================================================

/// BoxConditionalStatefulMutatingFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxStatefulMutatingFunction` and `BoxPredicate` for single ownership
/// semantics.
///
/// This type is typically created by calling
/// `BoxStatefulMutatingFunction::when()` and is designed to work with the
/// `or_else()` method to create if-then-else logic.
///
/// # Type Parameters
///
/// * `T` - The input type borrowed by the predicate and mutably borrowed by the
///   wrapped callbacks.
/// * `R` - The result type returned by the wrapped callback.
///
/// # Features
///
/// - **Owned builder state**: Holds the selected callback and predicate; the
///   builder is consumed by `or_else` to produce a
///   `BoxStatefulMutatingFunction`
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// The value returned by `when` is an intermediate builder and does not
/// implement `StatefulMutatingFunction`. Its predicate receives shared input;
/// the selected callback receives mutable input. Call `or_else` to provide
/// the fallback and obtain a callable wrapper. Invoking that wrapper follows
/// the `StatefulMutatingFunction` receiver contract (`&mut self`).
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction, BoxStatefulMutatingFunction};
///
/// let double = BoxStatefulMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let negate = BoxStatefulMutatingFunction::new(|x: &mut i32| {
///     *x = -*x;
///     *x
/// });
/// let mut conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// let mut positive = 5;
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut positive), 10); // when branch executed
/// assert_eq!(conditional.apply(&mut negative), 5); // or_else branch executed
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalStatefulMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxStatefulMutatingFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalStatefulMutatingFunction<T, R>,
    BoxStatefulMutatingFunction,
    StatefulMutatingFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalStatefulMutatingFunction<T, R>);
