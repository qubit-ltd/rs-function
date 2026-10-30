// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `CallableWith` semantic trait.

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
///
/// # Examples
///
/// ```rust
/// use qubit_function::tasks::CallableWith;
///
/// let mut task = |value: &mut i32| {
///     *value += 1;
///     Ok::<i32, ()>(*value)
/// };
/// let mut value = 41;
/// assert_eq!(task.call_with(&mut value), Ok(42));
/// assert_eq!(value, 42);
/// ```
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
    ///
    /// # Errors
    ///
    /// Returns `Err(E)` when the underlying computation fails.
    fn call_with(&mut self, input: &mut T) -> Result<R, E>;
}

impl<T, R, E, F> CallableWith<T, R, E> for F
where
    F: FnMut(&mut T) -> Result<R, E>,
{
    #[inline(always)]
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        self(input)
    }
}
