// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiTransformer Types
//!
//! Provides Rust implementations of bi-transformer traits for type conversion
//! and value transformation with two inputs. BiTransformers consume two input
//! values (taking ownership) and produce an output value.
//!
//! This module provides the `BiTransformer<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiTransformer`]: Single ownership, not cloneable
//! - [`ArcBiTransformer`]: Thread-safe shared ownership, cloneable
//! - `RcBiTransformer`: Single-threaded shared ownership, cloneable

mod box_bi_transformer;
pub use box_bi_transformer::BoxBiTransformer;
#[cfg(feature = "rc")]
mod rc_bi_transformer;
#[cfg(feature = "rc")]
pub use rc_bi_transformer::RcBiTransformer;
mod arc_bi_transformer;
pub use arc_bi_transformer::ArcBiTransformer;
mod binary_operator;
pub use binary_operator::BinaryOperator;
mod box_binary_operator;
pub use box_binary_operator::BoxBinaryOperator;
mod arc_binary_operator;
pub use arc_binary_operator::ArcBinaryOperator;
#[cfg(feature = "rc")]
mod rc_binary_operator;
#[cfg(feature = "rc")]
pub use rc_binary_operator::RcBinaryOperator;
mod box_conditional_bi_transformer;
pub use box_conditional_bi_transformer::BoxConditionalBiTransformer;
#[cfg(feature = "rc")]
mod rc_conditional_bi_transformer;
#[cfg(feature = "rc")]
pub use rc_conditional_bi_transformer::RcConditionalBiTransformer;
mod arc_conditional_bi_transformer;
pub use arc_conditional_bi_transformer::ArcConditionalBiTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// BiTransformer trait - transforms two values to produce a result
///
/// Defines the behavior of a bi-transformation: converting two values of types
/// `T` and `U` to a value of type `R` by consuming the inputs. This is
/// analogous to `Fn(T, U) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (consumed)
/// * `U` - The type of the second input value (consumed)
/// * `R` - The type of the output value
pub trait BiTransformer<T, U, R> {
    /// Transforms two input values to produce an output value
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value to transform (consumed)
    /// * `second` - The second input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    #[must_use = "the computed callback result should be used"]
    fn apply(&self, first: T, second: U) -> R;
}

impl<T, U, R, F> BiTransformer<T, U, R> for F
where
    F: Fn(T, U) -> R,
{
    #[inline(always)]
    fn apply(&self, first: T, second: U) -> R {
        self(first, second)
    }
}
