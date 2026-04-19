/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Callable Types
//!
//! Provides fallible, one-time, zero-argument computations.
//!
//! A `Callable<R, E>` is equivalent to `FnOnce() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `Runnable<E>` when the operation only needs
//! to report success or failure.
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
    tasks::runnable::BoxRunnable,
};

// ============================================================================
// Callable Trait
// ============================================================================

/// A fallible one-time computation.
///
/// `Callable<R, E>` consumes itself and returns `Result<R, E>`. It is a
/// semantic specialization of `SupplierOnce<Result<R, E>>` for executable
/// computations and deferred tasks.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::Callable;
///
/// let task = || Ok::<i32, String>(21 * 2);
/// assert_eq!(task.call(), Ok(42));
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait Callable<R, E> {
    /// Executes the computation, consuming `self`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    fn call(self) -> Result<R, E>;

    /// Converts this callable into a boxed callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallable<R, E>` that executes this callable when `call()` is
    /// invoked.
    fn into_box(self) -> BoxCallable<R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallable::new(move || self.call())
    }

    /// Converts this callable into a closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnOnce() -> Result<R, E>`.
    fn into_fn(self) -> impl FnOnce() -> Result<R, E>
    where
        Self: Sized + 'static,
    {
        move || self.call()
    }

    /// Converts this callable into a boxed callable without consuming `self`.
    ///
    /// The method clones `self` and boxes the clone. Use this for cloneable
    /// callable values that need to be reused after boxing.
    ///
    /// # Returns
    ///
    /// A new `BoxCallable<R, E>` built from a clone of this callable.
    fn to_box(&self) -> BoxCallable<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this callable into a closure without consuming `self`.
    ///
    /// The method clones `self` and returns a one-time closure that executes
    /// the clone.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnOnce() -> Result<R, E>`.
    fn to_fn(&self) -> impl FnOnce() -> Result<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this callable into a runnable by discarding the success value.
    ///
    /// The returned runnable preserves errors and maps any `Ok(R)` to
    /// `Ok(())`.
    ///
    /// # Returns
    ///
    /// A `BoxRunnable<E>` that executes this callable and discards its success
    /// value.
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(move || self.call().map(|_| ()))
    }
}

// ============================================================================
// BoxCallable
// ============================================================================

/// Box-based one-time callable.
///
/// `BoxCallable<R, E>` stores a `Box<dyn FnOnce() -> Result<R, E>>` and can be
/// executed only once. It is the boxed concrete implementation of
/// [`Callable`].
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxCallable, Callable};
///
/// let task = BoxCallable::new(|| Ok::<i32, String>(42));
/// assert_eq!(task.call(), Ok(42));
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxCallable<R, E> {
    /// The one-time closure executed by this callable.
    function: Box<dyn FnOnce() -> Result<R, E>>,
    /// The optional name of this callable.
    name: Option<String>,
}

impl<R, E> BoxCallable<R, E> {
    /// Creates a new boxed callable.
    ///
    /// # Parameters
    ///
    /// * `function` - The one-time closure executed by this callable.
    ///
    /// # Returns
    ///
    /// A new unnamed `BoxCallable<R, E>`.
    #[inline]
    pub fn new<F>(function: F) -> Self
    where
        F: FnOnce() -> Result<R, E> + 'static,
    {
        Self {
            function: Box::new(function),
            name: None,
        }
    }

    /// Creates a new named boxed callable.
    ///
    /// # Parameters
    ///
    /// * `name` - Name used by `Debug` and `Display`.
    /// * `function` - The one-time closure executed by this callable.
    ///
    /// # Returns
    ///
    /// A new named `BoxCallable<R, E>`.
    #[inline]
    pub fn new_with_name<F>(name: &str, function: F) -> Self
    where
        F: FnOnce() -> Result<R, E> + 'static,
    {
        Self {
            function: Box::new(function),
            name: Some(name.to_string()),
        }
    }

    /// Creates a new boxed callable with an optional name.
    ///
    /// # Parameters
    ///
    /// * `function` - The one-time closure executed by this callable.
    /// * `name` - Optional name used by `Debug` and `Display`.
    ///
    /// # Returns
    ///
    /// A new `BoxCallable<R, E>`.
    #[inline]
    pub fn new_with_optional_name<F>(function: F, name: Option<String>) -> Self
    where
        F: FnOnce() -> Result<R, E> + 'static,
    {
        Self {
            function: Box::new(function),
            name,
        }
    }

    /// Creates a boxed callable from a one-time supplier.
    ///
    /// This is an explicit bridge from `SupplierOnce<Result<R, E>>` to
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
        S: SupplierOnce<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    /// Gets the optional callable name.
    ///
    /// # Returns
    ///
    /// Returns `Some(&str)` if a name was set, or `None` otherwise.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the callable name.
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

    /// Clears the callable name.
    #[inline]
    pub fn clear_name(&mut self) {
        self.name = None;
    }

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
    pub fn map<U, M>(self, mapper: M) -> BoxCallable<U, E>
    where
        M: FnOnce(R) -> U + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallable::new_with_optional_name(move || function().map(mapper), name)
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
    pub fn map_err<E2, M>(self, mapper: M) -> BoxCallable<R, E2>
    where
        M: FnOnce(E) -> E2 + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallable::new_with_optional_name(move || function().map_err(mapper), name)
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
        N: FnOnce(R) -> Result<U, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxCallable::new_with_optional_name(move || function().and_then(next), name)
    }
}

impl<R, E> Callable<R, E> for BoxCallable<R, E> {
    /// Executes the boxed callable.
    #[inline]
    fn call(self) -> Result<R, E> {
        (self.function)()
    }

    /// Returns this boxed callable without re-boxing it.
    #[inline]
    fn into_box(self) -> BoxCallable<R, E> {
        self
    }

    /// Extracts the underlying one-time closure.
    #[inline]
    fn into_fn(self) -> impl FnOnce() -> Result<R, E> {
        self.function
    }

    /// Converts this boxed callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnable::new_with_optional_name(move || function().map(|_| ()), name)
    }
}

impl<R, E> SupplierOnce<Result<R, E>> for BoxCallable<R, E> {
    /// Executes the boxed callable as a one-time supplier of `Result<R, E>`.
    #[inline]
    fn get(self) -> Result<R, E> {
        self.call()
    }
}

impl<F, R, E> Callable<R, E> for F
where
    F: FnOnce() -> Result<R, E>,
{
    /// Executes the closure as a callable.
    #[inline]
    fn call(self) -> Result<R, E> {
        self()
    }

    /// Converts the closure to a boxed callable.
    #[inline]
    fn into_box(self) -> BoxCallable<R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallable::new(self)
    }

    /// Returns the closure unchanged.
    #[inline]
    fn into_fn(self) -> impl FnOnce() -> Result<R, E>
    where
        Self: Sized + 'static,
    {
        self
    }
}

impl<R, E> fmt::Debug for BoxCallable<R, E> {
    /// Formats this boxed callable for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxCallable")
            .field("name", &self.name)
            .field("function", &"<function>")
            .finish()
    }
}

impl<R, E> fmt::Display for BoxCallable<R, E> {
    /// Formats this boxed callable for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "BoxCallable({name})"),
            None => write!(f, "BoxCallable"),
        }
    }
}
