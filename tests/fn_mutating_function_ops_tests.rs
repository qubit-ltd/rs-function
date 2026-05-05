/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnMutatingFunctionOps;

#[test]
fn test_fn_mutating_function_ops_observable_behavior() {
    fn assert_ops<F: FnMutatingFunctionOps<i32, i32>>(_: &F) {}

    let function = |value: &mut i32| {
        *value += 1;
        *value
    };
    assert_ops(&function);
    let mut value = 41;
    assert_eq!(function(&mut value), 42);
}
