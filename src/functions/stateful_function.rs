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
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::functions::macros::{
        impl_box_conditional_function, impl_box_function_methods, impl_conditional_function_clone,
        impl_conditional_function_debug_display, impl_fn_ops_trait, impl_function_clone,
        impl_function_common_methods, impl_function_constant_method, impl_function_debug_display,
        impl_function_identity_method, impl_shared_conditional_function,
        impl_shared_function_methods,
    };
use crate::predicates::predicate::{ArcPredicate, BoxPredicate, Predicate, RcPredicate};

mod box_stateful_function;
pub use box_stateful_function::BoxStatefulFunction;
mod rc_stateful_function;
pub use rc_stateful_function::RcStatefulFunction;
mod arc_stateful_function;
pub use arc_stateful_function::ArcStatefulFunction;
mod box_conditional_stateful_function;
pub use box_conditional_stateful_function::BoxConditionalStatefulFunction;
mod rc_conditional_stateful_function;
pub use rc_conditional_stateful_function::RcConditionalStatefulFunction;
mod arc_conditional_stateful_function;
pub use arc_conditional_stateful_function::ArcConditionalStatefulFunction;
mod fn_stateful_function_ops;
pub use fn_stateful_function_ops::FnStatefulFunctionOps;

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulFunction trait - transforms values from type T to type R with state
///
/// Defines the behavior of a stateful transformation: converting a value
/// of type `T` to a value of type `R` by consuming the input while
/// allowing modification of internal state. This is analogous to
/// `FnMut(&T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
pub trait StatefulFunction<T, R> {
    /// Applies the mapping to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - The input value to transform (consumed)
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
