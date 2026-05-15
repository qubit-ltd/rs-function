/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `BoxTransformerOnce` public type.

use super::{
    BoxConditionalTransformerOnce,
    Predicate,
    TransformerOnce,
    impl_box_once_conversions,
    impl_box_transformer_methods,
    impl_closure_once_trait,
    impl_transformer_common_methods,
    impl_transformer_constant_method,
    impl_transformer_debug_display,
};

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
///
pub struct BoxTransformerOnce<T, R> {
    pub(super) function: Box<dyn FnOnce(T) -> R>,
    pub(super) name: Option<String>,
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
    fn apply(self, input: T) -> R {
        (self.function)(input)
    }

    impl_box_once_conversions!(
        BoxTransformerOnce<T, R>,
        TransformerOnce,
        FnOnce(T) -> R
    );
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
