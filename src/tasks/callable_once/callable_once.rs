// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `CallableOnce` trait.

/// A fallible one-time computation.
///
/// Conceptually this matches `FnOnce() -> Result<R, E>`: `call_once` consumes
/// `self` and returns `Result<R, E>`, but the surface uses task-oriented naming
/// and helpers instead of closure types. It is a semantic specialization of
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
/// use qubit_function::CallableOnce;
///
/// let task = || Ok::<i32, String>(21 * 2);
/// assert_eq!(CallableOnce::call_once(task), Ok(42));
/// ```
pub trait CallableOnce<R, E> {
    /// Executes the computation, consuming `self`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    ///
    /// # Errors
    ///
    /// Returns `Err(E)` when the underlying computation fails.
    fn call_once(self) -> Result<R, E>;
}

impl<R, E, F> CallableOnce<R, E> for F
where
    F: FnOnce() -> Result<R, E>,
{
    /// Executes the closure and returns its result unchanged.
    ///
    /// # Returns
    ///
    /// Returns the `Result` produced by the closure.
    ///
    /// # Errors
    ///
    /// Returns the error produced by the closure when it fails.
    #[inline(always)]
    fn call_once(self) -> Result<R, E> {
        self()
    }
}
