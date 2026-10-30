// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxConditionalTransformer` public type.

use crate::BoxPredicate;
use crate::BoxTransformer;
use crate::Predicate;
use crate::Transformer;
use crate::transformers::macros::impl_box_conditional_transformer;
use crate::transformers::macros::impl_conditional_transformer_debug_display;

// ============================================================================
// BoxConditionalTransformer - Box-based Conditional Transformer
// ============================================================================

/// BoxConditionalTransformer struct
///
/// A conditional transformer that only executes when a predicate is satisfied.
/// Uses `BoxTransformer` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxTransformer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Owned builder state**: Holds the selected callback and predicate; the
///   builder is consumed by `or_else` to produce a `BoxTransformer`
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// The value returned by `when` is an intermediate builder and does not
/// implement `Transformer`. Call `or_else` to provide the fallback and obtain
/// a callable wrapper. Invoking that wrapper follows the `Transformer`
/// receiver contract (`&self`).
///
/// # Type Parameters
///
/// * `T` - The input type consumed by the wrapped transformer and inspected by
///   the predicate.
/// * `R` - The output type produced by the wrapped transformer.
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{Transformer, BoxTransformer};
///
/// let double = BoxTransformer::new(|x: i32| x * 2);
/// let negate = BoxTransformer::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(5), 10); // when branch executed
/// assert_eq!(conditional.apply(-5), 5); // or_else branch executed
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxConditionalTransformer<T, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: BoxTransformer<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: BoxPredicate<T>,
}

// Implement BoxConditionalTransformer
impl_box_conditional_transformer!(
    BoxConditionalTransformer<T, R>,
    BoxTransformer,
    Transformer
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalTransformer<T, R>);
