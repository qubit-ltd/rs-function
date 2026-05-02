/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `BoxConditionalTransformerOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalTransformerOnce - Box-based Conditional Transformer
// ============================================================================

/// BoxConditionalTransformerOnce struct
///
/// A conditional consuming transformer that only executes when a predicate is
/// satisfied. Uses `BoxTransformerOnce` and `BoxPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxTransformerOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{TransformerOnce, BoxTransformerOnce};
///
/// let double = BoxTransformerOnce::new(|x: i32| x * 2);
/// let negate = BoxTransformerOnce::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// assert_eq!(conditional.apply(5), 10); // when branch executed
///
/// let double2 = BoxTransformerOnce::new(|x: i32| x * 2);
/// let negate2 = BoxTransformerOnce::new(|x: i32| -x);
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// assert_eq!(conditional2.apply(-5), 5); // or_else branch executed
/// ```
///
pub struct BoxConditionalTransformerOnce<T, R> {
    pub(super) transformer: BoxTransformerOnce<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Implement BoxConditionalTransformerOnce
impl_box_conditional_transformer!(
    BoxConditionalTransformerOnce<T, R>,
    BoxTransformerOnce,
    TransformerOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalTransformerOnce<T, R>);
