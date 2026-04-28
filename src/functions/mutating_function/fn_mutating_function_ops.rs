/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnMutatingFunctionOps` public type.

#![allow(unused_imports)]

use super::*;

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
