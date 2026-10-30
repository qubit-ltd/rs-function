// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalBiFunction` public type.

use crate::ArcBiFunction;
use crate::ArcBiPredicate;
use crate::BiFunction;
use crate::BiPredicate;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// ArcConditionalBiFunction - Arc-based Conditional BiFunction
// ============================================================================

/// ArcConditionalBiFunction struct
///
/// A conditional bi-function that shares its callback and bi-predicate through
/// `ArcBiFunction` and `ArcBiPredicate`.
///
/// Call `when` to create an intermediate conditional builder, then `or_else`
/// to provide the fallback and obtain an `ArcBiFunction` callable wrapper. The
/// builder is not itself a `BiFunction`. Calling `apply` tests the predicate
/// with `&T` and `&U` before running the selected callback.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Type Parameters
///
/// - `T` - The first input type borrowed by the bi-predicate and callback.
/// - `U` - The second input type borrowed by the bi-predicate and callback.
/// - `R` - The value produced by the selected bi-function.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiFunction, ArcBiFunction};
///
/// let add = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = ArcBiFunction::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(&5, &3), 8);
/// assert_eq!(conditional_clone.apply(&-5, &3), -15);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalBiFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcBiFunction<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiFunction
impl_shared_conditional_function!(
    ArcConditionalBiFunction<T, U, R>,
    ArcBiFunction,
    BiFunction,
    callback_bounds = (Send + Sync + 'static)
);

// Implement Debug and Display for ArcConditionalBiFunction
impl_conditional_function_debug_display!(ArcConditionalBiFunction<T, U, R>);

// Implement Clone for ArcConditionalBiFunction
impl_conditional_function_clone!(ArcConditionalBiFunction<T, U, R>);
