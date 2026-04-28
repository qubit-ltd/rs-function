/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcConditionalBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcConditionalBiTransformer - Rc-based Conditional BiTransformer
// ============================================================================

/// RcConditionalBiTransformer struct
///
/// A single-threaded conditional bi-transformer that only executes when a
/// bi-predicate is satisfied. Uses `RcBiTransformer` and `RcBiPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcBiTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when bi-predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalBiTransformer`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiTransformer, RcBiTransformer};
///
/// let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = RcBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalBiTransformer<T, U, R> {
    pub(super) transformer: RcBiTransformer<T, U, R>,
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalBiTransformer
impl_shared_conditional_transformer!(
    RcConditionalBiTransformer<T, U, R>,
    RcBiTransformer,
    BiTransformer,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalBiTransformer<T, U, R>);

// Implement Clone for RcConditionalBiTransformer
impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, R>);
