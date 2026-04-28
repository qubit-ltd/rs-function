/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalBiMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcConditionalBiMutatingFunction - Arc-based Conditional BiMutatingFunction
// ============================================================================

/// ArcConditionalBiMutatingFunction struct
///
/// A thread-safe conditional bi-mutating-function that only executes when a
/// bi-predicate is satisfied. Uses `ArcBiMutatingFunction` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcBiMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalBiMutatingFunction<T, U, R> {
    pub(super) function: ArcBiMutatingFunction<T, U, R>,
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiMutatingFunction
impl_shared_conditional_function!(
    ArcConditionalBiMutatingFunction<T, U, R>,
    ArcBiMutatingFunction,
    BiMutatingFunction,
    into_arc,
    Send + Sync + 'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(ArcConditionalBiMutatingFunction<T, U, R>);

// Generate Clone implementation
impl_conditional_function_clone!(ArcConditionalBiMutatingFunction<T, U, R>);
