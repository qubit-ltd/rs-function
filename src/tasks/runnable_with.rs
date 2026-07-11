// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
}
