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
//! Defines the `FnStatefulMutatingFunctionOps` public type.

use super::{
    BoxConditionalStatefulMutatingFunction,
    BoxStatefulMutatingFunction,
    Predicate,
    StatefulMutatingFunction,
    impl_fn_ops_trait,
};

// =======================================================================
// 7. Provide extension methods for closures
// =======================================================================

// Generates: FnMutStatefulMutatingFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnMut(&mut T) -> R),
    FnStatefulMutatingFunctionOps,
    BoxStatefulMutatingFunction,
    StatefulMutatingFunction,
    BoxConditionalStatefulMutatingFunction
);
