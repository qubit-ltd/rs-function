// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcBiFunction` public type.

use {
    crate::ArcConditionalBiFunction,
    crate::BiPredicate,
    crate::Function,
};
use {
    crate::BiFunction,
    crate::functions::macros::impl_function_clone,
    crate::functions::macros::impl_function_common_methods,
    crate::functions::macros::impl_function_constant_method,
    crate::functions::macros::impl_function_debug_display,
    crate::functions::macros::impl_shared_function_methods,
    std::sync::Arc,
};

type ArcBiFunctionFn<T, U, R> = Arc<dyn Fn(&T, &U) -> R + Send + Sync>;

// ============================================================================
// ArcBiFunction - Arc<dyn Fn(&T, &U) -> R + Send + Sync>
// ============================================================================

/// ArcBiFunction - thread-safe bi-function wrapper
///
/// A thread-safe, clonable bi-function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T, &U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
pub struct ArcBiFunction<T, U, R> {
    pub(super) function: ArcBiFunctionFn<T, U, R>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

impl<T, U, R> ArcBiFunction<T, U, R> {
    impl_function_common_methods!(
        ArcBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );
    impl_shared_function_methods!(
        ArcBiFunction<T, U, R>,
        ArcConditionalBiFunction,
        ArcBiPredicate,
        Function,
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + Sync + 'static)
    );
}

// Implement constant method for ArcBiFunction
impl_function_constant_method!(ArcBiFunction<T, U, R>, Send + Sync + 'static);

// Implement Debug and Display for ArcBiFunction
impl_function_debug_display!(ArcBiFunction<T, U, R>);

// Implement Clone for ArcBiFunction
impl_function_clone!(ArcBiFunction<T, U, R>);

// Implement BiFunction trait for ArcBiFunction
impl<T, U, R> BiFunction<T, U, R> for ArcBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }
}
