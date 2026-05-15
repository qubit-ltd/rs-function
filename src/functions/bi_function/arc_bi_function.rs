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
//! Defines the `ArcBiFunction` public type.

use super::{
    Arc,
    ArcConditionalBiFunction,
    BiFunction,
    BiPredicate,
    BoxBiFunction,
    BoxBiFunctionOnce,
    Function,
    RcBiFunction,
    impl_arc_conversions,
    impl_closure_trait,
    impl_function_clone,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
    impl_shared_function_methods,
};

// ============================================================================
// ArcBiFunction - Arc<dyn Fn(&T, &U) -> R + Send + Sync>
// ============================================================================

/// ArcBiFunction - thread-safe bi-function wrapper
///
/// A thread-safe, clonable bi-function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T, &U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
pub struct ArcBiFunction<T, U, R> {
    pub(super) function: Arc<dyn Fn(&T, &U) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T, U, R> ArcBiFunction<T, U, R> {
    impl_function_common_methods!(
        ArcBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );
    impl_shared_function_methods!(
        ArcBiFunction<T, U, R>,
        ArcConditionalBiFunction,
        into_arc,
        Function,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcBiFunction
impl_function_constant_method!(ArcBiFunction<T, U, R>, Send + Sync + 'static);

// Implement Debug and Display for ArcBiFunction
impl_function_debug_display!(ArcBiFunction<T, U, R>);

// Implement Clone for ArcBiFunction
impl_function_clone!(ArcBiFunction<T, U, R>);

// Implement BiFunction trait for ArcBiFunction
impl<T, U, R> BiFunction<T, U, R> for ArcBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcBiFunction<T, U, R>,
        BoxBiFunction,
        RcBiFunction,
        BoxBiFunctionOnce,
        Fn(t: &T, u: &U) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement BiFunction<T, U, R> for any type that implements Fn(&T, &U) -> R
impl_closure_trait!(
    BiFunction<T, U, R>,
    apply,
    BoxBiFunctionOnce,
    Fn(first: &T, second: &U) -> R
);
