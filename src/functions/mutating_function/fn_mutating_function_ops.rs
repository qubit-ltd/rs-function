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
//! Defines the `FnMutatingFunctionOps` public type.

use super::{
    BoxConditionalMutatingFunction,
    BoxMutatingFunction,
    Function,
    Predicate,
    impl_fn_ops_trait,
};

// =======================================================================
// 7. Provide extension methods for closures
// =======================================================================

// Generates: FnFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (Fn(&mut T) -> R),
    FnMutatingFunctionOps,
    BoxMutatingFunction,
    Function, // chains a non-mutating function after this mutating function
    BoxConditionalMutatingFunction
);
