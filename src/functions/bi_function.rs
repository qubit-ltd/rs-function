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
//! This module provides the `BiFunction<T, U, R>` trait and boxed, shared,
//! and conditional implementations:
//!
//! - [`BoxBiFunction`]: Single ownership, not cloneable
//! - [`ArcBiFunction`]: Thread-safe shared ownership, cloneable
//! - `RcBiFunction`: Single-threaded shared ownership, cloneable

mod box_bi_function;
pub use box_bi_function::BoxBiFunction;
#[path = "bi_function/bi_function.rs"]
mod bi_function_trait;
pub use bi_function_trait::BiFunction;
#[cfg(feature = "rc")]
mod rc_bi_function;
#[cfg(feature = "rc")]
pub use rc_bi_function::RcBiFunction;
mod arc_bi_function;
pub use arc_bi_function::ArcBiFunction;
mod box_binary_function;
pub use box_binary_function::BoxBinaryFunction;
mod arc_binary_function;
pub use arc_binary_function::ArcBinaryFunction;
#[cfg(feature = "rc")]
mod rc_binary_function;
#[cfg(feature = "rc")]
pub use rc_binary_function::RcBinaryFunction;
mod box_conditional_bi_function;
pub use box_conditional_bi_function::BoxConditionalBiFunction;
#[cfg(feature = "rc")]
mod rc_conditional_bi_function;
#[cfg(feature = "rc")]
pub use rc_conditional_bi_function::RcConditionalBiFunction;
mod arc_conditional_bi_function;
pub use arc_conditional_bi_function::ArcConditionalBiFunction;
