// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxStatefulFunction` public type.

use crate::BoxConditionalStatefulFunction;
use crate::Predicate;
use crate::StatefulFunction;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;

// ============================================================================
// BoxStatefulFunction - Box<dyn FnMut(&T) -> R>
// ============================================================================

/// BoxStatefulFunction - stateful function wrapper based on `Box<dyn FnMut>`
///
/// A stateful function wrapper that provides single ownership with reusable
/// stateful transformation. The stateful function borrows the input and can be
/// called multiple times while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnMut(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows its input each
///   time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
/// - **Statefulness**: Can modify internal state between calls
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxStatefulFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnMut(&T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> BoxStatefulFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxStatefulFunction<T, R>,
        (FnMut(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxStatefulFunction<T, R>,
        BoxConditionalStatefulFunction,
        StatefulFunction
    );
}

// Generates: constant() method for BoxStatefulFunction<T, R>
impl_function_constant_method!(BoxStatefulFunction<T, R>, 'static);

// Generates: identity() method for BoxStatefulFunction<T, T>
impl_function_identity_method!(BoxStatefulFunction<T, T>);

// Generates: Debug and Display implementations for BoxStatefulFunction<T, R>
impl_function_debug_display!(BoxStatefulFunction<T, R>);

// Implement StatefulFunction trait for BoxStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for BoxStatefulFunction<T, R> {
    #[inline(always)]
    fn apply(&mut self, t: &T) -> R {
        (self.function)(t)
    }
}
