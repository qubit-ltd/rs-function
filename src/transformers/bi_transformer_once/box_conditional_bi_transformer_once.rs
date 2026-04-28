/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxConditionalBiTransformerOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalBiTransformerOnce - Box-based Conditional BiTransformer
// ============================================================================

/// BoxConditionalBiTransformerOnce struct
///
/// A conditional consuming bi-transformer that only executes when a bi-predicate
/// is satisfied. Uses `BoxBiTransformerOnce` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiTransformerOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiTransformerOnce, BoxBiTransformerOnce};
///
/// let add = BoxBiTransformerOnce::new(|x: i32, y: i32| x + y);
/// let multiply = BoxBiTransformerOnce::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// assert_eq!(conditional.apply(5, 3), 8); // when branch executed
///
/// let add2 = BoxBiTransformerOnce::new(|x: i32, y: i32| x + y);
/// let multiply2 = BoxBiTransformerOnce::new(|x: i32, y: i32| x * y);
/// let conditional2 = add2.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply2);
/// assert_eq!(conditional2.apply(-5, 3), -15); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiTransformerOnce<T, U, R> {
    pub(super) transformer: BoxBiTransformerOnce<T, U, R>,
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalTransformerOnce
impl_box_conditional_transformer!(
    BoxConditionalBiTransformerOnce<T, U, R>,
    BoxBiTransformerOnce,
    BiTransformerOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalBiTransformerOnce<T, U, R>);
