// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiTransformer Types
//!
//! Provides Rust implementations of bi-transformer traits for type conversion
//! and value transformation with two inputs. BiTransformers consume two input
//! values (taking ownership) and produce an output value.
//!
//! This module provides the `BiTransformer<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiTransformer`]: Single ownership, not cloneable
//! - [`ArcBiTransformer`]: Thread-safe shared ownership, cloneable
//! - `RcBiTransformer`: Single-threaded shared ownership, cloneable

mod box_bi_transformer;
pub use box_bi_transformer::BoxBiTransformer;
#[path = "bi_transformer/bi_transformer.rs"]
mod bi_transformer_trait;
pub use bi_transformer_trait::BiTransformer;
#[cfg(feature = "rc")]
mod rc_bi_transformer;
#[cfg(feature = "rc")]
pub use rc_bi_transformer::RcBiTransformer;
mod arc_bi_transformer;
pub use arc_bi_transformer::ArcBiTransformer;
mod binary_operator;
pub use binary_operator::BinaryOperator;
mod box_binary_operator;
pub use box_binary_operator::BoxBinaryOperator;
mod arc_binary_operator;
pub use arc_binary_operator::ArcBinaryOperator;
#[cfg(feature = "rc")]
mod rc_binary_operator;
#[cfg(feature = "rc")]
pub use rc_binary_operator::RcBinaryOperator;
mod box_conditional_bi_transformer;
pub use box_conditional_bi_transformer::BoxConditionalBiTransformer;
#[cfg(feature = "rc")]
mod rc_conditional_bi_transformer;
#[cfg(feature = "rc")]
pub use rc_conditional_bi_transformer::RcConditionalBiTransformer;
mod arc_conditional_bi_transformer;
pub use arc_conditional_bi_transformer::ArcConditionalBiTransformer;
