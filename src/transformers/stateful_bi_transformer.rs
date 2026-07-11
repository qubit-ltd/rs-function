// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulBiTransformer Types
//!
//! Provides Rust implementations of stateful bi-transformer traits for type
//! conversion and value transformation with two inputs. StatefulBiTransformers
//! consume two input values (taking ownership) and produce an output value.
//!
//! This module provides the `StatefulBiTransformer<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxStatefulBiTransformer`]: Single ownership, not cloneable
//! - [`ArcStatefulBiTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulBiTransformer`]: Single-threaded shared ownership, cloneable
use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::impl_closure_trait;
#[cfg(feature = "rc")]
use crate::predicates::bi_predicate::RcBiPredicate;
use crate::predicates::bi_predicate::{
    ArcBiPredicate,
    BiPredicate,
    BoxBiPredicate,
};
use crate::transformers::{
    macros::{
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
    },
    stateful_transformer::StatefulTransformer,
};

mod box_stateful_bi_transformer;
pub use box_stateful_bi_transformer::BoxStatefulBiTransformer;
#[cfg(feature = "rc")]
mod rc_stateful_bi_transformer;
#[cfg(feature = "rc")]
pub use rc_stateful_bi_transformer::RcStatefulBiTransformer;
mod arc_stateful_bi_transformer;
pub use arc_stateful_bi_transformer::ArcStatefulBiTransformer;
#[cfg(feature = "combinators")]
mod fn_stateful_bi_transformer_ops;
#[cfg(feature = "combinators")]
pub use fn_stateful_bi_transformer_ops::FnStatefulBiTransformerOps;
mod stateful_binary_operator;
pub use stateful_binary_operator::StatefulBinaryOperator;
mod box_stateful_binary_operator;
pub use box_stateful_binary_operator::BoxStatefulBinaryOperator;
mod arc_stateful_binary_operator;
pub use arc_stateful_binary_operator::ArcStatefulBinaryOperator;
#[cfg(feature = "rc")]
mod rc_stateful_binary_operator;
#[cfg(feature = "rc")]
pub use rc_stateful_binary_operator::RcStatefulBinaryOperator;
mod box_conditional_stateful_bi_transformer;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_stateful_bi_transformer::BoxConditionalStatefulBiTransformer;
#[cfg(feature = "combinators")]
pub use box_conditional_stateful_bi_transformer::BoxConditionalStatefulBiTransformer;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_bi_transformer;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_stateful_bi_transformer::RcConditionalStatefulBiTransformer;
#[cfg(all(feature = "rc", feature = "combinators"))]
pub use rc_conditional_stateful_bi_transformer::RcConditionalStatefulBiTransformer;
mod arc_conditional_stateful_bi_transformer;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_stateful_bi_transformer::ArcConditionalStatefulBiTransformer;
#[cfg(feature = "combinators")]
pub use arc_conditional_stateful_bi_transformer::ArcConditionalStatefulBiTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulBiTransformer trait - transforms two values to produce a result
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
pub trait StatefulBiTransformer<T, U, R> {
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
    fn apply(&mut self, first: T, second: U) -> R;
}
