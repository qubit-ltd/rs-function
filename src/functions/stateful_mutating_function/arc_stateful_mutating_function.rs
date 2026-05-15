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
//! Defines the `ArcStatefulMutatingFunction` public type.

use super::{
    Arc,
    ArcConditionalStatefulMutatingFunction,
    ArcStatefulMutatingFunctionFn,
    BoxMutatingFunctionOnce,
    BoxStatefulMutatingFunction,
    Function,
    Mutex,
    Predicate,
    RcStatefulMutatingFunction,
    StatefulMutatingFunction,
    impl_arc_conversions,
    impl_function_clone,
    impl_function_common_methods,
    impl_function_debug_display,
    impl_function_identity_method,
    impl_shared_function_methods,
};

// =======================================================================
// 5. ArcStatefulMutatingFunction - Thread-Safe Shared Ownership
// =======================================================================

/// ArcStatefulMutatingFunction struct
///
/// A stateful mutating function implementation based on
/// `Arc<Mutex<dyn FnMut(&mut T) -> R + Send>>` for thread-safe shared
/// ownership scenarios. This type allows the function to be safely shared
/// and used across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcStatefulMutatingFunction` when:
/// - The function needs to be shared across multiple threads for stateful
///   operations
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       ArcStatefulMutatingFunction};
///
/// let counter = {
///     let mut count = 0;
///     ArcStatefulMutatingFunction::new(move |x: &mut i32| {
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
pub struct ArcStatefulMutatingFunction<T, R> {
    pub(super) function: ArcStatefulMutatingFunctionFn<T, R>,
    pub(super) name: Option<String>,
}

impl<T, R> ArcStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcStatefulMutatingFunction<T, R>,
        ArcConditionalStatefulMutatingFunction,
        into_arc,
        Function,  // chains a non-mutating function after this mutating function
        Send + Sync + 'static
    );
}

// Generates: Clone implementation for ArcStatefulMutatingFunction<T, R>
impl_function_clone!(ArcStatefulMutatingFunction<T, R>);

// Generates: Debug and Display implementations for ArcStatefulMutatingFunction<T, R>
impl_function_debug_display!(ArcStatefulMutatingFunction<T, R>);

// Generates: identity() method for ArcStatefulMutatingFunction<T, T>
impl_function_identity_method!(ArcStatefulMutatingFunction<T, T>, mutating);

// Implement StatefulMutatingFunction trait for ArcStatefulMutatingFunction<T, R>
impl<T, R> StatefulMutatingFunction<T, R> for ArcStatefulMutatingFunction<T, R> {
    fn apply(&mut self, t: &mut T) -> R {
        (self.function.lock())(t)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulMutatingFunction<T, R>,
        BoxStatefulMutatingFunction,
        RcStatefulMutatingFunction,
        BoxMutatingFunctionOnce,
        FnMut(input: &mut T) -> R
    );
}

// =======================================================================
// 6. Implement StatefulMutatingFunction trait for closures
// =======================================================================

impl<T, R, F> StatefulMutatingFunction<T, R> for F
where
    F: FnMut(&mut T) -> R,
{
    fn apply(&mut self, input: &mut T) -> R {
        self(input)
    }

    fn into_box(self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxStatefulMutatingFunction::new(self)
    }

    fn into_rc(self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcStatefulMutatingFunction::new(self)
    }

    fn into_arc(self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulMutatingFunction::new(self)
    }

    fn into_fn(self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + 'static,
    {
        self
    }

    fn to_box(&self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        let cloned = self.clone();
        BoxStatefulMutatingFunction::new(cloned)
    }

    fn to_rc(&self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        let cloned = self.clone();
        RcStatefulMutatingFunction::new(cloned)
    }

    fn to_arc(&self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + 'static,
    {
        let cloned = self.clone();
        ArcStatefulMutatingFunction::new(cloned)
    }

    fn to_fn(&self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone()
    }

    fn into_once(self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxMutatingFunctionOnce::new(self)
    }

    fn to_once(&self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        BoxMutatingFunctionOnce::new(self.clone())
    }
}
