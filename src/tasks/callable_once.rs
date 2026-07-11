// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
/// success or failure matters, use
/// [`RunnableOnce`](crate::tasks::runnable_once::RunnableOnce), whose success
/// type is `()`.
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
pub trait CallableOnce<R, E> {
    /// Executes the computation, consuming `self`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    fn call(self) -> Result<R, E>;
}

impl<R, E, F> CallableOnce<R, E> for F
where
    F: FnOnce() -> Result<R, E>,
{
    fn call(self) -> Result<R, E> {
        self()
    }
}
