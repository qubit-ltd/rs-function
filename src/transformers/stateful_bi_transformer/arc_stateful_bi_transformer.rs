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
//! Defines the `ArcStatefulBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcStatefulBiTransformer - Arc<dyn FnMut(T, U) -> R + Send + Sync>
// ============================================================================

/// ArcStatefulBiTransformer - thread-safe bi-transformer wrapper
///
/// A thread-safe, clonable bi-transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn FnMut(T, U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
pub struct ArcStatefulBiTransformer<T, U, R> {
    pub(super) function: Arc<Mutex<dyn FnMut(T, U) -> R + Send>>,
    pub(super) name: Option<String>,
}

impl<T, U, R> ArcStatefulBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        ArcStatefulBiTransformer<T, U, R>,
        (FnMut(T, U) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    impl_shared_transformer_methods!(
        ArcStatefulBiTransformer<T, U, R>,
        ArcConditionalStatefulBiTransformer,
        into_arc,
        StatefulTransformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcStatefulBiTransformer
impl_transformer_constant_method!(stateful thread_safe ArcStatefulBiTransformer<T, U, R>);

// Implement Debug and Display for ArcStatefulBiTransformer
impl_transformer_debug_display!(ArcStatefulBiTransformer<T, U, R>);

// Implement Clone for ArcStatefulBiTransformer
impl_transformer_clone!(ArcStatefulBiTransformer<T, U, R>);

// Implement StatefulBiTransformer trait for ArcStatefulBiTransformer
impl<T, U, R> StatefulBiTransformer<T, U, R> for ArcStatefulBiTransformer<T, U, R> {
    fn apply(&mut self, first: T, second: U) -> R {
        let mut func = self.function.lock();
        func(first, second)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulBiTransformer<T, U, R>,
        BoxStatefulBiTransformer,
        RcStatefulBiTransformer,
        BoxBiTransformerOnce,
        FnMut(t: T, u: U) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement StatefulBiTransformer<T, U, R> for any type that implements FnMut(T, U) -> R
impl_closure_trait!(
    StatefulBiTransformer<T, U, R>,
    apply,
    BoxBiTransformerOnce,
    FnMut(first: T, second: U) -> R
);
