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
//! Defines the `ArcStatefulFunction` public type.

use super::{
    Arc,
    ArcConditionalStatefulFunction,
    BoxFunctionOnce,
    BoxStatefulFunction,
    Mutex,
    Predicate,
    RcStatefulFunction,
    StatefulFunction,
    impl_arc_conversions,
    impl_function_clone,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
    impl_function_identity_method,
    impl_shared_function_methods,
};

// ============================================================================
// ArcStatefulFunction - Arc<Mutex<dyn FnMut(&T) -> R + Send>>
// ============================================================================

/// ArcStatefulFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads
/// while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Arc<Mutex<dyn FnMut(&T) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Thread-safe (`Send` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
pub struct ArcStatefulFunction<T, R> {
    pub(super) function: ArcStatefulFn<T, R>,
    pub(super) name: Option<String>,
}

type ArcStatefulFn<T, R> = Arc<Mutex<dyn FnMut(&T) -> R + Send + 'static>>;

impl<T, R> ArcStatefulFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcStatefulFunction<T, R>,
        (FnMut(&T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcStatefulFunction<T, R>,
        ArcConditionalStatefulFunction,
        into_arc,
        StatefulFunction,
        Send + Sync + 'static
    );
}

// Generates: constant() method for ArcStatefulFunction<T, R>
impl_function_constant_method!(ArcStatefulFunction<T, R>, Send + Sync + 'static);

// Generates: identity() method for ArcStatefulFunction<T, T>
impl_function_identity_method!(ArcStatefulFunction<T, T>);

// Generates: Clone implementation for ArcStatefulFunction<T, R>
impl_function_clone!(ArcStatefulFunction<T, R>);

// Generates: Debug and Display implementations for ArcStatefulFunction<T, R>
impl_function_debug_display!(ArcStatefulFunction<T, R>);

// Implement StatefulFunction trait for ArcStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for ArcStatefulFunction<T, R> {
    fn apply(&mut self, t: &T) -> R {
        (self.function.lock())(t)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulFunction<T, R>,
        BoxStatefulFunction,
        RcStatefulFunction,
        BoxFunctionOnce,
        FnMut(t: &T) -> R
    );
}

// ============================================================================
// Blanket implementation for standard FnMut trait
// ============================================================================

/// Implement StatefulFunction<T, R> for any type that implements FnMut(&T) -> R
///
/// This allows closures to be used directly with our StatefulFunction trait
/// without wrapping.
///
/// # Examples
///
/// ```rust
/// use qubit_function::StatefulFunction;
///
/// let mut counter = 0;
/// let mut function = |x: &i32| {
///     counter += 1;
///     *x + counter
/// };
///
/// assert_eq!(function.apply(&10), 11);
/// assert_eq!(function.apply(&10), 12);
/// ```
///
impl<F, T, R> StatefulFunction<T, R> for F
where
    F: FnMut(&T) -> R,
{
    fn apply(&mut self, t: &T) -> R {
        self(t)
    }

    fn into_box(self) -> BoxStatefulFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxStatefulFunction::new(self)
    }

    fn into_rc(self) -> RcStatefulFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcStatefulFunction::new(self)
    }

    fn into_arc(self) -> ArcStatefulFunction<T, R>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulFunction::new(self)
    }

    fn into_fn(self) -> impl FnMut(&T) -> R
    where
        Self: Sized + 'static,
    {
        self
    }

    fn to_box(&self) -> BoxStatefulFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    fn to_rc(&self) -> RcStatefulFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    fn to_arc(&self) -> ArcStatefulFunction<T, R>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    fn to_fn(&self) -> impl FnMut(&T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone()
    }

    fn into_once(self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxFunctionOnce::new(self)
    }

    fn to_once(&self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        BoxFunctionOnce::new(self.clone())
    }
}
