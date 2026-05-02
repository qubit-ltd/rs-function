/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Runnable Types
//!
//! Provides fallible, reusable, zero-argument actions.
//!
//! A `Runnable<E>` is equivalent to `FnMut() -> Result<(), E>`, but uses
//! task-oriented vocabulary. Use it when the operation's side effect matters
//! and only success or failure should be reported.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    macros::{
        impl_arc_conversions,
        impl_box_conversions,
        impl_closure_trait,
        impl_common_name_methods,
        impl_common_new_methods,
        impl_rc_conversions,
    },
    suppliers::macros::impl_supplier_debug_display,
    suppliers::supplier::Supplier,
    suppliers::supplier_once::SupplierOnce,
    tasks::callable::BoxCallable,
};

mod box_runnable;
pub use box_runnable::BoxRunnable;
mod rc_runnable;
pub use rc_runnable::RcRunnable;
mod arc_runnable;
pub use arc_runnable::ArcRunnable;

// ============================================================================
// Runnable Trait
// ============================================================================

/// A fallible, reusable, zero-argument action.
///
/// Conceptually, `Runnable<E>` matches [`FnMut`] `() -> Result<(), E>`, but
/// uses task-oriented vocabulary. Prefer it when the operation's side effect
/// matters and only success or failure need to be reported.
///
/// Each call borrows `self` mutably and returns [`Result::Ok`] with unit or
/// [`Result::Err`] with `E`. Semantically, this is a specialization of
/// [`SupplierOnce`]`<Result<(), E>>` for executable actions and deferred side
/// effects.
///
/// The trait does not require [`Send`]. Concurrent executors should require
/// `Runnable<E> + Send + 'static` (or similar) at their API boundary.
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
/// let mut task = || Ok::<(), String>(());
/// assert_eq!(task.run(), Ok(()));
/// ```
///
pub trait Runnable<E> {
    /// Executes the action, borrowing `self` mutably.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run(&mut self) -> Result<(), E>;

    /// Converts this runnable into a boxed runnable.
    ///
    /// # Returns
    ///
    /// A `BoxRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_box(mut self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(move || self.run())
    }

    /// Converts this runnable into a shared single-threaded runnable.
    ///
    /// # Returns
    ///
    /// An `RcRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_rc(mut self) -> RcRunnable<E>
    where
        Self: Sized + 'static,
    {
        RcRunnable::new(move || self.run())
    }

    /// Converts this runnable into a shared thread-safe runnable.
    ///
    /// # Returns
    ///
    /// An `ArcRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_arc(mut self) -> ArcRunnable<E>
    where
        Self: Sized + Send + 'static,
    {
        ArcRunnable::new(move || self.run())
    }

    /// Converts this runnable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<(), E>`.
    fn into_fn(mut self) -> impl FnMut() -> Result<(), E>
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

    /// Converts this runnable into a mutable closure without consuming `self`.
    ///
    /// The method clones `self` and returns a mutable closure that executes
    /// the clone.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<(), E>`.
    fn to_fn(&self) -> impl FnMut() -> Result<(), E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this runnable into a shared single-threaded runnable without
    /// consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// A new `RcRunnable<E>` built from a clone of this runnable.
    fn to_rc(&self) -> RcRunnable<E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this runnable into a shared thread-safe runnable without
    /// consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// A new `ArcRunnable<E>` built from a clone of this runnable.
    fn to_arc(&self) -> ArcRunnable<E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
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
        let mut runnable = self;
        BoxCallable::new(move || runnable.run())
    }
}
