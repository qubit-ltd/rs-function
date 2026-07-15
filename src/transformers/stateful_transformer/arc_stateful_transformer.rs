// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulTransformer` public type.

use {
    crate::ArcConditionalStatefulTransformer,
    crate::Predicate,
};
use {
    crate::StatefulTransformer,
    crate::transformers::macros::impl_shared_transformer_methods,
    crate::transformers::macros::impl_transformer_clone,
    crate::transformers::macros::impl_transformer_common_methods,
    crate::transformers::macros::impl_transformer_constant_method,
    crate::transformers::macros::impl_transformer_debug_display,
    parking_lot::Mutex,
    std::sync::Arc,
};

// ============================================================================
// ArcStatefulTransformer - Arc<Mutex<dyn FnMut(T) -> R + Send>>
// ============================================================================

/// ArcStatefulTransformer - thread-safe transformer wrapper
///
/// A thread-safe, clonable transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads
/// while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Arc<Mutex<dyn FnMut(T) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared state
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
pub struct ArcStatefulTransformer<T, R> {
    pub(super) function: Arc<Mutex<dyn FnMut(T) -> R + Send>>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

impl<T, R> ArcStatefulTransformer<T, R> {
    impl_transformer_common_methods!(
        ArcStatefulTransformer<T, R>,
        (FnMut(T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    impl_shared_transformer_methods!(
        ArcStatefulTransformer<T, R>,
        ArcConditionalStatefulTransformer,
        ArcPredicate,
        StatefulTransformer,
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + 'static)
    );
}

// Implement constant method for ArcStatefulTransformer
impl_transformer_constant_method!(stateful thread_safe ArcStatefulTransformer<T, R>);

// Implement Debug and Display for ArcStatefulTransformer
impl_transformer_debug_display!(ArcStatefulTransformer<T, R>);

// Implement Clone for ArcStatefulTransformer
impl_transformer_clone!(ArcStatefulTransformer<T, R>);

impl<T, R> StatefulTransformer<T, R> for ArcStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        let mut func = self.function.lock();
        func(input)
    }
}
