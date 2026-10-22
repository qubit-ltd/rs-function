// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiMutatingFunction` public type.

use crate::BiMutatingFunction;
use crate::BiPredicate;
use crate::BoxConditionalBiMutatingFunction;
use crate::MutatingFunction;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;

/// The erased callback representation used by this implementation.
///
/// The callback mutably borrows both inputs for each invocation and may be
/// called repeatedly.
type BoxBiMutatingFunctionFn<T, U, R> = Box<dyn Fn(&mut T, &mut U) -> R>;

// ============================================================================
// BoxBiMutatingFunction - Box<dyn Fn(&mut T, &mut U) -> R>
// ============================================================================

/// BoxBiMutatingFunction - bi-mutating-function wrapper based on `Box<dyn Fn>`
///
/// A bi-mutating-function wrapper that provides single ownership with reusable
/// computation. Borrows both inputs mutably and can be called multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&mut T, &mut U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each
///   time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Type Parameters
///
/// - `T` - The type of the first mutably borrowed input.
/// - `U` - The type of the second mutably borrowed input.
/// - `R` - The value produced by the callback.
///
/// # Examples
///
/// ```
/// use qubit_function::{BiMutatingFunction, BoxBiMutatingFunction};
///
/// let add_second = BoxBiMutatingFunction::new(|first: &mut i32, second: &mut i32| {
///     *first += *second;
///     *first
/// });
/// let mut first = 2;
/// let mut second = 3;
///
/// assert_eq!(add_second.apply(&mut first, &mut second), 5);
/// assert_eq!(first, 5);
/// assert_eq!(second, 3);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiMutatingFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxBiMutatingFunctionFn<T, U, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement BoxBiMutatingFunction
impl<T, U, R> BoxBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then()
    impl_box_function_methods!(
        BoxBiMutatingFunction<T, U, R>,
        BoxConditionalBiMutatingFunction,
        MutatingFunction
    );
}

// Implement BiMutatingFunction trait for BoxBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for BoxBiMutatingFunction<T, U, R> {
    #[inline(always)]
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }
}

// Implement constant method for BoxBiMutatingFunction
impl_function_constant_method!(BoxBiMutatingFunction<T, U, R>, mut 'static);

// Implement Debug and Display for BoxBiMutatingFunction
impl_function_debug_display!(BoxBiMutatingFunction<T, U, R>);
