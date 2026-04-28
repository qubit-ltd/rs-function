/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxConditionalBiFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalBiFunction - Box-based Conditional BiFunction
// ============================================================================

/// BoxConditionalBiFunction struct
///
/// A conditional bi-function that only executes when a bi-predicate is
/// satisfied. Uses `BoxBiFunction` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiFunction**: Can be used anywhere a `BiFunction` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiFunction, BoxBiFunction};
///
/// let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = BoxBiFunction::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(&5, &3), 8);  // when branch executed
/// assert_eq!(conditional.apply(&-5, &3), -15); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiFunction<T, U, R> {
    pub(super) function: BoxBiFunction<T, U, R>,
    pub(super) predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiFunction
impl_box_conditional_function!(
    BoxConditionalBiFunction<T, U, R>,
    BoxBiFunction,
    BiFunction
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiFunction<T, U, R>);
