/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulMutatingFunctionOps;

#[test]
fn test_fn_stateful_mutating_function_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulMutatingFunctionOps<i32, i32>>(_: &F) {}

    let mut calls = 0;
    let mut function = |value: &mut i32| {
        calls += 1;
        *value += calls;
        *value
    };
    assert_ops(&function);
    let mut value = 41;
    assert_eq!(function(&mut value), 42);
}
