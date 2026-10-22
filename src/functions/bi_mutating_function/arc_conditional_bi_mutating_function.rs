// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalBiMutatingFunction` public type.

use crate::ArcBiMutatingFunction;
use crate::ArcBiPredicate;
use crate::BiMutatingFunction;
use crate::BiPredicate;
use crate::functions::macros::impl_conditional_function_clone;
use crate::functions::macros::impl_conditional_function_debug_display;
use crate::functions::macros::impl_shared_conditional_function;

// ============================================================================
// ArcConditionalBiMutatingFunction - Arc-based Conditional BiMutatingFunction
// ============================================================================

/// A shared conditional callback that may mutate two inputs.
///
/// The stored [`ArcBiMutatingFunction`] runs only when the stored
/// [`ArcBiPredicate`] accepts both inputs. This wrapper shares its callback and
/// predicate through `Arc`, so it can be cloned and used across threads when
/// its generic types satisfy the required thread-safety bounds.
///
/// Create this type with [`ArcBiMutatingFunction::when`]. Use [`Self::or_else`]
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
/// use qubit_function::{ArcBiMutatingFunction, BiMutatingFunction};
///
/// let add_if_positive = ArcBiMutatingFunction::new(
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
pub struct ArcConditionalBiMutatingFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcBiMutatingFunction<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiMutatingFunction
impl_shared_conditional_function!(
    ArcConditionalBiMutatingFunction<T, U, R>,
    ArcBiMutatingFunction,
    BiMutatingFunction,
    callback_bounds = (Send + Sync + 'static)
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(ArcConditionalBiMutatingFunction<T, U, R>);

// Generate Clone implementation
impl_conditional_function_clone!(ArcConditionalBiMutatingFunction<T, U, R>);
