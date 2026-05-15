/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
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

use crate::tasks::callable_once::{
    BoxCallableOnce,
    LocalBoxCallableOnce,
};

mod box_runnable_once;
pub use box_runnable_once::BoxRunnableOnce;
mod local_box_runnable_once;
pub use local_box_runnable_once::LocalBoxRunnableOnce;

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
/// [`CallableOnce`](crate::tasks::callable_once::CallableOnce).
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
        Self: Sized + Send + 'static,
    {
        BoxRunnableOnce::new(move || self.run())
    }

    /// Converts this runnable into a local boxed runnable.
    ///
    /// # Returns
    ///
    /// A `LocalBoxRunnableOnce<E>` that may hold non-`Send` captures and must
    /// be executed on the local thread.
    fn into_local_box(self) -> LocalBoxRunnableOnce<E>
    where
        Self: Sized + 'static,
    {
        LocalBoxRunnableOnce::new(move || self.run())
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
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this runnable into a local boxed runnable without consuming
    /// `self`.
    ///
    /// The method clones `self` and boxes the clone without requiring `Send`.
    ///
    /// # Returns
    ///
    /// A new `LocalBoxRunnableOnce<E>` built from a clone of this runnable.
    fn to_local_box(&self) -> LocalBoxRunnableOnce<E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_local_box()
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
        Self: Sized + Send + 'static,
    {
        BoxCallableOnce::new(move || self.run())
    }

    /// Converts this runnable into a local callable returning unit.
    ///
    /// # Returns
    ///
    /// A `LocalBoxCallableOnce<(), E>` that may hold non-`Send` captures and
    /// returns `Ok(())` on success.
    fn into_local_callable(self) -> LocalBoxCallableOnce<(), E>
    where
        Self: Sized + 'static,
    {
        LocalBoxCallableOnce::new(move || self.run())
    }
}
