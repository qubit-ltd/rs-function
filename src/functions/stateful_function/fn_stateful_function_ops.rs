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
//! Defines the `FnStatefulFunctionOps` public type.

use super::{
    BoxConditionalStatefulFunction,
    BoxStatefulFunction,
    Predicate,
    StatefulFunction,
    impl_fn_ops_trait,
};

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
