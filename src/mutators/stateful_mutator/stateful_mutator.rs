// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulMutator` trait.

/// Unified stateful mutator interface.
///
/// Defines operations that may modify both a mutable input and captured state.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T)`
/// - `BoxStatefulMutator<T>`, `ArcStatefulMutator<T>`, and
///   `RcStatefulMutator<T>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models,
/// allowing generic code to work with any stateful mutator type.
///
/// # Type Parameters
///
/// * `T` - The input value that an implementation can mutate.
///
/// # Features
///
/// - **Unified Interface**: All stateful mutator types share the same `apply`
///   method signature
/// - **Automatic Implementation**: Closures implement this trait directly,
///   without allocating an adapter
/// - **Generic Programming**: Write functions that work with any mutator type
///
/// # Examples
///
/// ## Generic Stateful Mutator Function
///
/// ```rust
/// use qubit_function::{BoxStatefulMutator, StatefulMutator};
///
/// fn apply_mutator<M: StatefulMutator<i32>>(mutator: &mut M, value: i32) -> i32 {
///     let mut value = value;
///     mutator.apply(&mut value);
///     value
/// }
///
/// let mut calls = 0;
/// let mut mutator = BoxStatefulMutator::new(move |value: &mut i32| {
///     calls += 1;
///     *value += calls;
/// });
/// assert_eq!(apply_mutator(&mut mutator, 10), 11);
/// assert_eq!(apply_mutator(&mut mutator, 10), 12);
/// ```
///
/// ## Wrapper Construction
///
/// ```rust
/// use qubit_function::BoxStatefulMutator;
///
/// let closure = |value: &mut i32| *value *= 2;
/// let box_mutator = BoxStatefulMutator::new(closure);
/// ```
pub trait StatefulMutator<T> {
    /// Mutates the input and may update state captured by the mutator.
    ///
    /// # Parameters
    ///
    /// * `value` - Mutable reference to the value being changed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxStatefulMutator, StatefulMutator};
    ///
    /// let mut mutator = BoxStatefulMutator::new(|value: &mut i32| *value += 1);
    /// let mut value = 10;
    /// mutator.apply(&mut value);
    /// assert_eq!(value, 11);
    /// ```
    fn apply(&mut self, value: &mut T);
}

crate::macros::impl_closure_trait!(
    StatefulMutator<T>,
    apply,
    FnMut(value: &mut T)
);
