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
//! Defines the `LocalBoxRunnableOnce` public type.

use crate::{
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::{
        macros::impl_supplier_debug_display,
        supplier_once::SupplierOnce,
    },
    tasks::{
        callable_once::{
            CallableOnce,
            LocalBoxCallableOnce,
        },
        runnable_once::RunnableOnce,
    },
};

// ============================================================================
// LocalBoxRunnableOnce
// ============================================================================

/// Local box-based one-time runnable.
///
/// `LocalBoxRunnableOnce<E>` stores a `Box<dyn FnOnce() -> Result<(), E>>` and
/// can be executed only once on the local thread. Use
/// [`BoxRunnableOnce`](crate::tasks::runnable_once::BoxRunnableOnce) when the
/// runnable must be movable across threads.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
pub struct LocalBoxRunnableOnce<E> {
    /// The one-time closure executed by this runnable.
    pub(super) function: Box<dyn FnOnce() -> Result<(), E>>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<E> LocalBoxRunnableOnce<E> {
    impl_common_new_methods!(
        (FnOnce() -> Result<(), E> + 'static),
        |function| Box::new(function),
        "local runnable"
    );

    /// Creates a local boxed runnable from a one-time supplier.
    ///
    /// This is an explicit bridge from `SupplierOnce<Result<(), E>>` to
    /// `RunnableOnce<E>` without requiring `Send`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `LocalBoxRunnableOnce<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: SupplierOnce<Result<(), E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("local runnable");

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
    /// A new local runnable executing both actions in sequence.
    #[inline]
    pub fn and_then<N>(self, next: N) -> LocalBoxRunnableOnce<E>
    where
        N: RunnableOnce<E> + 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxRunnableOnce::new_with_optional_name(
            move || {
                function()?;
                next.run()
            },
            name,
        )
    }

    /// Runs this runnable before a local callable.
    ///
    /// The callable is not executed if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `callable` - The callable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A local callable producing the second computation's result.
    #[inline]
    pub fn then_callable<R, C>(self, callable: C) -> LocalBoxCallableOnce<R, E>
    where
        C: CallableOnce<R, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        LocalBoxCallableOnce::new_with_optional_name(
            move || {
                function()?;
                callable.call()
            },
            name,
        )
    }
}

impl<E> RunnableOnce<E> for LocalBoxRunnableOnce<E> {
    /// Executes the local boxed runnable.
    #[inline]
    fn run(self) -> Result<(), E> {
        (self.function)()
    }

    /// Converts this local boxed runnable into itself.
    #[inline]
    fn into_local_box(self) -> LocalBoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        self
    }

    /// Extracts the underlying local one-time closure.
    #[inline]
    fn into_fn(self) -> impl FnOnce() -> Result<(), E>
    where
        Self: Sized + 'static,
    {
        self.function
    }

    /// Converts this local boxed runnable into a local boxed callable while
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

impl<E> SupplierOnce<Result<(), E>> for LocalBoxRunnableOnce<E> {
    /// Executes the local boxed runnable as a one-time supplier of
    /// `Result<(), E>`.
    #[inline]
    fn get(self) -> Result<(), E> {
        self.run()
    }
}

impl_supplier_debug_display!(LocalBoxRunnableOnce<E>);
