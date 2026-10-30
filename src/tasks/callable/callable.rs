// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the semantic `Callable` trait.

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
    /// Executes the closure-backed computation.
    ///
    /// The closure's result and error are returned unchanged.
    #[inline(always)]
    fn call(&mut self) -> Result<R, E> {
        self()
    }
}
