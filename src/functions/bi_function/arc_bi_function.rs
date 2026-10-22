// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcBiFunction` public type.

use std::sync::Arc;

use crate::ArcConditionalBiFunction;
use crate::BiFunction;
use crate::BiPredicate;
use crate::Function;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_shared_function_methods;

/// The erased callback representation used by this implementation.
///
/// The shared allocation permits cheap clones while requiring callbacks to be
/// safe for concurrent calls.
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
///
/// # Type Parameters
///
/// - `T` - The type of the first borrowed input.
/// - `U` - The type of the second borrowed input.
/// - `R` - The value produced by the callback.
///
/// # Examples
///
/// ```
/// use qubit_function::{ArcBiFunction, BiFunction};
///
/// let add = ArcBiFunction::new(|first: &i32, second: &i32| first + second);
/// assert_eq!(add.apply(&2, &3), 5);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcBiFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcBiFunctionFn<T, U, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
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
    /// Invokes the stored callback with borrowed inputs and returns its result.
    #[inline(always)]
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }
}
