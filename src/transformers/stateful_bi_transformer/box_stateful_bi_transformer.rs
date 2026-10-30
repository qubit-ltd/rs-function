// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxStatefulBiTransformer` public type.

use crate::BiPredicate;
use crate::BoxConditionalStatefulBiTransformer;
use crate::StatefulBiTransformer;
use crate::StatefulTransformer;
use crate::transformers::macros::impl_box_transformer_methods;
use crate::transformers::macros::impl_transformer_common_methods;
use crate::transformers::macros::impl_transformer_constant_method;
use crate::transformers::macros::impl_transformer_debug_display;

// ============================================================================
// BoxStatefulBiTransformer - Box<dyn FnMut(T, U) -> R>
// ============================================================================

/// Owns a reusable two-input callback whose internal state can change between
/// calls.
///
/// The callback is stored in a single-owner `Box` and must be `'static`. Each
/// call consumes both inputs and mutably borrows the wrapper. The wrapper does
/// not require the callback to be `Send` or `Sync` and cannot be cloned.
///
/// # Type Parameters
///
/// * `T` - Type of the first input consumed by the callback.
/// * `U` - Type of the second input consumed by the callback.
/// * `R` - Type of the value returned by the callback.
///
/// # Examples
///
/// The callback can retain mutable state between calls:
///
/// ```
/// use qubit_function::BoxStatefulBiTransformer;
/// use qubit_function::StatefulBiTransformer;
///
/// let mut total = 0;
/// let mut sum_and_accumulate = BoxStatefulBiTransformer::new(
///     move |first: i32, second: i32| {
///         total += first + second;
///         total
///     },
/// );
///
/// assert_eq!(sum_and_accumulate.apply(2, 3), 5);
/// assert_eq!(sum_and_accumulate.apply(1, 4), 10);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxStatefulBiTransformer<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnMut(T, U) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U, R> BoxStatefulBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        BoxStatefulBiTransformer<T, U, R>,
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxStatefulBiTransformer<T, U, R>,
        BoxConditionalStatefulBiTransformer,
        StatefulTransformer
    );
}

// Implement constant method for BoxStatefulBiTransformer
impl_transformer_constant_method!(stateful BoxStatefulBiTransformer<T, U, R>);

// Implement Debug and Display for BoxStatefulBiTransformer
impl_transformer_debug_display!(BoxStatefulBiTransformer<T, U, R>);

impl<T, U, R> StatefulBiTransformer<T, U, R> for BoxStatefulBiTransformer<T, U, R> {
    #[inline(always)]
    fn apply(&mut self, first: T, second: U) -> R {
        (self.function)(first, second)
    }
}
