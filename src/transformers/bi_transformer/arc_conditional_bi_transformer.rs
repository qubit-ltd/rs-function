// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalBiTransformer` public type.

use crate::ArcBiPredicate;
use crate::ArcBiTransformer;
use crate::BiPredicate;
use crate::BiTransformer;
use crate::transformers::macros::impl_conditional_transformer_clone;
use crate::transformers::macros::impl_conditional_transformer_debug_display;
use crate::transformers::macros::impl_shared_conditional_transformer;

// ============================================================================
// ArcConditionalBiTransformer - Arc-based Conditional BiTransformer
// ============================================================================

/// An intermediate builder holding an `ArcBiTransformer` and its
/// `ArcBiPredicate`.
///
/// Created by `ArcBiTransformer::when()`. This builder does not implement
/// `BiTransformer` and cannot be invoked directly. Call `or_else()` to
/// provide the fallback and obtain an `ArcBiTransformer`. Calling `apply()`
/// on that returned transformer evaluates the predicate and invokes the
/// selected branch.
/// The transformer and predicate use shared ownership across threads.
///
/// # Type Parameters
///
/// - `T` - The type of the first input consumed by the wrapped transformer.
/// - `U` - The type of the second input consumed by the wrapped transformer.
/// - `R` - The output type produced by the wrapped transformer.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Branch Selection**: The completed wrapper invokes one selected branch
/// - **Completion**: Call `or_else()` to obtain an invocable wrapper
///
/// # Examples
///
/// ```rust
/// use qubit_function::BiTransformer;
/// use qubit_function::ArcBiTransformer;
///
/// let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
/// let multiply = ArcBiTransformer::new(|x: i32, y: i32| x * y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5, 3), 8);
/// assert_eq!(conditional_clone.apply(-5, 3), -15);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalBiTransformer<T, U, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: ArcBiTransformer<T, U, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiTransformer
impl_shared_conditional_transformer!(
    ArcConditionalBiTransformer<T, U, R>,
    ArcBiTransformer,
    BiTransformer,
    callback_bounds = (Send + Sync + 'static)
);

// Implement Debug and Display for ArcConditionalBiTransformer
impl_conditional_transformer_debug_display!(ArcConditionalBiTransformer<T, U, R>);

// Implement Clone for ArcConditionalBiTransformer
impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, R>);
