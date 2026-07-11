// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformer Types
//!
//! Provides Rust implementations of transformer traits for type conversion
//! and value transformation. Transformers consume input values (taking
//! ownership) and produce output values. This is analogous to
//! `Fn(T) -> R` in Rust's standard library.
//!
//! This module provides the `Transformer<T, R>` trait and three
//! implementations:
//!
//! - [`BoxTransformer`]: Single ownership, not cloneable
//! - [`ArcTransformer`]: Thread-safe shared ownership, cloneable
//! - `RcTransformer`: Single-threaded shared ownership, cloneable
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use crate::macros::impl_closure_trait;
#[cfg(feature = "rc")]
use crate::predicates::predicate::RcPredicate;
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
};
use crate::transformers::macros::{
    impl_box_conditional_transformer,
    impl_box_transformer_methods,
    impl_conditional_transformer_clone,
    impl_conditional_transformer_debug_display,
    impl_shared_conditional_transformer,
    impl_shared_transformer_methods,
    impl_transformer_clone,
    impl_transformer_common_methods,
    impl_transformer_constant_method,
    impl_transformer_debug_display,
};

mod box_transformer;
pub use box_transformer::BoxTransformer;
#[cfg(feature = "rc")]
mod rc_transformer;
#[cfg(feature = "rc")]
pub use rc_transformer::RcTransformer;
mod arc_transformer;
pub use arc_transformer::ArcTransformer;
#[cfg(feature = "combinators")]
mod fn_transformer_ops;
#[cfg(feature = "combinators")]
pub use fn_transformer_ops::FnTransformerOps;
mod unary_operator;
pub use unary_operator::UnaryOperator;
mod box_unary_operator;
pub use box_unary_operator::BoxUnaryOperator;
mod arc_unary_operator;
pub use arc_unary_operator::ArcUnaryOperator;
#[cfg(feature = "rc")]
mod rc_unary_operator;
#[cfg(feature = "rc")]
pub use rc_unary_operator::RcUnaryOperator;
mod box_conditional_transformer;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_transformer::BoxConditionalTransformer;
#[cfg(feature = "combinators")]
pub use box_conditional_transformer::BoxConditionalTransformer;
#[cfg(feature = "rc")]
mod rc_conditional_transformer;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_transformer::RcConditionalTransformer;
#[cfg(all(feature = "rc", feature = "combinators"))]
pub use rc_conditional_transformer::RcConditionalTransformer;
mod arc_conditional_transformer;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_transformer::ArcConditionalTransformer;
#[cfg(feature = "combinators")]
pub use arc_conditional_transformer::ArcConditionalTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// Transformer trait - transforms values from type T to type R
///
/// Defines the behavior of a transformation: converting a value of type `T`
/// to a value of type `R` by consuming the input. This is analogous to
/// `Fn(T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
pub trait Transformer<T, R> {
    /// Applies the transformation to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&self, input: T) -> R;
}
