// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformers Module
//!
//! This module provides transformer-related functional programming
//! abstractions for converting values from one type to another, including
//! single-parameter transformers, bi-transformers, and their stateful
//! variants.

pub mod bi_transformer;
#[cfg(feature = "once")]
pub mod bi_transformer_once;
pub(crate) mod macros;
#[cfg(feature = "stateful")]
pub mod stateful_bi_transformer;
#[cfg(feature = "stateful")]
pub mod stateful_transformer;
pub mod transformer;
#[cfg(feature = "once")]
pub mod transformer_once;

pub use bi_transformer::ArcBiTransformer;
pub use bi_transformer::ArcBinaryOperator;
pub use bi_transformer::ArcConditionalBiTransformer;
pub use bi_transformer::BiTransformer;
pub use bi_transformer::BinaryOperator;
pub use bi_transformer::BoxBiTransformer;
pub use bi_transformer::BoxBinaryOperator;
pub use bi_transformer::BoxConditionalBiTransformer;
#[cfg(feature = "rc")]
pub use bi_transformer::RcBiTransformer;
#[cfg(feature = "rc")]
pub use bi_transformer::RcBinaryOperator;
#[cfg(feature = "rc")]
pub use bi_transformer::RcConditionalBiTransformer;
#[cfg(feature = "once")]
pub use bi_transformer_once::BiTransformerOnce;
#[cfg(feature = "once")]
pub use bi_transformer_once::BinaryOperatorOnce;
#[cfg(feature = "once")]
pub use bi_transformer_once::BoxBiTransformerOnce;
#[cfg(feature = "once")]
pub use bi_transformer_once::BoxBinaryOperatorOnce;
#[cfg(feature = "once")]
pub use bi_transformer_once::BoxConditionalBiTransformerOnce;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::ArcConditionalStatefulBiTransformer;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::ArcStatefulBiTransformer;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::ArcStatefulBinaryOperator;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::BoxConditionalStatefulBiTransformer;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::BoxStatefulBiTransformer;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::BoxStatefulBinaryOperator;
#[cfg(feature = "stateful")]
#[cfg(feature = "rc")]
pub use stateful_bi_transformer::RcConditionalStatefulBiTransformer;
#[cfg(feature = "stateful")]
#[cfg(feature = "rc")]
pub use stateful_bi_transformer::RcStatefulBiTransformer;
#[cfg(feature = "stateful")]
#[cfg(feature = "rc")]
pub use stateful_bi_transformer::RcStatefulBinaryOperator;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::StatefulBiTransformer;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::StatefulBinaryOperator;
#[cfg(feature = "stateful")]
pub use stateful_transformer::ArcConditionalStatefulTransformer;
#[cfg(feature = "stateful")]
pub use stateful_transformer::ArcStatefulTransformer;
#[cfg(feature = "stateful")]
pub use stateful_transformer::BoxConditionalStatefulTransformer;
#[cfg(feature = "stateful")]
pub use stateful_transformer::BoxStatefulTransformer;
#[cfg(feature = "stateful")]
#[cfg(feature = "rc")]
pub use stateful_transformer::RcConditionalStatefulTransformer;
#[cfg(feature = "stateful")]
#[cfg(feature = "rc")]
pub use stateful_transformer::RcStatefulTransformer;
#[cfg(feature = "stateful")]
pub use stateful_transformer::StatefulTransformer;
pub use transformer::ArcConditionalTransformer;
pub use transformer::ArcTransformer;
pub use transformer::ArcUnaryOperator;
pub use transformer::BoxConditionalTransformer;
pub use transformer::BoxTransformer;
pub use transformer::BoxUnaryOperator;
#[cfg(feature = "rc")]
pub use transformer::RcConditionalTransformer;
#[cfg(feature = "rc")]
pub use transformer::RcTransformer;
#[cfg(feature = "rc")]
pub use transformer::RcUnaryOperator;
pub use transformer::Transformer;
pub use transformer::UnaryOperator;
#[cfg(feature = "once")]
pub use transformer_once::BoxConditionalTransformerOnce;
#[cfg(feature = "once")]
pub use transformer_once::BoxTransformerOnce;
#[cfg(feature = "once")]
pub use transformer_once::BoxUnaryOperatorOnce;
#[cfg(feature = "once")]
pub use transformer_once::TransformerOnce;
#[cfg(feature = "once")]
pub use transformer_once::UnaryOperatorOnce;
