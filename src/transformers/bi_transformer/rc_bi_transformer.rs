// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcBiTransformer` public type.

use {
    crate::BiPredicate,
    crate::RcConditionalBiTransformer,
    crate::Transformer,
};
use {
    crate::BiTransformer,
    crate::transformers::macros::impl_shared_transformer_methods,
    crate::transformers::macros::impl_transformer_clone,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
    std::rc::Rc,
};

// ============================================================================
// RcBiTransformer - Rc<dyn Fn(T, U) -> R>
// ============================================================================

/// RcBiTransformer - single-threaded bi-transformer wrapper
///
/// A single-threaded, clonable bi-transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(T, U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcBiTransformer<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: Rc<dyn Fn(T, U) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U, R> RcBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        RcBiTransformer<T, U, R>,
        (Fn(T, U) -> R + 'static),
        |f| Rc::new(f)
    );

    impl_shared_transformer_methods!(
        RcBiTransformer<T, U, R>,
        RcConditionalBiTransformer,
        RcBiPredicate,
        Transformer,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Implement constant method for RcBiTransformer
impl_transformer_constant_method!(RcBiTransformer<T, U, R>);

// Implement Debug and Display for RcBiTransformer
impl_transformer_debug_display!(RcBiTransformer<T, U, R>);

// Implement Clone for RcBiTransformer
impl_transformer_clone!(RcBiTransformer<T, U, R>);

// Implement BiTransformer trait for RcBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for RcBiTransformer<T, U, R> {
    #[inline(always)]
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }
}
