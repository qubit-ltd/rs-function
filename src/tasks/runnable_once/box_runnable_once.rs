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
//! Defines the `BoxRunnableOnce` public type.

use crate::{
    macros::{
        impl_box_once_conversions,
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::{
        macros::impl_supplier_debug_display,
        supplier_once::SupplierOnce,
    },
    tasks::{
        callable_once::{
            BoxCallableOnce,
            CallableOnce,
            LocalBoxCallableOnce,
        },
        runnable_once::{
            LocalBoxRunnableOnce,
            RunnableOnce,
        },
    },
};

// ============================================================================
// BoxRunnableOnce
// ============================================================================

/// Box-based one-time runnable.
///
/// `BoxRunnableOnce<E>` stores a
/// `Box<dyn FnOnce() -> Result<(), E> + Send>` and can be executed only once.
/// It is the boxed concrete implementation of [`RunnableOnce`] for task
/// objects that may be moved across threads.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxRunnableOnce, RunnableOnce};
///
/// let task = BoxRunnableOnce::new(|| Ok::<(), String>(()));
/// assert_eq!(task.run(), Ok(()));
/// ```
///
pub struct BoxRunnableOnce<E> {
    /// The one-time closure executed by this runnable.
    pub(super) function: Box<dyn FnOnce() -> Result<(), E> + Send>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<E> BoxRunnableOnce<E> {
    impl_common_new_methods!(
        (FnOnce() -> Result<(), E> + Send + 'static),
        |function| Box::new(function),
        "runnable"
    );

    /// Creates a boxed runnable from a one-time supplier.
    ///
    /// This is an explicit bridge from `SupplierOnce<Result<(), E>>` to
    /// `RunnableOnce<E>`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `BoxRunnableOnce<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: SupplierOnce<Result<(), E>> + Send + 'static,
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
    /// A new runnable executing both actions in sequence.
    #[inline]
    pub fn and_then<N>(self, next: N) -> BoxRunnableOnce<E>
    where
        N: RunnableOnce<E> + Send + 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnableOnce::new_with_optional_name(
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
    pub fn then_callable<R, C>(self, callable: C) -> BoxCallableOnce<R, E>
    where
        C: CallableOnce<R, E> + Send + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallableOnce::new_with_optional_name(
            move || {
                function()?;
                callable.call()
            },
            name,
        )
    }
}

impl<E> RunnableOnce<E> for BoxRunnableOnce<E> {
    /// Executes the boxed runnable.
    #[inline]
    fn run(self) -> Result<(), E> {
        (self.function)()
    }

    impl_box_once_conversions!(BoxRunnableOnce<E>, RunnableOnce, FnOnce() -> Result<(), E>);

    /// Converts this boxed runnable into a boxed callable while preserving its
    /// name.
    #[inline]
    fn into_callable(self) -> BoxCallableOnce<(), E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallableOnce::new_with_optional_name(function, name)
    }

    /// Converts this boxed runnable into a local boxed runnable while
    /// preserving its name.
    #[inline]
    fn into_local_box(self) -> LocalBoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxRunnableOnce::new_with_optional_name(function, name)
    }

    /// Converts this boxed runnable into a local boxed callable while
    /// preserving its name.
    #[inline]
    fn into_local_callable(self) -> LocalBoxCallableOnce<(), E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(function, name)
    }
}

impl<E> SupplierOnce<Result<(), E>> for BoxRunnableOnce<E> {
    /// Executes the boxed runnable as a one-time supplier of `Result<(), E>`.
    #[inline]
    fn get(self) -> Result<(), E> {
        self.run()
    }
}

impl<F, E> RunnableOnce<E> for F
where
    F: FnOnce() -> Result<(), E>,
{
    /// Executes the closure as a one-time runnable.
    #[inline]
    fn run(self) -> Result<(), E> {
        self()
    }
}

impl_supplier_debug_display!(BoxRunnableOnce<E>);
