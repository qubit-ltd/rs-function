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
//! Defines the `FnFunctionOps` public type.

use super::{
    BoxConditionalFunction,
    BoxFunction,
    Function,
    Predicate,
    impl_fn_ops_trait,
};

// ============================================================================
// FnFunctionOps - Extension trait for closure functions
// ============================================================================

// Generates: FnFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (Fn(&T) -> R),
    FnFunctionOps,
    BoxFunction,
    Function,
    BoxConditionalFunction
);
