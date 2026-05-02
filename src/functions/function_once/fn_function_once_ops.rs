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
//! Defines the `FnFunctionOnceOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnFunctionOnceOps - Extension trait for FnOnce transformers
// ============================================================================

// Generates: FnFunctionOnceOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnOnce(&T) -> R),
    FnFunctionOnceOps,
    BoxFunctionOnce,
    FunctionOnce,
    BoxConditionalFunctionOnce
);
