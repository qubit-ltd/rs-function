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
//! Defines the `BoxCallableOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxCallableOnce
// ============================================================================

/// Box-based one-time callable.
///
/// `BoxCallableOnce<R, E>` stores a `Box<dyn FnOnce() -> Result<R, E>>` and can
/// be executed only once. It is the boxed concrete implementation of
/// [`CallableOnce`].
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxCallableOnce, CallableOnce};
///
/// let task = BoxCallableOnce::new(|| Ok::<i32, String>(42));
/// assert_eq!(task.call(), Ok(42));
/// ```
///
pub struct BoxCallableOnce<R, E> {
    /// The one-time closure executed by this callable.
    pub(super) function: Box<dyn FnOnce() -> Result<R, E>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<R, E> BoxCallableOnce<R, E> {
    impl_common_new_methods!(
        (FnOnce() -> Result<R, E> + 'static),
        |function| Box::new(function),
        "callable"
    );

    /// Creates a boxed callable from a one-time supplier.
    ///
    /// This is an explicit bridge from `SupplierOnce<Result<R, E>>` to
    /// `CallableOnce<R, E>`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `BoxCallableOnce<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: SupplierOnce<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");

    /// Maps the success value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the success value.
    ///
    /// # Returns
    ///
    /// A new callable that applies `mapper` when this callable succeeds.
    #[inline]
    pub fn map<U, M>(self, mapper: M) -> BoxCallableOnce<U, E>
    where
        M: FnOnce(R) -> U + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallableOnce::new_with_optional_name(move || function().map(mapper), name)
    }

    /// Maps the error value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the error value.
    ///
    /// # Returns
    ///
    /// A new callable that applies `mapper` when this callable fails.
    #[inline]
    pub fn map_err<E2, M>(self, mapper: M) -> BoxCallableOnce<R, E2>
    where
        M: FnOnce(E) -> E2 + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallableOnce::new_with_optional_name(move || function().map_err(mapper), name)
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
    /// A new callable that runs `next` only when this callable succeeds.
    #[inline]
    pub fn and_then<U, N>(self, next: N) -> BoxCallableOnce<U, E>
    where
        N: FnOnce(R) -> Result<U, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallableOnce::new_with_optional_name(move || function().and_then(next), name)
    }
}

impl<R, E> CallableOnce<R, E> for BoxCallableOnce<R, E> {
    /// Executes the boxed callable.
    #[inline]
    fn call(self) -> Result<R, E> {
        (self.function)()
    }

    impl_box_once_conversions!(BoxCallableOnce<R, E>, CallableOnce, FnOnce() -> Result<R, E>);

    /// Converts this boxed callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnableOnce::new_with_optional_name(move || function().map(|_| ()), name)
    }
}

impl<R, E> SupplierOnce<Result<R, E>> for BoxCallableOnce<R, E> {
    /// Executes the boxed callable as a one-time supplier of `Result<R, E>`.
    #[inline]
    fn get(self) -> Result<R, E> {
        self.call()
    }
}

impl_closure_once_trait!(
    CallableOnce<R, E>,
    call,
    BoxCallableOnce,
    FnOnce() -> Result<R, E>
);

impl_function_debug_display!(BoxCallableOnce<R, E>);
