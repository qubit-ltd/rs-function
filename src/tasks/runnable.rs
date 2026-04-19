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

use std::fmt;

use crate::{
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
    /// Creates a new boxed runnable.
    ///
    /// # Parameters
    ///
    /// * `function` - The one-time closure executed by this runnable.
    ///
    /// # Returns
    ///
    /// A new unnamed `BoxRunnable<E>`.
    #[inline]
    pub fn new<F>(function: F) -> Self
    where
        F: FnOnce() -> Result<(), E> + 'static,
    {
        Self {
            function: Box::new(function),
            name: None,
        }
    }

    /// Creates a new named boxed runnable.
    ///
    /// # Parameters
    ///
    /// * `name` - Name used by `Debug` and `Display`.
    /// * `function` - The one-time closure executed by this runnable.
    ///
    /// # Returns
    ///
    /// A new named `BoxRunnable<E>`.
    #[inline]
    pub fn new_with_name<F>(name: &str, function: F) -> Self
    where
        F: FnOnce() -> Result<(), E> + 'static,
    {
        Self {
            function: Box::new(function),
            name: Some(name.to_string()),
        }
    }

    /// Creates a new boxed runnable with an optional name.
    ///
    /// # Parameters
    ///
    /// * `function` - The one-time closure executed by this runnable.
    /// * `name` - Optional name used by `Debug` and `Display`.
    ///
    /// # Returns
    ///
    /// A new `BoxRunnable<E>`.
    #[inline]
    pub fn new_with_optional_name<F>(function: F, name: Option<String>) -> Self
    where
        F: FnOnce() -> Result<(), E> + 'static,
    {
        Self {
            function: Box::new(function),
            name,
        }
    }

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

    /// Gets the optional runnable name.
    ///
    /// # Returns
    ///
    /// Returns `Some(&str)` if a name was set, or `None` otherwise.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the runnable name.
    ///
    /// # Parameters
    ///
    /// * `name` - The new name.
    #[inline]
    pub fn set_name(&mut self, name: &str) {
        if self.name.as_deref() != Some(name) {
            self.name = Some(name.to_owned());
        }
    }

    /// Clears the runnable name.
    #[inline]
    pub fn clear_name(&mut self) {
        self.name = None;
    }

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

    /// Returns this boxed runnable without re-boxing it.
    #[inline]
    fn into_box(self) -> BoxRunnable<E> {
        self
    }

    /// Extracts the underlying one-time closure.
    #[inline]
    fn into_fn(self) -> impl FnOnce() -> Result<(), E> {
        self.function
    }

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

impl<F, E> Runnable<E> for F
where
    F: FnOnce() -> Result<(), E>,
{
    /// Executes the closure as a runnable.
    #[inline]
    fn run(self) -> Result<(), E> {
        self()
    }

    /// Converts the closure to a boxed runnable.
    #[inline]
    fn into_box(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(self)
    }

    /// Returns the closure unchanged.
    #[inline]
    fn into_fn(self) -> impl FnOnce() -> Result<(), E>
    where
        Self: Sized + 'static,
    {
        self
    }
}

impl<E> fmt::Debug for BoxRunnable<E> {
    /// Formats this boxed runnable for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxRunnable")
            .field("name", &self.name)
            .field("function", &"<function>")
            .finish()
    }
}

impl<E> fmt::Display for BoxRunnable<E> {
    /// Formats this boxed runnable for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "BoxRunnable({name})"),
            None => write!(f, "BoxRunnable"),
        }
    }
}
