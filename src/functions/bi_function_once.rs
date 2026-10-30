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

mod box_bi_function_once;
pub use box_bi_function_once::BoxBiFunctionOnce;
mod box_conditional_bi_function_once;
pub use box_conditional_bi_function_once::BoxConditionalBiFunctionOnce;
#[path = "bi_function_once/bi_function_once.rs"]
mod bi_function_once_trait;
pub use bi_function_once_trait::BiFunctionOnce;
