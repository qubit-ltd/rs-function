// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `ArcBiTransformer` public type.

use super::{
    Arc, ArcConditionalBiTransformer, BiPredicate, BiTransformer, Transformer,
    impl_shared_transformer_methods, impl_transformer_clone, impl_transformer_common_methods,
    impl_transformer_constant_method, impl_transformer_debug_display,
};

// ============================================================================
// ArcBiTransformer - Arc<dyn Fn(T, U) -> R + Send + Sync>
// ============================================================================

/// ArcBiTransformer - thread-safe bi-transformer wrapper
///
/// A thread-safe, clonable bi-transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(T, U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
pub struct ArcBiTransformer<T, U, R> {
    pub(super) function: Arc<dyn Fn(T, U) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T, U, R> ArcBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        ArcBiTransformer<T, U, R>,
        (Fn(T, U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    impl_shared_transformer_methods!(
        ArcBiTransformer<T, U, R>,
        ArcConditionalBiTransformer,
        ArcBiPredicate,
        Transformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcBiTransformer
impl_transformer_constant_method!(thread_safe ArcBiTransformer<T, U, R>);

// Implement Debug and Display for ArcBiTransformer
impl_transformer_debug_display!(ArcBiTransformer<T, U, R>);

// Implement Clone for ArcBiTransformer
impl_transformer_clone!(ArcBiTransformer<T, U, R>);

// Implement BiTransformer trait for ArcBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for ArcBiTransformer<T, U, R> {
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }
}
