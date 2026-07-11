// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
/// `SupplierOnce<Result<(), E>>` for executable actions and deferred side
/// effects.
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
pub trait RunnableOnce<E> {
    /// Executes the action, consuming `self`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run(self) -> Result<(), E>;
}

impl<E, F> RunnableOnce<E> for F
where
    F: FnOnce() -> Result<(), E>,
{
    fn run(self) -> Result<(), E> {
        self()
    }
}
