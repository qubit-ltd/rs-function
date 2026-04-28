/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxConditionalBiMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalBiMutatingFunction - Box-based Conditional BiMutatingFunction
// ============================================================================

/// BoxConditionalBiMutatingFunction struct
///
/// A conditional bi-mutating-function that only executes when a bi-predicate is
/// satisfied. Uses `BoxBiMutatingFunction` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiMutatingFunction**: Can be used anywhere a `BiMutatingFunction` is expected
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiMutatingFunction<T, U, R> {
    pub(super) function: BoxBiMutatingFunction<T, U, R>,
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
