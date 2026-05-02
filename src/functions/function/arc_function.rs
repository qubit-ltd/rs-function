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
//! Defines the `ArcFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcFunction - Arc<dyn Fn(&T) -> R + Send + Sync>
// ============================================================================

/// ArcFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
pub struct ArcFunction<T, R> {
    pub(super) function: Arc<dyn Fn(&T) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T, R> ArcFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcFunction<T, R>,
        (Fn(&T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcFunction<T, R>,
        ArcConditionalFunction,
        into_arc,
        Function,
        Send + Sync + 'static
    );
}

// Generates: constant() method for ArcFunction<T, R>
impl_function_constant_method!(ArcFunction<T, R>, Send + Sync + 'static);

// Generates: identity() method for ArcFunction<T, T>
impl_function_identity_method!(ArcFunction<T, T>);

// Generates: Clone implementation for ArcFunction<T, R>
impl_function_clone!(ArcFunction<T, R>);

// Generates: Debug and Display implementations for ArcFunction<T, R>
impl_function_debug_display!(ArcFunction<T, R>);

// Implement Function trait for ArcFunction<T, R>
impl<T, R> Function<T, R> for ArcFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcFunction<T, R>,
        BoxFunction,
        RcFunction,
        BoxFunctionOnce,
        Fn(t: &T) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement Function<T, R> for any type that implements Fn(&T) -> R
impl_closure_trait!(
    Function<T, R>,
    apply,
    BoxFunctionOnce,
    Fn(input: &T) -> R
);
