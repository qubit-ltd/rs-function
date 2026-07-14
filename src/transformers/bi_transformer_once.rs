// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiTransformerOnce Types
//!
//! Provides Rust implementations of consuming bi-transformer traits similar to
//! Rust's `FnOnce` trait, but with value-oriented semantics for functional
//! programming patterns with two inputs.
//!
//! This module provides the `BiTransformerOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiTransformerOnce`]: Single ownership, one-time use

mod box_bi_transformer_once;
pub use box_bi_transformer_once::BoxBiTransformerOnce;
mod binary_operator_once;
pub use binary_operator_once::BinaryOperatorOnce;
mod box_binary_operator_once;
pub use box_binary_operator_once::BoxBinaryOperatorOnce;
mod box_conditional_bi_transformer_once;
pub use box_conditional_bi_transformer_once::BoxConditionalBiTransformerOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// BiTransformerOnce trait - consuming bi-transformation that takes ownership
///
/// Defines the behavior of a consuming bi-transformer: converting two values of
/// types `T` and `U` to a value of type `R` by taking ownership of self and
/// both inputs. This trait is analogous to `FnOnce(T, U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (consumed)
/// * `U` - The type of the second input value (consumed)
/// * `R` - The type of the output value
pub trait BiTransformerOnce<T, U, R> {
    /// Transforms two input values, consuming self and both inputs
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value (consumed)
    /// * `second` - The second input value (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(self, first: T, second: U) -> R;
}
