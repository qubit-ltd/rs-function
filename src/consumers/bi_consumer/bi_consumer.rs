// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the shared-reference bi-consumer interface.

/// Provides a shared-reference interface for consuming two input values.
///
/// Calls use `&self` and shared references, so the consumer and inputs are not
/// directly mutable through this API. Implementations may still use interior
/// mutability or perform external side effects. Closures implementing
/// `Fn(&T, &U)` implement this trait automatically.
///
/// # Type Parameters
///
/// * `T` - Type of the first value passed to the callback.
/// * `U` - Type of the second value passed to the callback.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// let consumer = BoxBiConsumer::new(|first: &i32, second: &i32| {
///     assert_eq!(first + second, 8);
/// });
/// consumer.accept(&5, &3);
/// ```
pub trait BiConsumer<T, U> {
    /// Runs the consumer operation with shared references to both inputs.
    ///
    /// The method does not grant direct mutable access to either input or the
    /// consumer wrapper; implementations may use interior mutability.
    ///
    /// # Parameters
    ///
    /// * `first` - First input value to observe.
    /// * `second` - Second input value to observe.
    fn accept(&self, first: &T, second: &U);
}

crate::macros::impl_closure_trait!(
    BiConsumer<T, U>,
    accept,
    Fn(first: &T, second: &U)
);
