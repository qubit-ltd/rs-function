// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalFunctionOnce` public type.

use crate::BoxFunctionOnce;
use crate::BoxPredicate;
use crate::FunctionOnce;
use crate::Predicate;
use crate::functions::macros::impl_box_conditional_function;
use crate::functions::macros::impl_conditional_function_debug_display;

// ============================================================================
// BoxConditionalFunctionOnce - Box-based Conditional Function
// ============================================================================

/// A boxed, one-shot conditional transformer that owns its predicate and
/// callback and consumes them when applied.
///
/// The wrapped callback runs when the predicate matches. `when` returns this
/// intermediate object; call [`Self::or_else`] to provide a second one-shot
/// callback and obtain a callable wrapper for inputs that do not match. This
/// type is typically created with [`BoxFunctionOnce::when`].
///
/// # Features
///
/// - **Single Ownership**: Not cloneable; applying it consumes the callback
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Chooses a callback based on the predicate
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Type Parameters
///
/// - `T`: The input type borrowed by the predicate and callback.
/// - `R`: The output type produced by the selected callback.
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{FunctionOnce, BoxFunctionOnce};
///
/// let double = BoxFunctionOnce::new(|x: &i32| x * 2);
/// let negate = BoxFunctionOnce::new(|x: &i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// assert_eq!(conditional.apply(&5), 10); // when branch executed
///
/// let double2 = BoxFunctionOnce::new(|x: &i32| x * 2);
/// let negate2 = BoxFunctionOnce::new(|x: &i32| -x);
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// assert_eq!(conditional2.apply(&-5), 5); // or_else branch executed
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalFunctionOnce<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxFunctionOnce<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalFunctionOnce<T, R>,
    BoxFunctionOnce,
    FunctionOnce
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalFunctionOnce<T, R>);
