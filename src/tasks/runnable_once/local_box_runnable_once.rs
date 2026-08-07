// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `LocalBoxRunnableOnce` public type.

use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;
use crate::suppliers::macros::impl_supplier_debug_display;
use crate::suppliers::supplier_once::SupplierOnce;
use crate::tasks::callable_once::CallableOnce;
use crate::tasks::callable_once::LocalBoxCallableOnce;
use crate::tasks::runnable_once::RunnableOnce;

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
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct LocalBoxRunnableOnce<E> {
    /// The one-time closure executed by this runnable.
    pub(super) function: Box<dyn FnOnce() -> Result<(), E>>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<E> LocalBoxRunnableOnce<E> {
    impl_common_new_methods!(
        semantic(RunnableOnce<E> + 'static),
        |source| move || source.run_once(),
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
        let function = self.function;
        LocalBoxRunnableOnce::new(move || {
            function()?;
            next.run_once()
        })
    }

    /// Runs this runnable before a local callable.
    ///
    /// The callable is not executed if this runnable returns `Err`.
    /// Because this operation sequences two independent callbacks, the
    /// returned callable is unnamed.
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
        let function = self.function;
        LocalBoxCallableOnce::new(move || {
            function()?;
            callable.call_once()
        })
    }
}

impl<E> RunnableOnce<E> for LocalBoxRunnableOnce<E> {
    /// Executes the local boxed runnable.
    #[inline(always)]
    fn run_once(self) -> Result<(), E> {
        (self.function)()
    }
}

impl<E> SupplierOnce<Result<(), E>> for LocalBoxRunnableOnce<E> {
    /// Executes the local boxed runnable as a one-time supplier of
    /// `Result<(), E>`.
    #[inline(always)]
    fn get(self) -> Result<(), E> {
        self.run_once()
    }
}

impl_supplier_debug_display!(LocalBoxRunnableOnce<E>);
