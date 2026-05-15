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
//! Defines the `BoxBiFunctionOnce` public type.

use super::{
    BiFunctionOnce,
    BiPredicate,
    BoxConditionalBiFunctionOnce,
    FunctionOnce,
    impl_box_function_methods,
    impl_box_once_conversions,
    impl_closure_once_trait,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
};

type BoxBiFunctionOnceFn<T, U, R> = Box<dyn FnOnce(&T, &U) -> R>;

// ============================================================================
// BoxBiFunctionOnce - Box<dyn FnOnce(&T, &U) -> R>
// ============================================================================

/// BoxBiFunctionOnce - consuming bi-function wrapper based on
/// `Box<dyn FnOnce>`
///
/// A bi-function wrapper that provides single ownership with one-time use
/// semantics. Consumes self and borrows both input values.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&T, &U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
pub struct BoxBiFunctionOnce<T, U, R> {
    pub(super) function: BoxBiFunctionOnceFn<T, U, R>,
    pub(super) name: Option<String>,
}

// Implement BoxBiFunctionOnce
impl<T, U, R> BoxBiFunctionOnce<T, U, R> {
    // Generate new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxBiFunctionOnce<T, U, R>,
        (FnOnce(&T, &U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generate when(), and_then()
    impl_box_function_methods!(
        BoxBiFunctionOnce<T, U, R>,
        BoxConditionalBiFunctionOnce,
        FunctionOnce
    );
}

// Implement BiFunctionOnce trait for BoxBiFunctionOnce
impl<T, U, R> BiFunctionOnce<T, U, R> for BoxBiFunctionOnce<T, U, R> {
    fn apply(self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_fn(), to_box()
    impl_box_once_conversions!(
        BoxBiFunctionOnce<T, U, R>,
        BiFunctionOnce,
        FnOnce(&T, &U) -> R
    );
}

// Implement constant method for BoxBiFunctionOnce
impl_function_constant_method!(BoxBiFunctionOnce<T, U, R>);

// Use macro to generate Debug and Display implementations
impl_function_debug_display!(BoxBiFunctionOnce<T, U, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement BiFunctionOnce for all FnOnce(&T, &U) -> R using macro
impl_closure_once_trait!(
    BiFunctionOnce<T, U, R>,
    apply,
    BoxBiFunctionOnce,
    FnOnce(first: &T, second: &U) -> R
);
