// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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

mod box_callable_with;
pub use box_callable_with::BoxCallableWith;
#[cfg(feature = "rc")]
mod rc_callable_with;
#[cfg(feature = "rc")]
pub use rc_callable_with::RcCallableWith;
#[cfg(feature = "stateful")]
mod arc_callable_with;
#[cfg(feature = "stateful")]
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
}

impl<T, R, E, F> CallableWith<T, R, E> for F
where
    F: FnMut(&mut T) -> Result<R, E>,
{
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        self(input)
    }
}
