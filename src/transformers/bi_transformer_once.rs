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
use crate::macros::impl_closure_once_trait;
use crate::predicates::bi_predicate::{
    BiPredicate,
    BoxBiPredicate,
};
use crate::transformers::{
    macros::{
        impl_box_conditional_transformer,
        impl_box_transformer_methods,
        impl_conditional_transformer_debug_display,
        impl_transformer_common_methods,
        impl_transformer_constant_method,
        impl_transformer_debug_display,
    },
    transformer_once::TransformerOnce,
};

mod box_bi_transformer_once;
pub use box_bi_transformer_once::BoxBiTransformerOnce;
#[cfg(feature = "combinators")]
mod fn_bi_transformer_once_ops;
#[cfg(feature = "combinators")]
pub use fn_bi_transformer_once_ops::FnBiTransformerOnceOps;
mod binary_operator_once;
pub use binary_operator_once::BinaryOperatorOnce;
mod box_binary_operator_once;
pub use box_binary_operator_once::BoxBinaryOperatorOnce;
mod box_conditional_bi_transformer_once;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_bi_transformer_once::BoxConditionalBiTransformerOnce;
#[cfg(feature = "combinators")]
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
