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
//! Defines the `BoxRunnable` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxRunnable
// ============================================================================

/// Box-based reusable runnable.
///
/// `BoxRunnable<E>` stores a `Box<dyn FnMut() -> Result<(), E>>` and can be
/// executed repeatedly. It is the boxed concrete implementation of
/// [`Runnable`].
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxRunnable, Runnable};
///
/// let mut task = BoxRunnable::new(|| Ok::<(), String>(()));
/// assert_eq!(task.run(), Ok(()));
/// ```
///
pub struct BoxRunnable<E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Box<dyn FnMut() -> Result<(), E>>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<E> BoxRunnable<E> {
    impl_common_new_methods!(
        (FnMut() -> Result<(), E> + 'static),
        |function| Box::new(function),
        "runnable"
    );

    /// Creates a boxed runnable from a reusable supplier.
    ///
    /// This is an explicit bridge from `Supplier<Result<(), E>>` to
    /// `Runnable<E>`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `BoxRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("runnable");

    /// Chains another runnable after this runnable succeeds.
    ///
    /// The second runnable is not executed if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `next` - The runnable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A runnable executing both actions in sequence.
    #[inline]
    pub fn and_then<N>(self, next: N) -> BoxRunnable<E>
    where
        N: Runnable<E> + 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxRunnable::new_with_optional_name(
            move || {
                function()?;
                next.run()
            },
            name,
        )
    }

    /// Runs this runnable before a callable.
    ///
    /// The callable is not executed if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `callable` - The callable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A callable producing the second computation's result.
    #[inline]
    pub fn then_callable<R, C>(self, callable: C) -> BoxCallable<R, E>
    where
        C: crate::tasks::callable::Callable<R, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut callable = callable;
        BoxCallable::new_with_optional_name(
            move || {
                function()?;
                callable.call()
            },
            name,
        )
    }
}

impl<E> Runnable<E> for BoxRunnable<E> {
    /// Executes the boxed runnable.
    #[inline]
    fn run(&mut self) -> Result<(), E> {
        (self.function)()
    }

    impl_box_conversions!(
        BoxRunnable<E>,
        RcRunnable,
        FnMut() -> Result<(), E>
    );

    /// Converts this boxed runnable into a boxed callable while preserving its
    /// name.
    #[inline]
    fn into_callable(self) -> BoxCallable<(), E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallable::new_with_optional_name(
            move || {
                function()?;
                Ok(())
            },
            name,
        )
    }
}

impl<E> SupplierOnce<Result<(), E>> for BoxRunnable<E> {
    /// Executes the boxed runnable as a one-time supplier of `Result<(), E>`.
    #[inline]
    fn get(mut self) -> Result<(), E> {
        self.run()
    }
}

impl_supplier_debug_display!(BoxRunnable<E>);
