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
//! Defines the `RcCallable` public type.

use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
        impl_rc_conversions,
    },
    suppliers::supplier::Supplier,
    tasks::{
        callable::{
            BoxCallable,
            Callable,
        },
        callable_once::LocalBoxCallableOnce,
        runnable::BoxRunnable,
    },
};

// ============================================================================
// RcCallable
// ============================================================================

/// Single-threaded shared callable.
///
/// `RcCallable<R, E>` stores a `Rc<RefCell<dyn FnMut() -> Result<R, E>>>` and
/// can be called repeatedly through shared ownership.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
pub struct RcCallable<R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Rc<RefCell<dyn FnMut() -> Result<R, E>>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<R, E> Clone for RcCallable<R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<R, E> RcCallable<R, E> {
    impl_common_new_methods!(
        (FnMut() -> Result<R, E> + 'static),
        |function| Rc::new(RefCell::new(function)),
        "callable"
    );

    /// Creates an `RcCallable` from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `RcCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");
}

impl<R, E> Callable<R, E> for RcCallable<R, E> {
    /// Executes the shared callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function.borrow_mut())()
    }

    impl_rc_conversions!(
        RcCallable<R, E>,
        BoxCallable,
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
        LocalBoxCallableOnce::new_with_optional_name(move || (function.borrow_mut())(), name)
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
        BoxRunnable::new_with_optional_name(move || (function.borrow_mut())().map(|_| ()), name)
    }
}
