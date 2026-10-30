// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcBiMutatingFunction` public type.

use std::rc::Rc;

use crate::BiMutatingFunction;
use crate::BiPredicate;
use crate::MutatingFunction;
use crate::RcConditionalBiMutatingFunction;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_constant_method;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_shared_function_methods;

/// The erased callback representation used by this implementation.
type RcBiMutatingFunctionFn<T, U, R> = Rc<dyn Fn(&mut T, &mut U) -> R>;

// ============================================================================
// RcBiMutatingFunction - Rc<dyn Fn(&mut T, &mut U) -> R>
// ============================================================================

/// RcBiMutatingFunction - single-threaded bi-mutating-function wrapper
///
/// A single-threaded, clonable bi-mutating-function wrapper optimized for
/// scenarios that require sharing without thread-safety overhead.
///
/// # Type Parameters
///
/// * `T` - Type of the first value mutably borrowed by the callback.
/// * `U` - Type of the second value mutably borrowed by the callback.
/// * `R` - Type of the value returned by the callback.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&mut T, &mut U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each
///   time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Examples
///
/// ```
/// use qubit_function::{BiMutatingFunction, RcBiMutatingFunction};
///
/// let add_second = RcBiMutatingFunction::new(|first: &mut i32, second: &mut i32| {
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
pub struct RcBiMutatingFunction<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcBiMutatingFunctionFn<T, U, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U, R> RcBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        RcBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generate shared-wrapper composition methods.
    impl_shared_function_methods!(
        RcBiMutatingFunction<T, U, R>,
        RcConditionalBiMutatingFunction,
        RcBiPredicate,
        MutatingFunction,
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );
}

// Implement BiMutatingFunction trait for RcBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for RcBiMutatingFunction<T, U, R> {
    #[inline(always)]
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }
}

// Implement constant method for RcBiMutatingFunction
impl_function_constant_method!(RcBiMutatingFunction<T, U, R>, mut 'static);

// Implement Debug and Display for RcBiMutatingFunction
impl_function_debug_display!(RcBiMutatingFunction<T, U, R>);

// Implement Clone for RcBiMutatingFunction
impl_function_clone!(RcBiMutatingFunction<T, U, R>);
