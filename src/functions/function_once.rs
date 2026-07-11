// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # FunctionOnce Types
//!
//! Provides Rust implementations of consuming function traits similar to
//! Rust's `FnOnce(&T) -> R` trait, for computing output from input references.
//!
//! This module provides the `FunctionOnce<T, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxFunctionOnce`]: Single ownership, one-time use
use crate::functions::macros::{
    impl_box_conditional_function, impl_box_function_methods,
    impl_conditional_function_debug_display, impl_fn_ops_trait, impl_function_common_methods,
    impl_function_constant_method, impl_function_debug_display, impl_function_identity_method,
};
use crate::macros::{impl_box_once_conversions, impl_closure_once_trait};
use crate::predicates::predicate::{BoxPredicate, Predicate};

mod box_function_once;
pub use box_function_once::BoxFunctionOnce;
mod box_conditional_function_once;
pub use box_conditional_function_once::BoxConditionalFunctionOnce;
mod fn_function_once_ops;
pub use fn_function_once_ops::FnFunctionOnceOps;

// ============================================================================
// Core Trait
// ============================================================================

/// FunctionOnce trait - consuming function that takes ownership
///
/// Defines the behavior of a consuming function: computing a value of
/// type `R` from a reference to type `T` by taking ownership of self.
/// This trait is analogous to `FnOnce(&T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
pub trait FunctionOnce<T, R> {
    /// Applies the function to the input reference, consuming self
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, t: &T) -> R;
}
