// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `LocalBoxRunnable` public type.

use crate::macros::impl_common_new_methods;
use crate::suppliers::macros::impl_supplier_debug_display;
use crate::suppliers::supplier::Supplier;
#[cfg(feature = "once")]
use crate::suppliers::supplier_once::SupplierOnce;
use crate::tasks::callable::Callable;
use crate::tasks::callable::LocalBoxCallable;
use crate::tasks::runnable::Runnable;

/// Local box-based reusable runnable.
///
/// `LocalBoxRunnable<E>` stores a `Box<dyn FnMut() -> Result<(), E>>` and can
/// be executed repeatedly on the local thread. Use
/// [`BoxRunnable`](crate::tasks::runnable::BoxRunnable) when the runnable must
/// be movable across threads.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{LocalBoxRunnable, Runnable};
///
/// let mut task = LocalBoxRunnable::new(|| Ok::<(), String>(()));
/// assert_eq!(task.run(), Ok(()));
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct LocalBoxRunnable<E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Box<dyn FnMut() -> Result<(), E>>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<E> LocalBoxRunnable<E> {
    impl_common_new_methods!(
        semantic_mut(Runnable<E> + 'static),
        |source| move || source.run(),
        |function| Box::new(function),
        "local runnable"
    );

    /// Creates a local boxed runnable from a reusable supplier.
    ///
    /// This adapts `Supplier<Result<(), E>>` to `Runnable<E>` without requiring
    /// the supplier to be `Send`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `LocalBoxRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    /// Returns the diagnostic name assigned to this runnable, if any.
    ///
    /// # Returns
    ///
    /// Returns `Some(name)` when a name is set, or `None` otherwise.
    #[inline(always)]
    pub fn name(&self) -> Option<&str> {
        self.metadata.name()
    }

    /// Sets the diagnostic name used when displaying this runnable.
    ///
    /// # Parameters
    ///
    /// * `name` - The diagnostic name to assign.
    #[inline(always)]
    pub fn set_name(&mut self, name: &str) {
        self.metadata.set_name(name);
    }

    /// Sets the diagnostic name and returns this runnable for further setup.
    ///
    /// # Parameters
    ///
    /// * `name` - The diagnostic name to assign.
    ///
    /// # Returns
    ///
    /// This runnable with the supplied name.
    #[inline(always)]
    pub fn with_name(mut self, name: &str) -> Self {
        self.metadata.set_name(name);
        self
    }

    /// Removes the diagnostic name assigned to this runnable.
    #[inline(always)]
    pub fn clear_name(&mut self) {
        self.metadata.clear_name();
    }

    /// Chains another runnable after this runnable succeeds.
    ///
    /// The next runnable is skipped if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `next` - The runnable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A local runnable executing both actions in sequence.
    #[inline]
    pub fn and_then<N>(self, mut next: N) -> LocalBoxRunnable<E>
    where
        N: Runnable<E> + 'static,
        E: 'static,
    {
        let mut function = self.function;
        LocalBoxRunnable::new(move || {
            function()?;
            next.run()
        })
    }

    /// Runs this runnable before a local callable.
    ///
    /// The callable is skipped if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `callable` - The callable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A local callable producing the second computation's result.
    #[inline]
    pub fn then_callable<R, C>(self, mut callable: C) -> LocalBoxCallable<R, E>
    where
        C: Callable<R, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let mut function = self.function;
        LocalBoxCallable::new(move || {
            function()?;
            callable.call()
        })
    }
}

impl<E> Runnable<E> for LocalBoxRunnable<E> {
    /// Executes the stored callback on the local thread.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the callback succeeds, or propagates its `Err(E)`.
    ///
    /// # Errors
    ///
    /// Returns the error produced by the stored callback.
    #[inline(always)]
    fn run(&mut self) -> Result<(), E> {
        (self.function)()
    }
}

#[cfg(feature = "once")]
impl<E> SupplierOnce<Result<(), E>> for LocalBoxRunnable<E> {
    /// Executes the local boxed runnable as a one-time supplier.
    ///
    /// # Returns
    ///
    /// Returns the callback's `Ok(())` or propagates its `Err(E)`.
    ///
    /// # Errors
    ///
    /// Returns the error produced by the stored callback.
    #[inline(always)]
    fn get(mut self) -> Result<(), E> {
        self.run()
    }
}

impl_supplier_debug_display!(LocalBoxRunnable<E>);
