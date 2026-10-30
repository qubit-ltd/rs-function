// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// ============================================================================
// RunnableOnce Trait
// ============================================================================

/// A fallible one-time action.
///
/// Conceptually this matches `FnOnce() -> Result<(), E>`: `run_once` consumes
/// `self` and returns `Result<(), E>`, but the surface uses task-oriented
/// naming and helpers instead of closure types. It is a semantic
/// specialization of `SupplierOnce<Result<(), E>>` for executable actions and
/// deferred side effects.
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
/// use qubit_function::RunnableOnce;
///
/// let task = || Ok::<(), String>(());
/// assert_eq!(RunnableOnce::run_once(task), Ok(()));
/// ```
pub trait RunnableOnce<E> {
    /// Executes the action, consuming `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - The action to execute; it is consumed by the call.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    ///
    /// # Errors
    ///
    /// Returns `Err(E)` when the underlying action fails.
    ///
    /// # Panics
    ///
    /// Propagates a panic raised by the underlying action.
    fn run_once(self) -> Result<(), E>;
}

impl<E, F> RunnableOnce<E> for F
where
    F: FnOnce() -> Result<(), E>,
{
    #[inline(always)]
    fn run_once(self) -> Result<(), E> {
        self()
    }
}
