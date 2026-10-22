// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulFunction Types
//!
//! Provides Rust implementations of stateful function traits for transforming
//! borrowed input values while allowing internal state modification.
//!
//! It is similar to the `FnMut(&T) -> R` trait in the standard library.
//!
//! This module provides the `StatefulFunction<T, R>` trait and three
//! implementations:
//!
//! - [`BoxStatefulFunction`]: Single ownership, not cloneable
//! - [`ArcStatefulFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulFunction`]: Single-threaded shared ownership, cloneable

mod box_stateful_function;
pub use box_stateful_function::BoxStatefulFunction;
#[cfg(feature = "rc")]
mod rc_stateful_function;
#[cfg(feature = "rc")]
pub use rc_stateful_function::RcStatefulFunction;
mod arc_stateful_function;
pub use arc_stateful_function::ArcStatefulFunction;
mod box_conditional_stateful_function;
pub use box_conditional_stateful_function::BoxConditionalStatefulFunction;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_function;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_function::RcConditionalStatefulFunction;
mod arc_conditional_stateful_function;
pub use arc_conditional_stateful_function::ArcConditionalStatefulFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// Transforms borrowed values from type `T` to type `R` while retaining state
///
/// A stateful function receives each input by shared reference, so it does not
/// take ownership of the input. Its mutable receiver allows it to update state
/// between calls, similarly to the standard library's `FnMut(&T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of each borrowed input value.
/// * `R` - The type of each returned output value.
///
/// # Examples
///
/// ```
/// use qubit_function::{BoxStatefulFunction, StatefulFunction};
///
/// let mut call_count = 0;
/// let mut counter = BoxStatefulFunction::new(move |value: &i32| {
///     call_count += 1;
///     value + call_count
/// });
///
/// assert_eq!(counter.apply(&5), 6);
/// assert_eq!(counter.apply(&5), 7);
/// ```
pub trait StatefulFunction<T, R> {
    /// Applies the mapping to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - The borrowed input value to transform
    ///
    /// # Returns
    ///
    /// The transformed output value
    #[must_use = "the computed callback result should be used"]
    fn apply(&mut self, t: &T) -> R;
}

impl<T, R, F> StatefulFunction<T, R> for F
where
    F: FnMut(&T) -> R,
{
    #[inline(always)]
    fn apply(&mut self, t: &T) -> R {
        self(t)
    }
}
