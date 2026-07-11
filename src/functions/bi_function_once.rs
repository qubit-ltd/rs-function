// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # BiFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-function traits similar to
//! Rust's `FnOnce(&T, &U) -> R` trait, but with value-oriented semantics for
//! functional programming patterns with two input references.
//!
//! This module provides the `BiFunctionOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiFunctionOnce`]: Single ownership, one-time use
use crate::macros::impl_closure_once_trait;
use crate::predicates::bi_predicate::{
    BiPredicate,
    BoxBiPredicate,
};
use crate::{
    functions::function_once::FunctionOnce,
    functions::macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_debug_display,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
    },
};

mod box_bi_function_once;
pub use box_bi_function_once::BoxBiFunctionOnce;
#[cfg(feature = "combinators")]
mod fn_bi_function_once_ops;
#[cfg(feature = "combinators")]
pub use fn_bi_function_once_ops::FnBiFunctionOnceOps;
mod box_conditional_bi_function_once;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_bi_function_once::BoxConditionalBiFunctionOnce;
#[cfg(feature = "combinators")]
pub use box_conditional_bi_function_once::BoxConditionalBiFunctionOnce;

// ============================================================================
// Core Trait
// ============================================================================

/// BiFunctionOnce trait - consuming bi-function that takes references
///
/// Defines the behavior of a consuming bi-function: computing a value of
/// type `R` from references to types `T` and `U` by taking ownership of self.
/// This trait is analogous to `FnOnce(&T, &U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (borrowed)
/// * `U` - The type of the second input value (borrowed)
/// * `R` - The type of the output value
pub trait BiFunctionOnce<T, U, R> {
    /// Computes output from two input references, consuming self
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, first: &T, second: &U) -> R;
}
