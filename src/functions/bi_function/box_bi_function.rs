// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiFunction` public type.

use {
    crate::BiFunction,
    crate::functions::macros::impl_box_function_methods,
    crate::functions::macros::impl_function_common_methods,
    crate::functions::macros::impl_function_constant_method,
    crate::functions::macros::impl_function_debug_display,
};
use {
    crate::BiPredicate,
    crate::BoxConditionalBiFunction,
    crate::Function,
};

/// The erased callback representation used by this implementation.
type BoxBiFunctionFn<T, U, R> = Box<dyn Fn(&T, &U) -> R>;

// ============================================================================
// BoxBiFunction - Box<dyn Fn(&T, &U) -> R>
// ============================================================================

/// BoxBiFunction - bi-function wrapper based on `Box<dyn Fn>`
///
/// A bi-function wrapper that provides single ownership with reusable
/// computation. Borrows both inputs and can be called multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T, &U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxBiFunctionFn<T, U, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement BoxBiFunction
impl<T, U, R> BoxBiFunction<T, U, R> {
    impl_function_common_methods!(
        BoxBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_function_methods!(
        BoxBiFunction<T, U, R>,
        BoxConditionalBiFunction,
        Function
    );
}

// Implement BiFunction trait for BoxBiFunction
impl<T, U, R> BiFunction<T, U, R> for BoxBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }
}

// Implement constant method for BoxBiFunction
impl_function_constant_method!(BoxBiFunction<T, U, R>);

// Implement Debug and Display for BoxBiFunction
impl_function_debug_display!(BoxBiFunction<T, U, R>);
