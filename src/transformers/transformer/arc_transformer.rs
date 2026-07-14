// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcTransformer` public type.

use {
    crate::ArcConditionalTransformer,
    crate::Predicate,
};
use {
    crate::Transformer,
    crate::macros::impl_closure_trait,
    crate::transformers::macros::impl_shared_transformer_methods,
    crate::transformers::macros::impl_transformer_clone,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
    std::sync::Arc,
};

// ============================================================================
// ArcTransformer - Arc<dyn Fn(T) -> R + Send + Sync>
// ============================================================================

/// ArcTransformer - thread-safe transformer wrapper
///
/// A thread-safe, clonable transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
pub struct ArcTransformer<T, R> {
    pub(super) function: Arc<dyn Fn(T) -> R + Send + Sync>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

// Implement ArcTransformer
impl<T, R> ArcTransformer<T, R> {
    impl_transformer_common_methods!(
        ArcTransformer<T, R>,
        (Fn(T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    impl_shared_transformer_methods!(
        ArcTransformer<T, R>,
        ArcConditionalTransformer,
        ArcPredicate,
        Transformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcTransformer
impl_transformer_constant_method!(thread_safe ArcTransformer<T, R>);

// Implement Debug and Display for ArcTransformer
impl_transformer_debug_display!(ArcTransformer<T, R>);

// Implement Clone for ArcTransformer
impl_transformer_clone!(ArcTransformer<T, R>);

// Implement Transformer for ArcTransformer
impl<T, R> Transformer<T, R> for ArcTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement Transformer<T, R> for any type that implements Fn(T) -> R
impl_closure_trait!(
    Transformer<T, R>,
    apply,
    BoxTransformerOnce,
    Fn(input: T) -> R
);
