/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `FnTransformerOps` public type.

use super::{
    BoxConditionalTransformer,
    BoxTransformer,
    Predicate,
    Transformer,
};
use crate::transformers::macros::impl_transformer_fn_ops_trait;

// ============================================================================
// FnTransformerOps - Extension trait for closure transformers
// ============================================================================

impl_transformer_fn_ops_trait!(
    (Fn(T) -> R),
    FnTransformerOps,
    BoxTransformer,
    Transformer,
    BoxConditionalTransformer
);
