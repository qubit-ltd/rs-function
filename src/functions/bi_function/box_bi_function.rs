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
//! Defines the `BoxBiFunction` public type.

use super::{
    BiFunction,
    BiPredicate,
    BoxBiFunctionOnce,
    BoxConditionalBiFunction,
    Function,
    RcBiFunction,
    impl_box_conversions,
    impl_box_function_methods,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
};

// ============================================================================
// BoxBiFunction - Box<dyn Fn(&T, &U) -> R>
// ============================================================================

/// BoxBiFunction - bi-function wrapper based on `Box<dyn Fn>`
///
/// A bi-function wrapper that provides single ownership with reusable
/// computation. Borrows both inputs and can be called multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T, &U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
pub struct BoxBiFunction<T, U, R> {
    #[allow(clippy::type_complexity)]
    pub(super) function: Box<dyn Fn(&T, &U) -> R>,
    pub(super) name: Option<String>,
}

// Implement BoxBiFunction
impl<T, U, R> BoxBiFunction<T, U, R> {
    impl_function_common_methods!(
        BoxBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_function_methods!(
        BoxBiFunction<T, U, R>,
        BoxConditionalBiFunction,
        Function
    );
}

// Implement BiFunction trait for BoxBiFunction
impl<T, U, R> BiFunction<T, U, R> for BoxBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiFunction<T, U, R>,
        RcBiFunction,
        Fn(&T, &U) -> R,
        BoxBiFunctionOnce
    );
}

// Implement constant method for BoxBiFunction
impl_function_constant_method!(BoxBiFunction<T, U, R>);

// Implement Debug and Display for BoxBiFunction
impl_function_debug_display!(BoxBiFunction<T, U, R>);
