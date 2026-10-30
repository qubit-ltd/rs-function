// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalStatefulTransformer` public type.

use crate::Predicate;
use crate::RcPredicate;
use crate::RcStatefulTransformer;
use crate::StatefulTransformer;
use crate::transformers::macros::impl_conditional_transformer_clone;
use crate::transformers::macros::impl_conditional_transformer_debug_display;

// ============================================================================
// RcConditionalStatefulTransformer - Rc-based Conditional StatefulTransformer
// ============================================================================

/// A shared, single-threaded transformer that selects a callback by predicate.
///
/// A single-threaded conditional transformer that only executes when a
/// predicate is satisfied. Uses `RcStatefulTransformer` and `RcPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulTransformer::when()`
/// and is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Shared callback storage**: Uses `Rc` and `RefCell` for shared
///   callback ownership and runtime borrow checking.
///
/// # Type Parameters
///
/// * `T` - The input consumed by the selected transformer and borrowed by the
///   predicate.
/// * `R` - The output produced by the selected transformer.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulTransformer, RcStatefulTransformer};
///
/// let mut transformer = RcStatefulTransformer::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut transformer_clone = transformer.clone();
///
/// assert_eq!(transformer.apply(5), 10);
/// assert_eq!(transformer_clone.apply(-5), 5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalStatefulTransformer<T, R> {
    /// The wrapped transformer callback.
    pub(super) transformer: RcStatefulTransformer<T, R>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

impl<T, R> RcConditionalStatefulTransformer<T, R> {
    /// Adds an else branch to the conditional transformation.
    ///
    /// The original transformer runs when the predicate matches; otherwise,
    /// the supplied stateful transformer handles the input.
    ///
    /// # Parameters
    ///
    /// * `else_transformer` - The stateful transformer used when the predicate
    ///   does not match.
    ///
    /// # Returns
    ///
    /// A shared stateful transformer that applies the selected branch.
    pub fn or_else<F>(&self, else_transformer: F) -> RcStatefulTransformer<T, R>
    where
        T: 'static,
        R: 'static,
        F: StatefulTransformer<T, R> + 'static,
    {
        let predicate = self.predicate.clone();
        let mut then_transformer = self.transformer.clone();
        let mut else_transformer = else_transformer;
        RcStatefulTransformer::new(move |input| {
            if predicate.test(&input) {
                then_transformer.apply(input)
            } else {
                else_transformer.apply(input)
            }
        })
    }
}

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(RcConditionalStatefulTransformer<T, R>);

// Implement Clone for RcConditionalStatefulTransformer
impl_conditional_transformer_clone!(RcConditionalStatefulTransformer<T, R>);
