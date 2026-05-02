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
//! Defines the `RcBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

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
///
pub struct RcBiTransformer<T, U, R> {
    pub(super) function: Rc<dyn Fn(T, U) -> R>,
    pub(super) name: Option<String>,
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
        into_rc,
        Transformer,
        'static
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
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcBiTransformer<T, U, R>,
        BoxBiTransformer,
        BoxBiTransformerOnce,
        Fn(first: T, second: U) -> R
    );

    // do NOT override RcBiTransformer::into_arc() because RcBiTransformer is not Send + Sync
    // and calling RcBiTransformer::into_arc() will cause a compile error

    // do NOT override RcBiTransformer::to_arc() because RcBiTransformer is not Send + Sync
    // and calling RcBiTransformer::to_arc() will cause a compile error
}
