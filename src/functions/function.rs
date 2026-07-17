// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Function Types
//!
//! Provides Rust implementations of function traits for computing output values
//! from input references. Functions borrow input values (not consuming them)
//! and produce output values.
//!
//! It is similar to the `Fn(&T) -> R` trait in the standard library.
//!
//! This module provides the `Function<T, R>` trait and three
//! implementations:
//!
//! - [`BoxFunction`]: Single ownership, not cloneable
//! - [`ArcFunction`]: Thread-safe shared ownership, cloneable
//! - `RcFunction`: Single-threaded shared ownership, cloneable

mod box_function;
pub use box_function::BoxFunction;
#[cfg(feature = "rc")]
mod rc_function;
#[cfg(feature = "rc")]
pub use rc_function::RcFunction;
mod arc_function;
pub use arc_function::ArcFunction;
mod box_conditional_function;
pub use box_conditional_function::BoxConditionalFunction;
#[cfg(feature = "rc")]
mod rc_conditional_function;
#[cfg(feature = "rc")]
pub use rc_conditional_function::RcConditionalFunction;
mod arc_conditional_function;
pub use arc_conditional_function::ArcConditionalFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// Function trait - computes output from input reference
///
/// Defines the behavior of a function: computing a value of type `R`
/// from a reference to type `T` without consuming the input. This is analogous
/// to `Fn(&T) -> R` in Rust's standard library, similar to Java's `Function<T,
/// R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
pub trait Function<T, R> {
    /// Applies the function to the input reference to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    #[must_use = "the computed callback result should be used"]
    fn apply(&self, t: &T) -> R;
}

crate::macros::impl_closure_trait!(
    Function<T, R>,
    apply,
    Fn(input: &T) -> R
);
