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
//! - `RcBiMutatingFunction`: Single-threaded shared ownership, cloneable

mod box_bi_mutating_function;
pub use box_bi_mutating_function::BoxBiMutatingFunction;
#[path = "bi_mutating_function/bi_mutating_function.rs"]
mod bi_mutating_function_trait;
pub use bi_mutating_function_trait::BiMutatingFunction;
#[cfg(feature = "rc")]
mod rc_bi_mutating_function;
#[cfg(feature = "rc")]
pub use rc_bi_mutating_function::RcBiMutatingFunction;
mod arc_bi_mutating_function;
pub use arc_bi_mutating_function::ArcBiMutatingFunction;
mod box_binary_mutating_function;
pub use box_binary_mutating_function::BoxBinaryMutatingFunction;
mod arc_binary_mutating_function;
pub use arc_binary_mutating_function::ArcBinaryMutatingFunction;
#[cfg(feature = "rc")]
mod rc_binary_mutating_function;
#[cfg(feature = "rc")]
pub use rc_binary_mutating_function::RcBinaryMutatingFunction;
mod box_conditional_bi_mutating_function;
pub use box_conditional_bi_mutating_function::BoxConditionalBiMutatingFunction;
#[cfg(feature = "rc")]
mod rc_conditional_bi_mutating_function;
#[cfg(feature = "rc")]
pub use rc_conditional_bi_mutating_function::RcConditionalBiMutatingFunction;
mod arc_conditional_bi_mutating_function;
pub use arc_conditional_bi_mutating_function::ArcConditionalBiMutatingFunction;
