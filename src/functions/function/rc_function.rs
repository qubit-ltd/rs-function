// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcFunction` public type.

use {
    crate::Function,
    crate::functions::macros::impl_function_clone,
    crate::functions::macros::impl_function_common_methods,
    crate::functions::macros::impl_function_constant_method,
    crate::functions::macros::impl_function_debug_display,
    crate::functions::macros::impl_function_identity_method,
    crate::functions::macros::impl_shared_function_methods,
    std::rc::Rc,
};
use {
    crate::Predicate,
    crate::RcConditionalFunction,
};

// ============================================================================
// RcFunction - Rc<dyn Fn(&T) -> R>
// ============================================================================

/// RcFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&T) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows its input each
///   time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Rc<dyn Fn(&T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> RcFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        RcFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcFunction<T, R>,
        RcConditionalFunction,
        RcPredicate,
        Function,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Generates: constant() method for RcFunction<T, R>
impl_function_constant_method!(RcFunction<T, R>, 'static);

// Generates: identity() method for RcFunction<T, T>
impl_function_identity_method!(RcFunction<T, T>);

// Generates: Clone implementation for RcFunction<T, R>
impl_function_clone!(RcFunction<T, R>);

// Generates: Debug and Display implementations for RcFunction<T, R>
impl_function_debug_display!(RcFunction<T, R>);

// Implement Function trait for RcFunction<T, R>
impl<T, R> Function<T, R> for RcFunction<T, R> {
    #[inline(always)]
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }
}
