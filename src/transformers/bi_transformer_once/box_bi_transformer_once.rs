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
//! Defines the `BoxBiTransformerOnce` public type.

use super::{
    BiPredicate,
    BiTransformerOnce,
    BoxConditionalBiTransformerOnce,
    TransformerOnce,
    impl_box_once_conversions,
    impl_box_transformer_methods,
    impl_closure_once_trait,
    impl_transformer_common_methods,
    impl_transformer_constant_method,
    impl_transformer_debug_display,
};

// ============================================================================
// BoxBiTransformerOnce - Box<dyn FnOnce(T, U) -> R>
// ============================================================================

/// BoxBiTransformerOnce - consuming bi-transformer wrapper based on
/// `Box<dyn FnOnce>`
///
/// A bi-transformer wrapper that provides single ownership with one-time use
/// semantics. Consumes self and both input values.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(T, U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self and inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
pub struct BoxBiTransformerOnce<T, U, R> {
    pub(super) function: Box<dyn FnOnce(T, U) -> R>,
    pub(super) name: Option<String>,
}

// Implement BoxBiTransformerOnce
impl<T, U, R> BoxBiTransformerOnce<T, U, R> {
    impl_transformer_common_methods!(
        BoxBiTransformerOnce<T, U, R>,
        (FnOnce(T, U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxBiTransformerOnce<T, U, R>,
        BoxConditionalBiTransformerOnce,
        TransformerOnce
    );
}

// Implement BiTransformerOnce trait for BoxBiTransformerOnce
impl<T, U, R> BiTransformerOnce<T, U, R> for BoxBiTransformerOnce<T, U, R> {
    fn apply(self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    impl_box_once_conversions!(
        BoxBiTransformerOnce<T, U, R>,
        BiTransformerOnce,
        FnOnce(T, U) -> R
    );
}

// Implement constant method for BoxBiTransformerOnce
impl_transformer_constant_method!(BoxBiTransformerOnce<T, U, R>);

// Use macro to generate Debug and Display implementations
impl_transformer_debug_display!(BoxBiTransformerOnce<T, U, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement BiTransformerOnce for all FnOnce(T, U) -> R using macro
impl_closure_once_trait!(
    BiTransformerOnce<T, U, R>,
    apply,
    BoxBiTransformerOnce,
    FnOnce(first: T, second: U) -> R
);
