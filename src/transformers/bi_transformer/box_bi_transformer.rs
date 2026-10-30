// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiTransformer` public type.

use crate::BiPredicate;
use crate::BiTransformer;
use crate::BoxConditionalBiTransformer;
use crate::Transformer;
use crate::transformers::macros::impl_box_transformer_methods;
use crate::transformers::macros::impl_transformer_common_methods;
use crate::transformers::macros::impl_transformer_debug_display;

// Implements the constant constructor.
mod constant;

// ============================================================================
// BoxBiTransformer - Box<dyn Fn(T, U) -> R>
// ============================================================================

/// BoxBiTransformer - bi-transformer wrapper based on `Box<dyn Fn>`
///
/// A bi-transformer wrapper that provides single ownership with reusable
/// transformation. The bi-transformer consumes both inputs and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(T, U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Type Parameters
///
/// - `T` - The type of the first input consumed by the callback.
/// - `U` - The type of the second input consumed by the callback.
/// - `R` - The callback's output type.
///
/// # Examples
///
/// ```
/// use qubit_function::BiTransformer;
/// use qubit_function::BoxBiTransformer;
///
/// let add = BoxBiTransformer::new(|first: i32, second: i32| first + second);
/// assert_eq!(add.apply(2, 3), 5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiTransformer<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn Fn(T, U) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement BoxBiTransformer
impl<T, U, R> BoxBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        BoxBiTransformer<T, U, R>,
        (Fn(T, U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxBiTransformer<T, U, R>,
        BoxConditionalBiTransformer,
        Transformer
    );
}

// Implement Debug and Display for BoxBiTransformer
impl_transformer_debug_display!(BoxBiTransformer<T, U, R>);

// Implement BiTransformer trait for BoxBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for BoxBiTransformer<T, U, R> {
    #[inline(always)]
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }
}
