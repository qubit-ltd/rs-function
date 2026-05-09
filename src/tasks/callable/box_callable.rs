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
//! Defines the `BoxCallable` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxCallable
// ============================================================================

/// Box-based callable.
///
/// `BoxCallable<R, E>` stores a `Box<dyn FnMut() -> Result<R, E>>` and can be
/// called repeatedly. It is the boxed concrete implementation of
/// [`Callable`].
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Example
///
/// ```rust
/// use qubit_function::{BoxCallable, Callable};
///
/// let mut task = BoxCallable::new(|| Ok::<i32, String>(42));
/// assert_eq!(task.call().expect("call should succeed"), 42);
/// ```
///
pub struct BoxCallable<R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Box<dyn FnMut() -> Result<R, E>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<R, E> BoxCallable<R, E> {
    impl_common_new_methods!(
        (FnMut() -> Result<R, E> + 'static),
        |function| Box::new(function),
        "callable"
    );

    /// Creates a boxed callable from a reusable supplier.
    ///
    /// This is an explicit bridge from `Supplier<Result<R, E>>` to
    /// `Callable<R, E>`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `BoxCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + 'static,
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
    pub fn map<U, M>(self, mut mapper: M) -> BoxCallable<U, E>
    where
        M: FnMut(R) -> U + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallable::new_with_optional_name(move || function().map(&mut mapper), name)
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
    pub fn map_err<E2, M>(self, mut mapper: M) -> BoxCallable<R, E2>
    where
        M: FnMut(E) -> E2 + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallable::new_with_optional_name(move || function().map_err(&mut mapper), name)
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
    pub fn and_then<U, N>(self, next: N) -> BoxCallable<U, E>
    where
        N: FnMut(R) -> Result<U, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxCallable::new_with_optional_name(
            move || {
                let value = function()?;
                next(value)
            },
            name,
        )
    }
}

impl<R, E> Callable<R, E> for BoxCallable<R, E> {
    /// Executes the boxed callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function)()
    }

    impl_box_conversions!(
        BoxCallable<R, E>,
        RcCallable,
        FnMut() -> Result<R, E>
    );

    /// Converts this boxed callable into a local boxed one-time callable while
    /// preserving its name.
    #[inline]
    fn into_local_once(self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(function, name)
    }

    /// Converts this boxed callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxRunnable::new_with_optional_name(move || function().map(|_| ()), name)
    }
}
