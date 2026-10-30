// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `Mutator` trait for shared-receiver mutation.

/// Mutator trait - Unified shared-receiver mutator interface
///
/// Defines the core behavior of all stateless mutator types. Performs
/// operations that accept a mutable reference and modify the input value
/// without requiring `&mut self`. Interior mutability and external side effects
/// remain possible.
///
/// This trait is automatically implemented by:
/// - All closures implementing `Fn(&mut T)` (stateless)
/// - `BoxMutator<T>`, `ArcMutator<T>`, and `RcMutator<T>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models for
/// **stateless** operations. Unlike `StatefulMutator` which uses `FnMut` and
/// can modify ordinary captured state, `Mutator` uses `Fn`. Implementations may
/// still use interior mutability or external side effects.
///
/// # Features
///
/// - **Shared-receiver calls**: Uses `Fn`, so invocation does not require `&mut
///   self`; interior mutability and external side effects remain possible
/// - **Unified Interface**: All mutator types share the same `apply` method
///   signature
/// - **Automatic Implementation**: Closures automatically implement this trait
/// - **Generic Programming**: Write functions that work with any mutator type
///
/// # Type Parameters
///
/// - `T` - The value type that the mutator receives and modifies.
///
/// # Examples
///
/// ## Generic Mutator Function
///
/// ```rust
/// use qubit_function::{Mutator, BoxMutator, ArcMutator};
///
/// fn apply_mutator<M: Mutator<i32>>(
///     mutator: &M,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     mutator.apply(&mut val);
///     val
/// }
///
/// // Works with any mutator type
/// let box_mut = BoxMutator::new(|x: &mut i32| *x *= 2);
/// assert_eq!(apply_mutator(&box_mut, 5), 10);
///
/// let arc_mut = ArcMutator::new(|x: &mut i32| *x *= 2);
/// assert_eq!(apply_mutator(&arc_mut, 5), 10);
///
/// let closure = |x: &mut i32| *x *= 2;
/// assert_eq!(apply_mutator(&closure, 5), 10);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::BoxMutator;
///
/// let closure = |x: &mut i32| *x *= 2;
///
/// // Convert to different ownership models
/// let box_mutator = BoxMutator::new(closure);
/// ```
pub trait Mutator<T> {
    /// Performs the stateless mutation operation
    ///
    /// Executes an operation on the given mutable reference without modifying
    /// ordinary captured state through the `Fn` receiver. Implementations may
    /// still use interior mutability or external side effects.
    ///
    /// # Parameters
    ///
    /// * `value` - A mutable reference to the value to be mutated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Mutator, BoxMutator};
    ///
    /// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
    /// let mut value = 5;
    /// mutator.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn apply(&self, value: &mut T);
}

crate::macros::impl_closure_trait!(
    Mutator<T>,
    apply,
    Fn(value: &mut T)
);
