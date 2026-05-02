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
//! Defines the `RcStatefulFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// RcStatefulFunction - Rc<RefCell<dyn FnMut(&T) -> R>>
// ============================================================================

/// RcStatefulFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<RefCell<dyn FnMut(&T) -> R>>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
pub struct RcStatefulFunction<T, R> {
    pub(super) function: RcStatefulFn<T, R>,
    pub(super) name: Option<String>,
}

type RcStatefulFn<T, R> = Rc<RefCell<dyn FnMut(&T) -> R>>;

impl<T, R> RcStatefulFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcStatefulFunction<T, R>,
        (FnMut(&T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcStatefulFunction<T, R>,
        RcConditionalStatefulFunction,
        into_rc,
        StatefulFunction,
        'static
    );
}

// Generates: constant() method for RcStatefulFunction<T, R>
impl_function_constant_method!(RcStatefulFunction<T, R>, 'static);

// Generates: identity() method for RcStatefulFunction<T, T>
impl_function_identity_method!(RcStatefulFunction<T, T>);

// Generates: Clone implementation for RcStatefulFunction<T, R>
impl_function_clone!(RcStatefulFunction<T, R>);

// Generates: Debug and Display implementations for RcStatefulFunction<T, R>
impl_function_debug_display!(RcStatefulFunction<T, R>);

// Implement StatefulFunction trait for RcStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for RcStatefulFunction<T, R> {
    fn apply(&mut self, t: &T) -> R {
        (self.function.borrow_mut())(t)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcStatefulFunction<T, R>,
        BoxStatefulFunction,
        BoxFunctionOnce,
        FnMut(t: &T) -> R
    );
}
