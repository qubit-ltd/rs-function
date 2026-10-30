// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalMutatingFunction` public type.

use crate::BoxMutatingFunction;
use crate::BoxPredicate;
use crate::MutatingFunction;
use crate::Predicate;
use crate::functions::macros::impl_box_conditional_function;
use crate::functions::macros::impl_conditional_function_debug_display;

// ============================================================================
// BoxConditionalMutatingFunction - Box-based Conditional Mutating Function
// ============================================================================

/// A single-owner wrapper that applies a mutating callback when its predicate
/// accepts the input.
///
/// The predicate and callback receive the same mutable input. The callback's
/// result is returned to the caller, and [`or_else`](Self::or_else) can add an
/// alternative callback for inputs the predicate rejects. The wrapper owns its
/// callback and predicate through `BoxMutatingFunction` and `BoxPredicate`.
///
/// This type is typically created by calling `BoxMutatingFunction::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Owned builder state**: Holds the selected callback and predicate; the
///   builder is consumed by `or_else` to produce a `BoxMutatingFunction`
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Mutating Callback**: Operates on a mutable input and returns the
///   callback's result
///
/// The value returned by `when` is an intermediate builder and does not
/// implement `MutatingFunction`. Its predicate receives `&T`, while the
/// selected callback receives `&mut T`. Call `or_else` to provide the fallback
/// and obtain a callable wrapper. Invoking that wrapper follows the
/// `MutatingFunction` receiver contract (`&self`).
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{MutatingFunction, BoxMutatingFunction};
///
/// let double = BoxMutatingFunction::new(|x: &mut i32| *x * 2);
/// let negate = BoxMutatingFunction::new(|x: &mut i32| -*x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// let mut positive = 5;
/// assert_eq!(conditional.apply(&mut positive), 10); // when branch executed
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut negative), 5); // or_else branch executed
/// ```
///
/// # Type Parameters
///
/// * `T` - The input type mutably borrowed by the predicate and callbacks.
/// * `R` - The result type returned by the selected callback.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxMutatingFunction<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalMutatingFunction<T, R>,
    BoxMutatingFunction,
    MutatingFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalMutatingFunction<T, R>);
