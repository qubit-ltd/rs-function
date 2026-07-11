// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # BiFunction Types
//!
//! Provides Rust implementations of bi-function traits for computing output
//! values from two input references. BiFunctions borrow input values (not
//! consuming them) and produce output values.
//!
//! It is similar to the `Fn(&T, &U) -> R` trait in the standard library.
//!
//! This module provides the `BiFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiFunction`]: Single ownership, not cloneable
//! - [`ArcBiFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcBiFunction`]: Single-threaded shared ownership, cloneable
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    function::Function,
    macros::{
        impl_box_conditional_function, impl_box_function_methods, impl_conditional_function_clone,
        impl_conditional_function_debug_display, impl_function_clone, impl_function_common_methods,
        impl_function_constant_method, impl_function_debug_display,
        impl_shared_conditional_function, impl_shared_function_methods,
    },
};
use crate::macros::{ impl_closure_trait,
};
use crate::predicates::bi_predicate::{ArcBiPredicate, BiPredicate, BoxBiPredicate};
#[cfg(feature = "rc")]
use crate::predicates::bi_predicate::RcBiPredicate;

mod box_bi_function;
pub use box_bi_function::BoxBiFunction;
#[cfg(feature = "rc")]
mod rc_bi_function;
#[cfg(feature = "rc")]
pub use rc_bi_function::RcBiFunction;
mod arc_bi_function;
pub use arc_bi_function::ArcBiFunction;
#[cfg(feature = "combinators")]
mod fn_bi_function_ops;
#[cfg(feature = "combinators")]
pub use fn_bi_function_ops::FnBiFunctionOps;
mod box_binary_function;
pub use box_binary_function::BoxBinaryFunction;
mod arc_binary_function;
pub use arc_binary_function::ArcBinaryFunction;
#[cfg(feature = "rc")]
mod rc_binary_function;
#[cfg(feature = "rc")]
pub use rc_binary_function::RcBinaryFunction;
mod box_conditional_bi_function;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_bi_function::BoxConditionalBiFunction;
#[cfg(feature = "combinators")]
pub use box_conditional_bi_function::BoxConditionalBiFunction;
#[cfg(feature = "rc")]
mod rc_conditional_bi_function;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_bi_function::RcConditionalBiFunction;
#[cfg(all(feature = "rc", feature = "combinators"))]
pub use rc_conditional_bi_function::RcConditionalBiFunction;
mod arc_conditional_bi_function;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_bi_function::ArcConditionalBiFunction;
#[cfg(feature = "combinators")]
pub use arc_conditional_bi_function::ArcConditionalBiFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// BiFunction trait - computes output from two input references
///
/// Defines the behavior of a bi-function: computing a value of type `R`
/// from references to types `T` and `U` without consuming the inputs. This is
/// analogous to `Fn(&T, &U) -> R` in Rust's standard library, similar to Java's
/// `BiFunction<T, U, R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (borrowed)
/// * `U` - The type of the second input value (borrowed)
/// * `R` - The type of the output value
pub trait BiFunction<T, U, R> {
    /// Applies the bi-function to two input references to produce an output
    /// value
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &T, second: &U) -> R;
}
