/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnMutatingFunctionOnceOps` public type.

#![allow(unused_imports)]

use super::*;

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
