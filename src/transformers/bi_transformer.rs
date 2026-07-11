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
//! - [`RcBiTransformer`]: Single-threaded shared ownership, cloneable
use std::rc::Rc;
use std::sync::Arc;

use crate::predicates::bi_predicate::{ArcBiPredicate, BiPredicate, BoxBiPredicate, RcBiPredicate};
use crate::transformers::{
    macros::{
        impl_box_conditional_transformer, impl_box_transformer_methods,
        impl_conditional_transformer_clone, impl_conditional_transformer_debug_display,
        impl_shared_conditional_transformer, impl_shared_transformer_methods,
        impl_transformer_clone, impl_transformer_common_methods, impl_transformer_constant_method,
        impl_transformer_debug_display,
    },
    transformer::Transformer,
};

mod box_bi_transformer;
pub use box_bi_transformer::BoxBiTransformer;
mod rc_bi_transformer;
pub use rc_bi_transformer::RcBiTransformer;
mod arc_bi_transformer;
pub use arc_bi_transformer::ArcBiTransformer;
mod fn_bi_transformer_ops;
pub use fn_bi_transformer_ops::FnBiTransformerOps;
mod binary_operator;
pub use binary_operator::BinaryOperator;
mod box_binary_operator;
pub use box_binary_operator::BoxBinaryOperator;
mod arc_binary_operator;
pub use arc_binary_operator::ArcBinaryOperator;
mod rc_binary_operator;
pub use rc_binary_operator::RcBinaryOperator;
mod box_conditional_bi_transformer;
pub use box_conditional_bi_transformer::BoxConditionalBiTransformer;
mod rc_conditional_bi_transformer;
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
    fn apply(&self, first: T, second: U) -> R;
}

impl<T, U, R, F> BiTransformer<T, U, R> for F
where
    F: Fn(T, U) -> R,
{
    fn apply(&self, first: T, second: U) -> R {
        self(first, second)
    }
}
