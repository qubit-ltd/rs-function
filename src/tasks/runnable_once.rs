/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Runnable Once Types
//!
//! Provides fallible, one-time, zero-argument actions.
//!
//! A `RunnableOnce<E>` is equivalent to `FnOnce() -> Result<(), E>`, but uses
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
    tasks::callable_once::{
        BoxCallableOnce,
        CallableOnce,
    },
};

// ============================================================================
// RunnableOnce Trait
// ============================================================================

/// A fallible one-time action.
///
/// Conceptually this matches `FnOnce() -> Result<(), E>`: `run` consumes `self`
/// and returns `Result<(), E>`, but the surface uses task-oriented naming and
/// helpers instead of closure types. It is a semantic specialization of
/// `SupplierOnce<Result<(), E>>` for executable actions and deferred side effects.
///
/// Choose **`RunnableOnce`** when only success or failure matters; the success
/// type is `()`. When callers need the success value `R`, use
/// [`CallableOnce`].
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RunnableOnce, BoxRunnableOnce};
///
/// let task = || Ok::<(), String>(());
/// assert_eq!(task.run(), Ok(()));
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait RunnableOnce<E> {
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
    /// A `BoxRunnableOnce<E>` that executes this runnable when `run()` is
    /// invoked.
    fn into_box(self) -> BoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnableOnce::new(move || self.run())
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
    /// A new `BoxRunnableOnce<E>` built from a clone of this runnable.
    fn to_box(&self) -> BoxRunnableOnce<E>
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
    /// A `BoxCallableOnce<(), E>` that executes this runnable and returns
    /// `Ok(())` on success.
    fn into_callable(self) -> BoxCallableOnce<(), E>
    where
        Self: Sized + 'static,
    {
        BoxCallableOnce::new(move || self.run())
    }
}

// ============================================================================
// BoxRunnableOnce
// ============================================================================

/// Box-based one-time runnable.
///
/// `BoxRunnableOnce<E>` stores a `Box<dyn FnOnce() -> Result<(), E>>` and can be
/// executed only once. It is the boxed concrete implementation of
/// [`RunnableOnce`].
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
/// # Author
///
/// Haixing Hu
pub struct BoxRunnableOnce<E> {
    /// The one-time closure executed by this runnable.
    function: Box<dyn FnOnce() -> Result<(), E>>,
    /// The optional name of this runnable.
    name: Option<String>,
}

impl<E> BoxRunnableOnce<E> {
    impl_common_new_methods!(
        (FnOnce() -> Result<(), E> + 'static),
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
    pub fn and_then<N>(self, next: N) -> BoxRunnableOnce<E>
    where
        N: RunnableOnce<E> + 'static,
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
        C: CallableOnce<R, E> + 'static,
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
}

impl<E> SupplierOnce<Result<(), E>> for BoxRunnableOnce<E> {
    /// Executes the boxed runnable as a one-time supplier of `Result<(), E>`.
    #[inline]
    fn get(self) -> Result<(), E> {
        self.run()
    }
}

impl_closure_once_trait!(
    RunnableOnce<E>,
    run,
    BoxRunnableOnce,
    FnOnce() -> Result<(), E>
);

impl_supplier_debug_display!(BoxRunnableOnce<E>);
