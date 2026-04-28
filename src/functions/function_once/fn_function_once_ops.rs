/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
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
