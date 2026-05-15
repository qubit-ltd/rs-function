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
//! Defines the `BoxBiMutatingFunction` public type.

use super::{
    BiMutatingFunction,
    BiPredicate,
    BoxBiMutatingFunctionOnce,
    BoxConditionalBiMutatingFunction,
    MutatingFunction,
    RcBiMutatingFunction,
    impl_box_conversions,
    impl_box_function_methods,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
};

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
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
pub struct BoxBiMutatingFunction<T, U, R> {
    pub(super) function: BoxBiMutatingFunctionFn<T, U, R>,
    pub(super) name: Option<String>,
}

// Implement BoxBiMutatingFunction
impl<T, U, R> BoxBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
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
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiMutatingFunction<T, U, R>,
        RcBiMutatingFunction,
        Fn(&mut T, &mut U) -> R,
        BoxBiMutatingFunctionOnce
    );
}

// Implement constant method for BoxBiMutatingFunction
impl_function_constant_method!(BoxBiMutatingFunction<T, U, R>);

// Implement Debug and Display for BoxBiMutatingFunction
impl_function_debug_display!(BoxBiMutatingFunction<T, U, R>);
