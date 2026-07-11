// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # BiMutatingFunction Types
//!
//! Provides Rust implementations of bi-mutating-function traits for performing
//! operations that accept two mutable references and return a result.
//!
//! It is similar to the `Fn(&mut T, &mut U) -> R` trait in the standard
//! library.
//!
//! This module provides the `BiMutatingFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiMutatingFunction`]: Single ownership, not cloneable
//! - [`ArcBiMutatingFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcBiMutatingFunction`]: Single-threaded shared ownership, cloneable

use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    macros::{
        impl_box_conditional_function, impl_box_function_methods, impl_conditional_function_clone,
        impl_conditional_function_debug_display, impl_function_clone, impl_function_common_methods,
        impl_function_constant_method, impl_function_debug_display,
        impl_shared_conditional_function, impl_shared_function_methods,
    },
    mutating_function::MutatingFunction,
};
use crate::macros::{ impl_closure_trait,
};
use crate::predicates::bi_predicate::{ArcBiPredicate, BiPredicate, BoxBiPredicate, RcBiPredicate};

mod box_bi_mutating_function;
pub use box_bi_mutating_function::BoxBiMutatingFunction;
mod rc_bi_mutating_function;
pub use rc_bi_mutating_function::RcBiMutatingFunction;
mod arc_bi_mutating_function;
pub use arc_bi_mutating_function::ArcBiMutatingFunction;
mod fn_bi_mutating_function_ops;
pub use fn_bi_mutating_function_ops::FnBiMutatingFunctionOps;
mod box_binary_mutating_function;
pub use box_binary_mutating_function::BoxBinaryMutatingFunction;
mod arc_binary_mutating_function;
pub use arc_binary_mutating_function::ArcBinaryMutatingFunction;
mod rc_binary_mutating_function;
pub use rc_binary_mutating_function::RcBinaryMutatingFunction;
mod box_conditional_bi_mutating_function;
pub use box_conditional_bi_mutating_function::BoxConditionalBiMutatingFunction;
mod rc_conditional_bi_mutating_function;
pub use rc_conditional_bi_mutating_function::RcConditionalBiMutatingFunction;
mod arc_conditional_bi_mutating_function;
pub use arc_conditional_bi_mutating_function::ArcConditionalBiMutatingFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// BiMutatingFunction trait - performs operations on two mutable references
///
/// Defines the behavior of a bi-mutating-function: computing a value of type
/// `R` from mutable references to types `T` and `U`, potentially modifying both
/// inputs. This is analogous to `Fn(&mut T, &mut U) -> R` in Rust's standard
/// library.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (mutable reference)
/// * `U` - The type of the second input value (mutable reference)
/// * `R` - The type of the output value
pub trait BiMutatingFunction<T, U, R> {
    /// Applies the bi-mutating-function to two mutable references and returns a
    /// result
    ///
    /// # Parameters
    ///
    /// * `first` - Mutable reference to the first input value
    /// * `second` - Mutable reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &mut T, second: &mut U) -> R;
}
