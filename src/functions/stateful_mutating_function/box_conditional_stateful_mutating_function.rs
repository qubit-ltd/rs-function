/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxConditionalStatefulMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxConditionalStatefulMutatingFunction - Box-based Conditional Stateful Mutating Function
// ============================================================================

/// BoxConditionalStatefulMutatingFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxStatefulMutatingFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Function**: Can be used anywhere a `Function` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction, BoxStatefulMutatingFunction};
///
/// let double = BoxStatefulMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let negate = BoxStatefulMutatingFunction::new(|x: &mut i32| {
///     *x = -*x;
///     *x
/// });
/// let mut conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// let mut positive = 5;
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut positive), 10); // when branch executed
/// assert_eq!(conditional.apply(&mut negative), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalStatefulMutatingFunction<T, R> {
    pub(super) function: BoxStatefulMutatingFunction<T, R>,
    pub(super) predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalStatefulMutatingFunction<T, R>,
    BoxStatefulMutatingFunction,
    StatefulMutatingFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalStatefulMutatingFunction<T, R>);
