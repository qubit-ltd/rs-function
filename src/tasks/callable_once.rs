/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Callable Once Types
//!
//! Provides fallible, one-time, zero-argument computations.
//!
//! A `CallableOnce<R, E>` is equivalent to `FnOnce() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `RunnableOnce<E>` when the operation only
//! needs to report success or failure.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_box_once_conversions,
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::supplier_once::SupplierOnce,
    tasks::runnable_once::{
        BoxRunnableOnce,
        LocalBoxRunnableOnce,
    },
};

mod box_callable_once;
pub use box_callable_once::BoxCallableOnce;
mod local_box_callable_once;
pub use local_box_callable_once::LocalBoxCallableOnce;

// ============================================================================
// CallableOnce Trait
// ============================================================================

/// A fallible one-time computation.
///
/// Conceptually this matches `FnOnce() -> Result<R, E>`: `call` consumes `self`
/// and returns `Result<R, E>`, but the surface uses task-oriented naming and
/// helpers instead of closure types. It is a semantic specialization of
/// `SupplierOnce<Result<R, E>>` for executable computations and deferred tasks.
///
/// Choose **`CallableOnce`** when callers need the success value `R`. When only
/// success or failure matters, use [`RunnableOnce`](crate::tasks::runnable_once::RunnableOnce),
/// whose success type is `()`.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{CallableOnce, BoxCallableOnce};
///
/// let task = || Ok::<i32, String>(21 * 2);
/// assert_eq!(task.call(), Ok(42));
/// ```
///
pub trait CallableOnce<R, E> {
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
    /// A `BoxCallableOnce<R, E>` that executes this callable when `call()` is
    /// invoked.
    fn into_box(self) -> BoxCallableOnce<R, E>
    where
        Self: Sized + Send + 'static,
    {
        BoxCallableOnce::new(move || self.call())
    }

    /// Converts this callable into a local boxed callable.
    ///
    /// # Returns
    ///
    /// A `LocalBoxCallableOnce<R, E>` that may hold non-`Send` captures and
    /// must be executed on the local thread.
    fn into_local_box(self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Sized + 'static,
    {
        LocalBoxCallableOnce::new(move || self.call())
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
    /// A new `BoxCallableOnce<R, E>` built from a clone of this callable.
    fn to_box(&self) -> BoxCallableOnce<R, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this callable into a local boxed callable without consuming
    /// `self`.
    ///
    /// The method clones `self` and boxes the clone without requiring `Send`.
    ///
    /// # Returns
    ///
    /// A new `LocalBoxCallableOnce<R, E>` built from a clone of this callable.
    fn to_local_box(&self) -> LocalBoxCallableOnce<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_local_box()
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
    /// A `BoxRunnableOnce<E>` that executes this callable and discards its
    /// success value.
    fn into_runnable(self) -> BoxRunnableOnce<E>
    where
        Self: Sized + Send + 'static,
    {
        BoxRunnableOnce::new(move || self.call().map(|_| ()))
    }

    /// Converts this callable into a local runnable by discarding the success
    /// value.
    ///
    /// # Returns
    ///
    /// A `LocalBoxRunnableOnce<E>` that may hold non-`Send` captures and maps
    /// any `Ok(R)` to `Ok(())`.
    fn into_local_runnable(self) -> LocalBoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        LocalBoxRunnableOnce::new(move || self.call().map(|_| ()))
    }
}
