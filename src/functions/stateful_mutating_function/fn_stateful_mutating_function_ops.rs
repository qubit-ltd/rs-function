/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnStatefulMutatingFunctionOps` public type.

#![allow(unused_imports)]

use super::*;

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
