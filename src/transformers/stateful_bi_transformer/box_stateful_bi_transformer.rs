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
//! Defines the `BoxStatefulBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxStatefulBiTransformer - Box<dyn FnMut(T, U) -> R>
// ============================================================================

/// BoxStatefulBiTransformer - bi-transformer wrapper based on `Box<dyn Fn>`
///
/// A bi-transformer wrapper that provides single ownership with reusable
/// transformation. The bi-transformer consumes both inputs and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnMut(T, U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
pub struct BoxStatefulBiTransformer<T, U, R> {
    pub(super) function: Box<dyn FnMut(T, U) -> R>,
    pub(super) name: Option<String>,
}

impl<T, U, R> BoxStatefulBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        BoxStatefulBiTransformer<T, U, R>,
        (FnMut(T, U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxStatefulBiTransformer<T, U, R>,
        BoxConditionalStatefulBiTransformer,
        StatefulTransformer
    );
}

// Implement constant method for BoxStatefulBiTransformer
impl_transformer_constant_method!(stateful BoxStatefulBiTransformer<T, U, R>);

// Implement Debug and Display for BoxTransformer
impl_transformer_debug_display!(BoxStatefulBiTransformer<T, U, R>);

impl<T, U, R> StatefulBiTransformer<T, U, R> for BoxStatefulBiTransformer<T, U, R> {
    fn apply(&mut self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    fn into_box(self) -> BoxStatefulBiTransformer<T, U, R> {
        // Zero-cost: directly return itself
        self
    }

    fn into_rc(self) -> RcStatefulBiTransformer<T, U, R>
    where
        Self: 'static,
    {
        RcStatefulBiTransformer::new(self.function)
    }

    // do NOT override BoxStatefulBiTransformer::into_arc() because BoxStatefulBiTransformer is not Send + Sync
    // and calling BoxStatefulBiTransformer::into_arc() will cause a compile error

    fn into_fn(self) -> impl FnMut(T, U) -> R {
        self.function
    }

    // do NOT override BoxStatefulBiTransformer::to_xxx() because BoxStatefulBiTransformer is not Clone
    // and calling BoxStatefulBiTransformer::to_xxx() will cause a compile error
}
