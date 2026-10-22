// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalBiMutatingFunction` public type.

use crate::BiMutatingFunction;
use crate::BiPredicate;
use crate::BoxBiMutatingFunction;
use crate::BoxBiPredicate;
use crate::functions::macros::impl_box_conditional_function;
use crate::functions::macros::impl_conditional_function_debug_display;

// ============================================================================
// BoxConditionalBiMutatingFunction - Box-based Conditional BiMutatingFunction
// ============================================================================

/// A single-owner conditional callback that may mutate two inputs.
///
/// The wrapped [`BoxBiMutatingFunction`] runs only when the [`BoxBiPredicate`]
/// accepts both inputs. The callback and predicate are owned by this wrapper,
/// which cannot be cloned.
///
/// Create this type with [`BoxBiMutatingFunction::when`]. Use [`Self::or_else`]
/// to provide a callback for the predicate-false branch.
///
/// # Type Parameters
///
/// - `T` - Type of the first mutable input.
/// - `U` - Type of the second mutable input.
/// - `R` - Type of the callback result.
///
/// # Examples
///
/// ```
/// use qubit_function::{BiMutatingFunction, BoxBiMutatingFunction};
///
/// let add_if_positive = BoxBiMutatingFunction::new(
///     |first: &mut i32, second: &mut i32| {
///         *first += *second;
///         *first
///     },
/// )
/// .when(|first: &i32, second: &i32| *first > 0 && *second > 0);
/// let callback = add_if_positive.or_else(|_: &mut i32, _: &mut i32| 0);
/// let mut first = 2;
/// let mut second = 3;
///
/// assert_eq!(callback.apply(&mut first, &mut second), 5);
/// assert_eq!(first, 5);
/// assert_eq!(second, 3);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalBiMutatingFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxBiMutatingFunction<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiMutatingFunction
impl_box_conditional_function!(
    BoxConditionalBiMutatingFunction<T, U, R>,
    BoxBiMutatingFunction,
    BiMutatingFunction
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiMutatingFunction<T, U, R>);
