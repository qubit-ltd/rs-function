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

mod box_function_once;
pub use box_function_once::BoxFunctionOnce;
mod box_conditional_function_once;
pub use box_conditional_function_once::BoxConditionalFunctionOnce;
#[path = "function_once/function_once.rs"]
mod function_once_trait;
pub use function_once_trait::FunctionOnce;
