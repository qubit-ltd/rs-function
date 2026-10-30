// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # BiMutatingFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-mutating-function traits
//! similar to Rust's `FnOnce(&mut T, &mut U) -> R` trait, but with
//! value-oriented semantics for functional programming patterns with two
//! mutable input references.
//!
//! This module provides the `BiMutatingFunctionOnce<T, U, R>` trait and
//! one-time use implementations:
//!
//! - [`BoxBiMutatingFunctionOnce`]: Single ownership, one-time use

#[path = "bi_mutating_function_once/bi_mutating_function_once.rs"]
mod bi_mutating_function_once_trait;
pub use bi_mutating_function_once_trait::BiMutatingFunctionOnce;
mod box_bi_mutating_function_once;
pub use box_bi_mutating_function_once::BoxBiMutatingFunctionOnce;
mod box_conditional_bi_mutating_function_once;
pub use box_conditional_bi_mutating_function_once::BoxConditionalBiMutatingFunctionOnce;
