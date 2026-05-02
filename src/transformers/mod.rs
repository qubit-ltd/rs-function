/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Transformers Module
//!
//! This module provides transformer-related functional programming
//! abstractions for converting values from one type to another, including
//! single-parameter transformers, bi-transformers, and their stateful
//! variants.
//!

pub mod bi_transformer;
pub mod bi_transformer_once;
#[doc(hidden)]
pub mod macros;
pub mod stateful_bi_transformer;
pub mod stateful_transformer;
pub mod transformer;
pub mod transformer_once;

pub use bi_transformer::{
    ArcBiTransformer,
    ArcBinaryOperator,
    ArcConditionalBiTransformer,
    BiTransformer,
    BinaryOperator,
    BoxBiTransformer,
    BoxBinaryOperator,
    BoxConditionalBiTransformer,
    FnBiTransformerOps,
    RcBiTransformer,
    RcBinaryOperator,
    RcConditionalBiTransformer,
};
pub use bi_transformer_once::{
    BiTransformerOnce,
    BinaryOperatorOnce,
    BoxBiTransformerOnce,
    BoxBinaryOperatorOnce,
    BoxConditionalBiTransformerOnce,
    FnBiTransformerOnceOps,
};
pub use stateful_bi_transformer::{
    ArcConditionalStatefulBiTransformer,
    ArcStatefulBiTransformer,
    BoxConditionalStatefulBiTransformer,
    BoxStatefulBiTransformer,
    FnStatefulBiTransformerOps,
    RcConditionalStatefulBiTransformer,
    RcStatefulBiTransformer,
    StatefulBiTransformer,
};
pub use stateful_bi_transformer::{
    ArcStatefulBinaryOperator,
    BoxStatefulBinaryOperator,
    RcStatefulBinaryOperator,
    StatefulBinaryOperator,
};
pub use stateful_transformer::{
    ArcConditionalStatefulTransformer,
    ArcStatefulTransformer,
    BoxConditionalStatefulTransformer,
    BoxStatefulTransformer,
    FnStatefulTransformerOps,
    RcConditionalStatefulTransformer,
    RcStatefulTransformer,
    StatefulTransformer,
};
pub use transformer::{
    ArcConditionalTransformer,
    ArcTransformer,
    ArcUnaryOperator,
    BoxConditionalTransformer,
    BoxTransformer,
    BoxUnaryOperator,
    FnTransformerOps,
    RcConditionalTransformer,
    RcTransformer,
    RcUnaryOperator,
    Transformer,
    UnaryOperator,
};
pub use transformer_once::{
    BoxConditionalTransformerOnce,
    BoxTransformerOnce,
    BoxUnaryOperatorOnce,
    FnTransformerOnceOps,
    TransformerOnce,
    UnaryOperatorOnce,
};
