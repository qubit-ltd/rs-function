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
//! Defines the `LocalBoxCallableOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// LocalBoxCallableOnce
// ============================================================================

/// Local box-based one-time callable.
///
/// `LocalBoxCallableOnce<R, E>` stores a `Box<dyn FnOnce() -> Result<R, E>>`
/// and can be executed only once on the local thread. Use [`BoxCallableOnce`]
/// when the callable must be movable across threads.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
pub struct LocalBoxCallableOnce<R, E> {
    /// The one-time closure executed by this callable.
    pub(super) function: Box<dyn FnOnce() -> Result<R, E>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<R, E> LocalBoxCallableOnce<R, E> {
    impl_common_new_methods!(
        (FnOnce() -> Result<R, E> + 'static),
        |function| Box::new(function),
        "local callable"
    );

    /// Creates a local boxed callable from a one-time supplier.
    ///
    /// This is an explicit bridge from `SupplierOnce<Result<R, E>>` to
    /// `CallableOnce<R, E>` without requiring `Send`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `LocalBoxCallableOnce<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: SupplierOnce<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("local callable");

    /// Maps the success value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the success value.
    ///
    /// # Returns
    ///
    /// A new local callable that applies `mapper` when this callable succeeds.
    #[inline]
    pub fn map<U, M>(self, mapper: M) -> LocalBoxCallableOnce<U, E>
    where
        M: FnOnce(R) -> U + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(move || function().map(mapper), name)
    }

    /// Maps the error value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the error value.
    ///
    /// # Returns
    ///
    /// A new local callable that applies `mapper` when this callable fails.
    #[inline]
    pub fn map_err<E2, M>(self, mapper: M) -> LocalBoxCallableOnce<R, E2>
    where
        M: FnOnce(E) -> E2 + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(move || function().map_err(mapper), name)
    }

    /// Chains another fallible computation after this callable succeeds.
    ///
    /// # Parameters
    ///
    /// * `next` - Function that receives the success value and returns the next
    ///   result.
    ///
    /// # Returns
    ///
    /// A new local callable that runs `next` only when this callable succeeds.
    #[inline]
    pub fn and_then<U, N>(self, next: N) -> LocalBoxCallableOnce<U, E>
    where
        N: FnOnce(R) -> Result<U, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(move || function().and_then(next), name)
    }
}

impl<R, E> CallableOnce<R, E> for LocalBoxCallableOnce<R, E> {
    /// Executes the local boxed callable.
    #[inline]
    fn call(self) -> Result<R, E> {
        (self.function)()
    }

    /// Converts this local boxed callable into itself.
    #[inline]
    fn into_local_box(self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Sized + 'static,
    {
        self
    }

    /// Extracts the underlying local one-time closure.
    #[inline]
    fn into_fn(self) -> impl FnOnce() -> Result<R, E>
    where
        Self: Sized + 'static,
    {
        self.function
    }

    /// Converts this local boxed callable into a local boxed runnable while
    /// preserving its name.
    #[inline]
    fn into_local_runnable(self) -> LocalBoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxRunnableOnce::new_with_optional_name(move || function().map(|_| ()), name)
    }
}

impl<R, E> SupplierOnce<Result<R, E>> for LocalBoxCallableOnce<R, E> {
    /// Executes the local boxed callable as a one-time supplier of
    /// `Result<R, E>`.
    #[inline]
    fn get(self) -> Result<R, E> {
        self.call()
    }
}

impl_function_debug_display!(LocalBoxCallableOnce<R, E>);
