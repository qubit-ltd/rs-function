// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxTransformer` public type.

use {
    crate::BoxConditionalTransformer,
    crate::Predicate,
};
use {
    crate::Transformer,
    crate::transformers::macros::impl_box_transformer_methods,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
};

// ============================================================================
// BoxTransformer - Box<dyn Fn(T) -> R>
// ============================================================================

/// BoxTransformer - transformer wrapper based on `Box<dyn Fn>`
///
/// A transformer wrapper that provides single ownership with reusable
/// transformation. The transformer consumes the input and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxTransformer<T, R> {
    pub(super) function: Box<dyn Fn(T) -> R>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

// Implement BoxTransformer
impl<T, R> BoxTransformer<T, R> {
    impl_transformer_common_methods!(
        BoxTransformer<T, R>,
        (Fn(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxTransformer<T, R>,
        BoxConditionalTransformer,
        Transformer
    );
}

// Implement constant method for BoxTransformer
impl_transformer_constant_method!(BoxTransformer<T, R>);

// Implement Debug and Display for BoxTransformer
impl_transformer_debug_display!(BoxTransformer<T, R>);

// Implement Transformer for BoxTransformer
impl<T, R> Transformer<T, R> for BoxTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }
}
