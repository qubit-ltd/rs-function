// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxFunction` public type.

use crate::BoxConditionalFunction;
use crate::Function;
use crate::Predicate;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;

// ============================================================================
// BoxFunction - Box<dyn Fn(&T) -> R>
// ============================================================================

/// BoxFunction - function wrapper based on `Box<dyn Fn>`
///
/// A function wrapper that provides single ownership with reusable
/// transformation. The function borrows the input and can be called multiple
/// times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows its input each
///   time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn Fn(&T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> BoxFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxFunction<T, R>,
        BoxConditionalFunction,
        Function
    );
}

// Generates: constant() method for BoxFunction<T, R>
impl_function_constant_method!(BoxFunction<T, R>, 'static);

// Generates: identity() method for BoxFunction<T, T>
impl_function_identity_method!(BoxFunction<T, T>);

// Generates: Debug and Display implementations for BoxFunction<T, R>
impl_function_debug_display!(BoxFunction<T, R>);

// Implement Function trait for BoxFunction<T, R>
impl<T, R> Function<T, R> for BoxFunction<T, R> {
    #[inline(always)]
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }
}
