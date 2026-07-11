// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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

use crate::macros::impl_closure_trait;

mod box_runnable;
pub use box_runnable::BoxRunnable;
#[cfg(feature = "rc")]
#[cfg(feature = "rc")]
mod rc_runnable;
#[cfg(feature = "rc")]
#[cfg(feature = "rc")]
pub use rc_runnable::RcRunnable;
#[cfg(feature = "stateful")]
mod arc_runnable;
#[cfg(feature = "stateful")]
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
/// [`SupplierOnce`](crate::suppliers::SupplierOnce)`<Result<(), E>>` for
/// executable actions and deferred side effects.
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
pub trait Runnable<E> {
    /// Executes the action, borrowing `self` mutably.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run(&mut self) -> Result<(), E>;
}

impl_closure_trait!(
    Runnable<E>,
    run,
    FnMut() -> Result<(), E>
);
