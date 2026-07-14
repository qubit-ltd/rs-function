// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulBiTransformer` public type.

use {
    crate::ArcConditionalStatefulBiTransformer,
    crate::BiPredicate,
    crate::StatefulTransformer,
};
use {
    crate::StatefulBiTransformer,
    crate::macros::impl_closure_trait,
    crate::transformers::macros::impl_shared_transformer_methods,
    crate::transformers::macros::impl_transformer_clone,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
    parking_lot::Mutex,
    std::sync::Arc,
};

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
/// - **Based on**: `Arc<Mutex<dyn FnMut(T, U) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Thread-safe (`Send` required; calls are serialized)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared state
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
pub struct ArcStatefulBiTransformer<T, U, R> {
    pub(super) function: Arc<Mutex<dyn FnMut(T, U) -> R + Send>>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
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
        ArcBiPredicate,
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
impl<T, U, R> StatefulBiTransformer<T, U, R>
    for ArcStatefulBiTransformer<T, U, R>
{
    fn apply(&mut self, first: T, second: U) -> R {
        let mut func = self.function.lock();
        func(first, second)
    }
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement StatefulBiTransformer<T, U, R> for any type that implements
// FnMut(T, U) -> R
impl_closure_trait!(
    StatefulBiTransformer<T, U, R>,
    apply,
    BoxBiTransformerOnce,
    FnMut(first: T, second: U) -> R
);
