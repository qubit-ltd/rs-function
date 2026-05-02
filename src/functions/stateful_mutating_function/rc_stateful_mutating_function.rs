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
//! Defines the `RcStatefulMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 4. RcStatefulMutatingFunction - Single-Threaded Shared Ownership
// =======================================================================

/// RcStatefulMutatingFunction struct
///
/// A stateful mutating function implementation based on
/// `Rc<RefCell<dyn FnMut(&mut T) -> R>>` for single-threaded shared
/// ownership scenarios. This type allows multiple references to the same
/// function without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcStatefulMutatingFunction` (no
///   locking)
///
/// # Use Cases
///
/// Choose `RcStatefulMutatingFunction` when:
/// - The function needs to be shared within a single thread for stateful
///   operations
/// - Thread safety is not required
/// - Performance is important (avoiding lock overhead)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       RcStatefulMutatingFunction};
///
/// let counter = {
///     let mut count = 0;
///     RcStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x *= 2;
///         count
///     })
/// };
/// let mut clone = counter.clone();
///
/// let mut value = 5;
/// assert_eq!(clone.apply(&mut value), 1);
/// ```
///
pub struct RcStatefulMutatingFunction<T, R> {
    pub(super) function: RcStatefulMutatingFunctionFn<T, R>,
    pub(super) name: Option<String>,
}

impl<T, R> RcStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcStatefulMutatingFunction<T, R>,
        RcConditionalStatefulMutatingFunction,
        into_rc,
        Function,  // chains a non-mutating function after this mutating function
        'static
    );
}

// Generates: Clone implementation for RcStatefulMutatingFunction<T, R>
impl_function_clone!(RcStatefulMutatingFunction<T, R>);

// Generates: Debug and Display implementations for RcStatefulMutatingFunction<T, R>
impl_function_debug_display!(RcStatefulMutatingFunction<T, R>);

// Generates: identity() method for RcStatefulMutatingFunction<T, T>
impl_function_identity_method!(RcStatefulMutatingFunction<T, T>, mutating);

// Implement StatefulMutatingFunction trait for RcStatefulMutatingFunction<T, R>
impl<T, R> StatefulMutatingFunction<T, R> for RcStatefulMutatingFunction<T, R> {
    fn apply(&mut self, t: &mut T) -> R {
        (self.function.borrow_mut())(t)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcStatefulMutatingFunction<T, R>,
        BoxStatefulMutatingFunction,
        BoxMutatingFunctionOnce,
        FnMut(input: &mut T) -> R
    );
}
