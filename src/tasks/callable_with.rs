/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # CallableWith Types
//!
//! Provides fallible, reusable computations that operate on a mutable input.
//!
//! A `CallableWith<T, R, E>` is equivalent to
//! `FnMut(&mut T) -> Result<R, E>`, but uses task-oriented vocabulary. Use it
//! when the operation needs access to protected or caller-provided state and
//! returns a success value.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_arc_conversions,
        impl_box_conversions,
        impl_closure_trait,
        impl_common_name_methods,
        impl_common_new_methods,
        impl_rc_conversions,
    },
    tasks::runnable_with::BoxRunnableWith,
};

mod box_callable_with;
pub use box_callable_with::BoxCallableWith;
mod rc_callable_with;
pub use rc_callable_with::RcCallableWith;
mod arc_callable_with;
pub use arc_callable_with::ArcCallableWith;

/// A fallible, reusable computation that receives mutable input.
///
/// Conceptually this is `FnMut(&mut T) -> Result<R, E>` with task-oriented
/// naming. It is useful for executor-style APIs that run a task with access to
/// protected state, such as a value held under a lock.
///
/// # Type Parameters
///
/// * `T` - The mutable input type.
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
pub trait CallableWith<T, R, E> {
    /// Executes the computation with mutable input.
    ///
    /// # Parameters
    ///
    /// * `input` - The mutable input passed to this task.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    fn call_with(&mut self, input: &mut T) -> Result<R, E>;

    /// Converts this callable into a boxed callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallableWith<T, R, E>`.
    fn into_box(mut self) -> BoxCallableWith<T, R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallableWith::new(move |input| self.call_with(input))
    }

    /// Converts this callable into an `Rc` callable.
    ///
    /// # Returns
    ///
    /// A `RcCallableWith<T, R, E>`.
    fn into_rc(mut self) -> RcCallableWith<T, R, E>
    where
        Self: Sized + 'static,
    {
        RcCallableWith::new(move |input| self.call_with(input))
    }

    /// Converts this callable into an `Arc` callable.
    ///
    /// # Returns
    ///
    /// An `ArcCallableWith<T, R, E>`.
    fn into_arc(mut self) -> ArcCallableWith<T, R, E>
    where
        Self: Sized + Send + 'static,
    {
        ArcCallableWith::new(move |input| self.call_with(input))
    }

    /// Converts this callable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<R, E>`.
    fn into_fn(mut self) -> impl FnMut(&mut T) -> Result<R, E>
    where
        Self: Sized + 'static,
    {
        move |input| self.call_with(input)
    }

    /// Converts this callable into a boxed callable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `BoxCallableWith<T, R, E>` built from a clone of this callable.
    fn to_box(&self) -> BoxCallableWith<T, R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this callable into an `Rc` callable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `RcCallableWith<T, R, E>` built from a clone of this callable.
    fn to_rc(&self) -> RcCallableWith<T, R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this callable into an `Arc` callable without consuming `self`.
    ///
    /// # Returns
    ///
    /// An `ArcCallableWith<T, R, E>` built from a clone of this callable.
    fn to_arc(&self) -> ArcCallableWith<T, R, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this callable into a mutable closure without consuming `self`.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<R, E>`.
    fn to_fn(&self) -> impl FnMut(&mut T) -> Result<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this callable into a runnable by discarding the success value.
    ///
    /// # Returns
    ///
    /// A `BoxRunnableWith<T, E>` preserving errors and mapping success to unit.
    fn into_runnable_with(mut self) -> BoxRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        BoxRunnableWith::new(move |input| self.call_with(input).map(|_| ()))
    }
}
