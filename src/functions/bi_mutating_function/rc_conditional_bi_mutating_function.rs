// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalBiMutatingFunction` public type.

use {
    crate::BiMutatingFunction,
    crate::BiPredicate,
    crate::RcBiMutatingFunction,
    crate::RcBiPredicate,
    crate::functions::macros::impl_conditional_function_clone,
    crate::functions::macros::impl_conditional_function_debug_display,
    crate::functions::macros::impl_shared_conditional_function,
};

// ============================================================================
// RcConditionalBiMutatingFunction - Rc-based Conditional BiMutatingFunction
// ============================================================================

/// RcConditionalBiMutatingFunction struct
///
/// A single-threaded conditional bi-mutating-function that only executes when a
/// bi-predicate is satisfied. Uses `RcBiMutatingFunction` and `RcBiPredicate`
/// for shared ownership within a single thread.
///
/// This type is typically created by calling `RcBiMutatingFunction::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **No Lock Overhead**: More efficient than
///   `ArcConditionalBiMutatingFunction`
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
