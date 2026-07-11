// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulTransformer Types
//!
//! Provides Rust implementations of stateful transformer traits for stateful
//! value transformation. StatefulTransformers consume input values (taking
//! ownership) and produce output values while allowing internal state
//! modification. This is analogous to `FnMut(T) -> R` in Rust's standard
//! library.
//!
//! This module provides the `StatefulTransformer<T, R>` trait and three
//! implementations:
//!
//! - [`BoxStatefulTransformer`]: Single ownership, not cloneable
//! - [`ArcStatefulTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulTransformer`]: Single-threaded shared ownership, cloneable
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::{ impl_closure_trait,
};
use crate::predicates::predicate::{ArcPredicate, BoxPredicate, Predicate, RcPredicate};
use crate::transformers::macros::{
        impl_box_conditional_transformer, impl_box_transformer_methods,
        impl_conditional_transformer_clone, impl_conditional_transformer_debug_display,
        impl_shared_conditional_transformer, impl_shared_transformer_methods,
        impl_transformer_clone, impl_transformer_common_methods, impl_transformer_constant_method,
        impl_transformer_debug_display,
    };

mod box_stateful_transformer;
pub use box_stateful_transformer::BoxStatefulTransformer;
mod rc_stateful_transformer;
pub use rc_stateful_transformer::RcStatefulTransformer;
mod arc_stateful_transformer;
pub use arc_stateful_transformer::ArcStatefulTransformer;
mod fn_stateful_transformer_ops;
pub use fn_stateful_transformer_ops::FnStatefulTransformerOps;
mod box_conditional_stateful_transformer;
pub use box_conditional_stateful_transformer::BoxConditionalStatefulTransformer;
mod rc_conditional_stateful_transformer;
pub use rc_conditional_stateful_transformer::RcConditionalStatefulTransformer;
mod arc_conditional_stateful_transformer;
pub use arc_conditional_stateful_transformer::ArcConditionalStatefulTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulTransformer trait - transforms values from type T to type R with
/// state
///
/// Defines the behavior of a stateful transformation: converting a value
/// of type `T` to a value of type `R` by consuming the input while
/// allowing modification of internal state. This is analogous to
/// `FnMut(T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
pub trait StatefulTransformer<T, R> {
    /// Applies the transformation to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&mut self, input: T) -> R;
}
