// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcBiFunction` public type.

use std::rc::Rc;

use crate::BiFunction;
use crate::BiPredicate;
use crate::Function;
use crate::RcConditionalBiFunction;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_shared_function_methods;

/// The erased callback representation used by this implementation.
type RcBiFunctionFn<T, U, R> = Rc<dyn Fn(&T, &U) -> R>;

// ============================================================================
// RcBiFunction - Rc<dyn Fn(&T, &U) -> R>
// ============================================================================

/// RcBiFunction - single-threaded bi-function wrapper
///
/// A single-threaded, clonable bi-function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&T, &U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcBiFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcBiFunctionFn<T, U, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U, R> RcBiFunction<T, U, R> {
    impl_function_common_methods!(
        RcBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + 'static),
        |f| Rc::new(f)
    );
    impl_shared_function_methods!(
        RcBiFunction<T, U, R>,
        RcConditionalBiFunction,
        RcBiPredicate,
        Function,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Implement BiFunction trait for RcBiFunction
impl<T, U, R> BiFunction<T, U, R> for RcBiFunction<T, U, R> {
    #[inline(always)]
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }
}

// Implement constant method for RcBiFunction
impl_function_constant_method!(RcBiFunction<T, U, R>);

// Implement Debug and Display for RcBiFunction
impl_function_debug_display!(RcBiFunction<T, U, R>);

// Implement Clone for RcBiFunction
impl_function_clone!(RcBiFunction<T, U, R>);
