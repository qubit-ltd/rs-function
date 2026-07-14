// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # BiFunction Types
//!
//! Provides Rust implementations of bi-function traits for computing output
//! values from two input references. BiFunctions borrow input values (not
//! consuming them) and produce output values.
//!
//! It is similar to the `Fn(&T, &U) -> R` trait in the standard library.
//!
//! This module provides the `BiFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiFunction`]: Single ownership, not cloneable
//! - [`ArcBiFunction`]: Thread-safe shared ownership, cloneable
//! - `RcBiFunction`: Single-threaded shared ownership, cloneable

mod box_bi_function;
pub use box_bi_function::BoxBiFunction;
#[cfg(feature = "rc")]
mod rc_bi_function;
#[cfg(feature = "rc")]
pub use rc_bi_function::RcBiFunction;
mod arc_bi_function;
pub use arc_bi_function::ArcBiFunction;
mod box_binary_function;
pub use box_binary_function::BoxBinaryFunction;
mod arc_binary_function;
pub use arc_binary_function::ArcBinaryFunction;
#[cfg(feature = "rc")]
mod rc_binary_function;
#[cfg(feature = "rc")]
pub use rc_binary_function::RcBinaryFunction;
mod box_conditional_bi_function;
pub use box_conditional_bi_function::BoxConditionalBiFunction;
#[cfg(feature = "rc")]
mod rc_conditional_bi_function;
#[cfg(feature = "rc")]
pub use rc_conditional_bi_function::RcConditionalBiFunction;
mod arc_conditional_bi_function;
pub use arc_conditional_bi_function::ArcConditionalBiFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// BiFunction trait - computes output from two input references
///
/// Defines the behavior of a bi-function: computing a value of type `R`
/// from references to types `T` and `U` without consuming the inputs. This is
/// analogous to `Fn(&T, &U) -> R` in Rust's standard library, similar to Java's
/// `BiFunction<T, U, R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (borrowed)
/// * `U` - The type of the second input value (borrowed)
/// * `R` - The type of the output value
pub trait BiFunction<T, U, R> {
    /// Applies the bi-function to two input references to produce an output
    /// value
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &T, second: &U) -> R;
}

crate::macros::impl_closure_trait!(
    BiFunction<T, U, R>,
    apply,
    Fn(first: &T, second: &U) -> R
);
