/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `RcBiFunction` public type.

#![allow(unused_imports)]

use super::*;

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
///
pub struct RcBiFunction<T, U, R> {
    pub(super) function: Rc<dyn Fn(&T, &U) -> R>,
    pub(super) name: Option<String>,
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
        into_rc,
        Function,
        'static
    );
}

// Implement BiFunction trait for RcBiFunction
impl<T, U, R> BiFunction<T, U, R> for RcBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_rc_conversions!(
        RcBiFunction<T, U, R>,
        BoxBiFunction,
        BoxBiFunctionOnce,
        Fn(first: &T, second: &U) -> R
    );
}

// Implement constant method for RcBiFunction
impl_function_constant_method!(RcBiFunction<T, U, R>);

// Implement Debug and Display for RcBiFunction
impl_function_debug_display!(RcBiFunction<T, U, R>);

// Implement Clone for RcBiFunction
impl_function_clone!(RcBiFunction<T, U, R>);
