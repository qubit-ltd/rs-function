// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulFunction Types
//!
//! Provides Rust implementations of stateful function traits for stateful value
//! transformation. StatefulFunctions consume input values (taking ownership)
//! and produce output values while allowing internal state modification.
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

/// StatefulFunction trait - transforms values from type T to type R with state
///
/// Defines the behavior of a stateful transformation: converting a borrowed
/// value of type `T` to a value of type `R` while allowing modification of
/// internal state. This is analogous to
/// `FnMut(&T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the borrowed input value
/// * `R` - The type of the output value
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
    fn apply(&mut self, t: &T) -> R;
}

impl<T, R, F> StatefulFunction<T, R> for F
where
    F: FnMut(&T) -> R,
{
    fn apply(&mut self, t: &T) -> R {
        self(t)
    }
}
