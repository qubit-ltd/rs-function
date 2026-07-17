// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcTransformer` public type.

use {
    crate::Predicate,
    crate::RcConditionalTransformer,
};
use {
    crate::Transformer,
    crate::transformers::macros::impl_shared_transformer_methods,
    crate::transformers::macros::impl_transformer_clone,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
    std::rc::Rc,
};

// ============================================================================
// RcTransformer - Rc<dyn Fn(T) -> R>
// ============================================================================

/// RcTransformer - single-threaded transformer wrapper
///
/// A single-threaded, clonable transformer wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(T) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcTransformer<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Rc<dyn Fn(T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement RcTransformer
impl<T, R> RcTransformer<T, R> {
    impl_transformer_common_methods!(
        RcTransformer<T, R>,
        (Fn(T) -> R + 'static),
        |f| Rc::new(f)
    );

    impl_shared_transformer_methods!(
        RcTransformer<T, R>,
        RcConditionalTransformer,
        RcPredicate,
        Transformer,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

impl_transformer_constant_method!(RcTransformer<T, R>);

// Implement Debug and Display for RcTransformer
impl_transformer_debug_display!(RcTransformer<T, R>);

// Implement Clone for RcTransformer
impl_transformer_clone!(RcTransformer<T, R>);

// Implement Transformer for RcTransformer
impl<T, R> Transformer<T, R> for RcTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }
}
