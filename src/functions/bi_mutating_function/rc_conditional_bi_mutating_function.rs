// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalBiMutatingFunction` public type.

use crate::BiMutatingFunction;
use crate::BiPredicate;
use crate::RcBiMutatingFunction;
use crate::RcBiPredicate;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// RcConditionalBiMutatingFunction - Rc-based Conditional BiMutatingFunction
// ============================================================================

/// A shared conditional callback that may mutate two inputs.
///
/// The stored [`RcBiMutatingFunction`] runs only when the stored
/// [`RcBiPredicate`] accepts both inputs. This wrapper shares its callback and
/// predicate through `Rc`, so it is intended for use within a single thread.
///
/// Create this type with [`RcBiMutatingFunction::when`]. Use [`Self::or_else`]
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
/// use qubit_function::{BiMutatingFunction, RcBiMutatingFunction};
///
/// let add_if_positive = RcBiMutatingFunction::new(
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
pub struct RcConditionalBiMutatingFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcBiMutatingFunction<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalBiMutatingFunction
impl_shared_conditional_function!(
    RcConditionalBiMutatingFunction<T, U, R>,
    RcBiMutatingFunction,
    BiMutatingFunction,
    callback_bounds = ('static)
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(RcConditionalBiMutatingFunction<T, U, R>);

// Generate Clone implementation
impl_conditional_function_clone!(RcConditionalBiMutatingFunction<T, U, R>);
