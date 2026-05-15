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
//! Defines the `RcFunction` public type.

use super::{
    BoxFunction,
    BoxFunctionOnce,
    Function,
    Predicate,
    Rc,
    RcConditionalFunction,
    impl_function_clone,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
    impl_function_identity_method,
    impl_rc_conversions,
    impl_shared_function_methods,
};

// ============================================================================
// RcFunction - Rc<dyn Fn(&T) -> R>
// ============================================================================

/// RcFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&T) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
pub struct RcFunction<T, R> {
    pub(super) function: Rc<dyn Fn(&T) -> R>,
    pub(super) name: Option<String>,
}

impl<T, R> RcFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcFunction<T, R>,
        RcConditionalFunction,
        into_rc,
        Function,
        'static
    );
}

// Generates: constant() method for RcFunction<T, R>
impl_function_constant_method!(RcFunction<T, R>, 'static);

// Generates: identity() method for RcFunction<T, T>
impl_function_identity_method!(RcFunction<T, T>);

// Generates: Clone implementation for RcFunction<T, R>
impl_function_clone!(RcFunction<T, R>);

// Generates: Debug and Display implementations for RcFunction<T, R>
impl_function_debug_display!(RcFunction<T, R>);

// Implement Function trait for RcFunction<T, R>
impl<T, R> Function<T, R> for RcFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcFunction<T, R>,
        BoxFunction,
        BoxFunctionOnce,
        Fn(t: &T) -> R
    );
}
