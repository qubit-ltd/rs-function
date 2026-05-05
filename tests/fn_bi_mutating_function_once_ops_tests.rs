/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnBiMutatingFunctionOnceOps;

#[test]
fn test_fn_bi_mutating_function_once_ops_observable_behavior() {
    fn assert_ops<F: FnBiMutatingFunctionOnceOps<i32, i32, i32>>(_: &F) {}

    let function = |left: &mut i32, right: &mut i32| {
        *left += 1;
        *right += 1;
        *left + *right
    };
    assert_ops(&function);
    let mut left = 20;
    let mut right = 20;
    assert_eq!(function(&mut left, &mut right), 42);
}
