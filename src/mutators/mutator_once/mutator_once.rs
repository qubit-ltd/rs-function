// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// MutatorOnce trait - One-time mutator interface
///
/// Defines the core behavior of all one-time mutator types. Performs operations
/// that consume self and modify the input value.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnOnce(&mut T)`
/// - `BoxMutatorOnce<T>`
///
/// # Design Rationale
///
/// This trait provides a unified abstraction for one-time mutation operations.
/// The key difference from `Mutator`:
/// - `Mutator` uses `&self`, can be called multiple times
/// - `MutatorOnce` uses `self`, can only be called once
///
/// # Type Parameters
///
/// * `T` - The value type passed mutably to the one-time operation.
///
/// # Features
///
/// - **Unified Interface**: All one-time mutators share the same `apply` method
///   signature
/// - **Automatic Implementation**: Closures implement this trait directly,
///   without allocating an adapter
/// - **Generic Programming**: Write functions that work with any one-time
///   mutator type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatorOnce, BoxMutatorOnce};
///
/// fn apply_once<M: MutatorOnce<Vec<i32>>>(
///     mutator: M,
///     initial: Vec<i32>
/// ) -> Vec<i32> {
///     let mut val = initial;
///     mutator.apply(&mut val);
///     val
/// }
///
/// let data = vec![1, 2, 3];
/// let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
///     x.extend(data);
/// });
/// let result = apply_once(mutator, vec![0]);
/// assert_eq!(result, vec![0, 1, 2, 3]);
/// ```
///
/// ## Wrapper Construction
///
/// ```rust
/// use qubit_function::BoxMutatorOnce;
///
/// let data = vec![1, 2, 3];
/// let closure = move |x: &mut Vec<i32>| x.extend(data);
/// let box_mutator = BoxMutatorOnce::new(closure);
/// ```
pub trait MutatorOnce<T> {
    /// Performs the one-time mutation operation
    ///
    /// Consumes self and executes an operation on the given mutable reference.
    /// The operation typically modifies the input value or produces side
    /// effects, and can only be called once.
    ///
    /// # Parameters
    ///
    /// * `value` - A mutable reference to the value to be mutated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatorOnce, BoxMutatorOnce};
    ///
    /// let data = vec![1, 2, 3];
    /// let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
    ///     x.extend(data);
    /// });
    ///
    /// let mut target = vec![0];
    /// mutator.apply(&mut target);
    /// assert_eq!(target, vec![0, 1, 2, 3]);
    /// ```
    fn apply(self, value: &mut T);
}
