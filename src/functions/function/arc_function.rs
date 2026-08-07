// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcFunction` public type.

use std::sync::Arc;

use crate::ArcConditionalFunction;
use crate::Function;
use crate::Predicate;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;
use crate::functions::macros::impl_shared_function_methods;

// ============================================================================
// ArcFunction - Arc<dyn Fn(&T) -> R + Send + Sync>
// ============================================================================

/// ArcFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows its input each
///   time)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Arc<dyn Fn(&T) -> R + Send + Sync>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> ArcFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        ArcFunction<T, R>,
        (Fn(&T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcFunction<T, R>,
        ArcConditionalFunction,
        ArcPredicate,
        Function,
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + Sync + 'static)
    );
}

// Generates: constant() method for ArcFunction<T, R>
impl_function_constant_method!(ArcFunction<T, R>, Send + Sync + 'static);

// Generates: identity() method for ArcFunction<T, T>
impl_function_identity_method!(ArcFunction<T, T>);

// Generates: Clone implementation for ArcFunction<T, R>
impl_function_clone!(ArcFunction<T, R>);

// Generates: Debug and Display implementations for ArcFunction<T, R>
impl_function_debug_display!(ArcFunction<T, R>);

// Implement Function trait for ArcFunction<T, R>
impl<T, R> Function<T, R> for ArcFunction<T, R> {
    #[inline(always)]
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }
}
