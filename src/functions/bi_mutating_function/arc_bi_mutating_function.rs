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
//! Defines the `ArcBiMutatingFunction` public type.

use super::{
    Arc,
    ArcConditionalBiMutatingFunction,
    BiMutatingFunction,
    BiPredicate,
    BoxBiMutatingFunction,
    BoxBiMutatingFunctionOnce,
    MutatingFunction,
    RcBiMutatingFunction,
    impl_arc_conversions,
    impl_closure_trait,
    impl_function_clone,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
    impl_shared_function_methods,
};

// ============================================================================
// ArcBiMutatingFunction - Arc<dyn Fn(&mut T, &mut U) -> R + Send + Sync>
// ============================================================================

/// ArcBiMutatingFunction - thread-safe bi-mutating-function wrapper
///
/// A thread-safe, clonable bi-mutating-function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&mut T, &mut U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each time)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
pub struct ArcBiMutatingFunction<T, U, R> {
    #[allow(clippy::type_complexity)]
    pub(super) function: Arc<dyn Fn(&mut T, &mut U) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T, U, R> ArcBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_shared_function_methods!(
        ArcBiMutatingFunction<T, U, R>,
        ArcConditionalBiMutatingFunction,
        into_arc,
        MutatingFunction,
        Send + Sync + 'static
    );
}

// Implement BiMutatingFunction trait for ArcBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for ArcBiMutatingFunction<T, U, R> {
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_arc_conversions!(
        ArcBiMutatingFunction<T, U, R>,
        BoxBiMutatingFunction,
        RcBiMutatingFunction,
        BoxBiMutatingFunctionOnce,
        Fn(first: &mut T, second: &mut U) -> R
    );
}

// Implement constant method for ArcBiMutatingFunction
impl_function_constant_method!(ArcBiMutatingFunction<T, U, R>, Send + Sync + 'static);

// Implement Debug and Display for ArcBiMutatingFunction
impl_function_debug_display!(ArcBiMutatingFunction<T, U, R>);

// Implement Clone for ArcBiMutatingFunction
impl_function_clone!(ArcBiMutatingFunction<T, U, R>);

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement BiMutatingFunction<T, U, R> for any type that implements Fn(&mut T, &mut U) -> R
impl_closure_trait!(
    BiMutatingFunction<T, U, R>,
    apply,
    BoxBiMutatingFunctionOnce,
    Fn(first: &mut T, second: &mut U) -> R
);
