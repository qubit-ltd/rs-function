// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxFunctionOnce` public type.

use {
    crate::BoxConditionalFunctionOnce,
    crate::Predicate,
};
use {
    crate::FunctionOnce,
    crate::functions::macros::impl_box_function_methods,
    crate::functions::macros::impl_function_common_methods,
    crate::functions::macros::impl_function_constant_method,
    crate::functions::macros::impl_function_debug_display,
    crate::functions::macros::impl_function_identity_method,
    crate::macros::impl_closure_once_trait,
};

// ============================================================================
// BoxFunctionOnce - Box<dyn FnOnce(&T) -> R>
// ============================================================================

/// BoxFunctionOnce - consuming transformer wrapper based on
/// `Box<dyn FnOnce>`
///
/// A transformer wrapper that provides single ownership with one-time use
/// semantics. Consumes both self and the input value.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self and input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxFunctionOnce<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnOnce(&T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> BoxFunctionOnce<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxFunctionOnce<T, R>,
        (FnOnce(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxFunctionOnce<T, R>,
        BoxConditionalFunctionOnce,
        FunctionOnce
    );
}

impl<T, R> FunctionOnce<T, R> for BoxFunctionOnce<T, R> {
    #[inline(always)]
    fn apply(self, input: &T) -> R {
        (self.function)(input)
    }
}

// Generates: constant() method for BoxFunctionOnce<T, R>
impl_function_constant_method!(BoxFunctionOnce<T, R>, 'static);

// Generates: identity() method for BoxFunctionOnce<T, T>
impl_function_identity_method!(BoxFunctionOnce<T, T>);

// Generates: Debug and Display implementations for BoxFunctionOnce<T, R>
impl_function_debug_display!(BoxFunctionOnce<T, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement FunctionOnce for all FnOnce(&T) -> R using macro
impl_closure_once_trait!(
    FunctionOnce<T, R>,
    apply,
    BoxFunctionOnce,
    FnOnce(input: &T) -> R
);
