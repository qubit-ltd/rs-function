/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcConditionalStatefulFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcConditionalStatefulFunction - Arc-based Conditional StatefulFunction
// ============================================================================

/// ArcConditionalStatefulFunction struct
///
/// A thread-safe conditional function that only executes when a predicate
/// is satisfied. Uses `ArcStatefulFunction` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send`, safe for concurrent use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulFunction, ArcStatefulFunction};
///
/// let mut function = ArcStatefulFunction::new(|x: &i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: &i32| -x);
///
/// let mut function_clone = function.clone();
///
/// assert_eq!(function.apply(&5), 10);
/// assert_eq!(function_clone.apply(&-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulFunction<T, R> {
    pub(super) function: ArcStatefulFunction<T, R>,
    pub(super) predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalStatefulFunction<T, R>,
    ArcStatefulFunction,
    StatefulFunction,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalStatefulFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalStatefulFunction<T, R>);
