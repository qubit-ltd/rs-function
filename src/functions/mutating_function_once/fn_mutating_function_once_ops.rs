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
//! Defines the `FnMutatingFunctionOnceOps` public type.

use super::{
    BoxConditionalMutatingFunctionOnce,
    BoxMutatingFunctionOnce,
    FunctionOnce,
    Predicate,
    impl_fn_ops_trait,
};

// =======================================================================
// 4. Provide extension methods for closures
// =======================================================================

// Generates: FnMutatingFunctionOnceOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnOnce(&mut T) -> R),
    FnMutatingFunctionOnceOps,
    BoxMutatingFunctionOnce,
    FunctionOnce,
    BoxConditionalMutatingFunctionOnce
);
