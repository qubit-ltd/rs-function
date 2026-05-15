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
//! Defines the `BoxStatefulTransformer` public type.

use super::{
    BoxConditionalStatefulTransformer,
    BoxTransformerOnce,
    Predicate,
    RcStatefulTransformer,
    StatefulTransformer,
    impl_box_conversions,
    impl_box_transformer_methods,
    impl_transformer_common_methods,
    impl_transformer_constant_method,
    impl_transformer_debug_display,
};

// ============================================================================
// BoxStatefulTransformer - Box<dyn FnMut(T) -> R>
// ============================================================================

/// BoxStatefulTransformer - transformer wrapper based on `Box<dyn FnMut>`
///
/// A transformer wrapper that provides single ownership with reusable stateful
/// transformation. The transformer consumes the input and can be called
/// multiple times while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnMut(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
/// - **Statefulness**: Can modify internal state between calls
///
pub struct BoxStatefulTransformer<T, R> {
    pub(super) function: Box<dyn FnMut(T) -> R>,
    pub(super) name: Option<String>,
}

impl<T, R> BoxStatefulTransformer<T, R> {
    impl_transformer_common_methods!(
        BoxStatefulTransformer<T, R>,
        (FnMut(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxStatefulTransformer<T, R>,
        BoxConditionalStatefulTransformer,
        StatefulTransformer
    );
}

// Implement constant method for BoxStatefulTransformer
impl_transformer_constant_method!(stateful BoxStatefulTransformer<T, R>);

// Implement Debug and Display for BoxStatefulTransformer
impl_transformer_debug_display!(BoxStatefulTransformer<T, R>);

// Implement StatefulTransformer trait for BoxStatefulTransformer
impl<T, R> StatefulTransformer<T, R> for BoxStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        (self.function)(input)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulTransformer<T, R>,
        RcStatefulTransformer,
        FnMut(T) -> R,
        BoxTransformerOnce
    );

    // do NOT override StatefulTransformer::to_xxx() because BoxStatefulTransformer is not Clone
    // and calling BoxStatefulTransformer::to_xxx() will cause a compile error
}
