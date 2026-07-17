// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Callable Types
//!
//! Provides fallible, reusable, zero-argument computations.
//!
//! A `Callable<R, E>` is equivalent to `FnMut() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `Runnable<E>` when the operation only needs
//! to report success or failure.
//!
//! The semantic trait is thread-neutral. `Box*` task wrappers are `Send` for
//! executor submission, while matching `LocalBox*` wrappers permit non-`Send`
//! captures for local execution.

mod box_callable;
pub use box_callable::BoxCallable;
mod local_box_callable;
pub use local_box_callable::LocalBoxCallable;
#[cfg(feature = "rc")]
mod rc_callable;
#[cfg(feature = "rc")]
pub use rc_callable::RcCallable;
#[cfg(feature = "stateful")]
mod arc_callable;
#[cfg(feature = "stateful")]
pub use arc_callable::ArcCallable;

// ============================================================================
// Callable Trait
// ============================================================================

/// A fallible, reusable zero-argument computation.
///
/// Conceptually this is the same shape as `FnMut() -> Result<R, E>`: `call`
/// takes `&mut self` and returns `Result<R, E>`, but the API uses task-oriented
/// naming and helpers. In this crate it aligns with
/// [`Supplier`](crate::suppliers::Supplier) of `Result<R, E>`—a fallible
/// supplier—while emphasizing executable work rather than plain value
/// production.
///
/// Choose **`Callable`** when callers need the success value `R`. When only
/// success or failure matters, use [`Runnable`](crate::tasks::Runnable), whose
/// success type is `()`.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::Callable;
///
/// let mut task = || Ok::<i32, String>(21 * 2);
/// assert_eq!(task.call().expect("call should succeed"), 42);
/// ```
pub trait Callable<R, E> {
    /// Executes the computation, borrowing `self` mutably.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    ///
    /// # Errors
    ///
    /// Returns `Err(E)` when the underlying computation fails.
    fn call(&mut self) -> Result<R, E>;
}

impl<R, E, F> Callable<R, E> for F
where
    F: FnMut() -> Result<R, E>,
{
    #[inline(always)]
    fn call(&mut self) -> Result<R, E> {
        self()
    }
}
