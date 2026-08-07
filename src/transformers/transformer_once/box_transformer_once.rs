// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxTransformerOnce` public type.

use crate::BoxConditionalTransformerOnce;
use crate::Predicate;
use crate::TransformerOnce;
use crate::macros::impl_closure_once_trait;
use crate::transformers::macros::impl_box_transformer_methods;
use crate::transformers::macros::impl_transformer_common_methods;
use crate::transformers::macros::impl_transformer_constant_method;
use crate::transformers::macros::impl_transformer_debug_display;

// ============================================================================
// BoxTransformerOnce - Box<dyn FnOnce(T) -> R>
// ============================================================================

/// BoxTransformerOnce - consuming transformer wrapper based on
/// `Box<dyn FnOnce>`
///
/// A transformer wrapper that provides single ownership with one-time use
/// semantics. Consumes both self and the input value.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self and input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxTransformerOnce<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnOnce(T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement BoxTransformerOnce
impl<T, R> BoxTransformerOnce<T, R> {
    impl_transformer_common_methods!(
        BoxTransformerOnce<T, R>,
        (FnOnce(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxTransformerOnce<T, R>,
        BoxConditionalTransformerOnce,
        TransformerOnce
    );
}

// Implement TransformerOnce trait for BoxTransformerOnce
impl<T, R> TransformerOnce<T, R> for BoxTransformerOnce<T, R> {
    #[inline(always)]
    fn apply(self, input: T) -> R {
        (self.function)(input)
    }
}

// Implement constant method for BoxTransformerOnce
impl_transformer_constant_method!(BoxTransformerOnce<T, R>);

// Use macro to generate Debug and Display implementations
impl_transformer_debug_display!(BoxTransformerOnce<T, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement TransformerOnce for all FnOnce(T) -> R using macro
impl_closure_once_trait!(
    TransformerOnce<T, R>,
    apply,
    BoxTransformerOnce,
    FnOnce(input: T) -> R
);
