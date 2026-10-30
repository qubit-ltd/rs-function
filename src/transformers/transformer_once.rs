// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # TransformerOnce Types
//!
//! Provides Rust implementations of consuming transformer traits similar to
//! Rust's `FnOnce` trait, but with value-oriented semantics for functional
//! programming patterns.
//!
//! This module provides the `TransformerOnce<T, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxTransformerOnce`]: Single ownership, one-time use

mod box_transformer_once;
pub use box_transformer_once::BoxTransformerOnce;
mod unary_operator_once;
pub use unary_operator_once::UnaryOperatorOnce;
mod box_unary_operator_once;
pub use box_unary_operator_once::BoxUnaryOperatorOnce;
mod box_conditional_transformer_once;
pub use box_conditional_transformer_once::BoxConditionalTransformerOnce;
#[path = "transformer_once/transformer_once.rs"]
mod transformer_once_trait;
pub use transformer_once_trait::TransformerOnce;
