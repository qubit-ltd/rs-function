/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcConditionalStatefulMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcConditionalStatefulMutatingFunction - Rc-based Conditional Stateful Mutating Function
// ============================================================================

/// RcConditionalStatefulMutatingFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcStatefulMutatingFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalFunction`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction, RcStatefulMutatingFunction};
///
/// let double = RcStatefulMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let identity = RcStatefulMutatingFunction::<i32, i32>::identity();
/// let mut conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let mut conditional_clone = conditional.clone();
///
/// let mut positive = 5;
/// let mut negative = -5;
/// assert_eq!(conditional.apply(&mut positive), 10);
/// assert_eq!(conditional_clone.apply(&mut negative), -5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalStatefulMutatingFunction<T, R> {
    pub(super) function: RcStatefulMutatingFunction<T, R>,
    pub(super) predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalStatefulMutatingFunction<T, R>,
    RcStatefulMutatingFunction,
    StatefulMutatingFunction,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalStatefulMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalStatefulMutatingFunction<T, R>);
