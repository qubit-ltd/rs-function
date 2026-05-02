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
//! Defines the `RcMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 4. RcMutatingFunction - Single-Threaded Shared Ownership
// =======================================================================

/// RcMutatingFunction struct
///
/// A mutating function implementation based on `Rc<dyn Fn(&mut T) -> R>` for
/// single-threaded shared ownership scenarios. This type allows multiple
/// references to the same function without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not
///   `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcMutatingFunction` (no locking)
///
/// # Use Cases
///
/// Choose `RcMutatingFunction` when:
/// - The function needs to be shared within a single thread for stateless
///   operations
/// - Thread safety is not required
/// - Performance is important (avoiding lock overhead)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{MutatingFunction, RcMutatingFunction};
///
/// let func = RcMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let clone = func.clone();
///
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// ```
///
pub struct RcMutatingFunction<T, R> {
    pub(super) function: Rc<dyn Fn(&mut T) -> R>,
    pub(super) name: Option<String>,
}

impl<T, R> RcMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcMutatingFunction<T, R>,
        (Fn(&mut T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcMutatingFunction<T, R>,
        RcConditionalMutatingFunction,
        into_rc,
        Function,  // chains a non-mutating function after this mutating function
        'static
    );
}

// Generates: Clone implementation for RcMutatingFunction<T, R>
impl_function_clone!(RcMutatingFunction<T, R>);

// Generates: Debug and Display implementations for RcMutatingFunction<T, R>
impl_function_debug_display!(RcMutatingFunction<T, R>);

// Generates: identity() method for RcMutatingFunction<T, T>
impl_function_identity_method!(RcMutatingFunction<T, T>, mutating);

impl<T, R> MutatingFunction<T, R> for RcMutatingFunction<T, R> {
    fn apply(&self, input: &mut T) -> R {
        (self.function)(input)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcMutatingFunction<T, R>,
        BoxMutatingFunction,
        BoxMutatingFunctionOnce,
        Fn(input: &mut T) -> R
    );
}
