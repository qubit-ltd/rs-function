// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiTransformerOnce Types
//!
//! Provides Rust implementations of consuming bi-transformer traits similar to
//! Rust's `FnOnce` trait, but with value-oriented semantics for functional
//! programming patterns with two inputs.
//!
//! This module provides the `BiTransformerOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiTransformerOnce`]: Single ownership, one-time use

mod box_bi_transformer_once;
pub use box_bi_transformer_once::BoxBiTransformerOnce;
mod binary_operator_once;
pub use binary_operator_once::BinaryOperatorOnce;
mod box_binary_operator_once;
pub use box_binary_operator_once::BoxBinaryOperatorOnce;
mod box_conditional_bi_transformer_once;
pub use box_conditional_bi_transformer_once::BoxConditionalBiTransformerOnce;
#[path = "bi_transformer_once/bi_transformer_once.rs"]
mod bi_transformer_once_trait;
pub use bi_transformer_once_trait::BiTransformerOnce;
