// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalBiMutatingFunctionOnce` public type.

use crate::BiMutatingFunctionOnce;
use crate::BiPredicate;
use crate::BoxBiMutatingFunctionOnce;
use crate::BoxBiPredicate;
use crate::functions::macros::impl_box_conditional_function;
use crate::functions::macros::impl_conditional_function_debug_display;

// ============================================================================
// BoxConditionalBiMutatingFunctionOnce - Box-based Conditional
// BiMutatingFunction
// ============================================================================

/// BoxConditionalBiMutatingFunctionOnce struct
///
/// A conditional consuming bi-mutating-function that only executes when a
/// bi-predicate is satisfied. Uses `BoxBiMutatingFunctionOnce` and
/// `BoxBiPredicate` for single ownership semantics.
///
/// This type is typically created by calling
/// `BoxBiMutatingFunctionOnce::when()` and is designed to work with the
/// `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalBiMutatingFunctionOnce<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxBiMutatingFunctionOnce<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiMutatingFunctionOnce
impl_box_conditional_function!(
    BoxConditionalBiMutatingFunctionOnce<T, U, R>,
    BoxBiMutatingFunctionOnce,
    BiMutatingFunctionOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiMutatingFunctionOnce<T, U, R>);
