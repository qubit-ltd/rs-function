/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnStatefulFunctionOps` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// FnStatefulFunctionOps - Extension trait for closure functions
// ============================================================================

// Generates: FnStatefulFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnMut(&T) -> R),
    FnStatefulFunctionOps,
    BoxStatefulFunction,
    StatefulFunction,
    BoxConditionalStatefulFunction
);
