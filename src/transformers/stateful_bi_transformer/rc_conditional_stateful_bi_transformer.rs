/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcConditionalStatefulBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcConditionalStatefulBiTransformer - Rc-based Conditional StatefulBiTransformer
// ============================================================================

/// RcConditionalStatefulBiTransformer struct
///
/// A single-threaded conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `RcStatefulBiTransformer` and `RcBiPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulBiTransformer`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulBiTransformer, RcStatefulBiTransformer};
///
/// let add = RcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = RcStatefulBiTransformer::new(|x: i32, y: i32| x * y);
/// let mut conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let mut conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalStatefulBiTransformer<T, U, R> {
    pub(super) transformer: RcStatefulBiTransformer<T, U, R>,
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalStatefulBiTransformer
impl_shared_conditional_transformer!(
    RcConditionalStatefulBiTransformer<T, U, R>,
    RcStatefulBiTransformer,
    StatefulBiTransformer,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalStatefulBiTransformer<T, U, R>);

// Implement Clone for RcConditionalStatefulBiTransformer
impl_conditional_transformer_clone!(RcConditionalStatefulBiTransformer<T, U, R>);
