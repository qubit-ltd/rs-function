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
//! Defines the `ArcMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 5. ArcMutatingFunction - Thread-Safe Shared Ownership
// =======================================================================

/// ArcMutatingFunction struct
///
/// A mutating function implementation based on
/// `Arc<dyn Fn(&mut T) -> R + Send + Sync>` for thread-safe shared ownership
/// scenarios. This type allows the function to be safely shared and used
/// across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not
///   `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcMutatingFunction` when:
/// - The function needs to be shared across multiple threads for stateless
///   operations
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatingFunction, ArcMutatingFunction};
///
/// let func = ArcMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let clone = func.clone();
///
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// ```
///
pub struct ArcMutatingFunction<T, R> {
    pub(super) function: Arc<dyn Fn(&mut T) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T, R> ArcMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcMutatingFunction<T, R>,
        (Fn(&mut T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcMutatingFunction<T, R>,
        ArcConditionalMutatingFunction,
        into_arc,
        Function,  // chains a non-mutating function after this mutating function
        Send + Sync + 'static
    );
}

// Generates: Clone implementation for ArcMutatingFunction<T, R>
impl_function_clone!(ArcMutatingFunction<T, R>);

// Generates: Debug and Display implementations for ArcMutatingFunction<T, R>
impl_function_debug_display!(ArcMutatingFunction<T, R>);

// Generates: identity() method for ArcMutatingFunction<T, T>
impl_function_identity_method!(ArcMutatingFunction<T, T>, mutating);

impl<T, R> MutatingFunction<T, R> for ArcMutatingFunction<T, R> {
    fn apply(&self, input: &mut T) -> R {
        (self.function)(input)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcMutatingFunction<T, R>,
        BoxMutatingFunction,
        RcMutatingFunction,
        BoxMutatingFunctionOnce,
        Fn(input: &mut T) -> R
    );
}

// =======================================================================
// 6. Implement MutatingFunction trait for closures
// =======================================================================

impl_closure_trait!(
    MutatingFunction<T, R>,
    apply,
    BoxMutatingFunctionOnce,
    Fn(input: &mut T) -> R
);
