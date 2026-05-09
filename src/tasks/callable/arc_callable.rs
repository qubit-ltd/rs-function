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
//! Defines the `ArcCallable` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcCallable
// ============================================================================

/// Thread-safe callable.
///
/// `ArcCallable<R, E>` stores a `Arc<Mutex<dyn FnMut() -> Result<R, E> + Send>>`
/// and can be called repeatedly across threads.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
pub struct ArcCallable<R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Arc<Mutex<dyn FnMut() -> Result<R, E> + Send>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<R, E> Clone for ArcCallable<R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<R, E> ArcCallable<R, E> {
    impl_common_new_methods!(
        (FnMut() -> Result<R, E> + Send + 'static),
        |function| Arc::new(Mutex::new(function)),
        "callable"
    );

    /// Creates an `ArcCallable` from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `ArcCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + Send + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");
}

impl<R, E> Callable<R, E> for ArcCallable<R, E> {
    /// Executes the thread-safe callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function.lock())()
    }

    impl_arc_conversions!(
        ArcCallable<R, E>,
        BoxCallable,
        RcCallable,
        BoxCallableOnce,
        FnMut() -> Result<R, E>
    );

    /// Converts this shared callable into a local boxed one-time callable while
    /// preserving its name.
    #[inline]
    fn into_local_once(self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(move || (function.lock())(), name)
    }

    /// Converts this shared callable into a local boxed one-time callable
    /// without consuming `self`.
    #[inline]
    fn to_local_once(&self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_local_once()
    }

    /// Converts this shared callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnable::new_with_optional_name(move || (function.lock())().map(|_| ()), name)
    }
}

impl_closure_trait!(
    Callable<R, E>,
    call,
    FnMut() -> Result<R, E>
);

impl_function_debug_display!(BoxCallable<R, E>);
impl_function_debug_display!(RcCallable<R, E>);
impl_function_debug_display!(ArcCallable<R, E>);
