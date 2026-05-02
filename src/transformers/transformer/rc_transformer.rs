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
//! Defines the `RcTransformer` public type.

#![allow(unused_imports)]

use super::*;

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
///
pub struct RcTransformer<T, R> {
    pub(super) function: Rc<dyn Fn(T) -> R>,
    pub(super) name: Option<String>,
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
        into_rc,
        Transformer,
        'static
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

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcTransformer<T, R>,
        BoxTransformer,
        BoxTransformerOnce,
        Fn(input: T) -> R
    );
}
