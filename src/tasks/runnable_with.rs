/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # RunnableWith Types
//!
//! Provides fallible, reusable actions that operate on a mutable input.
//!
//! A `RunnableWith<T, E>` is equivalent to
//! `FnMut(&mut T) -> Result<(), E>`, but uses task-oriented vocabulary. Use it
//! when the operation needs access to protected or caller-provided state and
//! only success or failure should be reported.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!

use crate::tasks::callable_with::BoxCallableWith;

mod box_runnable_with;
pub use box_runnable_with::BoxRunnableWith;
mod rc_runnable_with;
pub use rc_runnable_with::RcRunnableWith;
mod arc_runnable_with;
pub use arc_runnable_with::ArcRunnableWith;

/// A fallible, reusable action that receives mutable input.
///
/// Conceptually this is `FnMut(&mut T) -> Result<(), E>` with task-oriented
/// naming. It is useful for executor-style APIs that run an action with access
/// to protected state, such as a value held under a lock.
///
/// # Type Parameters
///
/// * `T` - The mutable input type.
/// * `E` - The error value returned when the action fails.
///
pub trait RunnableWith<T, E> {
    /// Executes the action with mutable input.
    ///
    /// # Parameters
    ///
    /// * `input` - The mutable input passed to this task.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run_with(&mut self, input: &mut T) -> Result<(), E>;

    /// Converts this runnable into a boxed runnable.
    ///
    /// # Returns
    ///
    /// A `BoxRunnableWith<T, E>`.
    fn into_box(mut self) -> BoxRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        BoxRunnableWith::new(move |input| self.run_with(input))
    }

    /// Converts this runnable into an `Rc` runnable.
    ///
    /// # Returns
    ///
    /// A `RcRunnableWith<T, E>`.
    fn into_rc(mut self) -> RcRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        RcRunnableWith::new(move |input| self.run_with(input))
    }

    /// Converts this runnable into an `Arc` runnable.
    ///
    /// # Returns
    ///
    /// An `ArcRunnableWith<T, E>`.
    fn into_arc(mut self) -> ArcRunnableWith<T, E>
    where
        Self: Sized + Send + 'static,
    {
        ArcRunnableWith::new(move |input| self.run_with(input))
    }

    /// Converts this runnable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<(), E>`.
    fn into_fn(mut self) -> impl FnMut(&mut T) -> Result<(), E>
    where
        Self: Sized + 'static,
    {
        move |input| self.run_with(input)
    }

    /// Converts this runnable into a boxed runnable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `BoxRunnableWith<T, E>` built from a clone of this runnable.
    fn to_box(&self) -> BoxRunnableWith<T, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this runnable into an `Rc` runnable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `RcRunnableWith<T, E>` built from a clone of this runnable.
    fn to_rc(&self) -> RcRunnableWith<T, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this runnable into an `Arc` runnable without consuming `self`.
    ///
    /// # Returns
    ///
    /// An `ArcRunnableWith<T, E>` built from a clone of this runnable.
    fn to_arc(&self) -> ArcRunnableWith<T, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this runnable into a mutable closure without consuming `self`.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<(), E>`.
    fn to_fn(&self) -> impl FnMut(&mut T) -> Result<(), E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this runnable into a callable returning unit.
    ///
    /// # Returns
    ///
    /// A `BoxCallableWith<T, (), E>` that runs this task and returns unit on
    /// success.
    fn into_callable_with(mut self) -> BoxCallableWith<T, (), E>
    where
        Self: Sized + 'static,
    {
        BoxCallableWith::new(move |input| self.run_with(input))
    }
}
