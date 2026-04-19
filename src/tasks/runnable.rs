/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Runnable Types
//!
//! Provides fallible, one-time, zero-argument actions.
//!
//! A `Runnable<E>` is equivalent to `FnOnce() -> Result<(), E>`, but uses
//! task-oriented vocabulary. Use it when the operation's side effect matters
//! and only success or failure should be reported.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!
//! # Author
//!
//! Haixing Hu

use crate::{
    macros::{
        impl_box_once_conversions,
        impl_closure_once_trait,
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::macros::impl_supplier_debug_display,
    suppliers::supplier_once::SupplierOnce,
    tasks::callable::BoxCallable,
};

// ============================================================================
// Runnable Trait
// ============================================================================

/// A fallible one-time action.
///
/// `Runnable<E>` consumes itself and returns `Result<(), E>`. It is a semantic
/// specialization of `SupplierOnce<Result<(), E>>` for executable actions and
/// deferred side effects.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::Runnable;
///
/// let task = || Ok::<(), String>(());
/// assert_eq!(task.run(), Ok(()));
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait Runnable<E> {
    /// Executes the action, consuming `self`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run(self) -> Result<(), E>;

    /// Converts this runnable into a boxed runnable.
    ///
    /// # Returns
    ///
    /// A `BoxRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_box(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(move || self.run())
    }

    /// Converts this runnable into a closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnOnce() -> Result<(), E>`.
    fn into_fn(self) -> impl FnOnce() -> Result<(), E>
    where
        Self: Sized + 'static,
    {
        move || self.run()
    }

    /// Converts this runnable into a boxed runnable without consuming `self`.
    ///
    /// The method clones `self` and boxes the clone. Use this for cloneable
    /// runnable values that need to be reused after boxing.
    ///
    /// # Returns
    ///
    /// A new `BoxRunnable<E>` built from a clone of this runnable.
    fn to_box(&self) -> BoxRunnable<E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this runnable into a closure without consuming `self`.
    ///
    /// The method clones `self` and returns a one-time closure that executes
    /// the clone.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnOnce() -> Result<(), E>`.
    fn to_fn(&self) -> impl FnOnce() -> Result<(), E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this runnable into a callable returning unit.
    ///
    /// # Returns
    ///
    /// A `BoxCallable<(), E>` that executes this runnable and returns
    /// `Ok(())` on success.
    fn into_callable(self) -> BoxCallable<(), E>
    where
        Self: Sized + 'static,
    {
        BoxCallable::new(move || self.run())
    }
}

// ============================================================================
// BoxRunnable
// ============================================================================

/// Box-based one-time runnable.
///
/// `BoxRunnable<E>` stores a `Box<dyn FnOnce() -> Result<(), E>>` and can be
/// executed only once. It is the boxed concrete implementation of
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
/// let task = BoxRunnable::new(|| Ok::<(), String>(()));
/// assert_eq!(task.run(), Ok(()));
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxRunnable<E> {
    /// The one-time closure executed by this runnable.
    function: Box<dyn FnOnce() -> Result<(), E>>,
    /// The optional name of this runnable.
    name: Option<String>,
}

impl<E> BoxRunnable<E> {
    impl_common_new_methods!(
        (FnOnce() -> Result<(), E> + 'static),
        |function| Box::new(function),
        "runnable"
    );

    /// Creates a boxed runnable from a one-time supplier.
    ///
    /// This is an explicit bridge from `SupplierOnce<Result<(), E>>` to
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
        S: SupplierOnce<Result<(), E>> + 'static,
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
    pub fn and_then<N>(self, next: N) -> BoxRunnable<E>
    where
        N: Runnable<E> + 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
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
        let function = self.function;
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
    fn run(self) -> Result<(), E> {
        (self.function)()
    }

    impl_box_once_conversions!(BoxRunnable<E>, Runnable, FnOnce() -> Result<(), E>);

    /// Converts this boxed runnable into a boxed callable while preserving its
    /// name.
    #[inline]
    fn into_callable(self) -> BoxCallable<(), E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallable::new_with_optional_name(function, name)
    }
}

impl<E> SupplierOnce<Result<(), E>> for BoxRunnable<E> {
    /// Executes the boxed runnable as a one-time supplier of `Result<(), E>`.
    #[inline]
    fn get(self) -> Result<(), E> {
        self.run()
    }
}

impl_closure_once_trait!(
    Runnable<E>,
    run,
    BoxRunnable,
    FnOnce() -> Result<(), E>
);

impl_supplier_debug_display!(BoxRunnable<E>);
