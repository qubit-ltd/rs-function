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
//! Defines the `BoxBiMutatingFunctionOnce` public type.

use super::{
    BiMutatingFunctionOnce,
    BiPredicate,
    BoxConditionalBiMutatingFunctionOnce,
    MutatingFunctionOnce,
    impl_box_function_methods,
    impl_box_once_conversions,
    impl_closure_once_trait,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
};

// ============================================================================
// BoxBiMutatingFunctionOnce - Box<dyn FnOnce(&mut T, &mut U) -> R>
// ============================================================================

/// BoxBiMutatingFunctionOnce - consuming bi-mutating-function wrapper based on
/// `Box<dyn FnOnce>`
///
/// A bi-mutating-function wrapper that provides single ownership with one-time use
/// semantics. Consumes self and borrows both input values mutably.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&mut T, &mut U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
pub struct BoxBiMutatingFunctionOnce<T, U, R> {
    pub(super) function: Box<dyn FnOnce(&mut T, &mut U) -> R>,
    pub(super) name: Option<String>,
}

// Implement BoxBiMutatingFunctionOnce
impl<T, U, R> BoxBiMutatingFunctionOnce<T, U, R> {
    // Generate new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxBiMutatingFunctionOnce<T, U, R>,
        (FnOnce(&mut T, &mut U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generate when(), and_then()
    impl_box_function_methods!(
        BoxBiMutatingFunctionOnce<T, U, R>,
        BoxConditionalBiMutatingFunctionOnce,
        MutatingFunctionOnce
    );
}

// Implement BiMutatingFunctionOnce trait for BoxBiMutatingFunctionOnce
impl<T, U, R> BiMutatingFunctionOnce<T, U, R> for BoxBiMutatingFunctionOnce<T, U, R> {
    fn apply(self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_fn(), to_box()
    impl_box_once_conversions!(
        BoxBiMutatingFunctionOnce<T, U, R>,
        BiMutatingFunctionOnce,
        FnOnce(&mut T, &mut U) -> R
    );
}

// Implement constant method for BoxBiMutatingFunctionOnce
impl_function_constant_method!(BoxBiMutatingFunctionOnce<T, U, R>);

// Use macro to generate Debug and Display implementations
impl_function_debug_display!(BoxBiMutatingFunctionOnce<T, U, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement BiMutatingFunctionOnce for all FnOnce(&mut T, &mut U) -> R using macro
impl_closure_once_trait!(
    BiMutatingFunctionOnce<T, U, R>,
    apply,
    BoxBiMutatingFunctionOnce,
    FnOnce(first: &mut T, second: &mut U) -> R
);
