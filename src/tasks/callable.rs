/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Callable Types
//!
//! Provides fallible, reusable, zero-argument computations.
//!
//! A `Callable<R, E>` is equivalent to `FnMut() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `Runnable<E>` when the operation only needs
//! to report success or failure.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!

use crate::{
    tasks::callable_once::{
        BoxCallableOnce,
        LocalBoxCallableOnce,
    },
    tasks::runnable::BoxRunnable,
};

mod box_callable;
pub use box_callable::BoxCallable;
mod rc_callable;
pub use rc_callable::RcCallable;
mod arc_callable;
pub use arc_callable::ArcCallable;

// ============================================================================
// Callable Trait
// ============================================================================

/// A fallible, reusable zero-argument computation.
///
/// Conceptually this is the same shape as `FnMut() -> Result<R, E>`: `call` takes
/// `&mut self` and returns `Result<R, E>`, but the API uses task-oriented naming
/// and helpers. In this crate it aligns with
/// [`Supplier`](crate::suppliers::Supplier) of `Result<R, E>`—a fallible
/// supplier—while emphasizing executable work rather than plain value production.
///
/// Choose **`Callable`** when callers need the success value `R`. When only
/// success or failure matters, use [`Runnable`](crate::tasks::Runnable), whose
/// success type is `()`.
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
/// let mut task = || Ok::<i32, String>(21 * 2);
/// assert_eq!(task.call().expect("call should succeed"), 42);
/// ```
///
pub trait Callable<R, E> {
    /// Executes the computation, borrowing `self` mutably.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    fn call(&mut self) -> Result<R, E>;

    /// Converts this callable into a boxed callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallable<R, E>` that executes this callable when `call()` is
    /// invoked.
    fn into_box(mut self) -> BoxCallable<R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallable::new(move || self.call())
    }

    /// Converts this callable into an `Rc` callable.
    ///
    /// # Returns
    ///
    /// A `RcCallable<R, E>`.
    fn into_rc(mut self) -> RcCallable<R, E>
    where
        Self: Sized + 'static,
    {
        RcCallable::new(move || self.call())
    }

    /// Converts this callable into an `Arc` callable.
    ///
    /// # Returns
    ///
    /// An `ArcCallable<R, E>`.
    fn into_arc(mut self) -> ArcCallable<R, E>
    where
        Self: Sized + Send + 'static,
    {
        ArcCallable::new(move || self.call())
    }

    /// Converts this callable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<R, E>`.
    fn into_fn(mut self) -> impl FnMut() -> Result<R, E>
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

    /// Converts this callable into an `Rc` callable without consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// A `RcCallable<R, E>`.
    fn to_rc(&self) -> RcCallable<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this callable into an `Arc` callable without consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// An `ArcCallable<R, E>`.
    fn to_arc(&self) -> ArcCallable<R, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this callable into a mutable closure without consuming `self`.
    ///
    /// The method clones `self` and returns a closure that executes the clone
    /// on each call.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<R, E>`.
    fn to_fn(&self) -> impl FnMut() -> Result<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this callable into a one-time callable.
    ///
    /// The returned callable consumes itself on each invocation.
    ///
    /// # Returns
    ///
    /// A `BoxCallableOnce<R, E>`.
    fn into_once(mut self) -> BoxCallableOnce<R, E>
    where
        Self: Sized + Send + 'static,
    {
        BoxCallableOnce::new(move || self.call())
    }

    /// Converts this callable into a local one-time callable.
    ///
    /// The returned callable consumes itself on each invocation and may hold
    /// non-`Send` captures.
    ///
    /// # Returns
    ///
    /// A `LocalBoxCallableOnce<R, E>`.
    fn into_local_once(mut self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Sized + 'static,
    {
        LocalBoxCallableOnce::new(move || self.call())
    }

    /// Converts this callable into a one-time callable without consuming
    /// `self`.
    ///
    /// The method clones `self` and returns a one-time callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallableOnce<R, E>`.
    fn to_once(&self) -> BoxCallableOnce<R, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_once()
    }

    /// Converts this callable into a local one-time callable without consuming
    /// `self`.
    ///
    /// The method clones `self` and returns a local one-time callable.
    ///
    /// # Returns
    ///
    /// A `LocalBoxCallableOnce<R, E>`.
    fn to_local_once(&self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_local_once()
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
    fn into_runnable(mut self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(move || self.call().map(|_| ()))
    }
}
